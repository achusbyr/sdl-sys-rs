#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "args")]
use alloc::ffi::CString;
#[cfg(feature = "alloc")]
use alloc::format;
use core::ffi::{c_char, c_int, c_void};
use sdl_sys_bindgen::*;

/// Idiomatic Rust trait for SDL's callback-based application loop.
///
/// # Thread Safety
///
/// `Send` is required because the app state is passed across the FFI boundary
/// and SDL may invoke callbacks from a thread it manages internally.
pub trait SdlApp: Sized + Send {
    /// Custom error type returned on initialization failure.
    type Error: core::fmt::Debug;

    /// Called once at startup. Return `Ok(Self)` to continue, or `Err` to terminate.
    ///
    /// # Memory Management
    ///
    /// The returned `Self` instance is moved onto the heap and its ownership
    /// is handed to SDL. It will be passed back to `iterate`, `event`, and `quit`.
    #[cfg(feature = "args")]
    fn init(args: &[CString]) -> Result<Self, Self::Error>;
    /// Called once at startup. Return `Ok(Self)` to continue, or `Err` to terminate.
    ///
    /// # Memory Management
    ///
    /// The returned `Self` instance is moved onto the heap and its ownership
    /// is handed to SDL. It will be passed back to `iterate`, `event`, and `quit`.
    #[cfg(not(feature = "args"))]
    fn init() -> Result<Self, Self::Error>;

    /// Called repeatedly to process a single frame.
    fn iterate(&mut self) -> SDL_AppResult;

    /// Called when an event arrives.
    fn event(&mut self, event: &SDL_Event) -> SDL_AppResult;

    /// Called before the app exits.
    ///
    /// After this method returns, the app state is automatically dropped and freed.
    fn quit(&mut self, result: SDL_AppResult);
}

/// Starts the SDL application loop.
pub fn run_app<A: SdlApp>() -> i32 {
    // 1. The Init Trampoline
    extern "C" fn c_init<A: SdlApp>(
        appstate: *mut *mut c_void,
        #[allow(unused_variables)] argc: c_int,
        #[allow(unused_variables)] argv: *mut *mut c_char,
    ) -> SDL_AppResult {
        let init_result = {
            #[cfg(feature = "args")]
            {
                let args = unsafe {
                    let argc = argc as usize;
                    std::vec::Vec::from_raw_parts(argv, argc, argc)
                        .iter()
                        .map(|arg| CString::from_raw(*arg))
                        .collect::<std::vec::Vec<CString>>()
                };
                A::init(args.as_slice())
            }
            #[cfg(not(feature = "args"))]
            {
                A::init()
            }
        };

        match init_result {
            Ok(app) => {
                unsafe {
                    let ptr = SDL_malloc(core::mem::size_of::<A>());
                    if ptr.is_null() {
                        log_error(SDL_GetError());
                        return SDL_AppResult::SDL_APP_FAILURE;
                    }
                    let state_ptr = ptr as *mut A;
                    core::ptr::write(state_ptr, app);
                    *appstate = ptr;
                }
                SDL_AppResult::SDL_APP_CONTINUE
            }
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "alloc")]
                let err_msg = format!("{:?}\0", e);
                #[cfg(not(feature = "alloc"))]
                let err_msg = c"SDL app initialization failed";
                log_error(err_msg.as_ptr() as *const c_char);
                SDL_AppResult::SDL_APP_FAILURE
            }
        }
    }

    // 2. The Iterate Trampoline
    extern "C" fn c_iter<A: SdlApp>(appstate: *mut c_void) -> SDL_AppResult {
        if appstate.is_null() {
            return SDL_AppResult::SDL_APP_FAILURE;
        }
        let app = unsafe { &mut *(appstate as *mut A) };
        app.iterate()
    }

    // 3. The Event Trampoline
    extern "C" fn c_event<A: SdlApp>(
        appstate: *mut c_void,
        event: *mut SDL_Event,
    ) -> SDL_AppResult {
        if appstate.is_null() || event.is_null() {
            return SDL_AppResult::SDL_APP_FAILURE;
        }
        let app = unsafe { &mut *(appstate as *mut A) };
        app.event(unsafe { &*event })
    }

    // 4. The Quit Trampoline
    extern "C" fn c_quit<A: SdlApp>(appstate: *mut c_void, result: SDL_AppResult) {
        if !appstate.is_null() {
            unsafe {
                let mut app = core::ptr::read(appstate as *mut A);
                app.quit(result);
                SDL_free(appstate);
            }
        }
    }

    extern "C" fn enter_callbacks<A: SdlApp>(argc: c_int, argv: *mut *mut c_char) -> i32 {
        unsafe {
            SDL_EnterAppMainCallbacks(
                argc,
                argv,
                Some(c_init::<A>),
                Some(c_iter::<A>),
                Some(c_event::<A>),
                Some(c_quit::<A>),
            )
        }
    }

    unsafe {
        #[cfg(feature = "args")]
        let arg_count = std::env::args().count();
        SDL_RunApp(
            {
                #[cfg(feature = "args")]
                {
                    arg_count as c_int
                }
                #[cfg(not(feature = "args"))]
                {
                    0
                }
            },
            {
                #[cfg(feature = "args")]
                {
                    let mut vec: std::vec::Vec<*mut c_char> =
                        std::vec::Vec::with_capacity(arg_count);
                    for arg in std::env::args() {
                        let c_string = CString::new(arg);
                        if let Ok(c_string) = c_string {
                            vec.push(c_string.into_raw());
                        }
                    }
                    vec.into_raw_parts().0
                }
                #[cfg(not(feature = "args"))]
                {
                    core::ptr::null_mut()
                }
            },
            Some(enter_callbacks::<A>),
            core::ptr::null_mut(), // reserved
        )
    }
}

fn log_error(msg: *const c_char) {
    unsafe {
        SDL_LogError(
            SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION.0 as c_int,
            msg,
        );
    }
}

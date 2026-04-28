//! # SDL Main Wrapper
//!
//! This crate provides a high-level, idiomatic Rust wrapper around SDL3's callback-based
//! application loop (`SDL_EnterAppMainCallbacks`).
//!
//! It allows you to define your application state as a Rust struct implementing the [`SdlApp`]
//! trait, handling the low-level FFI trampolines and memory management for you.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "args")]
use alloc::{ffi::CString, vec::Vec};
#[cfg(feature = "args")]
use core::ffi::CStr;
use core::ffi::{c_char, c_int, c_void};
use sdl_sys_bindgen::*;

/// Idiomatic Rust trait for SDL's callback-based application loop.
///
/// Implement this trait on your application state struct to hook into SDL's
/// main loop.
pub trait SdlApp: Sized + Send {
    /// Custom error type returned on initialization failure.
    type Error: core::fmt::Debug;

    /// Called once at startup to initialize the application state.
    ///
    /// * `args`: The command-line arguments passed to the application.
    #[cfg(feature = "args")]
    fn init(args: &[CString]) -> Result<Self, Self::Error>;

    /// Called once at startup to initialize the application state.
    #[cfg(not(feature = "args"))]
    fn init() -> Result<Self, Self::Error>;

    /// Called repeatedly by SDL to process a single frame.
    ///
    /// Return `SDL_AppResult::SDL_APP_CONTINUE` to keep running, or
    /// other values to terminate.
    fn iterate(&mut self) -> SDL_AppResult;

    /// Called by SDL whenever a new event is available.
    fn event(&mut self, event: &SDL_Event) -> SDL_AppResult;

    /// Called before the application terminates.
    ///
    /// The app state is automatically dropped after this method returns.
    fn quit(&mut self, result: SDL_AppResult);
}

/// Starts the SDL application loop using the specified [`SdlApp`] implementation.
///
/// This function handles the setup of C trampolines and invokes `SDL_RunApp`.
/// It will block until the application terminates.
pub fn run_app<A: SdlApp>() -> i32 {
    // 1. The Init Trampoline: Moves Rust app state onto the C heap managed by SDL.
    extern "C" fn c_init<A: SdlApp>(
        appstate: *mut *mut c_void,
        argc: c_int,
        argv: *mut *mut c_char,
    ) -> SDL_AppResult {
        let init_result = {
            #[cfg(feature = "args")]
            {
                let args = unsafe {
                    if argv.is_null() || argc == 0 {
                        Vec::new()
                    } else {
                        core::slice::from_raw_parts(argv, argc as usize)
                            .iter()
                            .map(|&ptr| CStr::from_ptr(ptr).to_owned())
                            .collect::<Vec<CString>>()
                    }
                };
                A::init(&args)
            }
            #[cfg(not(feature = "args"))]
            {
                let _ = (argc, argv);
                A::init()
            }
        };

        match init_result {
            Ok(app) => {
                unsafe {
                    // Allocate on the SDL heap to ensure consistency with c_quit's SDL_free.
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

    // 2. The Iterate Trampoline: Routes C calls back to A::iterate.
    extern "C" fn c_iter<A: SdlApp>(appstate: *mut c_void) -> SDL_AppResult {
        if appstate.is_null() {
            return SDL_AppResult::SDL_APP_FAILURE;
        }
        let app = unsafe { &mut *(appstate as *mut A) };
        app.iterate()
    }

    // 3. The Event Trampoline: Routes C calls back to A::event.
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

    // 4. The Quit Trampoline: Cleans up the Rust app state and frees the SDL heap pointer.
    extern "C" fn c_quit<A: SdlApp>(appstate: *mut c_void, result: SDL_AppResult) {
        if !appstate.is_null() {
            unsafe {
                let mut app = core::ptr::read(appstate as *mut A);
                app.quit(result);
                SDL_free(appstate);
            }
        }
    }

    // Internal callback wrapper for SDL_RunApp.
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
        #[cfg(all(feature = "std", feature = "args"))]
        {
            let args_collected: Vec<CString> = std::env::args()
                .map(|s| CString::new(s).expect("Argument contained null byte"))
                .collect();
            let mut arg_ptrs: Vec<*mut c_char> = args_collected
                .iter()
                .map(|cs| cs.as_ptr() as *mut c_char)
                .collect();

            SDL_RunApp(
                arg_ptrs.len() as c_int,
                arg_ptrs.as_mut_ptr(),
                Some(enter_callbacks::<A>),
                core::ptr::null_mut(),
            )
        }
        #[cfg(not(all(feature = "std", feature = "args")))]
        {
            SDL_RunApp(
                0,
                core::ptr::null_mut(),
                Some(enter_callbacks::<A>),
                core::ptr::null_mut(),
            )
        }
    }
}

#[inline]
fn log_error(msg: *const c_char) {
    unsafe {
        SDL_LogError(
            SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION.0 as c_int,
            msg,
        );
    }
}

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
#[cfg(feature = "alloc")]
use alloc::format;
use core::ffi::{c_char, c_int, c_void};
use sdl_sys_bindgen::*;

/// Idiomatic Rust trait for SDL's callback-based application loop.
///
/// Implement this trait on your application state struct to hook into SDL's
/// main loop.
///
/// # Panics
///
/// The methods are called from `extern "C"` trampolines, so a panic that escapes
/// `init`, `iterate`, `event` or `quit` aborts the process instead of unwinding
/// into SDL. Handle recoverable failures by returning `SDL_APP_FAILURE`
/// (or an `Err` from `init`).
pub trait SdlApp: Sized + Send {
    /// Custom error type returned on initialization failure.
    type Error: core::fmt::Debug;

    /// Called once at startup to initialize the application state.
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
/// On desktop platforms it blocks until the application terminates. Platforms
/// such as Emscripten may schedule callbacks and return before termination.
pub fn run_app<A: SdlApp>() -> i32 {
    // 1. The Init Trampoline: Moves Rust app state onto the C heap managed by SDL.
    extern "C" fn c_init<A: SdlApp>(
        appstate: *mut *mut c_void,
        _argc: c_int,
        _argv: *mut *mut c_char,
    ) -> SDL_AppResult {
        match A::init() {
            Ok(app) => {
                // SAFETY: SDL passes a valid, writable `appstate` slot to the init
                // callback, and `SDL_GetError` returns a valid C string.
                unsafe {
                    let ptr = allocate_app(app);
                    if ptr.is_null() {
                        log_error(SDL_GetError());
                        return SDL_AppResult::SDL_APP_FAILURE;
                    }
                    *appstate = ptr.cast();
                }
                SDL_AppResult::SDL_APP_CONTINUE
            }
            Err(error) => {
                #[cfg(feature = "alloc")]
                let err_msg = format!("{:?}\0", error);
                #[cfg(not(feature = "alloc"))]
                drop(error);
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
        // SAFETY: `appstate` was produced by `c_init::<A>` and stays valid until
        // `c_quit`. SDL serializes the app callbacks, so this `&mut A` is unique.
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
        // SAFETY: as in `c_iter`; `event` is non-null and valid for the call.
        let app = unsafe { &mut *(appstate as *mut A) };
        app.event(unsafe { &*event })
    }

    // 4. The Quit Trampoline: Cleans up the Rust app state and frees the SDL heap pointer.
    extern "C" fn c_quit<A: SdlApp>(appstate: *mut c_void, result: SDL_AppResult) {
        if !appstate.is_null() {
            // SAFETY: `appstate` came from `allocate_app::<A>`, holds an initialized
            // `A`, and SDL never uses it after the quit callback, so moving the value
            // out and freeing the allocation exactly once is sound.
            unsafe {
                let mut app = core::ptr::read(appstate as *mut A);
                app.quit(result);
                drop(app);
                SDL_aligned_free(appstate);
            }
        }
    }

    // Internal callback wrapper for SDL_RunApp.
    extern "C" fn enter_callbacks<A: SdlApp>(argc: c_int, argv: *mut *mut c_char) -> i32 {
        // SAFETY: SDL supplies the `argc`/`argv` it received, and the trampolines
        // match the callback signatures SDL expects.
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

    // SAFETY: `enter_callbacks` has the `SDL_main_func` signature; a null `argv`
    // with `argc == 0` and a null `reserved` pointer are accepted by `SDL_RunApp`.
    unsafe {
        SDL_RunApp(
            0,
            core::ptr::null_mut(),
            Some(enter_callbacks::<A>),
            core::ptr::null_mut(),
        )
    }
}

fn allocate_app<A>(app: A) -> *mut A {
    // SDL rounds the allocation size to the requested alignment. Allocate at
    // least one byte so zero-sized applications still have a valid state pointer.
    let ptr =
        unsafe { SDL_aligned_alloc(core::mem::align_of::<A>(), core::mem::size_of::<A>().max(1)) }
            .cast::<A>();
    if !ptr.is_null() {
        // SAFETY: SDL supplied enough storage aligned for A, and it is uninitialized.
        unsafe { ptr.write(app) };
    }
    ptr
}

#[inline]
fn log_error(msg: *const c_char) {
    // SAFETY: the `%s` format consumes exactly one NUL-terminated string, so
    // `msg` is never interpreted as a format string.
    unsafe {
        SDL_LogError(
            SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION.0 as c_int,
            c"%s".as_ptr(),
            msg,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{allocate_app, log_error};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use sdl_sys_bindgen::SDL_aligned_free;

    #[test]
    fn allocation_preserves_overaligned_state_and_drops_once() {
        #[repr(align(64))]
        struct App<'a>(&'a AtomicUsize);

        impl Drop for App<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let drops = AtomicUsize::new(0);
        let ptr = allocate_app(App(&drops));
        assert!(!ptr.is_null());
        assert_eq!(ptr as usize % 64, 0);
        unsafe {
            ptr.drop_in_place();
            SDL_aligned_free(ptr.cast());
        }
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn zero_sized_state_has_a_nonnull_aligned_pointer() {
        #[repr(align(64))]
        struct App;

        assert_eq!(core::mem::size_of::<App>(), 0);
        let ptr = allocate_app(App);
        assert!(!ptr.is_null());
        assert_eq!(ptr as usize % 64, 0);
        unsafe {
            ptr.drop_in_place();
            SDL_aligned_free(ptr.cast());
        }
    }

    #[test]
    fn error_messages_are_not_interpreted_as_format_strings() {
        log_error(c"literal %s %n 100%".as_ptr());
    }
}

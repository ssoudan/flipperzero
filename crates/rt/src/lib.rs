//! Rust Runtime for the Flipper Zero.
//!
//! This must be build with `-Z no-unique-section-names` to ensure that this module
//! is linked directly into the `.text` section.

#![no_std]

pub mod manifest;
pub mod panic_handler;

/// The C entry point.
/// This just delegates to the user's Rust entry point.
#[no_mangle]
pub unsafe extern "C" fn _start(args: *mut u8) -> i32 {
    extern "Rust" {
        fn main(args: *mut u8) -> i32;
    }

    main(args)
}

/// Define the entry point.
/// Must have the following signature: `fn(*mut u8) -> i32`.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        // Force the section to `.text` instead of `.text.main`.
        // lld seems not to automatically rename `.rel.text.main` properly.
        #[export_name = "main"]
        pub unsafe fn __main(args: *mut u8) -> i32 {
            use core::{ffi::CStr, mem, ptr, slice};
            use flipperzero_sys as sys;

            // type check the entry function
            let f: fn(*mut u8) -> i32 = $path;

            let ret = f(args);

            // Wait for threads with the same app ID to finish. We assume there are not
            // more than 30 threads running in the background outside this app, so we will
            // observe at least one of any thread the app might have left running.
            unsafe {
                sys::furi_log_print_format(
                    sys::FuriLogLevel_FuriLogLevelDebug,
                    sys::c_string!("flipperzero-rt"),
                    sys::c_string!("Waiting for FAP threads to complete..."),
                );
            }

            const MAX_THREADS: usize = 32;
            let cur_thread_id = unsafe { sys::furi_thread_get_current_id() };
            let app_id = unsafe { CStr::from_ptr(sys::furi_thread_get_appid(cur_thread_id)) };
            let mut thread_ids: [sys::FuriThreadId; MAX_THREADS] = [ptr::null_mut(); MAX_THREADS];

            'outer: loop {
                let thread_count = unsafe {
                    sys::furi_thread_enumerate(thread_ids.as_mut_ptr(), MAX_THREADS as u32)
                } as usize;

                for &thread_id in thread_ids[..thread_count].into_iter() {
                    let thread_app_id =
                        unsafe { CStr::from_ptr(sys::furi_thread_get_appid(thread_id)) };

                    if thread_id == cur_thread_id || thread_app_id != app_id {
                        // Ignore this thread or the threads of other apps
                        continue;
                    }

                    let thread_name =
                        unsafe { CStr::from_ptr(sys::furi_thread_get_name(thread_id)) };

                    if thread_name.to_bytes().ends_with(b"Srv") {
                        // This is a workaround for an issue where the current appid matches
                        // one of the built-in service names (e.g. "gui"). Otherwise we will
                        // see the service thread (e.g. "GuiSrv") and assume that it's one of
                        // our threads that still needs to exit, thus causing the app to hang
                        // at exit.
                        continue;
                    }

                    // There is a thread that is still running, so wait for it to exit...

                    unsafe {
                        sys::furi_delay_ms(10);
                    }

                    continue 'outer;
                }

                break;
            }

            unsafe {
                sys::furi_log_print_format(
                    sys::FuriLogLevel_FuriLogLevelDebug,
                    sys::c_string!("flipperzero-rt"),
                    sys::c_string!("All threads completed, exiting FAP"),
                );
            }

            ret
        }
    };
}

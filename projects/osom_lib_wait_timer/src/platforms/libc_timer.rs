use core::time::Duration;

use crate::traits::{MAX_WAIT_DURATION, WaitTimer};

/// The Macos variant of [`WaitTimer`].
#[must_use]
#[repr(transparent)]
pub struct LibcWaitTimer;

impl LibcWaitTimer {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }

    pub fn wait(&mut self, dur: Duration) {
        let _ = self;
        debug_assert!(dur <= MAX_WAIT_DURATION);

        let mut secs = dur.as_secs();
        let mut nsecs = dur.subsec_nanos().into();

        // If we're awoken with a signal then the return value will be -1 and
        // nanosleep will fill in `ts` with the remaining time.
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        unsafe {
            while secs > 0 || nsecs > 0 {
                let mut ts = libc::timespec {
                    tv_sec: secs as libc::time_t,
                    tv_nsec: nsecs,
                };
                secs -= ts.tv_sec as u64;
                let ts_ptr = &raw mut ts;
                if libc::nanosleep(ts_ptr, ts_ptr) == -1 {
                    assert!(is_eintr());
                    secs += ts.tv_sec as u64;
                    nsecs = ts.tv_nsec;
                } else {
                    nsecs = 0;
                }
            }
        }
    }
}

impl WaitTimer for LibcWaitTimer {
    #[inline(always)]
    fn wait(&mut self, dur: Duration) {
        self.wait(dur);
    }
}

impl Default for LibcWaitTimer {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[inline(always)]
#[allow(clippy::needless_late_init)]
fn is_eintr() -> bool {
    let errno: i32;

    osom_lib_cfg_ext::cfg_match!(
        (target_os="macos") => {
            errno = unsafe { *libc::__error() };
        },
        (target_os="linux") => {
            errno = unsafe { *libc::__errno_location() };
        },
        _ => {
            compile_error!("is_eintr() requires macos or linux")
        }
    );

    errno == libc::EINTR
}

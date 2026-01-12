use core::time::Duration;

use crate::traits::{MAX_WAIT_DURATION, WaitTimer};

osom_lib_cfg_ext::cfg_match!(
    (target_os="windows") => {
        mod windows_timer;
        use windows_timer::WindowsWaitTimer as PlaformWaitTimer;
    },
    (any(target_os="macos", target_os="linux")) => {
        mod libc_timer;
        use libc_timer::LibcWaitTimer as PlaformWaitTimer;
    },
    _ => {
        compile_error!("Current target is not supported.");
    }
);

/// The platform dependent implementation of [`WaitTimer`]. This struct is guaranteed
/// to have size at most 8.
#[derive(Default)]
#[repr(transparent)]
#[must_use]
pub struct TheWaitTimer {
    inner: PlaformWaitTimer,
}

impl TheWaitTimer {
    /// Creates a new [`TheWaitTimer`].
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            inner: PlaformWaitTimer::new(),
        }
    }

    /// Sleeps for the given duration, blocking the thread. This function aims
    /// to have 1ms resolution, assuming the underlying platform allows it.
    ///
    /// # Panics
    ///
    /// When `dur` is above [`MAX_WAIT_DURATION`].
    #[inline(always)]
    pub fn wait(&mut self, dur: Duration) {
        assert!(
            dur <= MAX_WAIT_DURATION,
            ".wait() cannot be called with duration above MAX_WAIT_DURATION."
        );
        self.inner.wait(dur);
    }
}

impl WaitTimer for TheWaitTimer {
    #[inline(always)]
    fn wait(&mut self, dur: Duration) {
        self.wait(dur);
    }
}

const _CHECK: () = const {
    assert!(size_of::<TheWaitTimer>() <= 8, "Size of TheWaitTimer cannot exceed 8");
};

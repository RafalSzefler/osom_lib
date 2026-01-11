use core::time::Duration;

use crate::traits::WaitTimer;

osom_lib_cfg_ext::cfg_match!(
    (windows) => {
        mod windows;
        use windows::WindowsWaitTimer as PlaformWaitTimer;
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
    #[inline(always)]
    pub fn wait(&mut self, dur: Duration) {
        self.inner.wait(dur);
    }
}

impl WaitTimer for TheWaitTimer {
    #[inline(always)]
    fn wait(&mut self, dur: Duration) {
        self.inner.wait(dur);
    }
}

const _CHECK: () = const {
    assert!(size_of::<TheWaitTimer>() <= 8, "Size of TheWaitTimer cannot exceed 8");
};

#![cfg(windows)]

use core::{ptr, time::Duration};

use windows_sys::Win32::{
    Foundation::{FALSE, HANDLE, WAIT_FAILED},
    System::Threading::{
        CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, INFINITE, TIMER_ALL_ACCESS,
        CreateWaitableTimerExW, SetWaitableTimer, Sleep, SwitchToThread, WaitForSingleObject,
    },
};

use crate::traits::{MAX_WAIT_DURATION, WaitTimer};

const NANOS_PER_SEC: u64 = 1_000_000_000;
const INTERVALS_PER_SEC: u64 = NANOS_PER_SEC / 100;

/// The Windows variant of [`WaitTimer`].
#[must_use]
#[repr(transparent)]
pub struct WindowsWaitTimer {
    handle: HANDLE,
}

impl WindowsWaitTimer {
    pub fn new() -> Self {
        let handle = unsafe {
            CreateWaitableTimerExW(
                ptr::null(),
                ptr::null(),
                CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                TIMER_ALL_ACCESS,
            )
        };

        Self { handle }
    }

    pub fn wait(&mut self, dur: Duration) {
        debug_assert!(dur <= MAX_WAIT_DURATION);

        if dur.is_zero() {
            unsafe { SwitchToThread() };
            return;
        }

        if self.handle.is_null() {
            fallback_sleep(dur);
            return;
        }

        let waitable_timeout = dur_to_waitable_timeout(dur);

        let result = unsafe { SetWaitableTimer(self.handle, &raw const waitable_timeout, 0, None, ptr::null(), FALSE) };
        if result == 0 {
            fallback_sleep(dur);
            return;
        }

        let result = unsafe { WaitForSingleObject(self.handle, INFINITE) };
        if result == WAIT_FAILED {
            fallback_sleep(dur);
            return;
        }
    }
}

unsafe impl Send for WindowsWaitTimer {}
unsafe impl Sync for WindowsWaitTimer {}

impl Default for WindowsWaitTimer {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl WaitTimer for WindowsWaitTimer {
    #[inline(always)]
    fn wait(&mut self, dur: Duration) {
        self.wait(dur);
    }
}

#[inline(always)]
fn dur_to_waitable_timeout(dur: Duration) -> i64 {
    let nanos = u64::from(dur.subsec_nanos()) / 100;
    -unsafe {
        dur.as_secs()
            .unchecked_mul(INTERVALS_PER_SEC)
            .unchecked_add(nanos)
            .cast_signed()
    }
}

#[inline(always)]
fn dur_to_sleep_timeout(dur: Duration) -> u32 {
    let nanos = u64::from(dur.subsec_nanos());

    u32::try_from(
        dur.as_secs()
            .checked_mul(1000)
            .unwrap()
            .checked_add(nanos / 1_000_000)
            .unwrap()
            .checked_add(u64::from(!nanos.is_multiple_of(1_000_000)))
            .unwrap(),
    )
    .unwrap()
}

fn fallback_sleep(dur: Duration) {
    let timeout = dur_to_sleep_timeout(dur);
    unsafe {
        Sleep(timeout);
    }
}

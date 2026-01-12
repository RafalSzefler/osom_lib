//! Defines the [`WaitTimer`] trait and holds [`MAX_WAIT_DURATION`] const.

use core::time::Duration;

/// The maximum duration allowed on [`WaitTimer`], which is one week.
pub const MAX_WAIT_DURATION: Duration = Duration::from_hours(24 * 7);

/// Represents structs that can be waited of for a given duration. Functionaly
/// equivalent to `std::thread::sleep`.
pub trait WaitTimer: Send + Sync + Default + 'static {
    /// Sleeps for the given duration, blocking the thread. This function aims
    /// to have 1ms resolution, assuming the underlying platform allows it.
    fn wait(&mut self, dur: Duration);
}

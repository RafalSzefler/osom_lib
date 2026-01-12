use std::time::{Duration, Instant};

use osom_lib_wait_timer::TheWaitTimer;
use rstest::rstest;

#[cfg_attr(feature = "ci", ignore)]
#[rstest]
#[case(Duration::from_micros(950), Duration::from_micros(1000), Duration::from_micros(1500))]
#[case(
    Duration::from_micros(1950),
    Duration::from_micros(2000),
    Duration::from_micros(2800)
)]
#[case(
    Duration::from_micros(4950),
    Duration::from_micros(5000),
    Duration::from_micros(6500)
)]
fn test_sleep_local(#[case] lower: Duration, #[case] dur: Duration, #[case] upper: Duration) {
    internal_test_sleep(lower, dur, upper);
}

#[cfg_attr(not(feature = "ci"), ignore)]
#[test]
fn test_sleep_ci() {
    let lower = Duration::from_micros(950);
    let dur = Duration::from_micros(1000);
    let upper = if cfg!(target_os = "macos") {
        // The macos on CI/CD seems to have very inaccurate sleep for some reason.
        Duration::from_micros(6000)
    } else {
        Duration::from_micros(1500)
    };
    internal_test_sleep(lower, dur, upper);
}

fn internal_test_sleep(lower: Duration, dur: Duration, upper: Duration) {
    const ITERS: u32 = 100;

    let mut total_duration = Duration::ZERO;
    let mut timer = TheWaitTimer::new();

    for _ in 0..ITERS {
        let start = Instant::now();
        timer.wait(dur);
        total_duration += Instant::now() - start;
    }

    let avg_duration = total_duration / ITERS;
    assert!(
        lower < avg_duration,
        "avg_duration {:?} is below the lower bound {:?}",
        avg_duration,
        lower
    );
    assert!(
        avg_duration < upper,
        "avg_duration {:?} exceeded the upper bound {:?}",
        avg_duration,
        upper
    );
}

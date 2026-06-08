use rstest::rstest;

use osom_structa_cvr::CVRFloat;

fn round_trips(v: f64) -> bool {
    let cvr = CVRFloat::new(v);
    let back = cvr.inner();
    back == v
}

#[rstest]
#[case(0.0)]
#[case(0.1)]
#[case(0.2)]
#[case(0.3)]
#[case(1.0 / 3.0)]
#[case(2.0 / 3.0)]
#[case(0.123456789)]
#[case(1e-15)]
#[case(1e-16)]
#[case(1e-17)]
#[case(9007199254740992.0)]
#[case(3.141592653589793)]
#[case(-42.5)]
fn test_f64_fraction_roundtrip_samples(#[case] v: f64) {
    assert!(round_trips(v), "failed round-trip for {v}");
}

use osom_lib_primitives::coption::COption;

#[test]
fn test_unpack_some() {
    fn test_unpack_inner(value: COption<i32>) -> COption<i32> {
        match value {
            COption::Some(v) => COption::Some(v + 1),
            COption::None => COption::None,
        }
    }

    for x in -10..15 {
        let o = COption::Some(x);
        assert_eq!(test_unpack_inner(o).unwrap(), x + 1);
    }
}

#[test]
fn test_unpack_none() {
    fn test_unpack_inner(value: COption<i32>) -> COption<i32> {
        match value {
            COption::Some(v) => COption::Some(v + 1),
            COption::None => COption::None,
        }
    }

    let none = COption::None;
    assert!(test_unpack_inner(none).is_none());
}

#[test]
fn test_is_some_is_none() {
    assert!(COption::Some(0).is_some());
    assert!(!COption::Some(0).is_none());

    assert!(!COption::<i32>::None.is_some());
    assert!(COption::<i32>::None.is_none());
}

#[test]
fn test_expect() {
    assert_eq!(COption::Some(7).expect("expected some"), 7);
}

#[test]
#[should_panic(expected = "`COption::expect()`: not present.")]
fn test_expect_panics_on_none() {
    let _ = COption::<i32>::None.expect("not present.");
}

#[test]
fn test_unwrap() {
    assert_eq!(COption::Some(-3).unwrap(), -3);
}

#[test]
#[should_panic(expected = "called `COption::unwrap()` on a `None` value.")]
fn test_unwrap_panics_on_none() {
    let _ = COption::<i32>::None.unwrap();
}

#[test]
fn test_unwrap_unchecked_some() {
    let copt = COption::Some(42);
    assert_eq!(unsafe { copt.unwrap_unchecked() }, 42);
}

#[test]
fn test_into_option() {
    assert_eq!(COption::Some(1).into_option(), Some(1));
    assert_eq!(COption::<i32>::None.into_option(), None);
}

#[test]
fn test_from_option() {
    assert!(COption::from_option(Some(2)).is_some());
    assert_eq!(COption::from_option(Some(2)).unwrap(), 2);
    assert!(COption::from_option(None::<i32>).is_none());
}

#[test]
fn test_from_impls() {
    for x in -10..15 {
        let copt: COption<i32> = Some(x).into();
        assert_eq!(copt.into_option(), Some(x));

        let copt: COption<i32> = Some(x).into();
        let opt: Option<i32> = copt.into();
        assert_eq!(opt, Some(x));
    }

    let cnone: COption<i32> = None.into();
    assert!(cnone.is_none());
    let opt: Option<i32> = cnone.into();
    assert_eq!(opt, None);
}

#[test]
fn test_as_ref() {
    assert_eq!(COption::Some(3).as_ref(), COption::Some(&3));
    assert_eq!(COption::<i32>::None.as_ref(), COption::None);
}

#[test]
fn test_as_mut() {
    let mut copt = COption::Some(5);
    if let COption::Some(v) = copt.as_mut() {
        *v += 1;
    }
    assert_eq!(copt.unwrap(), 6);

    let mut cnone = COption::<i32>::None;
    assert_eq!(cnone.as_mut(), COption::None);
}

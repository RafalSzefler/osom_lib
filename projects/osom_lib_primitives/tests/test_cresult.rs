use osom_lib_primitives::cresult::CResult;
use osom_lib_primitives::macros::try_unpack;
use osom_lib_reprc::macros::reprc;

#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Error {
    pub val: u32,
}

#[test]
fn test_unpack_ok() {
    fn test_unpack_inner(value: CResult<i32, Error>) -> CResult<i32, Error> {
        let result = try_unpack!(value);
        CResult::Ok(result + 1)
    }

    for x in -10..15 {
        let r = CResult::Ok(x);
        assert_eq!(test_unpack_inner(r).unwrap(), x + 1);
    }
}

#[test]
fn test_unpack_err() {
    fn test_unpack_inner(value: CResult<i32, Error>) -> CResult<i32, Error> {
        let result = try_unpack!(value);
        CResult::Ok(result + 1)
    }

    const ERR: Error = Error { val: 15 };

    let err = CResult::Err(ERR);
    let result = test_unpack_inner(err).unwrap_err();
    assert_eq!(result, ERR);
}

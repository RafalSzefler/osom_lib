use osom_lib_arrays::fixed_array::ConstBuffer;

const fn array_eq_const(left: &[i32], right: &[i32]) {
    let mut len = left.len();
    assert!(len == right.len(), "Arrays have different lengths");
    while len > 0 {
        len -= 1;
        assert!(left[len] == right[len], "Arrays are not equal");
    }
}

macro_rules! run_const_bufferer_test_1 {
    () => {{
        let mut bufferer = ConstBuffer::<4, i32>::new();
        let mut iterator = bufferer.buffer_const(&[1]);
        assert!(iterator.next().is_none(), "No block should be available");

        let mut iterator = bufferer.buffer_const(&[2, 3, 4]);
        array_eq_const(iterator.next().unwrap(), &[1, 2, 3, 4]);
        assert!(iterator.next().is_none(), "No block should be available");
    }};
}

macro_rules! run_const_bufferer_test_2 {
    () => {{
        let mut bufferer = ConstBuffer::<5, i32>::new();
        let mut iterator = bufferer.buffer_const(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13]);
        array_eq_const(iterator.next().unwrap(), &[1, 2, 3, 4, 5]);
        array_eq_const(iterator.next().unwrap(), &[6, 7, 8, 9, 11]);
        assert!(iterator.next().is_none(), "No more blocks should be available");
        let remaining = bufferer.release_const();
        array_eq_const(remaining.as_slice_const(), &[12, 13]);
    }};
}

#[test]
fn test_const_bufferer_incomplete() {
    run_const_bufferer_test_1!();
}

#[test]
fn test_const_bufferer_incomplete_const() {
    const {
        run_const_bufferer_test_1!();
    }
}

#[test]
fn test_const_bufferer_chunked() {
    run_const_bufferer_test_2!();
}

#[test]
fn test_const_bufferer_chunked_const() {
    const {
        run_const_bufferer_test_2!();
    }
}

#[test]
fn test_equal_chunks() {
    let mut bufferer = ConstBuffer::<3, i32>::new();
    let mut iterator = bufferer.buffer_const(&[-3, -2, -1, 1, 2, 3]);
    array_eq_const(iterator.next().unwrap(), &[-3, -2, -1]);
    array_eq_const(iterator.next().unwrap(), &[1, 2, 3]);
    let remaining = bufferer.release_const();
    let remaining = remaining.as_slice_const();
    array_eq_const(remaining, &[]);
}

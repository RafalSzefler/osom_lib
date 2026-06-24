use osom_lib::reprc::macros::reprc;
use osom_lib_alloc::traits::{Allocator, WithMessage};
use osom_lib_boxed::cbox::CBox;
use osom_lib_primitives::coption::COption;

#[reprc]
#[derive(Debug, Default)]
struct DummyAllocator;

#[reprc]
#[derive(Debug)]
struct DummyAllocationError;

impl WithMessage for DummyAllocationError {
    fn message(&self) -> &str {
        "DummyAllocationError"
    }
}

unsafe impl Allocator for DummyAllocator {
    type SpecificAllocationError = DummyAllocationError;

    fn allocate(&mut self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, Self::SpecificAllocationError> {
        panic!("DummyAllocator does not allocate");
    }

    unsafe fn deallocate(&mut self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {
        panic!("DummyAllocator does not deallocate");
    }
}

#[reprc]
struct MyStruct {
    a: u32,
    b: u32,
}

#[test]
fn test_reprc() {
    let my_struct = MyStruct { a: 1, b: 2 };
    let ptr = &raw const my_struct as *const [u8; size_of::<MyStruct>()];
    let data = unsafe { *ptr };

    assert!(&data == &[1, 0, 0, 0, 2, 0, 0, 0] || &data == &[2, 0, 0, 0, 1, 0, 0, 0]);
}

#[reprc]
struct RecursiveReprC {
    value: bool,
    next: COption<CBox<RecursiveReprC, DummyAllocator>>,
}

#[test]
fn test_recursive() {
    let recursive = RecursiveReprC {
        value: true,
        next: COption::None,
    };
    assert!(recursive.value);
}

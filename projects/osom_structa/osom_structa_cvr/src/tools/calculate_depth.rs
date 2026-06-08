use osom_lib_alloc::traits::Allocator;

use crate::cvr::CVR;

/// Calculates the depth of the given [`CVR`] value, by traversing the value recursively.
pub fn calculate_depth<TAllocator: Allocator>(cvr: &CVR<TAllocator>) -> usize {
    match cvr {
        CVR::Null | CVR::Bool(_) | CVR::Int(_) | CVR::String(_) | CVR::Float(_) => 0,

        CVR::Array(cvrarray) => {
            let array_ref = cvrarray.inner_ref().as_ref();
            if array_ref.is_empty() {
                return 0;
            }
            let mut current = 0;
            for item in array_ref {
                current = current.max(calculate_depth(item));
            }
            current + 1
        }

        CVR::Object(cvrobject) => {
            if cvrobject.is_empty() {
                return 0;
            }
            let mut current = 0;
            for (_, value) in cvrobject.iter() {
                current = current.max(calculate_depth(value));
            }
            current + 1
        }
    }
}

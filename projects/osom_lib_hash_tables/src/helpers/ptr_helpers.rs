macro_rules! ptr_to_ref {
    ( $e: expr ) => {{
        let result;
        #[allow(unused_unsafe)]
        {
            result = unsafe { &*($e) };
        }
        result
    }};
}

pub(crate) use ptr_to_ref;

macro_rules! ptr_to_mut {
    ( $e: expr ) => {{
        let result;
        #[allow(unused_unsafe)]
        {
            result = unsafe { &mut *($e) };
        }
        result
    }};
}

pub(crate) use ptr_to_mut;

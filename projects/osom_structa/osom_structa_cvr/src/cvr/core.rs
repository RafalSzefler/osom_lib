use core::fmt::Display;

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::{CVRConvertionError, TryCloneCVRError};

use super::{CVRArray, CVRBool, CVRFloat, CVRInt, CVRObject, CVRString};

/// The main `CVR` (Canonical Value Representation) enum.
#[repr(C)]
#[repr(u8)]
#[derive(Debug)]
pub enum CVR<TAllocator: Allocator> {
    /// A null value.
    Null = 0,

    /// A boolean value.
    Bool(CVRBool) = 1,

    /// An integer value (internally represented by `i128`).
    Int(CVRInt) = 2,

    /// A string value (internally represented by shared string behind reference counting).
    String(CVRString<TAllocator>) = 3,

    /// A fraction value (internally represented by `i64` numerator and denominator pair).
    Float(CVRFloat) = 4,

    /// An array value.
    Array(CVRArray<TAllocator>) = 5,

    /// An object value.
    Object(CVRObject<TAllocator>) = 6,
}

impl<TAllocator: Allocator> PartialEq for CVR<TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        if self.tag_value() != other.tag_value() {
            return false;
        }

        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Int(left), Self::Int(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            (Self::Array(left), Self::Array(right)) => left == right,
            (Self::Object(left), Self::Object(right)) => left == right,
            _ => unreachable!("CVR tag value are not equal"),
        }
    }
}

impl<TAllocator: Allocator> Eq for CVR<TAllocator> {}

impl<TAllocator: Allocator> core::hash::Hash for CVR<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let tag_value = self.tag_value();
        tag_value.hash(state);
        match self {
            CVR::Null => {}
            CVR::Bool(cvrbool) => cvrbool.hash(state),
            CVR::Int(cvrint) => cvrint.hash(state),
            CVR::String(cvrstring) => cvrstring.hash(state),
            CVR::Float(cvrfloat) => cvrfloat.hash(state),
            CVR::Array(cvrarray) => cvrarray.hash(state),
            CVR::Object(cvrobject) => cvrobject.hash(state),
        }
    }
}

unsafe impl<TAllocator: Allocator> ReprC for CVR<TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TAllocator>();
        osom_lib_reprc::hidden::is_reprc::<CVRBool>();
        osom_lib_reprc::hidden::is_reprc::<CVRInt>();
        osom_lib_reprc::hidden::is_reprc::<CVRString<TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<CVRFloat>();
        osom_lib_reprc::hidden::is_reprc::<CVRArray<TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<CVRObject<TAllocator>>();
    };
}

impl<TAllocator: Allocator + TryClone> TryClone for CVR<TAllocator> {
    type Error = TryCloneCVRError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        match self {
            Self::Null => Ok(Self::Null),
            Self::Bool(value) => Ok(Self::Bool(value.try_clone()?)),
            Self::Int(value) => Ok(Self::Int(value.try_clone()?)),
            Self::Float(value) => Ok(Self::Float(value.try_clone()?)),
            Self::Array(value) => Ok(Self::Array(value.try_clone()?)),
            Self::String(value) => Ok(Self::String(value.try_clone()?)),
            Self::Object(value) => Ok(Self::Object(value.try_clone()?)),
        }
    }
}

impl<TAllocator: Allocator + TryClone> Clone for CVR<TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone CVR")
    }
}

impl<TAllocator: Allocator> Display for CVR<TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Array(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Object(value) => write!(f, "{value}"),
        }
    }
}

impl<TAllocator: Allocator> CVR<TAllocator> {
    /// Returns the internal tag value of the [`CVR`] value.
    #[inline]
    #[must_use]
    pub const fn tag_value(&self) -> u8 {
        unsafe { *core::ptr::from_ref(self).cast::<u8>() }
    }

    /// Returns `true` if the [`CVR`] value is null.
    #[inline]
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Converts the [`CVR`] value into a [`CVRBool`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRBool`] value.
    #[inline]
    pub const fn into_bool(self) -> Result<CVRBool, CVRConvertionError> {
        let result = match self {
            Self::Bool(value) => Ok(value),
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRBool`] value if the [`CVR`] value is a [`CVRBool`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRBool`] value.
    #[inline]
    pub const fn as_bool(&self) -> Result<&CVRBool, CVRConvertionError> {
        match self {
            Self::Bool(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRBool`] value if the [`CVR`] value is a [`CVRBool`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRBool`] value.
    pub const fn as_bool_mut(&mut self) -> Result<&mut CVRBool, CVRConvertionError> {
        match self {
            Self::Bool(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Converts the [`CVR`] value into a [`CVRInt`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRInt`] value.
    #[inline]
    pub const fn into_int(self) -> Result<CVRInt, CVRConvertionError> {
        let result = match self {
            Self::Int(value) => Ok(value),
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRInt`] value if the [`CVR`] value is a [`CVRInt`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRInt`] value.
    #[inline]
    pub const fn as_int(&self) -> Result<&CVRInt, CVRConvertionError> {
        match self {
            Self::Int(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRInt`] value if the [`CVR`] value is a [`CVRInt`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRInt`] value.
    pub const fn as_int_mut(&mut self) -> Result<&mut CVRInt, CVRConvertionError> {
        match self {
            Self::Int(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Converts the [`CVR`] value into a [`CVRString`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRString`] value.
    #[inline]
    pub const fn into_string(self) -> Result<CVRString<TAllocator>, CVRConvertionError> {
        let result = match &self {
            Self::String(value) => unsafe { Ok(core::ptr::read(value)) },
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRString`] value if the [`CVR`] value is a [`CVRString`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRString`] value.
    #[inline]
    pub const fn as_string(&self) -> Result<&CVRString<TAllocator>, CVRConvertionError> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRString`] value if the [`CVR`] value is a [`CVRString`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRString`] value.
    pub const fn as_string_mut(&mut self) -> Result<&mut CVRString<TAllocator>, CVRConvertionError> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Converts the [`CVR`] value into a [`CVRFloat`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRFloat`] value.
    #[inline]
    pub const fn into_fraction(self) -> Result<CVRFloat, CVRConvertionError> {
        let result = match self {
            Self::Float(value) => Ok(value),
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRFloat`] value if the [`CVR`] value is a [`CVRFloat`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRFloat`] value.
    #[inline]
    pub const fn as_fraction(&self) -> Result<&CVRFloat, CVRConvertionError> {
        match self {
            Self::Float(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRFloat`] value if the [`CVR`] value is a [`CVRFloat`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRFloat`] value.
    pub const fn as_fraction_mut(&mut self) -> Result<&mut CVRFloat, CVRConvertionError> {
        match self {
            Self::Float(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Converts the [`CVR`] value into a [`CVRArray`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRArray`] value.
    #[inline]
    pub const fn into_array(self) -> Result<CVRArray<TAllocator>, CVRConvertionError> {
        let result = match &self {
            Self::Array(value) => unsafe { Ok(core::ptr::read(value)) },
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRArray`] value if the [`CVR`] value is a [`CVRArray`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRArray`] value.
    #[inline]
    pub const fn as_array(&self) -> Result<&CVRArray<TAllocator>, CVRConvertionError> {
        match self {
            Self::Array(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRArray`] value if the [`CVR`] value is a [`CVRArray`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRArray`] value.
    pub const fn as_array_mut(&mut self) -> Result<&mut CVRArray<TAllocator>, CVRConvertionError> {
        match self {
            Self::Array(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Converts the [`CVR`] value into a [`CVRObject`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRObject`] value.
    #[inline]
    pub const fn into_object(self) -> Result<CVRObject<TAllocator>, CVRConvertionError> {
        let result = match &self {
            Self::Object(value) => unsafe { Ok(core::ptr::read(value)) },
            _ => Err(CVRConvertionError),
        };
        core::mem::forget(self);
        result
    }

    /// Returns a reference to the [`CVRObject`] value if the [`CVR`] value is a [`CVRObject`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRObject`] value.
    #[inline]
    pub const fn as_object(&self) -> Result<&CVRObject<TAllocator>, CVRConvertionError> {
        match self {
            Self::Object(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }

    /// Returns a mutable reference to the [`CVRObject`] value if the [`CVR`] value is a [`CVRObject`] value.
    ///
    /// # Errors
    ///
    /// Returns [`CVRConvertionError`] if the [`CVR`] value is not a [`CVRObject`] value.
    pub const fn as_object_mut(&mut self) -> Result<&mut CVRObject<TAllocator>, CVRConvertionError> {
        match self {
            Self::Object(value) => Ok(value),
            _ => Err(CVRConvertionError),
        }
    }
}

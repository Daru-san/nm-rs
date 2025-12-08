use glib::translate::FromGlibPtrNone;

use crate::ffi;
use std::ptr;

#[allow(dead_code)]
pub struct VariantAttributeSpec(ptr::NonNull<ffi::NMVariantAttributeSpec>);

impl FromGlibPtrNone<*const *const ffi::NMVariantAttributeSpec> for VariantAttributeSpec {
    unsafe fn from_glib_none(ptr: *const *const ffi::NMVariantAttributeSpec) -> Self {
        let ptr = std::ptr::read(ptr);
        Self(ptr::NonNull::new_unchecked(ptr.cast_mut()))
    }
}

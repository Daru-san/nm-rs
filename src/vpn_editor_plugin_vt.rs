use glib::translate::{Stash, ToGlibPtr, ToGlibPtrMut};

use crate::ffi;

#[allow(dead_code)]
pub struct VpnEditorPluginVT(std::ptr::NonNull<ffi::NMVpnEditorPluginVT>);

impl VpnEditorPluginVT {
    pub fn uninitialized() -> Self {
        Self(std::ptr::NonNull::dangling())
    }
}

impl<'b> ToGlibPtr<'b, ffi::NMVpnEditorPluginVT> for VpnEditorPluginVT {
    type Storage = std::ptr::NonNull<ffi::NMVpnEditorPluginVT>;

    fn to_glib_none(&'b self) -> glib::translate::Stash<'b, ffi::NMVpnEditorPluginVT, Self> {
        unimplemented!()
    }
}

impl<'b> ToGlibPtrMut<'b, ffi::NMVpnEditorPluginVT> for VpnEditorPluginVT {
    type Storage = std::ptr::NonNull<ffi::NMVpnEditorPluginVT>;
    fn to_glib_none_mut(
        &'b mut self,
    ) -> glib::translate::StashMut<'b, ffi::NMVpnEditorPluginVT, Self> {
        unimplemented!()
    }
}

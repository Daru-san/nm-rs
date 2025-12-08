#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(deprecated)]
#![allow(unused_imports)]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use auto::*;
use libnm_sys as ffi;

mod auto;
pub mod prelude;

mod variant_attribute_spec;
pub use variant_attribute_spec::VariantAttributeSpec;
mod vpn_editor_plugin_vt;
pub use vpn_editor_plugin_vt::VpnEditorPluginVT;

pub mod functions {
    pub use super::auto::functions::*;
}

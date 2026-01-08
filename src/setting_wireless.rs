use std::borrow::Borrow;

use glib::object::{IsA, ObjectExt};
use glib::translate::*;
use libnm_api_sys as ffi;

use crate::SettingWireless;
use crate::builders::SettingWirelessBuilder;

pub trait SettingWirelessExtManual:
    IsA<SettingWireless, GlibType = ffi::NMSettingWireless>
{
    ///
    /// # Returns
    ///
    /// the #NMSettingWireless:ssid property of the setting
    #[doc(alias = "nm_setting_wireless_get_ssid")]
    #[doc(alias = "get_ssid")]
    fn ssid(&self) -> glib::Bytes {
        unsafe {
            glib::Bytes::from_glib_none(ffi::nm_setting_wireless_get_ssid(self.to_glib_none().0))
        }
    }

    ///
    /// # Returns
    ///
    /// the #NMSettingWireless:ssid property of the setting as a string checked for UTF8
    /// compatibility.
    fn ssid_utf8(&self) -> Option<glib::GString> {
        let ssid = unsafe {
            glib::Bytes::from_glib_none(ffi::nm_setting_wireless_get_ssid(self.to_glib_none().0))
        };

        glib::GString::from_utf8(ssid.to_vec()).ok()
    }

    // SSID of the Wi-Fi network. Must be specified.
    fn set_ssid<B: Borrow<[u8]> + ?Sized>(&self, ssid: &B) {
        let bytes = glib::Bytes::from(ssid);
        ObjectExt::set_property(self, "ssid", bytes);
    }
}

impl<T: IsA<SettingWireless, GlibType = ffi::NMSettingWireless>> SettingWirelessExtManual for T {}

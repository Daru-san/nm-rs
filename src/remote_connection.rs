use std::pin::Pin;

use glib::object::IsA;
use glib::translate::FromGlibPtrFull;
use glib::translate::ToGlibPtr;
use glib::translate::from_glib_full;

use crate::RemoteConnection;
use crate::ffi;

pub trait RemoteConnectionExtManual:
    IsA<RemoteConnection, GlibType = crate::ffi::NMRemoteConnection>
{
    #[doc(alias = "nm_remote_connection_get_secrets")]
    #[doc(alias = "get_secrets")]
    fn secrets(
        &self,
        setting_name: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let res = ffi::nm_remote_connection_get_secrets(
                self.to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(glib::Variant::from_glib_full(res))
            } else {
                Err(glib::Error::from_glib_full(error))
            }
        }
    }

    #[doc(alias = "nm_remote_connection_get_secrets_async")]
    #[doc(alias = "get_secrets_async")]
    fn secrets_async<P: FnOnce(Result<glib::Variant, glib::Error>) + 'static>(
        &self,
        setting_name: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box<glib::thread_guard::ThreadGuard<P>> =
            Box::new(glib::thread_guard::ThreadGuard::new(callback));

        unsafe extern "C" fn secrets_async_trampoline<
            P: FnOnce(Result<glib::Variant, glib::Error>) + 'static,
        >(
            _connection: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets_finish(
                _connection as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<glib::thread_guard::ThreadGuard<P>> =
                Box::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = secrets_async_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_get_secrets_async(
                self.as_ref().to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn secrets_future(
        &self,
        setting_name: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let setting_name = String::from(setting_name);
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.secrets_async(&setting_name, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}

impl<T: IsA<RemoteConnection, GlibType = ffi::NMRemoteConnection>> RemoteConnectionExtManual for T {}

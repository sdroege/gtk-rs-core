// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{SocketConnection, SocketListener, SocketService};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GThreadedSocketService")]
    pub struct ThreadedSocketService(Object<ffi::GThreadedSocketService, ffi::GThreadedSocketServiceClass>) @extends SocketService, SocketListener;

    match fn {
        type_ => || ffi::g_threaded_socket_service_get_type(),
    }
}

impl ThreadedSocketService {
    pub const NONE: Option<&'static ThreadedSocketService> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ThreadedSocketService>> Sealed for T {}
}

pub trait ThreadedSocketServiceExt: IsA<ThreadedSocketService> + sealed::Sealed + 'static {
    #[doc(alias = "max-threads")]
    fn max_threads(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "max-threads")
    }

    #[doc(alias = "run")]
    fn connect_run<F: Fn(&Self, &SocketConnection, Option<&glib::Object>) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn run_trampoline<
            P: IsA<ThreadedSocketService>,
            F: Fn(&P, &SocketConnection, Option<&glib::Object>) -> bool + 'static,
        >(
            this: *mut ffi::GThreadedSocketService,
            connection: *mut ffi::GSocketConnection,
            source_object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                ThreadedSocketService::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                Option::<glib::Object>::from_glib_borrow(source_object)
                    .as_ref()
                    .as_ref(),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"run\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    run_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ThreadedSocketService>> ThreadedSocketServiceExt for O {}

impl fmt::Display for ThreadedSocketService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ThreadedSocketService")
    }
}

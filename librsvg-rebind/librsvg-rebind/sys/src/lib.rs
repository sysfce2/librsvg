// Generated by gir (https://github.com/gtk-rs/gir @ 727e064a9792)
// from ../builddir/rsvg (@ 3f6b01041a94)
// from gir-files (@ 4d1189172a70)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use cairo_sys as cairo;
use gdk_pixbuf_sys as gdk_pixbuf;
use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type RsvgError = c_int;
pub const RSVG_ERROR_FAILED: RsvgError = 0;

pub type RsvgUnit = c_int;
pub const RSVG_UNIT_PERCENT: RsvgUnit = 0;
pub const RSVG_UNIT_PX: RsvgUnit = 1;
pub const RSVG_UNIT_EM: RsvgUnit = 2;
pub const RSVG_UNIT_EX: RsvgUnit = 3;
pub const RSVG_UNIT_IN: RsvgUnit = 4;
pub const RSVG_UNIT_CM: RsvgUnit = 5;
pub const RSVG_UNIT_MM: RsvgUnit = 6;
pub const RSVG_UNIT_PT: RsvgUnit = 7;
pub const RSVG_UNIT_PC: RsvgUnit = 8;
#[cfg(feature = "v2_58")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
pub const RSVG_UNIT_CH: RsvgUnit = 9;

// Constants

// Flags
pub type RsvgHandleFlags = c_uint;
pub const RSVG_HANDLE_FLAGS_NONE: RsvgHandleFlags = 0;
pub const RSVG_HANDLE_FLAG_UNLIMITED: RsvgHandleFlags = 1;
pub const RSVG_HANDLE_FLAG_KEEP_IMAGE_DATA: RsvgHandleFlags = 2;

// Callbacks
pub type RsvgSizeFunc = Option<unsafe extern "C" fn(*mut c_int, *mut c_int, gpointer)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgDimensionData {
    pub width: c_int,
    pub height: c_int,
    pub em: c_double,
    pub ex: c_double,
}

impl ::std::fmt::Debug for RsvgDimensionData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgDimensionData @ {self:p}"))
            .field("width", &self.width)
            .field("height", &self.height)
            .field("em", &self.em)
            .field("ex", &self.ex)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgHandleClass {
    pub parent: gobject::GObjectClass,
    pub _abi_padding: [gpointer; 15],
}

impl ::std::fmt::Debug for RsvgHandleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgHandleClass @ {self:p}"))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgLength {
    pub length: c_double,
    pub unit: RsvgUnit,
}

impl ::std::fmt::Debug for RsvgLength {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgLength @ {self:p}"))
            .field("length", &self.length)
            .field("unit", &self.unit)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgPositionData {
    pub x: c_int,
    pub y: c_int,
}

impl ::std::fmt::Debug for RsvgPositionData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgPositionData @ {self:p}"))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgRectangle {
    pub x: c_double,
    pub y: c_double,
    pub width: c_double,
    pub height: c_double,
}

impl ::std::fmt::Debug for RsvgRectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgRectangle @ {self:p}"))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RsvgHandle {
    pub parent: gobject::GObject,
    pub _abi_padding: [gpointer; 16],
}

impl ::std::fmt::Debug for RsvgHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgHandle @ {self:p}"))
            .field("parent", &self.parent)
            .finish()
    }
}

#[link(name = "rsvg-2")]
extern "C" {

    //=========================================================================
    // RsvgError
    //=========================================================================
    pub fn rsvg_error_get_type() -> GType;
    pub fn rsvg_error_quark() -> glib::GQuark;

    //=========================================================================
    // RsvgHandleFlags
    //=========================================================================
    pub fn rsvg_handle_flags_get_type() -> GType;

    //=========================================================================
    // RsvgHandle
    //=========================================================================
    pub fn rsvg_handle_get_type() -> GType;
    pub fn rsvg_handle_new() -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_data(
        data: *const u8,
        data_len: size_t,
        error: *mut *mut glib::GError,
    ) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_gfile_sync(
        file: *mut gio::GFile,
        flags: RsvgHandleFlags,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_stream_sync(
        input_stream: *mut gio::GInputStream,
        base_file: *mut gio::GFile,
        flags: RsvgHandleFlags,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_with_flags(flags: RsvgHandleFlags) -> *mut RsvgHandle;
    pub fn rsvg_handle_close(handle: *mut RsvgHandle, error: *mut *mut glib::GError) -> gboolean;
    pub fn rsvg_handle_free(handle: *mut RsvgHandle);
    pub fn rsvg_handle_get_base_uri(handle: *mut RsvgHandle) -> *const c_char;
    pub fn rsvg_handle_get_desc(handle: *mut RsvgHandle) -> *const c_char;
    pub fn rsvg_handle_get_dimensions(
        handle: *mut RsvgHandle,
        dimension_data: *mut RsvgDimensionData,
    );
    pub fn rsvg_handle_get_dimensions_sub(
        handle: *mut RsvgHandle,
        dimension_data: *mut RsvgDimensionData,
        id: *const c_char,
    ) -> gboolean;
    pub fn rsvg_handle_get_geometry_for_element(
        handle: *mut RsvgHandle,
        id: *const c_char,
        out_ink_rect: *mut RsvgRectangle,
        out_logical_rect: *mut RsvgRectangle,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_get_geometry_for_layer(
        handle: *mut RsvgHandle,
        id: *const c_char,
        viewport: *const RsvgRectangle,
        out_ink_rect: *mut RsvgRectangle,
        out_logical_rect: *mut RsvgRectangle,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_get_intrinsic_dimensions(
        handle: *mut RsvgHandle,
        out_has_width: *mut gboolean,
        out_width: *mut RsvgLength,
        out_has_height: *mut gboolean,
        out_height: *mut RsvgLength,
        out_has_viewbox: *mut gboolean,
        out_viewbox: *mut RsvgRectangle,
    );
    pub fn rsvg_handle_get_intrinsic_size_in_pixels(
        handle: *mut RsvgHandle,
        out_width: *mut c_double,
        out_height: *mut c_double,
    ) -> gboolean;
    pub fn rsvg_handle_get_metadata(handle: *mut RsvgHandle) -> *const c_char;
    pub fn rsvg_handle_get_pixbuf(handle: *mut RsvgHandle) -> *mut gdk_pixbuf::GdkPixbuf;
    #[cfg(feature = "v2_58")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
    pub fn rsvg_handle_get_pixbuf_and_error(
        handle: *mut RsvgHandle,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_handle_get_pixbuf_sub(
        handle: *mut RsvgHandle,
        id: *const c_char,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_handle_get_position_sub(
        handle: *mut RsvgHandle,
        position_data: *mut RsvgPositionData,
        id: *const c_char,
    ) -> gboolean;
    pub fn rsvg_handle_get_title(handle: *mut RsvgHandle) -> *const c_char;
    pub fn rsvg_handle_has_sub(handle: *mut RsvgHandle, id: *const c_char) -> gboolean;
    pub fn rsvg_handle_internal_set_testing(handle: *mut RsvgHandle, testing: gboolean);
    pub fn rsvg_handle_read_stream_sync(
        handle: *mut RsvgHandle,
        stream: *mut gio::GInputStream,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_render_cairo(handle: *mut RsvgHandle, cr: *mut cairo::cairo_t) -> gboolean;
    pub fn rsvg_handle_render_cairo_sub(
        handle: *mut RsvgHandle,
        cr: *mut cairo::cairo_t,
        id: *const c_char,
    ) -> gboolean;
    pub fn rsvg_handle_render_document(
        handle: *mut RsvgHandle,
        cr: *mut cairo::cairo_t,
        viewport: *const RsvgRectangle,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_render_element(
        handle: *mut RsvgHandle,
        cr: *mut cairo::cairo_t,
        id: *const c_char,
        element_viewport: *const RsvgRectangle,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_render_layer(
        handle: *mut RsvgHandle,
        cr: *mut cairo::cairo_t,
        id: *const c_char,
        viewport: *const RsvgRectangle,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_set_base_gfile(handle: *mut RsvgHandle, base_file: *mut gio::GFile);
    pub fn rsvg_handle_set_base_uri(handle: *mut RsvgHandle, base_uri: *const c_char);
    pub fn rsvg_handle_set_cancellable_for_rendering(
        handle: *mut RsvgHandle,
        cancellable: *mut gio::GCancellable,
    );
    pub fn rsvg_handle_set_dpi(handle: *mut RsvgHandle, dpi: c_double);
    pub fn rsvg_handle_set_dpi_x_y(handle: *mut RsvgHandle, dpi_x: c_double, dpi_y: c_double);
    pub fn rsvg_handle_set_size_callback(
        handle: *mut RsvgHandle,
        size_func: RsvgSizeFunc,
        user_data: gpointer,
        user_data_destroy: glib::GDestroyNotify,
    );
    pub fn rsvg_handle_set_stylesheet(
        handle: *mut RsvgHandle,
        css: *const u8,
        css_len: size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn rsvg_handle_write(
        handle: *mut RsvgHandle,
        buf: *const u8,
        count: size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn rsvg_cleanup();
    pub fn rsvg_init();
    pub fn rsvg_pixbuf_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_pixbuf_from_file_at_max_size(
        filename: *const c_char,
        max_width: c_int,
        max_height: c_int,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_pixbuf_from_file_at_size(
        filename: *const c_char,
        width: c_int,
        height: c_int,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_pixbuf_from_file_at_zoom(
        filename: *const c_char,
        x_zoom: c_double,
        y_zoom: c_double,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_pixbuf_from_file_at_zoom_with_max(
        filename: *const c_char,
        x_zoom: c_double,
        y_zoom: c_double,
        max_width: c_int,
        max_height: c_int,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_set_default_dpi(dpi: c_double);
    pub fn rsvg_set_default_dpi_x_y(dpi_x: c_double, dpi_y: c_double);
    pub fn rsvg_term();

}

// This file was generated by gir (9bd51ed) from gir-files (ec4c204)
// DO NOT EDIT

mod handle;
pub use self::handle::Handle;
pub use self::handle::HandleExt;

mod flags;
pub use self::flags::HandleFlags;
pub use self::flags::HANDLE_FLAGS_NONE;
pub use self::flags::HANDLE_FLAG_UNLIMITED;
pub use self::flags::HANDLE_FLAG_KEEP_IMAGE_DATA;

#[doc(hidden)]
pub mod traits {
    pub use super::HandleExt;
}

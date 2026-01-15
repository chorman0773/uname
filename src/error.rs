use error_repr::kind::FromRawOsError;
#[cfg(feature = "std")]
use error_repr::kind::{FromIoKind, IntoIoKind};

/// [`ErrorKind`][error_repr::kind::ErrorKind] for [`Error`][crate::Error]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Indicates that the current platform is not supported.
    Unsupported,
    /// Indicates that the operation failed for other reasons
    Other,

    #[doc(hidden)]
    __Uncategorized,
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::Unsupported => f.write_str("Unsupported System"),
            ErrorKind::Other => f.write_str("Other"),
            ErrorKind::__Uncategorized => todo!(),
        }
    }
}

impl error_repr::kind::ErrorKind for ErrorKind {
    const OTHER: Self = ErrorKind::Other;

    fn uncategorized() -> Self {
        ErrorKind::__Uncategorized
    }
}

impl FromRawOsError for ErrorKind {
    fn from_raw_os_error(raw: error_repr::RawOsError) -> Self {
        cfg_match::cfg_match! {
            target_os = "lilium" => ({
                match raw {
                    lilium_sys::sys::result::errors::INVALID_OPTION | lilium_sys::sys::result::errors::UNSUPPORTED_KERNEL_FUNCTION => ErrorKind::Unsupported,
                    _ => ErrorKind::__Uncategorized,
                }
            }),
            target_family = "unix" => ({
                match raw {
                    libc::ENOSYS => ErrorKind::Unsupported,
                    _ => ErrorKind::__Uncategorized
                }
            }),
            target_family = "windows" => ErrorKind::__Uncategorized, // I don't think there are any others yet
            _ => if raw == 0xDEADBEEF { // Surely this error code will never show up on a real OS :ferrisClueless:
                ErrorKind::Unsupported
            } else {
                ErrorKind::__Uncategorized
            }, // Catch all for non-standard OS
        }
    }
}

#[cfg(feature = "std")]
impl FromIoKind for ErrorKind {
    fn from_io_error_kind(kind: std::io::ErrorKind) -> Self {
        match kind {
            std::io::ErrorKind::Unsupported => ErrorKind::Unsupported,
            std::io::ErrorKind::Other => ErrorKind::Other,
            _ => ErrorKind::__Uncategorized,
        }
    }
}

#[cfg(feature = "std")]
impl IntoIoKind for ErrorKind {
    fn into_io_error_kind(self) -> std::io::ErrorKind {
        match self {
            ErrorKind::Unsupported => std::io::ErrorKind::Unsupported,
            ErrorKind::Other => std::io::ErrorKind::Other,
            ErrorKind::__Uncategorized => std::io::ErrorKind::Other,
        }
    }
}

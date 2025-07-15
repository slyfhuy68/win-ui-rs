pub use std::result::Result as sResult;
use std::string::FromUtf8Error;
use windows_sys::Win32::Foundation as win32f;
pub type Result<T> = sResult<T, WinError>;
use std::fmt::Debug;
use std::string::FromUtf16Error;
use windows_sys::Win32::Foundation::{GetLastError, NTSTATUS, WIN32_ERROR};
use windows_sys::core::*;
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct WinError(WinErrorKind);
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum WinErrorKind {
    ///这一项应该是负的
    Win32(HRESULT),
    Local(u32),
}
use WinErrorKind::*;
use std::error::Error;
impl WinError {
    pub fn from_error_kind(kind: std::io::ErrorKind) -> Option<Self> {
        use std::io::ErrorKind::*;
        Some(match kind {
            NotFound => ERROR_NOT_FOUND,
            PermissionDenied => ERROR_ACCESS_DENIED,
            ConnectionReset => ERROR_CONNECTION_RESET,
            HostUnreachable => ERROR_HOST_UNREACHABLE,
            NetworkUnreachable => ERROR_NETWORK_UNREACHABLE,
            ConnectionAborted => ERROR_CONNECTION_ABORTED,
            AddrInUse => ERROR_ADDRESS_ALREADY_ASSOCIATED,
            AddrNotAvailable => ERROR_ADDRESS_NOT_AVAILABLE,
            NetworkDown => ERROR_NETWORK_DOWN,
            BrokenPipe => ERROR_BROKEN_PIPE,
            AlreadyExists => ERROR_ALREADY_EXISTS,
            WouldBlock => ERROR_WOULD_BLOCK,
            NotADirectory => ERROR_DIRECTORY_NAME_INVALID,
            IsADirectory => ERROR_DIRECTORY,
            DirectoryNotEmpty => ERROR_DIR_NOT_EMPTY,
            ReadOnlyFilesystem => ERROR_WRITE_PROTECT,
            StaleNetworkFileHandle => ERROR_NET_OPEN_FAILED,
            InvalidInput => ERROR_INVALID_PARAMETER,
            InvalidData => ERROR_INVALID_DATA,
            StorageFull => ERROR_DISK_FULL,
            QuotaExceeded => ERROR_DISK_QUOTA_EXCEEDED,
            FileTooLarge => ERROR_FILE_TOO_LARGE,
            Deadlock => ERROR_LOCK_VIOLATION,
            // InvalidFilename => ERROR_INVALID_FILENAME,
            ArgumentListTooLong => ERROR_INVALID_PARAMETER,
            Interrupted => ERROR_OPERATION_ABORTED,
            OutOfMemory => ERROR_NOT_ENOUGH_MEMORY,
            // InProgress => ERROR_IO_PENDING,
            ConnectionRefused => ERROR_CONNECTION_REFUSED,
            NotConnected => ERROR_NOT_CONNECTED,
            // FilesystemLoop => ERROR_FILESYSTEM_LOOP,
            TimedOut => ERROR_TIMED_OUT,
            WriteZero => ERROR_NOT_SUPPORT_ZERO,
            NotSeekable => ERROR_NOT_SEEKABLE,
            ResourceBusy => ERROR_RESOURCE_BUSY,
            ExecutableFileBusy => ERROR_EXECUTABLE_FILE_BUSY,
            CrossesDevices => ERROR_CROSSES_DEVICES,
            TooManyLinks => ERROR_TOO_MANY_LINKS,
            Unsupported => ERROR_NOT_SUPPORTED,
            UnexpectedEof => ERROR_UNEXPECTED_EOF,
            _ => return None,
        })
    }
    #[inline]
    pub const fn code(&self) -> i32 {
        match self.0 {
            Win32(i) => i,
            Local(u) => u as i32,
        }
    }
    #[inline]
    pub fn correct_error_result<T>(data: T) -> Result<T> {
        #[allow(unused_unsafe)]
        let error = unsafe { GetLastError() };
        if error == 0 {
            Ok(data)
        } else {
            Err(unsafe { Self::from_win32(error) })
        }
    }
    #[inline]
    pub fn from_win32api_result(error: i32) -> Result<()> {
        #[allow(unused_unsafe)]
        if error == 0 {
            Ok(())
        } else {
            Err(unsafe { Self::from_win32(unsafe { GetLastError() }) })
        }
    }
    #[inline]
    pub fn from_win32api_ptr(ptr: *mut std::ffi::c_void) -> Result<*mut std::ffi::c_void> {
        #[allow(unused_unsafe)]
        if ptr.addr() != 0 {
            Ok(ptr)
        } else {
            Err(unsafe { Self::from_win32(unsafe { GetLastError() }) })
        }
    }
    #[inline]
    pub fn from_win32api_maybe_zero(ptr: *mut std::ffi::c_void) -> Result<*mut std::ffi::c_void> {
        #[allow(unused_unsafe)]
        if ptr.addr() != 0 {
            Ok(ptr)
        } else {
            Self::correct_error_result(ptr)
        }
    }
    #[inline]
    ///不检查当前错误是不是0
    pub unsafe fn correct_error() -> Self {
        unsafe { Self::from_win32(GetLastError()) }
    }
    #[inline]
    pub const unsafe fn from_win32(error: WIN32_ERROR) -> Self {
        Self(Win32(if (error as i32) < 0 {
            error
        } else {
            (error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
        } as i32))
    }
    #[inline]
    pub const unsafe fn from_nt(error: NTSTATUS) -> Self {
        Self(Win32(if error >= 0 {
            error
        } else {
            error | 0x1000_0000
        }))
    }
    #[inline]
    pub const fn from_local(code: u32) -> Self {
        Self(Local(code))
    }
    #[inline]
    pub fn from_hresult_with<T>(err: HRESULT, data: T) -> Result<T> {
        if err >= 0 {
            Ok(data)
        } else {
            Err(Self(Win32(err)))
        }
    }
}
impl Error for WinError {}
use std::fmt;
impl fmt::Display for WinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
use std::num::TryFromIntError;
impl From<TryFromIntError> for WinError {
    #[inline]
    fn from(_: TryFromIntError) -> Self {
        ERROR_INT_OVERFLOW
    }
}
impl From<FromUtf8Error> for WinError {
    #[inline]
    fn from(_: FromUtf8Error) -> Self {
        ERROR_NO_UNICODE_TRANSLATION
    }
}
impl From<FromUtf16Error> for WinError {
    #[inline]
    fn from(_: FromUtf16Error) -> Self {
        ERROR_NO_UNICODE_TRANSLATION
    }
}
macro_rules! def_windows_error {
    ($($name:ident),* $(,)?) => {
        $(
            pub const $name: WinError = unsafe {WinError::from_win32(win32f::$name)};
        )*
    };
}
macro_rules! def_local_error {
    ($($name:ident => $num:expr),* $(,)?) => {
        $(
            pub const $name: WinError = WinError::from_local($num);
        )*
    };
}
pub mod win32_errors;
pub use win32_errors::{
    ERROR_ACCESS_DENIED, ERROR_ADDRESS_ALREADY_ASSOCIATED, ERROR_ALREADY_EXISTS, ERROR_BROKEN_PIPE,
    ERROR_CONNECTION_ABORTED, ERROR_DIR_NOT_EMPTY, ERROR_DIRECTORY, ERROR_DISK_FULL,
    ERROR_DISK_QUOTA_EXCEEDED, ERROR_FILE_TOO_LARGE, ERROR_HANDLE_EOF, ERROR_HOST_UNREACHABLE,
    ERROR_INCORRECT_ADDRESS, ERROR_INVALID_ADDRESS, ERROR_INVALID_DATA, ERROR_INVALID_PARAMETER,
    ERROR_INVALID_WINDOW_HANDLE, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, ERROR_LOCK_VIOLATION,
    ERROR_NET_OPEN_FAILED, ERROR_NETWORK_UNREACHABLE, ERROR_NO_UNICODE_TRANSLATION,
    ERROR_NOT_ENOUGH_MEMORY, ERROR_NOT_FOUND, ERROR_NOT_SUPPORTED, ERROR_OBJECT_ALREADY_EXISTS,
    ERROR_OPERATION_ABORTED, ERROR_OUTOFMEMORY, ERROR_TIMEOUT, ERROR_WRITE_PROTECT,
};
#[rustfmt::skip]
def_local_error! {
	ERROR_CLASS_NAME_TOO_LONG       => 01,
	ERROR_TIME_TOO_LONG             => 02,
	ERROR_INT_OVERFLOW              => 03,
	ERROR_INVALID_RESOURCE_ID       => 04,
	ERROR_NULL_POINTER              => 05,
	ERROR_MSG_CODE_NOT_SUPPORT      => 06,
	ERROR_NOT_SUPPORT_ZERO          => 07,
	ERROR_NOT_PRESENT               => 08,
	ERROR_CANNOT_REMOVE_DEFAULT     => 09,
	ERROR_WINDOW_TYPE_NOT_SUPPORT   => 10,
	ERROR_INVALID_STRING_ID         => 11,
	ERROR_CONNECTION_REFUSED        => 12,
	ERROR_NOT_CONNECTED             => 13,
	ERROR_FILESYSTEM_LOOP           => 14,
	ERROR_TIMED_OUT                 => 15,
	ERROR_UNEXPECTED_EOF            => 16,
	ERROR_NOT_SEEKABLE              => 17,
	ERROR_RESOURCE_BUSY             => 18,
	ERROR_EXECUTABLE_FILE_BUSY      => 19,
	ERROR_CROSSES_DEVICES           => 20,
	ERROR_TOO_MANY_LINKS            => 21,
	ERROR_COMBO_BOX_ERR             => 22,
	ERROR_CONNECTION_RESET          => 23,
	ERROR_NETWORK_DOWN              => 24,
	ERROR_ADDRESS_NOT_AVAILABLE     => 25,
	ERROR_WOULD_BLOCK               => 26,
	ERROR_DIRECTORY_NAME_INVALID    => 27,
	ERROR_INVALID_COMBINE           => 28,
	ERROR_MUSTNOT_CHILD             => 29,
	ERROR_NOT_FOUND_MENU            => 30,
	ERROR_INSUFFICIENT_SPACE        => 31,
}

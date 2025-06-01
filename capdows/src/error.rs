pub use std::result::Result as sResult;
use std::string::FromUtf8Error;
use windows::Win32::Foundation as win32f;
pub type Result<T> = sResult<T, WinError>;
use std::string::FromUtf16Error;
pub fn correct_error_data<T>(data: T) -> Result<T> {
    WinError::correct_error_data(data)
}
pub fn correct_error() -> WinError {
    WinError::correct_error()
}
pub fn correct_error_result<T>(data: T) -> Result<T> {
    let code = wError::from_win32().code();
    if code.is_ok() {
        Ok(data)
    } else {
        Err(code.into())
    }
}
use std::fmt::Debug;
use windows::{
    Win32::Foundation::{GetLastError, NTSTATUS, WIN32_ERROR},
    core::{Error as wError, HRESULT},
};
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct WinError(WinErrorKind);
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum WinErrorKind {
    Win32(i32),
    Local(u32),
    NotSupport,
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
    pub const fn code(&self) -> i32 {
        match self.0 {
            Win32(i) => i,
            Local(u) => u as i32,
            WinErrorKind::NotSupport => -1554,
        }
    }
    pub fn correct_error_data<T>(data: T) -> Result<T> {
        #[allow(unused_unsafe)]
        let error = unsafe { GetLastError() };
        if error.is_ok() {
            Ok(data)
        } else {
            Err(Self::from_win32(error))
        }
    }
    pub fn correct_error() -> Self {
        wError::from_win32().into()
    }
    pub const fn from_win32(werror: WIN32_ERROR) -> Self {
        let WIN32_ERROR(error) = werror;
        Self(Win32(if error as i32 <= 0 {
            error
        } else {
            (error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
        } as i32))
    }
    pub const fn from_nt(nerror: NTSTATUS) -> Self {
        let NTSTATUS(error) = nerror;
        Self(Win32(if error >= 0 {
            error
        } else {
            error | 0x1000_0000
        }))
    }
    pub const fn from_local(code: u32) -> Self {
        Self(Local(code))
    }
}
impl Error for WinError {}
use std::fmt;
impl fmt::Display for WinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl From<WIN32_ERROR> for WinError {
    fn from(err: WIN32_ERROR) -> Self {
        Self::from_win32(err)
    }
}
impl From<wError> for WinError {
    fn from(err: wError) -> Self {
        err.code().into()
    }
}
impl From<NTSTATUS> for WinError {
    fn from(err: NTSTATUS) -> Self {
        Self::from_nt(err)
    }
}
impl From<HRESULT> for WinError {
    fn from(err: HRESULT) -> Self {
        Self(Win32(err.0))
    }
}
use std::num::TryFromIntError;
impl From<TryFromIntError> for WinError {
    fn from(_: TryFromIntError) -> Self {
        ERROR_INT_OVERFLOW
    }
}
impl From<std::io::Error> for WinError {
    fn from(from: std::io::Error) -> Self {
        match from.raw_os_error() {
            Some(status) => HRESULT::from_win32(status as u32).into(),
            None => match WinError::from_error_kind(from.kind()) {
                Some(s) => s,
                None => Self(NotSupport),
            },
        }
    }
}
impl From<FromUtf8Error> for WinError {
    fn from(_: FromUtf8Error) -> Self {
        ERROR_NO_UNICODE_TRANSLATION
    }
}
impl From<FromUtf16Error> for WinError {
    fn from(_: FromUtf16Error) -> Self {
        ERROR_NO_UNICODE_TRANSLATION
    }
}
macro_rules! def_windows_error {
    ($($name:ident),* $(,)?) => {
        $(
            pub const $name: WinError = WinError::from_win32(win32f::$name);
        )*
    };
}
pub mod errors {
    use super::*;
    def_windows_error!(
        ERROR_NO_UNICODE_TRANSLATION,
        ERROR_INVALID_WINDOW_HANDLE,
        ERROR_INVALID_DATA,
        ERROR_NOT_ENOUGH_MEMORY,
        ERROR_OBJECT_ALREADY_EXISTS,
        ERROR_NOT_SUPPORTED,
        ERROR_HOST_UNREACHABLE,
        ERROR_NETWORK_UNREACHABLE,
        ERROR_CONNECTION_ABORTED,
        ERROR_INCORRECT_ADDRESS,
        ERROR_INVALID_ADDRESS,
        ERROR_TIMEOUT,
        ERROR_DISK_FULL,
        ERROR_DIR_NOT_EMPTY,
        ERROR_OPERATION_ABORTED,
        ERROR_HANDLE_EOF,
        ERROR_OUTOFMEMORY,
        ERROR_IO_INCOMPLETE,
        ERROR_NOT_FOUND,
        ERROR_ACCESS_DENIED,
        ERROR_ADDRESS_ALREADY_ASSOCIATED,
        ERROR_BROKEN_PIPE,
        ERROR_ALREADY_EXISTS,
        ERROR_DIRECTORY,
        ERROR_WRITE_PROTECT,
        ERROR_NET_OPEN_FAILED,
        ERROR_INVALID_PARAMETER,
        ERROR_DISK_QUOTA_EXCEEDED,
        ERROR_FILE_TOO_LARGE,
        ERROR_LOCK_VIOLATION,
        ERROR_IO_PENDING,
    );
    //---------自定义----------------------------------------------------------------------------
    pub const ERROR_CLASS_NAME_TOO_LONG: WinError = WinError::from_local(1);
    pub const ERROR_TIME_TOO_LONG: WinError = WinError::from_local(2);
    pub const ERROR_INT_OVERFLOW: WinError = WinError::from_local(3);
    pub const ERROR_INVALID_RESOURCE_ID: WinError = WinError::from_local(4);
    pub const ERROR_NULL_POINTER: WinError = WinError::from_local(5);
    pub const ERROR_MSG_CODE_NOT_SUPPORT: WinError = WinError::from_local(6);
    pub const ERROR_NOT_SUPPORT_ZERO: WinError = WinError::from_local(7);
    pub const ERROR_NOT_PRESENT: WinError = WinError::from_local(8);
    pub const ERROR_CANNOT_REMOVE_DEFAULT: WinError = WinError::from_local(9);
    pub const ERROR_WINDOW_TYPE_NOT_SUPPORT: WinError = WinError::from_local(10);
    pub const ERROR_INVALID_STRING_ID: WinError = WinError::from_local(11);
    pub const ERROR_CONNECTION_REFUSED: WinError = WinError::from_local(12);
    pub const ERROR_NOT_CONNECTED: WinError = WinError::from_local(13);
    pub const ERROR_FILESYSTEM_LOOP: WinError = WinError::from_local(14);
    pub const ERROR_TIMED_OUT: WinError = WinError::from_local(15);
    pub const ERROR_UNEXPECTED_EOF: WinError = WinError::from_local(16);
    pub const ERROR_NOT_SEEKABLE: WinError = WinError::from_local(17);
    pub const ERROR_RESOURCE_BUSY: WinError = WinError::from_local(18);
    pub const ERROR_EXECUTABLE_FILE_BUSY: WinError = WinError::from_local(19);
    pub const ERROR_CROSSES_DEVICES: WinError = WinError::from_local(20);
    pub const ERROR_TOO_MANY_LINKS: WinError = WinError::from_local(21);
    // ERROR_UNSUPPORTED = ERROR_NOT_SUPPORTED;
    pub const ERROR_COMBO_BOX_ERR: WinError = WinError::from_local(22);
    pub const ERROR_CONNECTION_RESET: WinError = WinError::from_local(23);
    pub const ERROR_NETWORK_DOWN: WinError = WinError::from_local(24);
    pub const ERROR_ADDRESS_NOT_AVAILABLE: WinError = WinError::from_local(25);
    pub const ERROR_WOULD_BLOCK: WinError = WinError::from_local(26);
    pub const ERROR_DIRECTORY_NAME_INVALID: WinError = WinError::from_local(27);
    pub const ERROR_INVALID_COMBINE: WinError = WinError::from_local(28);
    pub const ERROR_MUSTNOT_CHILD: WinError = WinError::from_local(29);
    pub const ERROR_NOT_HAVE_MENU: WinError = WinError::from_local(30);
}
use errors::*;

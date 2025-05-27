use super::*;

pub fn msg_box(
    text: &str,
    caption: Option<&str>,
    owner: MessageBoxOwnerWindow,
    options: MessageBoxOptions,
) -> Result<MessageBoxResult> {
    let (text, _buffer1) = str_to_pcwstr(text);
    let (caption, _buffer2) = match caption {
        None => (PCWSTR::null(), Vec::new()),
        Some(s) => str_to_pcwstr(s),
    };
    let (style1, lang_id) = options.into();
    let (style2, hwnd) = owner.into();
    match unsafe { MessageBoxExW(Some(hwnd), text, caption, style1 | style2, lang_id) } {
        MESSAGEBOX_RESULT(0) => Err(correct_error()),
        x => Ok(x.try_into()?),
    }
}

#[cfg(feature = "timeout_msg_box")]
pub fn msg_box_timeout(
    text: &str,
    caption: Option<&str>,
    time_out: Option<Duration>,
    owner: MessageBoxOwnerWindow,
    options: MessageBoxOptions,
) -> Result<MessageBoxResult> {
    let (text, _buffer1) = str_to_pcwstr(text);
    let (caption, _buffer2) = match caption {
        None => (PCWSTR::null(), Vec::new()),
        Some(s) => str_to_pcwstr(s),
    };
    let (style1, lang_id) = options.into();
    let (style2, hwnd) = owner.into();
    match unsafe { MessageBoxTimeoutW(hwnd, text, caption, style1 | style2, lang_id) } {
        MESSAGEBOX_RESULT(0) => Err(correct_error()),
        x => Ok(x.try_into()?),
    }
}
#[macro_export]
macro_rules! msg_box {
    // 最简形式: text
    ($text:expr) => {
        $crate::win32::tools::msg_box($text, None, Default::default(), Default::default())
    };

    // 支持 caption
    ($text:expr, $caption:expr) => {
        $crate::win32::tools::msg_box(
            $text,
            Some($caption),
            Default::default(),
            Default::default(),
        )
    };

    // 支持 owner
    ($text:expr, $caption:expr, $owner:expr) => {
        $crate::win32::tools::msg_box($text, Some($caption), $owner, Default::default())
    };

    // 完整形式（所有参数）
    ($text:expr, $caption:expr, $owner:expr, $options:expr) => {
        $crate::win32::tools::msg_box($text, Some($caption), $owner, $options)
    };
}
windows_link::link!("user32.dll" "system" fn MessageBoxTimeoutW(hwnd : HWND, lptext : windows::core::PCWSTR, lpcaption : windows::core::PCWSTR, utype : MESSAGEBOX_STYLE, wlanguageid : u16, dwMilliseconds: u32) -> MESSAGEBOX_RESULT);
pub mod msg_box_style {
    use super::HWND;
    use crate::win32::LangID;
    use crate::win32::Window;
    use std::ffi::c_void;
    use windows::Win32::UI::WindowsAndMessaging::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum MessageBoxButton {
        #[default]
        OkOnly,
        OkCancel,
        RetryCancel,
        YesNo,
        YesNoCancel,
        AbortRetryIgnore,
        CancelTryContinue,
    }
    impl From<MESSAGEBOX_STYLE> for MessageBoxButton {
        fn from(value: MESSAGEBOX_STYLE) -> Self {
            if value.contains(MB_YESNO) {
                Self::YesNo
            } else if value.contains(MB_OKCANCEL) {
                Self::OkCancel
            } else if value.contains(MB_RETRYCANCEL) {
                Self::RetryCancel
            } else if value.contains(MB_YESNOCANCEL) {
                Self::YesNoCancel
            } else if value.contains(MB_ABORTRETRYIGNORE) {
                Self::AbortRetryIgnore
            } else if value.contains(MB_CANCELTRYCONTINUE) {
                Self::CancelTryContinue
            } else {
                Self::OkOnly
            }
        }
    }
    impl Into<MESSAGEBOX_STYLE> for MessageBoxButton {
        fn into(self) -> MESSAGEBOX_STYLE {
            match self {
                Self::OkOnly => MB_OK,
                Self::OkCancel => MB_OKCANCEL,
                Self::RetryCancel => MB_RETRYCANCEL,
                Self::YesNo => MB_YESNO,
                Self::YesNoCancel => MB_YESNOCANCEL,
                Self::AbortRetryIgnore => MB_ABORTRETRYIGNORE,
                Self::CancelTryContinue => MB_CANCELTRYCONTINUE,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum MessageBoxIcon {
        #[default]
        None,
        Information,
        Question,
        Warning,
        Error,
    }

    impl Into<MESSAGEBOX_STYLE> for MessageBoxIcon {
        fn into(self) -> MESSAGEBOX_STYLE {
            match self {
                Self::None => MESSAGEBOX_STYLE(0),
                Self::Information => MB_ICONINFORMATION,
                Self::Question => MB_ICONQUESTION,
                Self::Warning => MB_ICONEXCLAMATION,
                Self::Error => MB_ICONSTOP,
            }
        }
    }

    impl From<MESSAGEBOX_STYLE> for MessageBoxIcon {
        fn from(style: MESSAGEBOX_STYLE) -> Self {
            if style.contains(MB_ICONINFORMATION) {
                Self::Information
            } else if style.contains(MB_ICONQUESTION) {
                Self::Question
            } else if style.contains(MB_ICONWARNING) {
                Self::Warning
            } else if style.contains(MB_ICONERROR) {
                Self::Error
            } else {
                Self::None
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum MessageBoxDefaultButton {
        #[default]
        Button1,
        Button2,
        Button3,
        Button4,
    }
    impl Into<MESSAGEBOX_STYLE> for MessageBoxDefaultButton {
        fn into(self) -> MESSAGEBOX_STYLE {
            match self {
                Self::Button1 => MB_DEFBUTTON1,
                Self::Button2 => MB_DEFBUTTON2,
                Self::Button3 => MB_DEFBUTTON3,
                Self::Button4 => MB_DEFBUTTON4,
            }
        }
    }
    impl From<MESSAGEBOX_STYLE> for MessageBoxDefaultButton {
        fn from(value: MESSAGEBOX_STYLE) -> Self {
            if value.contains(MB_DEFBUTTON2) {
                Self::Button2
            } else if value.contains(MB_DEFBUTTON3) {
                Self::Button3
            } else if value.contains(MB_DEFBUTTON4) {
                Self::Button4
            } else {
                Self::Button1
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct MessageBoxOptions {
        pub icon: MessageBoxIcon,
        pub button: MessageBoxButton,
        pub default_button: MessageBoxDefaultButton,
        pub default_desktop_only: bool,
        pub help_button: bool,
        pub right_justified: bool,
        pub right_to_left_reading: bool,
        pub top_most: bool,
        pub lang_id: Option<LangID>,
    }
    impl From<(MESSAGEBOX_STYLE, u16)> for MessageBoxOptions {
        fn from((style, lang_id): (MESSAGEBOX_STYLE, u16)) -> Self {
            Self {
                icon: MessageBoxIcon::from(style),
                button: MessageBoxButton::from(style),
                default_button: MessageBoxDefaultButton::from(style),
                default_desktop_only: style.contains(MB_DEFAULT_DESKTOP_ONLY),
                help_button: style.contains(MB_HELP),
                right_justified: style.contains(MB_RIGHT),
                right_to_left_reading: style.contains(MB_RTLREADING),
                top_most: style.contains(MB_TOPMOST),
                lang_id: LangID::from_id(lang_id), //from_id返回Option<LangID>
            }
        }
    }
    impl Into<(MESSAGEBOX_STYLE, u16)> for MessageBoxOptions {
        fn into(self) -> (MESSAGEBOX_STYLE, u16) {
            let mut style: MESSAGEBOX_STYLE = self.button.into();
            style |= self.icon.into();
            style |= self.default_button.into();

            if self.default_desktop_only {
                style |= MB_DEFAULT_DESKTOP_ONLY;
            }

            if self.help_button {
                style |= MB_HELP;
            }

            if self.right_justified {
                style |= MB_RIGHT;
            }

            if self.right_to_left_reading {
                style |= MB_RTLREADING;
            }

            if self.top_most {
                style |= MB_TOPMOST;
            }

            (style, self.lang_id.map(LangID::id).unwrap_or(0))
        }
    }
    #[derive(Debug, Default)]
    pub enum MessageBoxOwnerWindow<'a> {
        #[default]
        None,
        AppModal(&'a Window),
        SystemModal(&'a Window),
        TaskModal(&'a Window),
        ServiceNotification,
    }
    impl<'a> Into<(MESSAGEBOX_STYLE, HWND)> for MessageBoxOwnerWindow<'a> {
        fn into(self) -> (MESSAGEBOX_STYLE, HWND) {
            match self {
                MessageBoxOwnerWindow::None => (MB_APPLMODAL, HWND(0 as *mut c_void)),

                MessageBoxOwnerWindow::AppModal(window) => {
                    (MB_APPLMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::SystemModal(window) => {
                    (MB_SYSTEMMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::TaskModal(window) => {
                    (MB_TASKMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::ServiceNotification => (
                    MB_APPLMODAL | MB_SERVICE_NOTIFICATION,
                    HWND(0 as *mut c_void),
                ),
            }
        }
    }
}
pub use msg_box_style::{MessageBoxOptions, MessageBoxOwnerWindow};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxResult {
    Ok,
    Cancel,
    Yes,
    No,
    Abort,
    Retry,
    Ignore,
    Continue,
    TryAgain,
    #[cfg(feature = "timeout_msg_box")]
    TimeOut,
}
impl Into<MESSAGEBOX_RESULT> for MessageBoxResult {
    fn into(self) -> MESSAGEBOX_RESULT {
        match self {
            MessageBoxResult::Ok => IDOK,
            MessageBoxResult::Cancel => IDCANCEL,
            MessageBoxResult::Yes => IDYES,
            MessageBoxResult::No => IDNO,
            MessageBoxResult::Abort => IDABORT,
            MessageBoxResult::Retry => IDRETRY,
            MessageBoxResult::Ignore => IDIGNORE,
            MessageBoxResult::Continue => IDCONTINUE,
            MessageBoxResult::TryAgain => IDTRYAGAIN,
            #[cfg(feature = "timeout_msg_box")]
            MessageBoxResult::TimeOut => MESSAGEBOX_RESULT(32000),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct InvalidMessageBoxResult;
impl std::error::Error for InvalidMessageBoxResult {}
use std::convert::TryFrom;
impl std::fmt::Display for InvalidMessageBoxResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid MESSAGEBOX_RESULT value")
    }
}
impl From<InvalidMessageBoxResult> for WinError {
    fn from(_: InvalidMessageBoxResult) -> WinError {
        ERROR_INVALID_DATA
    }
}
impl TryFrom<MESSAGEBOX_RESULT> for MessageBoxResult {
    type Error = InvalidMessageBoxResult;

    fn try_from(value: MESSAGEBOX_RESULT) -> sResult<Self, Self::Error> {
        match value {
            IDOK => Ok(Self::Ok),
            IDCANCEL => Ok(Self::Cancel),
            IDYES => Ok(Self::Yes),
            IDNO => Ok(Self::No),
            IDABORT => Ok(Self::Abort),
            IDRETRY => Ok(Self::Retry),
            IDIGNORE => Ok(Self::Ignore),
            IDCONTINUE => Ok(Self::Continue),
            IDTRYAGAIN => Ok(Self::TryAgain),
            #[cfg(feature = "timeout_msg_box")]
            MESSAGEBOX_RESULT(32000) => Ok(Self::TimeOut),
            _ => Err(InvalidMessageBoxResult),
        }
    }
}

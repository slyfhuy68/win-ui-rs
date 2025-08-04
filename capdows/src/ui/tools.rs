use super::*;
use crate::error::WinError;
use crate::error::sResult;
pub fn msg_box(
    text: &str,
    caption: Option<&str>,
    owner: MessageBoxOwnerWindow,
    options: MessageBoxOptions,
) -> Result<MessageBoxResult> {
    let (text, _buffer1) = str_to_pcwstr(text);
    let (caption, _buffer2) = match caption {
        None => (0 as *const u16, Vec::new()),
        Some(s) => str_to_pcwstr(s),
    };
    let (style1, lang_id) = options.into();
    let (style2, hwnd) = owner.into();

    Ok(
        error_from_win32_num!(MessageBoxExW(hwnd, text, caption, style1 | style2, lang_id,))?
            .try_into()?,
    )
}

#[cfg(feature = "timeout_msg_box")]
pub fn msg_box_timeout(
    text: &str,
    caption: Option<&str>,
    time_out: std::time::Duration,
    owner: MessageBoxOwnerWindow,
    options: MessageBoxOptions,
) -> Result<MessageBoxResult> {
    windows_link::link!("user32.dll" "system" fn MessageBoxTimeoutW(hwnd : HWND, lptext : PCWSTR, lpcaption : PCWSTR, utype : MESSAGEBOX_STYLE, wlanguageid : u16, dwMilliseconds: u32) -> MESSAGEBOX_RESULT);
    let (text, _buffer1) = str_to_pcwstr(text);
    let (caption, _buffer2) = match caption {
        None => (0 as *const u16, Vec::new()),
        Some(s) => str_to_pcwstr(s),
    };
    let (style1, lang_id) = options.into();
    let (style2, hwnd) = owner.into();
    Ok(error_from_win32_num!(MessageBoxTimeoutW(
        hwnd,
        text,
        caption,
        style1 | style2,
        lang_id,
        time_out
            .as_millis()
            .try_into()
            .map_err(|_| ERROR_INVALID_DATA)?
    ))?
    .try_into()?)
}
#[macro_export] //AI宏
macro_rules! msg_box {
    ($text:expr) => {
        $crate::ui::tools::msg_box($text, None, Default::default(), Default::default())
    };
    ($text:expr, $caption:expr) => {
        $crate::ui::tools::msg_box(
            $text,
            Some($caption),
            Default::default(),
            Default::default(),
        )
    };
    ($text:expr, $caption:expr, $owner:expr) => {
        $crate::ui::tools::msg_box($text, Some($caption), $owner, Default::default())
    };
    ($text:expr, $caption:expr, $owner:expr, $options:expr) => {
        $crate::ui::tools::msg_box($text, Some($caption), $owner, $options)
    };
}
pub mod msg_box_style {
    use super::*;
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
            if ucontain(value, MB_YESNO) {
                Self::YesNo
            } else if ucontain(value, MB_OKCANCEL) {
                Self::OkCancel
            } else if ucontain(value, MB_RETRYCANCEL) {
                Self::RetryCancel
            } else if ucontain(value, MB_YESNOCANCEL) {
                Self::YesNoCancel
            } else if ucontain(value, MB_ABORTRETRYIGNORE) {
                Self::AbortRetryIgnore
            } else if ucontain(value, MB_CANCELTRYCONTINUE) {
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
                Self::None => 0,
                Self::Information => MB_ICONINFORMATION,
                Self::Question => MB_ICONQUESTION,
                Self::Warning => MB_ICONEXCLAMATION,
                Self::Error => MB_ICONSTOP,
            }
        }
    }

    impl From<MESSAGEBOX_STYLE> for MessageBoxIcon {
        fn from(style: MESSAGEBOX_STYLE) -> Self {
            if ucontain(style, MB_ICONINFORMATION) {
                Self::Information
            } else if ucontain(style, MB_ICONQUESTION) {
                Self::Question
            } else if ucontain(style, MB_ICONWARNING) {
                Self::Warning
            } else if ucontain(style, MB_ICONERROR) {
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
            if ucontain(value, MB_DEFBUTTON2) {
                Self::Button2
            } else if ucontain(value, MB_DEFBUTTON3) {
                Self::Button3
            } else if ucontain(value, MB_DEFBUTTON4) {
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
                default_desktop_only: ucontain(style, MB_DEFAULT_DESKTOP_ONLY),
                help_button: ucontain(style, MB_HELP),
                right_justified: ucontain(style, MB_RIGHT),
                right_to_left_reading: ucontain(style, MB_RTLREADING),
                top_most: ucontain(style, MB_TOPMOST),
                lang_id: LangID::from_id(lang_id), //from_id返回Option<LangID>
            }
        }
    }
    impl Into<(MESSAGEBOX_STYLE, u16)> for MessageBoxOptions {
        fn into(self) -> (MESSAGEBOX_STYLE, u16) {
            let mut style: MESSAGEBOX_STYLE = self.button.into();
            style |= <MessageBoxIcon as Into<MESSAGEBOX_STYLE>>::into(self.icon);
            style |= <MessageBoxDefaultButton as Into<MESSAGEBOX_STYLE>>::into(self.default_button);
            set_style(
                &mut style,
                MB_DEFAULT_DESKTOP_ONLY,
                self.default_desktop_only,
            );
            set_style(&mut style, MB_HELP, self.help_button);
            set_style(&mut style, MB_RIGHT, self.right_justified);
            set_style(&mut style, MB_RTLREADING, self.right_to_left_reading);
            set_style(&mut style, MB_TOPMOST, self.top_most);
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
                MessageBoxOwnerWindow::None => (MB_APPLMODAL, 0 as HWND),

                MessageBoxOwnerWindow::AppModal(window) => {
                    (MB_APPLMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::SystemModal(window) => {
                    (MB_SYSTEMMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::TaskModal(window) => {
                    (MB_TASKMODAL, unsafe { window.handle() })
                }

                MessageBoxOwnerWindow::ServiceNotification => {
                    (MB_APPLMODAL | MB_SERVICE_NOTIFICATION, 0 as HWND)
                }
            }
        }
    }
}
#[doc(no_inline)]
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
            MessageBoxResult::TimeOut => 32000 as MESSAGEBOX_RESULT,
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
            32000 => Ok(Self::TimeOut),
            _ => Err(InvalidMessageBoxResult),
        }
    }
}

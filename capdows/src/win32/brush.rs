use super::*;
#[derive(Clone, PartialEq, Copy)]
pub struct Brush(HANDLE);

impl Brush {
    pub fn is_invalid(&self) -> bool {
        self.0 == NULL_PTR()
    }
}
impl Default for Brush {
    fn default() -> Self {
        Self(NULL_PTR())
    }
}
impl Into<HBRUSH> for Brush {
    fn into(self) -> HBRUSH {
        HBRUSH(self.0)
    }
}
impl From<HBRUSH> for Brush {
    fn from(hb: HBRUSH) -> Self {
        Self(hb.0)
    }
}
// impl std::fmt::Display for Brush {
//     fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"Brush({})",self.0 as usize)
//     }
// }
#[derive(Clone, PartialEq)]
pub enum BrushC {
    Brush(super::brush::Brush),
    //ai写的，ai写重复代码真好用
    ActiveBorder,
    ActiveCaption,
    AppWorkspace,
    Background,
    BtnFace,
    BtnShadow,
    BtnText,
    CaptionText,
    GrayText,
    Highlight,
    HighlightText,
    InactiveBorder,
    InactiveCaption,
    Menu,
    MenuText,
    Scrollbar,
    Window,
    WindowFrame,
    WindowText,
}
impl BrushC {
    pub fn __class_from(ush: HBRUSH) -> Self {
        match HBRUSH((ush.0 as usize - 1) as *mut c_void) {
            HBRUSH(val) if val == COLOR_ACTIVEBORDER.0 as *mut c_void => BrushC::ActiveBorder,
            HBRUSH(val) if val == COLOR_ACTIVECAPTION.0 as *mut c_void => BrushC::ActiveCaption,
            HBRUSH(val) if val == COLOR_APPWORKSPACE.0 as *mut c_void => BrushC::AppWorkspace,
            HBRUSH(val) if val == COLOR_BACKGROUND.0 as *mut c_void => BrushC::Background,
            HBRUSH(val) if val == COLOR_BTNFACE.0 as *mut c_void => BrushC::BtnFace,
            HBRUSH(val) if val == COLOR_BTNSHADOW.0 as *mut c_void => BrushC::BtnShadow,
            HBRUSH(val) if val == COLOR_BTNTEXT.0 as *mut c_void => BrushC::BtnText,
            HBRUSH(val) if val == COLOR_CAPTIONTEXT.0 as *mut c_void => BrushC::CaptionText,
            HBRUSH(val) if val == COLOR_GRAYTEXT.0 as *mut c_void => BrushC::GrayText,
            HBRUSH(val) if val == COLOR_HIGHLIGHT.0 as *mut c_void => BrushC::Highlight,
            HBRUSH(val) if val == COLOR_HIGHLIGHTTEXT.0 as *mut c_void => BrushC::HighlightText,
            HBRUSH(val) if val == COLOR_INACTIVEBORDER.0 as *mut c_void => BrushC::InactiveBorder,
            HBRUSH(val) if val == COLOR_INACTIVECAPTION.0 as *mut c_void => BrushC::InactiveCaption,
            HBRUSH(val) if val == COLOR_MENU.0 as *mut c_void => BrushC::Menu,
            HBRUSH(val) if val == COLOR_MENUTEXT.0 as *mut c_void => BrushC::MenuText,
            HBRUSH(val) if val == COLOR_SCROLLBAR.0 as *mut c_void => BrushC::Scrollbar,
            HBRUSH(val) if val == COLOR_WINDOW.0 as *mut c_void => BrushC::Window,
            HBRUSH(val) if val == COLOR_WINDOWFRAME.0 as *mut c_void => BrushC::WindowFrame,
            HBRUSH(val) if val == COLOR_WINDOWTEXT.0 as *mut c_void => BrushC::WindowText,
            HBRUSH(x) => BrushC::Brush(HBRUSH(x).into()),
        }
    }
    pub fn __class_into(self) -> HBRUSH {
        let result = match self {
            BrushC::Brush(ush) => ush.into(),
            //ai写的，ai写重复代码真好用
            BrushC::ActiveBorder => HBRUSH(COLOR_ACTIVEBORDER.0 as *mut c_void),
            BrushC::ActiveCaption => HBRUSH(COLOR_ACTIVECAPTION.0 as *mut c_void),
            BrushC::AppWorkspace => HBRUSH(COLOR_APPWORKSPACE.0 as *mut c_void),
            BrushC::Background => HBRUSH(COLOR_BACKGROUND.0 as *mut c_void),
            BrushC::BtnFace => HBRUSH(COLOR_BTNFACE.0 as *mut c_void),
            BrushC::BtnShadow => HBRUSH(COLOR_BTNSHADOW.0 as *mut c_void),
            BrushC::BtnText => HBRUSH(COLOR_BTNTEXT.0 as *mut c_void),
            BrushC::CaptionText => HBRUSH(COLOR_CAPTIONTEXT.0 as *mut c_void),
            BrushC::GrayText => HBRUSH(COLOR_GRAYTEXT.0 as *mut c_void),
            BrushC::Highlight => HBRUSH(COLOR_HIGHLIGHT.0 as *mut c_void),
            BrushC::HighlightText => HBRUSH(COLOR_HIGHLIGHTTEXT.0 as *mut c_void),
            BrushC::InactiveBorder => HBRUSH(COLOR_INACTIVEBORDER.0 as *mut c_void),
            BrushC::InactiveCaption => HBRUSH(COLOR_INACTIVECAPTION.0 as *mut c_void),
            BrushC::Menu => HBRUSH(COLOR_MENU.0 as *mut c_void),
            BrushC::MenuText => HBRUSH(COLOR_MENUTEXT.0 as *mut c_void),
            BrushC::Scrollbar => HBRUSH(COLOR_SCROLLBAR.0 as *mut c_void),
            BrushC::Window => HBRUSH(COLOR_WINDOW.0 as *mut c_void),
            BrushC::WindowFrame => HBRUSH(COLOR_WINDOWFRAME.0 as *mut c_void),
            BrushC::WindowText => HBRUSH(COLOR_WINDOWTEXT.0 as *mut c_void),
        };
        HBRUSH((result.0 as usize + 1) as *mut c_void)
    }
}

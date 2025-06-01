use super::*;
#[derive(Debug)]
pub struct Font {
    handle: HFONT,
}
#[derive(Debug, Default)]
pub enum ControlFont {
    DefaultDeviceFont, //DEVICE_DEFAULT_FONT
    DefaultGuiFont,    //DEFAULT_GUI_FONT
    OemFixedFont,      //OEM_FIXED_FONT
    AnsiFixedFont,     //ANSI_FIXED_FONT
    AnsiVarFont,       //ANSI_VAR_FONT
    #[default]
    SystemFont, //SYSTEM_FONT
    SystemFixedFont,   //SYSTEM_FIXED_FONT
    CaptionFont,       //NONCLIENTMETRICSW.lfCaptionFont + CreateFontIndirect
    SmallCaptionFont,  //NONCLIENTMETRICSW.lfSmCaptionFont + CreateFontIndirect
    MenuFont,          //NONCLIENTMETRICSW.lfMenuFont + CreateFontIndirect
    StatusBarFont,     //NONCLIENTMETRICSW.lfStatusFont + CreateFontIndirect
    MessageBoxFont,    //NONCLIENTMETRICSW.lfMessageFont + CreateFontIndirect
    Custom(Font),
}

impl Font {
    pub fn into_handle(mut self) -> HFONT {
        let handle = self.handle;
        self.handle = HFONT(0 as *mut c_void);
        handle
    }
}

#[inline]
fn create_font_from_logfont<F>(extractor: F) -> Result<HFONT>
where
    F: FnOnce(&NONCLIENTMETRICSW) -> LOGFONTW,
{
    unsafe {
        let mut nc_metrics: NONCLIENTMETRICSW = std::mem::zeroed();
        nc_metrics.cbSize = std::mem::size_of::<NONCLIENTMETRICSW>() as u32;

        SystemParametersInfoW(
            SPI_GETNONCLIENTMETRICS,
            0,
            Some(&mut nc_metrics as *mut _ as _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )?;
        let log = extractor(&nc_metrics);
        Ok(CreateFontIndirectW(&log as *const LOGFONTW))
    }
}

impl ControlFont {
    pub fn into_handle(self) -> Result<HFONT> {
        Ok(match self {
            ControlFont::DefaultDeviceFont => unsafe {
                HFONT(GetStockObject(DEVICE_DEFAULT_FONT).0 as _)
            },
            ControlFont::DefaultGuiFont => unsafe {
                HFONT(GetStockObject(DEFAULT_GUI_FONT).0 as _)
            },
            ControlFont::OemFixedFont => unsafe { HFONT(GetStockObject(OEM_FIXED_FONT).0 as _) },
            ControlFont::AnsiFixedFont => unsafe { HFONT(GetStockObject(ANSI_FIXED_FONT).0 as _) },
            ControlFont::AnsiVarFont => unsafe { HFONT(GetStockObject(ANSI_VAR_FONT).0 as _) },
            ControlFont::SystemFont => unsafe { HFONT(GetStockObject(SYSTEM_FONT).0 as _) },
            ControlFont::SystemFixedFont => unsafe {
                HFONT(GetStockObject(SYSTEM_FIXED_FONT).0 as _)
            },

            // 自定义非客户区字体
            ControlFont::CaptionFont => create_font_from_logfont(|nc| nc.lfCaptionFont)?,
            ControlFont::SmallCaptionFont => create_font_from_logfont(|nc| nc.lfSmCaptionFont)?,
            ControlFont::MenuFont => create_font_from_logfont(|nc| nc.lfMenuFont)?,
            ControlFont::StatusBarFont => create_font_from_logfont(|nc| nc.lfStatusFont)?,
            ControlFont::MessageBoxFont => create_font_from_logfont(|nc| nc.lfMessageFont)?,

            ControlFont::Custom(font) => font.into_handle(),
        })
    }
}

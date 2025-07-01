use super::*;
#[derive(Debug)]
pub struct Font {
    handle: HFONT,
}
//AI--
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontCharSet {
    Ansi = ANSI_CHARSET.0,
    #[default]
    DefaultCharSet = DEFAULT_CHARSET.0,
    Symbol = SYMBOL_CHARSET.0,
    Mac = MAC_CHARSET.0,
    ShiftJis = SHIFTJIS_CHARSET.0,
    Hangul = HANGUL_CHARSET.0,
    Johab = JOHAB_CHARSET.0,
    GB2312 = GB2312_CHARSET.0,
    ChineseBig5 = CHINESEBIG5_CHARSET.0,
    Greek = GREEK_CHARSET.0,
    Turkish = TURKISH_CHARSET.0,
    Vietnamese = VIETNAMESE_CHARSET.0,
    Hebrew = HEBREW_CHARSET.0,
    Arabic = ARABIC_CHARSET.0,
    Baltic = BALTIC_CHARSET.0,
    Russian = RUSSIAN_CHARSET.0,
    Thai = THAI_CHARSET.0,
    EasternEurope = EASTEUROPE_CHARSET.0,
    OEM = OEM_CHARSET.0,
}

/// 输出精度
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontOutPrecision {
    #[default]
    DefaultOutPrecision = OUT_DEFAULT_PRECIS.0,
    Device = OUT_DEVICE_PRECIS.0,
    Raster = OUT_RASTER_PRECIS.0,
    Outline = OUT_OUTLINE_PRECIS.0,
    PostScriptOnly = OUT_PS_ONLY_PRECIS.0,
    TrueTypeOnly = OUT_TT_ONLY_PRECIS.0,
    #[doc(hidden)]
    Stroke = OUT_STROKE_PRECIS.0,
    #[doc(hidden)]
    StringOut = OUT_STRING_PRECIS.0,
    TrueType = OUT_TT_PRECIS.0,
    ///未使用
    #[doc(hidden)]
    Character = OUT_CHARACTER_PRECIS.0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontClipPrecision {
    /// 如果要使用嵌入式只读字体，必须指定此标志。  
    embedded: bool,
    /// 使用此值后，所有字体的旋转方向取决于坐标系是左旋还是右旋。<br>
    /// 如果不使用此值，设备字体始终逆时针旋转，而其他字体的旋转则取决于坐标系的方向。
    lh_angles: bool,
    /// 关闭字体关联功能。
    disable_dfa: bool, //true CLIP_DFA_DISABLE 或 CLIP_DFA_OVERRIDE值相同，false 0
    /// 未使用。
    #[doc(hidden)]
    character_precis: bool,
    /// 未使用。
    #[doc(hidden)]
    mask: bool,
    /// 未使用。
    #[doc(hidden)]
    tt_always: bool,
}
/// 输出质量
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontQuality {
    #[default]
    DefaultQuality = DEFAULT_QUALITY.0,
    Draft = DRAFT_QUALITY.0,
    Proof = PROOF_QUALITY.0,
    NonAntiAliased = NONANTIALIASED_QUALITY.0,
    AntiAliased = ANTIALIASED_QUALITY.0,
    /// 启用 ClearType 渲染，使用默认的字体平滑方式。适用于大多数场景下的高质量文本渲染。
    ClearType = CLEARTYPE_QUALITY.0,
    /// 启用 ClearType，并启用 自然外观模式（也称自然清晰模式，Natural mode）。
    /// 更注重字体轮廓的真实还原，使得字体看起来更接近打印输出的效果。
    /// 这通常会减少 ClearType 对字符形状的扭曲，让字体线条更自然、更符合设计意图。
    /// 这种模式适合阅读长文或对排版要求较高的场合。
    ClearTypeNatural = CLEARTYPE_NATURAL_QUALITY as u8,
}
/// 间距
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontPitch {
    #[default]
    /// 默认间距，由系统选择合适的间距。
    DefaultPitch = DEFAULT_PITCH.0,
    /// 固定宽度字体（等宽），每个字符宽度相同（如 Courier New）。
    Fixed = FIXED_PITCH.0,
    /// 可变宽度字体，字符宽度按比例变化（如 Arial）。
    Variable = VARIABLE_PITCH.0,
}
/// 字体家族
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    #[default]
    /// 默认家族，由系统选择合适的字体家族。
    DefaultFontFamily = FF_DONTCARE.0,
    /// 衬线字体，具有可变笔画宽度和衬线（如 Times New Roman）。
    Roman = FF_ROMAN.0,
    /// 无衬线字体，具有可变笔画宽度但无衬线（如 Arial）。
    Swiss = FF_SWISS.0,
    /// 等宽字体，具有恒定笔画宽度（如 Courier New）。
    Modern = FF_MODERN.0,
    /// 手写风格字体（如 Script）。
    Script = FF_SCRIPT.0,
    /// 装饰性字体（如 Old English）。
    Decorative = FF_DECORATIVE.0,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontTemple {
    /// 字体名称（最多 30 个字符）。
    pub face_name: &'static str,
    /// 字体家族。
    pub family: FontFamily,
    /// 字符集。
    pub char_set: FontCharSet,
    /// 是否为斜体。
    pub italic: bool,
    /// 是否有下划线。
    pub underline: bool,
    /// 是否有删除线。
    pub strike_out: bool,
    /// 字体粗细（0 ~ 1000）。
    pub weight: i32,
    /// 字体间距。
    pub pitch: FontPitch,
    /// 字符高度（逻辑单位）。负值表示字符高度而非单元格高度。
    pub height: i32,
    /// 字符平均宽度（逻辑单位）。
    pub width: i32,
    /// 转义向量角度（以十分之一度为单位）。
    pub escapement: i32,
    /// 字符方向角度（以十分之一度为单位）。
    pub orientation: i32,
    /// 输出精度。
    pub out_precision: FontOutPrecision,
    /// 剪辑精度。
    pub clip_precision: Option<FontClipPrecision>,
    /// 渲染质量。
    pub quality: FontQuality,
}
// AI--
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

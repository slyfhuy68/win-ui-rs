use super::*;
#[derive(PartialEq, Eq, Hash)]
///此LangID与capdows::i18n::LangID不同，这是版本信息资源专用的语言ID
pub struct LangID(pub u16);
impl LangID {
    pub fn from_hex(value: &str) -> Self {
        let x = value;
        if x.len() > 4 {
            panic!("无效的语言ID，信息资源专用的语言ID的长度必须小于等于4, ")
        }
        let y = u16::from_str_radix(x, 16).expect("在把字符串转换为数字时出错");
        LangID(y)
    }
    pub fn to_hex_string(&self) -> String {
        format!("{:04x}", self.0)
    }
    pub fn value(&self) -> u16 {
        self.0
    }
}
#[derive(Default)]
pub enum ProductVariant {
    ///标准版本
    #[default]
    Standard,
    ///变体 字符串为默认变体说明
    Variant(String),
    ///私有 字符串为默认私有说明
    Private(String),
}
#[derive(Default)]
pub struct StringInfo {
    pub product_name: Option<String>, //自动cargo获取
    pub organization_name: String,
    pub description: Option<String>,       //自动cargo获取
    pub product_version: Option<String>,   //自动：CARGO_PKG_VERSION
    pub file_version: Option<String>,      //自动：CARGO_PKG_VERSION
    pub internal_name: Option<String>,     //自动获取CARGO_PKG_NAME
    pub copyright: Option<String>,         //可选
    pub trademarks: Option<String>,        //可选
    pub original_filename: Option<String>, //自动获取文件名带扩展名
    ///只有在ProductVariant为Variant或Private时才应指定，否则使用时panic
    ///如果在ProductVariant为Variant或Private时不指定，则使用默认说明
    pub special_info: Option<String>,
}
impl StringInfo {
    ///当StringInfo::special_info为Some变体时，如果variants为Standard变体，则panic
    fn pre_compile(
        self,
        id: LangID,
        variants: &ProductVariant,
    ) -> (PreCompilePruduct, PreCompilePruduct) {
        use ProductVariant::*;
        let variant = match self.special_info {
            None => match variants {
                Standard => "".to_string(),
                Variant(s) => format!("VALUE \"SpecialBuild\", \"{}\"", s),
                Private(s) => format!("VALUE \"PrivateBuild\", \"{}\"", s),
            },
            Some(x) => match variants {
                Standard => panic!(
                    "当StringInfo::special_info为Some变体时, StringInfo::variants不能为Standard变体"
                ),
                Variant(_) => format!("VALUE \"SpecialBuild\", \"{}\"", x),
                Private(_) => format!("VALUE \"PrivateBuild\", \"{}\"", x),
            },
        };
        let product_name = self
            .product_name
            .unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
        let organization_name = self.organization_name;
        let description = self
            .description
            .unwrap_or_else(|| env::var("CARGO_PKG_DESCRIPTION").unwrap());
        let product_version = self
            .product_version
            .unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());
        let file_version = self
            .file_version
            .unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());
        let internal_name = self
            .internal_name
            .unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
        let copyright = self.copyright.unwrap_or("".to_string());
        let trademarks = self.trademarks.unwrap_or("".to_string());
        let original_filename = self
            .original_filename
            .unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
        let result1 = format!(
            "
BLOCK \"{}04B0\"
{{
VALUE \"CompanyName\", \"{}\"
VALUE \"FileDescription\", \"{}\"
VALUE \"FileVersion\", \"{}\"
VALUE \"InternalName\", \"{}\"
VALUE \"LegalCopyright\", \"{}\"
VALUE \"LegalTrademarks\", \"{}\"
VALUE \"OriginalFilename\", \"{}\"
VALUE \"ProductName\", \"{}\"
VALUE \"ProductVersion\", \"{}\"
{}
}}
",
            id.to_hex_string(),
            organization_name,
            description,
            file_version,
            internal_name,
            copyright,
            trademarks,
            original_filename,
            product_name,
            product_version,
            variant
        );
        (
            PreCompilePruduct::from(result1),
            PreCompilePruduct::from(format!(
                "VALUE \"Translation\", 0x{}, 1200\n",
                id.to_hex_string()
            )),
        )
    }
}
//-----------------------------------------------AI开始
#[derive(Default, Debug, PartialEq)]
#[repr(u32)] //VS_FIXEDFILEINFO_FILE_OS
pub enum OperatingSystem {
    Unknown = VOS_UNKNOWN,
    Dos = VOS_DOS,
    Nt = VOS_NT,
    Windows16 = VOS__WINDOWS16,
    Windows32 = VOS__WINDOWS32,
    DosWindows16 = VOS_DOS_WINDOWS16,
    DosWindows32 = VOS_DOS_WINDOWS32,
    #[default]
    NtWindows32 = VOS_NT_WINDOWS32,
}

#[derive(Default, Debug, PartialEq)]
#[repr(i32)] //VS_FIXEDFILEINFO_FILE_SUBTYPE
pub enum SubtypeDrv {
    #[default]
    Unknown = VFT2_UNKNOWN,
    Comm = VFT2_DRV_COMM,
    Printer = VFT2_DRV_PRINTER,
    Keyboard = VFT2_DRV_KEYBOARD,
    Language = VFT2_DRV_LANGUAGE,
    Display = VFT2_DRV_DISPLAY,
    Mouse = VFT2_DRV_MOUSE,
    Network = VFT2_DRV_NETWORK,
    System = VFT2_DRV_SYSTEM,
    Installable = VFT2_DRV_INSTALLABLE,
    Sound = VFT2_DRV_SOUND,
    VersionedPrinter = VFT2_DRV_VERSIONED_PRINTER,
}

#[derive(Debug, PartialEq, Default)]
#[repr(i32)] //VS_FIXEDFILEINFO_FILE_SUBTYPE
pub enum SubtypeFont {
    #[default]
    Unknown = VFT2_UNKNOWN,
    Raster = VFT2_FONT_RASTER,
    Vector = VFT2_FONT_VECTOR,
    TrueType = VFT2_FONT_TRUETYPE,
}

#[derive(Debug, PartialEq, Default)]
// VS_FIXEDFILEINFO_FILE_TYPE
pub enum FileType {
    Unknown,
    #[default]
    App,
    Dll,
    Drv(SubtypeDrv),
    Font(SubtypeFont),
    ///参数是虚拟设备控制块中包含的虚拟设备标识符
    Vxd(i32),
    StaticLib,
}

impl Into<(VS_FIXEDFILEINFO_FILE_TYPE, VS_FIXEDFILEINFO_FILE_SUBTYPE)> for FileType {
    fn into(self) -> (VS_FIXEDFILEINFO_FILE_TYPE, VS_FIXEDFILEINFO_FILE_SUBTYPE) {
        match self {
            FileType::Unknown => (VFT_UNKNOWN, VFT2_UNKNOWN),
            FileType::App => (VFT_APP, 0 as VS_FIXEDFILEINFO_FILE_SUBTYPE),
            FileType::Dll => (VFT_DLL, 0 as VS_FIXEDFILEINFO_FILE_SUBTYPE),
            FileType::Drv(subtype) => (VFT_DRV, subtype as VS_FIXEDFILEINFO_FILE_SUBTYPE),
            FileType::Font(subtype) => (VFT_FONT, subtype as VS_FIXEDFILEINFO_FILE_SUBTYPE),
            FileType::Vxd(id) => (VFT_VXD, id as VS_FIXEDFILEINFO_FILE_SUBTYPE),
            FileType::StaticLib => (VFT_STATIC_LIB, 0 as VS_FIXEDFILEINFO_FILE_SUBTYPE),
        }
    }
}
//-----------------------------------------------AI结束
pub struct Version {
    pub product_internal_version: (u16, u16, u16, u16),
    /// None 表示与 product_internal_version 相同
    pub file_internal_version: Option<(u16, u16, u16, u16)>,
    pub debug: Option<bool>,
    pub pre_release: bool,
    pub pached: bool,
    pub variant: ProductVariant,
    pub strings: HashMap<LangID, StringInfo>,
    pub os: OperatingSystem,
    pub ftype: FileType,
}
impl Version {
    pub fn pre_compile(self) -> PreCompilePruduct {
        let piv = format!(
            "FILEVERSION {},{},{},{}",
            self.product_internal_version.0,
            self.product_internal_version.1,
            self.product_internal_version.2,
            self.product_internal_version.3
        );
        let fiv = match self.file_internal_version {
            Some((a, b, c, d)) => format!("PRODUCTVERSION {},{},{},{}", a, b, c, d),
            None => format!(
                "PRODUCTVERSION {},{},{},{}",
                self.product_internal_version.0,
                self.product_internal_version.1,
                self.product_internal_version.2,
                self.product_internal_version.3
            ),
        };
        let debug = match self.debug {
            None => env::var("DEBUG").unwrap() == "true",
            Some(x) => x,
        };
        let mut flag = 0u32 as VS_FIXEDFILEINFO_FILE_FLAGS;
        set_style(&mut flag, VS_FF_DEBUG, debug);
        set_style(&mut flag, VS_FF_PRERELEASE, self.pre_release);
        set_style(&mut flag, VS_FF_PATCHED, self.pached);
        // use ProductVariant::*;
        // let e_str = match self.variant {
        //     Standard => None,
        //     Variant(s) => {
        //         flag |= VS_FF_SPECIALBUILD;
        //         Some(s)
        //     },
        //     Private(s) => {
        //         flag |= VS_FF_PRIVATEBUILD;
        //         Some(s)
        //     }
        // };
        let (flags, marker) = if flag == 0 {
            (String::from(""), String::from(""))
        } else {
            (
                format!("FILEFLAGS 0x{:X}", flag),
                String::from("FILEFLAGSMASK 0x3F"),
            )
        };
        let os = format!("FILEOS 0x{:X}", self.os as u32);
        let (ftype, sftype) = self.ftype.into();
        let ft = format!("FILETYPE 0x{:X}", ftype);
        let sft = format!("FILESUBTYPE 0x{:X}", sftype);
        //e_str后续处理
        let mut sif = String::from("");
        let mut vif = String::from("");
        for (i, j) in self.strings.into_iter() {
            let (a1, a2) = j.pre_compile(i, &self.variant);
            sif += &a1.0;
            vif += &a2.0;
        }
        let result = format!(
            "
1 VERSIONINFO
{}
{}
{}
{}
{}
{}
{}
{{
BLOCK \"StringFileInfo\"
{{
{}
}}
BLOCK \"VarFileInfo\" 
{{
{}
}}
}}
",
            piv, fiv, flags, marker, os, ft, sft, sif, vif
        );
        PreCompilePruduct::from(result)
    }
}

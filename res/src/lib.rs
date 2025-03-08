use std::env;
use capdows::win32::allmods::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use std::collections::HashMap;
pub struct PreCompilePruduct(String);
impl PreCompilePruduct {
    pub fn from(s:&str) -> Self{
        Self(s.to_string())
    }
    pub fn get(self) -> String{
        self.0
    }
}
#[derive(PartialEq, Eq, Hash)]
pub struct LangID(pub u16);
impl LangID {
    pub fn new(value: &str) -> Result<Self> {
        let x = value;
        if x.len() != 4 {
            return Err(Error::new(ERROR_INVALID_DATA.into(), ""));
        }
        if x == "0000" {
            return Ok(LangID(0u16));
        }
        let y = match u16::from_str_radix(x, 16) {
            Ok(x) => x, 
            Err(_) => return Err(Error::new(ERROR_INVALID_DATA.into(), "")), //str包含无效数字。
        };
        Ok(LangID(y))
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
    Private(String)
}
#[derive(Default)]
pub struct StringInfo{
    pub product_name: Option<String>, //自动cargo获取
    pub organization_name: String, 
    pub description: Option<String>, //自动cargo获取
    pub product_version: Option<String>, //自动：CARGO_PKG_VERSION
    pub file_version: Option<String>, //自动：CARGO_PKG_VERSION
    pub internal_name: Option<String>, //自动获取CARGO_PKG_NAME
    pub copyright: Option<String>, //可选
    pub trademarks: Option<String>, //可选
    pub original_filename: Option<String>, //自动获取文件名带扩展名
    ///只有在ProductVariant为Variant或Private时才应指定，否则使用时返回Err
    ///如果在ProductVariant为Variant或Private时不指定，则使用默认说明
    pub special_info: Option<String>
}
impl StringInfo{
    fn pre_compile(self, id:LangID, variants:&ProductVariant) -> Result<(PreCompilePruduct, PreCompilePruduct)> {
        use ProductVariant::*;
        let variant = match self.special_info {
            None => match variants {
                Standard =>  "".to_string(), 
                Variant(s) => format!("VALUE \"SpecialBuild\", \"{}\"", s) , 
                Private(s) => format!("VALUE \"PrivateBuild\", \"{}\"", s) , 
            }
            Some(x) => match variants {
                Standard =>  return Err(Error::new(ERROR_INVALID_DATA.into(), "")), 
                Variant(_) => format!("VALUE \"SpecialBuild\", \"{}\"", x) , 
                Private(_) => format!("VALUE \"PrivateBuild\", \"{}\"", x) , 
            }
        };
        let product_name = self.product_name.unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
        let organization_name = self.organization_name;
        let description = self.description.unwrap_or_else(|| env::var("CARGO_PKG_DESCRIPTION").unwrap());
        let product_version = self.product_version.unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());
        let file_version = self.file_version.unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());
        let internal_name = self.internal_name.unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
        let copyright = self.copyright.unwrap_or("".to_string());
        let trademarks = self.trademarks.unwrap_or("".to_string());
        let original_filename = self.original_filename.unwrap_or_else(|| env::var("CARGO_PKG_NAME").unwrap());
let result1 = format!("
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
", id.to_hex_string(), organization_name, description, file_version, internal_name, copyright, trademarks, original_filename, product_name, product_version, variant);
    Ok((PreCompilePruduct::from(&result1), PreCompilePruduct::from(&format!("VALUE \"Translation\", 0x{}, 1200\n", id.to_hex_string()))))
    }
}
//-----------------------------------------------AI开始
#[derive(Default)]
#[derive(Debug, PartialEq)]
pub enum OperatingSystem {//ai
    Unknown,
    Dos,
    Nt,
    Windows16,
    Windows32,
    DosWindows16,
    DosWindows32,
    #[default]
    NtWindows32,
}

impl From<OperatingSystem> for u32 {//ai
    fn from(os: OperatingSystem) -> u32 {
        match os {
            OperatingSystem::Unknown => VOS_UNKNOWN.0,
            OperatingSystem::Dos => VOS_DOS.0,
            OperatingSystem::Nt => VOS_NT.0,
            OperatingSystem::Windows16 => VOS__WINDOWS16.0,
            OperatingSystem::Windows32 => VOS__WINDOWS32.0,
            OperatingSystem::DosWindows16 => VOS_DOS_WINDOWS16.0,
            OperatingSystem::DosWindows32 => VOS_DOS_WINDOWS32.0,
            OperatingSystem::NtWindows32 => VOS_NT_WINDOWS32.0,
        }
    }
}
#[derive(Default)]
#[derive(Debug, PartialEq)]
pub enum SubtypeDrv {
    #[default]
    Unknown,
    Comm,
    Printer,
    Keyboard,
    Language,
    Display,
    Mouse,
    Network,
    System,
    Installable,
    Sound,
    VersionedPrinter,
}

#[derive(Debug, PartialEq)]
#[derive(Default)]
pub enum SubtypeFont {
    #[default]
    Unknown,
    Raster,
    Vector,
    TrueType,
}

#[derive(Debug, PartialEq)]
#[derive(Default)]
pub enum FileType {
    Unknown,
    #[default]
    App,
    Dll,
    Drv { subtype: SubtypeDrv },
    Font { subtype: SubtypeFont },
    ///参数是虚拟设备控制块中包含的虚拟设备标识符
    Vxd(i32), 
    StaticLib,
}

impl Into<(i32, i32)> for FileType {
    fn into(self) -> (i32, i32) {
        match self {
            FileType::Unknown => (VFT_UNKNOWN.0, VFT2_UNKNOWN.0),
            FileType::App => (VFT_APP.0, 0),
            FileType::Dll => (VFT_DLL.0, 0),
            FileType::Drv { subtype } => {
                let subtype_val: i32 = subtype.into();
                (VFT_DRV.0, subtype_val)
            },
            FileType::Font { subtype } => {
                let subtype_val: i32 = subtype.into();
                (VFT_FONT.0, subtype_val)
            },
            FileType::Vxd(id) => (VFT_VXD.0, id),
            FileType::StaticLib => (VFT_STATIC_LIB.0, 0),
        }
    }
}

impl Into<i32> for SubtypeDrv {
    fn into(self) -> i32 {
        match self {
            SubtypeDrv::Unknown => VFT2_UNKNOWN.0,
            SubtypeDrv::Comm => VFT2_DRV_COMM.0,
            SubtypeDrv::Printer => VFT2_DRV_PRINTER.0,
            SubtypeDrv::Keyboard => VFT2_DRV_KEYBOARD.0,
            SubtypeDrv::Language => VFT2_DRV_LANGUAGE.0,
            SubtypeDrv::Display => VFT2_DRV_DISPLAY.0,
            SubtypeDrv::Mouse => VFT2_DRV_MOUSE.0,
            SubtypeDrv::Network => VFT2_DRV_NETWORK.0,
            SubtypeDrv::System => VFT2_DRV_SYSTEM.0,
            SubtypeDrv::Installable => VFT2_DRV_INSTALLABLE.0,
            SubtypeDrv::Sound => VFT2_DRV_SOUND.0,
            SubtypeDrv::VersionedPrinter => VFT2_DRV_VERSIONED_PRINTER.0,
        }
    }
}

impl Into<i32> for SubtypeFont {
    fn into(self) -> i32 {
        match self {
            SubtypeFont::Unknown => VFT2_UNKNOWN.0,
            SubtypeFont::Raster => VFT2_FONT_RASTER.0,
            SubtypeFont::Vector => VFT2_FONT_VECTOR.0,
            SubtypeFont::TrueType => VFT2_FONT_TRUETYPE.0,
        }
    }
}
//-----------------------------------------------AI结束
pub struct Version{
    pub product_internal_version: (u16, u16, u16, u16), 
    pub file_internal_version: Option<(u16, u16, u16, u16)>,//None表示与 product_internal_version 相同
    pub debug: Option<bool>, 
    pub pre_release: bool, 
    pub pached: bool, 
    pub variant: ProductVariant, 
    pub strings: HashMap<LangID, StringInfo>, 
    pub os: OperatingSystem, 
    pub ftype: FileType
}
impl Version{
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        let piv = format!("FILEVERSION {},{},{},{}", self.product_internal_version.0, self.product_internal_version.1, self.product_internal_version.2, self.product_internal_version.3);
        let fiv = match self.file_internal_version {
            Some((a, b, c, d)) => format!("PRODUCTVERSION {},{},{},{}", a, b, c, d), 
            None => format!("PRODUCTVERSION {},{},{},{}", self.product_internal_version.0, self.product_internal_version.1, self.product_internal_version.2, self.product_internal_version.3), 
        };
        let debug = match self.debug {
            None => env::var("DEBUG").unwrap()=="true", 
            Some(x) => x
        };
        let mut flag = VS_FIXEDFILEINFO_FILE_FLAGS(0);
        if debug {flag |= VS_FF_DEBUG};
        if self.pre_release {flag |= VS_FF_PRERELEASE};
        if self.pached {flag |= VS_FF_PATCHED};
        use ProductVariant::*;
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
        let (flags,marker) = if flag == VS_FIXEDFILEINFO_FILE_FLAGS(0) {
            (String::from(""), String::from(""))
        } else {
            (format!("FILEFLAGS 0x{:X}", flag.0), String::from("FILEFLAGSMASK 0x3F"))
        };
        let os = format!("FILEOS 0x{:X}", <OperatingSystem as Into<u32>>::into(self.os));
        let (ftype,sftype) = self.ftype.into();
        let ft = format!("FILETYPE 0x{:X}", ftype);
        let sft = format!("FILESUBTYPE 0x{:X}", sftype);
        //e_str后续处理
        let mut sif = String::from("");
        let mut vif = String::from("");
        for (i,j) in self.strings.into_iter() {
            let (a1, a2) = j.pre_compile(i, &self.variant)?;
            sif += &a1.0;
            vif += &a2.0;
        }
        let result = format!("
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
", piv, fiv, flags, marker, os, ft, sft, sif, vif);
    Ok(PreCompilePruduct::from(&result))
    }
}
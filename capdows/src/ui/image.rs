use super::*;
#[derive(Clone, PartialEq)]
pub struct Icon {
    pub handle: HICON,
}
unsafe impl Send for Icon {}
unsafe impl Sync for Icon {}
impl Icon {
    pub fn copy_handle(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
    pub const unsafe fn null() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
    pub fn load_from_module(
        module: ExecutableFile,
        id: ResourceID,
        size: Option<Size>,
        share: bool,
    ) -> Result<Self> {
        let pcw = id.to_pcwstr();
        let (cx, cy) = size.unwrap_or(Size::new(0, 0)).to_tuple();
        Ok(Self {
            handle: unsafe {
                WinError::from_win32api_ptr(LoadImageW(
                    module.into(),
                    pcw,
                    IMAGE_ICON,
                    cx,
                    cy,
                    if share {
                        IMAGE_FLAGS::default() | LR_SHARED
                    } else {
                        IMAGE_FLAGS::default()
                    },
                ))?
            },
        })
    }
}
impl From<HICON> for Icon {
    fn from(hi: HICON) -> Self {
        Self { handle: hi }
    }
}
impl Into<HICON> for Icon {
    fn into(self) -> HICON {
        self.handle
    }
}

use std::convert::TryFrom;
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SystemCursor {
    #[default]
    NormalSelection = 32512,
    TextSelection = 32513,
    BusyCursor = 32514,
    PrecisionSelection = 32515,
    AlternateSelection = 32516,
    DiagonalResize1 = 32642,
    DiagonalResize2 = 32643,
    HorizontalResize = 32644,
    VerticalResize = 32645,
    MoveCursor = 32646,
    Unavailable = 32648,
    LinkSelection = 32649,
    WorkingInBackground = 32650,
    HelpSelection = 32651,
    LocationSelection = 32671,
    PersonSelection = 32672,
    PenCursor = 32631,
    ScrollNS = 32652,
    ScrollEW = 32653,
    ScrollAll = 32654,
    ScrollN = 32655,
    ScrollS = 32656,
    ScrollW = 32657,
    ScrollE = 32658,
    ScrollNW = 32659,
    ScrollNE = 32660,
    ScrollSW = 32661,
    CdArrowCursor = 32663,
}
impl Into<u16> for SystemCursor {
    fn into(self) -> u16 {
        self as u16
    }
}
//AI开始--------------
impl TryFrom<u16> for SystemCursor {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        use SystemCursor::*;
        match value {
            32512 => Ok(NormalSelection),
            32513 => Ok(TextSelection),
            32514 => Ok(BusyCursor),
            32515 => Ok(PrecisionSelection),
            32516 => Ok(AlternateSelection),
            32642 => Ok(DiagonalResize1),
            32643 => Ok(DiagonalResize2),
            32644 => Ok(HorizontalResize),
            32645 => Ok(VerticalResize),
            32646 => Ok(MoveCursor),
            32648 => Ok(Unavailable),
            32649 => Ok(LinkSelection),
            32650 => Ok(WorkingInBackground),
            32651 => Ok(HelpSelection),
            32671 => Ok(LocationSelection),
            32672 => Ok(PersonSelection),
            32631 => Ok(PenCursor),
            32652 => Ok(ScrollNS),
            32653 => Ok(ScrollEW),
            32654 => Ok(ScrollAll),
            32655 => Ok(ScrollN),
            32656 => Ok(ScrollS),
            32657 => Ok(ScrollW),
            32658 => Ok(ScrollE),
            32659 => Ok(ScrollNW),
            32660 => Ok(ScrollNE),
            32661 => Ok(ScrollSW),
            32663 => Ok(CdArrowCursor),
            _ => Err(ERROR_INVALID_RESOURCE_ID),
        }
    }
}
//AI结束----------------------------
#[derive(Clone, PartialEq)]
pub struct Cursor {
    pub handle: HCURSOR,
}
unsafe impl Send for Cursor {}
unsafe impl Sync for Cursor {}
impl Cursor {
    pub fn copy_handle(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
    pub const unsafe fn null() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
    pub fn load_from_module(
        module: ExecutableFile,
        id: ResourceID,
        width: Option<Size>,
        share: bool,
    ) -> Result<Self> {
        let pcw = id.to_pcwstr();
        let (cx, cy) = width.unwrap_or(Size::new(0, 0)).to_tuple();
        Ok(Self {
            handle: unsafe {
                WinError::from_win32api_ptr(LoadImageW(
                    module.into(),
                    pcw,
                    IMAGE_CURSOR,
                    cx,
                    cy,
                    if share {
                        IMAGE_FLAGS::default() | LR_SHARED
                    } else {
                        IMAGE_FLAGS::default()
                    },
                ))?
            },
        })
    }
    pub fn from_system(cursor: SystemCursor) -> Result<Self> {
        let id = cursor as u16;
        Ok(Cursor {
            handle: WinError::from_win32api_ptr(unsafe {
                LoadCursorW(0 as HMODULE, id as PCWSTR)
            })?,
        })
    }
    pub fn apply(self) {
        unsafe { SetCursor(self.into()) };
    }
}
#[derive(Clone, PartialEq)]
pub struct Bitmap {
    pub handle: HBITMAP,
}
unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}
impl Bitmap {
    pub fn copy_handle(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
    pub const unsafe fn null() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
}
impl From<HCURSOR> for Cursor {
    fn from(hi: HCURSOR) -> Self {
        Self { handle: hi }
    }
}
impl Into<HCURSOR> for Cursor {
    fn into(self) -> HCURSOR {
        self.handle
    }
}
impl From<HBITMAP> for Bitmap {
    fn from(hi: HBITMAP) -> Self {
        Self { handle: hi }
    }
}
impl Into<HBITMAP> for Bitmap {
    fn into(self) -> HBITMAP {
        self.handle
    }
}
#[derive(Clone, PartialEq)]
pub struct EnhMetaFile {
    pub handle: HENHMETAFILE,
}
unsafe impl Send for EnhMetaFile {}
unsafe impl Sync for EnhMetaFile {}
impl EnhMetaFile {
    pub fn copy_handle(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
    pub const unsafe fn null() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
    pub fn load_from_module(// module: ExecutableFile,
        // id: Either<&str, usize>,
        // width: i32,
        // hight: i32,
        // black: bool,
        // map3d_colours: bool,
        // transparent: bool,
        // shared: bool,
        // vga_colour: bool,
    ) -> Self {
        todo!()
    }
}
impl From<HENHMETAFILE> for EnhMetaFile {
    fn from(hi: HENHMETAFILE) -> Self {
        Self { handle: hi }
    }
}
impl Into<HENHMETAFILE> for EnhMetaFile {
    fn into(self) -> HENHMETAFILE {
        self.handle
    }
}
pub enum Image {
    Icon(Icon),
    Cursor(Cursor),
    Bitmap(Bitmap),
    EnhMetaFile(EnhMetaFile),
}

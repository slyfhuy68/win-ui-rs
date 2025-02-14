use super::*;
#[derive(Clone, PartialEq)]
pub struct Icon {
    handle: HANDLE,
}
impl Icon {
    pub unsafe fn invalid() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
    pub fn load_from_module<T: IntOrName>(
        module: ExecutableFile,
        id: T,
        width: i32,
        hight: i32,
        black: bool,
        map3d_colours: bool,
        transparent: bool,
        shared: bool,
        vga_colour: bool,
    ) -> Self {
        todo!()
    }
}
impl From<HICON> for Icon {
    fn from(hi: HICON) -> Self {
        Self { handle: hi.0 }
    }
}
impl Into<HICON> for Icon {
    fn into(self) -> HICON {
        HICON(self.handle)
    }
}
#[derive(Clone, PartialEq)]
pub struct Cursor {
    handle: HANDLE,
}
impl Cursor {
    pub unsafe fn invalid() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
    pub fn load_from_module<T: IntOrName>(
        module: ExecutableFile,
        id: T,
        width: i32,
        hight: i32,
        black: bool,
        map3d_colours: bool,
        transparent: bool,
        shared: bool,
        vga_colour: bool,
    ) -> Self {
        todo!()
    }
}
impl From<HCURSOR> for Cursor {
    fn from(hi: HCURSOR) -> Self {
        Self { handle: hi.0 }
    }
}
impl Into<HCURSOR> for Cursor {
    fn into(self) -> HCURSOR {
        HCURSOR(self.handle)
    }
}
#[derive(Clone, PartialEq)]
pub struct Bitmap {
    handle: HANDLE,
}
impl Bitmap {
    pub unsafe fn invalid() -> Self {
        Self { handle: NULL_PTR() }
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    } //width或hight为0使用实际资源大小，如果资源包含多个图像，则使用第一个图像的大小。
      // width或hight为负使用系统指标值指定的宽度或高度。
    pub fn load_from_module<T: IntOrName>(
        module: ExecutableFile,
        id: T,
        width: i32,
        hight: i32,
        black: bool,
        map3d_colours: bool,
        transparent: bool,
        shared: bool,
        vga_colour: bool,
        DIB: bool,
    ) -> Self {
        todo!()
    }
}
// impl From<HCURSOR> for Cursor {
//     fn from(hi:HCURSOR) -> Self{
//         Self {
//             handle: hi.0,
//         }
//     }
// }
// impl Into<HCURSOR> for Cursor {
//     fn from(self) -> HCURSOR{
//         HCURSOR(self.handle)
//     }
// }

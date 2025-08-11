use super::*;
#[repr(C, packed(2))]
#[derive(Debug, Clone, Copy)]
struct DLGTEMPLATEEX_HEADER {
    pub dlg_ver: u16,
    pub signature: u16,
    pub help_id: u32,
    pub ex_style: u32,
    pub style: u32,
    pub item_count: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
//menu: CWideStr
//window_class:CWideStr
//title: CWideStr
//可选font: DLGTEMPLATEEX_FONT
#[repr(C, packed(2))]
#[derive(Debug, Clone, Copy)]
struct DLGTEMPLATEEX_FONT {
    pub pointsize: u16,
    pub weight: u16,
    pub italic: bool,
    pub charset: u8,
}
//typeface: CWideStr,
#[repr(C, packed(2))]
#[derive(Debug, Clone, Copy)]
struct DLGITEMTEMPLATEEX_HEADER {
    pub help_id: u32,
    pub ex_style: u32,
    pub style: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u32,
}
//window_class:CWideStr
//title: CWideStr
//extra_bytes: u16
//extra_data: [u8;extra_bytes]
//======================================
#[derive(Debug)]
pub struct DialogTempleInfo<'a> {
    pub help_id: u32,
    pub ex_style: u32,
    pub style: u32,
    pub item_count: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub menu: Option<&'a widestr>,
    pub window_class: Option<&'a widestr>,
    pub title: &'a widestr,
    pub font: Option<DialogFontInfo<'a>>,
    first_control: *mut DLGITEMTEMPLATEEX_HEADER,
}
impl DialogTempleInfo<'static> {
    ///需自行保证生命周期
    pub unsafe fn read_dialog_temple_ex(mut p: *const c_void) -> Option<Self> {
        unsafe {
            let raw_p = p as *mut DLGTEMPLATEEX_HEADER;
            if (*(p as *mut DLGTEMPLATEEX_HEADER)).dlg_ver != 1
                || (*(p as *mut DLGTEMPLATEEX_HEADER)).signature != 0xFFFF
            {
                //不打算支持非扩展版本
                return None;
            }
            let style = (*(p as *mut DLGTEMPLATEEX_HEADER)).style;
            p = p.add(1);

            let menu = match *(p as *const u16) {
                0x0000 => {
                    p = p.byte_add(2);
                    None
                }
                0xFFFF => Some({
                    p = p.byte_add(2);
                    let result = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        2,
                    ));
                    p = p.byte_add(2);
                    result
                }),
                _ => Some({
                    let len = lstrlenW(p as *const u16) as usize;
                    let menu = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        len,
                    ));
                    p = p.byte_add(size_of::<u16>() * (len + 1));
                    menu
                }),
            };

            let window_class = match *(p as *const u16) {
                0x0000 => {
                    p = p.byte_add(2);
                    None
                }
                0xFFFF => Some({
                    p = p.byte_add(2);
                    let result = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        2,
                    ));
                    p = p.byte_add(2);
                    result
                }),
                _ => Some({
                    let len = lstrlenW(p as *const u16) as usize;
                    let window_class = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        len,
                    ));
                    p = p.byte_add(size_of::<u16>() * (len + 1));
                    window_class
                }),
            };

            let len = lstrlenW(p as *const u16) as usize;
            let title =
                widestr::from_utf16_unchecked(std::slice::from_raw_parts(p as *const u16, len));
            p = p.byte_add(size_of::<u16>() * (len + 1));

            let font = if style & (DS_SETFONT as u32) == (DS_SETFONT as u32) {
                // #define DS_SHELLFONT        (DS_SETFONT | DS_FIXEDSYS)
                // 只需要检查DS_SETFONT
                Some(DialogFontInfo {
                    pointsize: (*(p as *mut DLGTEMPLATEEX_FONT)).pointsize,
                    weight: (*(p as *mut DLGTEMPLATEEX_FONT)).weight,
                    italic: (*(p as *mut DLGTEMPLATEEX_FONT)).italic,
                    charset: (*(p as *mut DLGTEMPLATEEX_FONT)).charset,
                    typeface: {
                        p = p.byte_add(size_of::<DLGTEMPLATEEX_FONT>());
                        let len = lstrlenW(p as *const u16) as usize;
                        let typeface = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                            p as *const u16,
                            len,
                        ));
                        p = p.byte_add(size_of::<u16>() * (len + 1));
                        typeface
                    },
                })
            } else {
                None
            };
            if (p.addr() - raw_p.addr()) % 4 != 0 {
                p = p.byte_add(2);
            }
            Some(DialogTempleInfo {
                help_id: (*raw_p).help_id,
                ex_style: (*raw_p).ex_style,
                style,
                item_count: (*raw_p).item_count,
                x: (*raw_p).x,
                y: (*raw_p).y,
                cx: (*raw_p).cx,
                cy: (*raw_p).cy,
                menu,
                window_class,
                title,
                font,
                first_control: p as *mut DLGITEMTEMPLATEEX_HEADER,
            })
        }
    }
    pub unsafe fn read_and_skip_item(&mut self) -> Option<DialogTempleItemInfo> {
        unsafe {
            if self.item_count == 0 {
                return None;
            }
            let mut p = self.first_control.add(1) as *mut u16;
            let raw_p = self.first_control;
            let window_class = match *(p as *const u16) {
                0xFFFF => {
                    p = p.byte_add(2);
                    let result = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        2,
                    ));
                    p = p.byte_add(2);
                    result
                }
                _ => {
                    let len = lstrlenW(p as *const u16) as usize;
                    let window_class = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        len,
                    ));
                    p = p.byte_add(size_of::<u16>() * (len + 1));
                    window_class
                }
            };
            let title = match *(p as *const u16) {
                0xFFFF => {
                    p = p.byte_add(2);
                    let result = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        2,
                    ));
                    p = p.byte_add(2);
                    result
                }
                _ => {
                    let len = lstrlenW(p as *const u16) as usize;
                    let window_class = widestr::from_utf16_unchecked(std::slice::from_raw_parts(
                        p as *const u16,
                        len,
                    ));
                    p = p.byte_add(size_of::<u16>() * (len + 1));
                    window_class
                }
            };
            let data_len = (*(p as *mut u16)) as usize;
            p = p.byte_add(2);
            let extra_data = std::slice::from_raw_parts(p as *const u8, data_len);
            p = p.byte_add(data_len);
            if p.addr() % 4 != 0 {
                p = p.byte_add(4 - (p.addr() % 4));
            }
            self.item_count -= 1;
            self.first_control = p as *mut DLGITEMTEMPLATEEX_HEADER;
            Some(DialogTempleItemInfo {
                help_id: (*raw_p).help_id,
                ex_style: (*raw_p).ex_style,
                style: (*raw_p).style,
                x: (*raw_p).x,
                y: (*raw_p).y,
                cx: (*raw_p).cx,
                cy: (*raw_p).cy,
                id: (*raw_p).id,
                window_class,
                title,
                extra_data,
            })
        }
    }
}
#[derive(Debug)]
pub struct DialogFontInfo<'a> {
    pub pointsize: u16,
    pub weight: u16,
    pub italic: bool,
    pub charset: u8,
    pub typeface: &'a widestr,
}
#[derive(Debug)]
pub struct DialogTempleItemInfo<'a> {
    pub help_id: u32,
    pub ex_style: u32,
    pub style: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u32,
    pub window_class: &'a widestr,
    pub title: &'a widestr,
    pub extra_data: &'a [u8],
}

#[repr(transparent)]
pub struct Dialog {
    wnd: Window,
}
unsafe impl WindowLike for Dialog {
    #[inline]
    fn from_hwnd_ref(handle: &HWND) -> &Self {
        unsafe { std::mem::transmute(handle) }
    }
    #[inline]
    fn from_hwnd_mut(handle: &mut HWND) -> &mut Self {
        unsafe { std::mem::transmute(handle) }
    }
}
use std::ops::Deref;
impl Deref for Dialog {
    type Target = Window;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.wnd
    }
}
use std::ops::DerefMut;
impl DerefMut for Dialog {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wnd
    }
}
impl AsRef<Window> for Dialog {
    #[inline]
    fn as_ref(&self) -> &Window {
        &self.wnd
    }
}
impl AsMut<Window> for Dialog {
    #[inline]
    fn as_mut(&mut self) -> &mut Window {
        &mut self.wnd
    }
}
impl Dialog {
    pub unsafe fn from_raw(wnd: HWND) -> Self {
        Self { wnd: wnd.into() }
    }
    pub unsafe fn handle(self) -> HWND {
        unsafe { self.wnd.handle() }
    }
    pub fn load<C: RawMessageHandler + Sync + 'static>(
        _module: ExecutableFile,
        _id: ResourceID,
        _msg_receiver: PhantomData<C>,
        _owner: Window,
    ) -> Self {
        todo!()
    }
    pub fn load_modal<C: RawMessageHandler + Sync + 'static>(
        _module: ExecutableFile,
        _id: ResourceID,
        _msg_receiver: PhantomData<C>,
        _owner: Window,
    ) {
        todo!()
    }
    pub fn end_modal_dialog(&mut self) -> Result<()> {
        todo!()
    }
    ///变成以对话框左上角为原点的屏幕单位坐标
    pub fn dialog_to_screen(self, _rect: FontRect) -> Result<Rect> {
        todo!()
    }
}
//CONTROL <content>, <id>, "<class>", <style>, <x>, <y>, <w>, <h>, <ex_style>
pub trait DialogTempleControl {
    ///失败时直接panic
    fn pre_compile(self, pos: FontPoint, size: FontSize, identifier: WindowID) -> String;
}

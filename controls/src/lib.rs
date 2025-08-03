use capdows::L;
use capdows::prelude::*;
// use capdows::ui::image::*;
use capdows::ui::{control::*, style::*, *};
use capdows_resource::dialog::{ControlPreCompilePruduct, DialogTempleControl};
use std::ffi::c_void;
use utility::*;
use windows_sys::Win32::Foundation::{
    HINSTANCE,
    // HMODULE,
    // HWND,
    LPARAM,
    // LRESULT,
    WPARAM,
    // POINT, POINTS, RECT, SIZE, WIN32_ERROR,
};
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::{UI::Controls::*, UI::WindowsAndMessaging::*};
use windows_sys::core::{PCWSTR, w};
pub mod button;
pub mod check_box;
pub mod combo_box;
pub mod edit;
pub mod group_box;
pub mod radio_box;
// pub mod view;
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        button::{
            BottonContentPos, Button, ButtonContent, ButtonMsg, ButtonMsgType, ButtonStyle,
            ButtonType,
        },
        check_box::{
            CheckBox, CheckBoxContent, CheckBoxContentPos, CheckBoxMsg, CheckBoxMsgType,
            CheckBoxState, CheckBoxState::*, CheckBoxStyle,
        },
        combo_box::{
            CaseType as ComboBoxCaseType, ComboBox, ComboBoxMsg, ComboBoxMsgType, ComboBoxShow,
            ComboBoxStyle, ListBoxItemPos, ListBoxMaxSize, OwnerDrawType as ComboBoxOwnerDrawType,
        },
        edit::{CaseType as EditCaseType, Edit, EditMsg, EditMsgType, EditStyle, EditType},
        group_box::{GroupBox, GroupBoxMsg, GroupBoxMsgType, GroupBoxStyle},
        radio_box::{
            RadioBox, RadioBoxContent, RadioBoxContentPos, RadioBoxMsg, RadioBoxMsgType,
            RadioBoxStyle,
        },
        // view::{
        //
        // },
        traits::*,
    };
}
pub mod prelude_build {
    #[doc(no_inline)]
    pub use crate::{
        // view::{
        //
        // },
        build::*,
        button::{BottonContentPos, ButtonTemple, ButtonTempleContent, ButtonType},
        check_box::{CheckBoxContentPos, CheckBoxState, CheckBoxTemple, CheckBoxTempleContent},
        combo_box::{
            CaseType as ComboBoxCaseType, ComboBoxShow, ComboBoxTemple,
            OwnerDrawType as ComboBoxOwnerDrawType,
        },
        edit::{CaseType as EditCaseType, EditTemple, EditTempleType},
        group_box::GroupBoxTemple,
        radio_box::{RadioBoxContentPos, RadioBoxTemple, RadioBoxTempleContent},
    };
}
// 警告：由于此mod用于build.rs在编译期嵌入资源, 遇到任何错误都会直接panic（也就是编译期错误）
pub mod build {
    use capdows_resource::{LinkFor, PreCompilePruduct};
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};
    pub fn init_controls() {
        let out_dir = env::var("OUT_DIR").expect("No OUT_DIR env var");
        let dest_path = Path::new(&out_dir).join(format!(
            "init_controls_manifest_auto_{}.manifest",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos()
        ));

        let mut f = File::create(&dest_path).expect("无法创建文件");
        f.write_all(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<dependency>
    <dependentAssembly>
        <assemblyIdentity
            type="win32"
            name="Microsoft.Windows.Common-Controls"
            version="6.0.0.0"
            processorArchitecture="*"
            publicKeyToken="6595b64144ccf1df"
            language="*"
        />
    </dependentAssembly>
</dependency>
</assembly>"#
                .as_bytes(),
        )
        .expect("无法写入文件");

        PreCompilePruduct::from(format!(
            "#define RT_MANIFEST 24
1 RT_MANIFEST \"{}\"",
            capdows::ui::utility::do_escapes(&dest_path.display().to_string())
        ))
        .compile_for(LinkFor::AllBinaries)
    }
}
pub type ButtonImage = Either<Bitmap, Icon>;
use either::*;
// fn style_of(wnd: &Window) -> WINDOW_STYLE {
//     style_of_raw(wnd) as WINDOW_STYLE
// }
fn style_of_raw(wnd: &Window) -> i32 {
    unsafe { GetWindowLongW(wnd.handle(), GWL_STYLE) as i32 }
}
fn new_control(
    wnd: &mut Window,
    control_class: PCWSTR,
    name: String,
    pos: Option<Rect>,
    id: u16,
    style: WINDOW_STYLE,
    style_ex: WINDOW_EX_STYLE,
    font: Option<ControlFont>,
) -> Result<Window> {
    unsafe {
        let id = id as HMENU;
        let parent = wnd.handle();
        let (ptr, _ptr_raw) = str_to_pcwstr(&name);
        let ((x, y), (width, height)) = match pos {
            None => (
                (CW_USEDEFAULT, CW_USEDEFAULT),
                (CW_USEDEFAULT, CW_USEDEFAULT),
            ),
            Some(euclid::Rect { origin, size }) => (origin.to_tuple(), size.to_tuple()),
        };
        let hinstance = GetWindowLongW(wnd.handle(), GWL_HINSTANCE) as HINSTANCE;
        let hwnd = error_from_win32!(CreateWindowExW(
            style_ex,
            control_class,
            ptr,
            style,
            x,
            y,
            width,
            height,
            parent,
            id,
            hinstance,
            0 as *const c_void,
        ))?;
        if let Some(font) = font {
            error_from_win32_num!(PostMessageW(
                hwnd,
                WM_SETFONT,
                font.into_handle()? as WPARAM,
                1 as LPARAM
            ))?;
        };
        Ok(Window::from_handle(hwnd))
    }
}
#[inline]
fn new_button(
    wnd: &mut Window,
    name: String,
    pos: Option<Rect>,
    id: u16,
    style: WINDOW_STYLE,
    style_ex: WINDOW_EX_STYLE,
    font: Option<ControlFont>,
    draw: Option<ButtonImage>,
) -> Result<Window> {
    let wnd = new_control(wnd, w!("Button"), name, pos, id, style, style_ex, font)?;
    match draw {
        Some(x) => unsafe {
            let _ = match x {
                Left(b) => PostMessageW(
                    wnd.handle(),
                    BM_SETIMAGE,
                    IMAGE_BITMAP as WPARAM,
                    b.handle as LPARAM,
                ),
                Right(c) => PostMessageW(
                    wnd.handle(),
                    BM_SETIMAGE,
                    IMAGE_ICON as WPARAM,
                    c.handle as LPARAM,
                ),
            };
        },
        None => {}
    };
    Ok(wnd)
}
fn is_some_window(wnd: &Window, class: &'static widestr) -> Result<bool> {
    let mut buffer = [0u16; 16]; //控件类名通常不超过16个字符
    let len = error_from_win32_num!(GetClassNameW(wnd.handle(), buffer.as_mut_ptr(), 16) as usize)?;
    let new_buffer = &buffer[..len];
    Ok(unsafe { class.eq_ignore_ascii_case(widestr::from_utf16_unchecked(new_buffer)) })
}
use capdows_macros::define_control;
mod traits {
    use super::*;
    pub trait CommonControl: Control + Sized {
        type Style: Send + Sync;
        fn new(
            wnd: &mut Window,
            pos: Option<Rect>,
            identifier: WindowID,
            control_style: Self::Style,
            font: Option<ControlFont>,
        ) -> Result<Self>;
    }
    pub trait TextControl: Control + Sized {
        const INSUFFICIENT_SPACE_RESULT: u32 = 0;
        const NOT_SUPPORT_RESULT: u32 = CB_ERR as u32;
        fn get_text(&self) -> Result<String> {
            let length = self.get_text_length()?;
            if length == 0 {
                return Ok(String::new());
            };
            let mut buffer: Vec<u16> = vec![0; length + 1];
            error_from_win32_bool!(SendMessageW(
                self.get_window().handle(),
                WM_GETTEXT,
                length as WPARAM,
                buffer.as_mut_ptr() as LPARAM,
            ))?;
            Ok(String::from_utf16_lossy(&buffer[..length]))
        }
        fn get_text_length(&self) -> Result<usize> {
            Ok(error_from_win32_zero_num!(SendMessageW(
                self.get_window().handle(),
                WM_GETTEXTLENGTH,
                0 as WPARAM,
                0 as LPARAM,
            ))? as usize)
        }
        fn set_text(&mut self, text: &str) -> Result<()> {
            let (text_ptr, _buffer) = str_to_pcwstr(text);
            let result = error_from_win32_zero_num!(SendMessageW(
                self.get_window().handle(),
                WM_SETTEXT,
                0 as WPARAM,
                text_ptr as LPARAM,
            ))? as u32;
            if result == Self::INSUFFICIENT_SPACE_RESULT {
                Err(ERROR_INSUFFICIENT_SPACE)
            } else if result == Self::NOT_SUPPORT_RESULT {
                Err(ERROR_NOT_SUPPORTED)
            } else {
                Ok(())
            }
        }
    }
}
use traits::*;

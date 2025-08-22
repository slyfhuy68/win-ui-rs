use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Block, Ident, LitStr, Result as SynResult, Token, parse::Parse, parse::ParseStream,
    parse_macro_input,
};

struct DefineControlArgs {
    control_name: Ident,
    class_name: LitStr,
    from_msg_block: Block,
    is_self_block: Block,
    into_raw_block: Block,
}

impl Parse for DefineControlArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let control_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let class_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let from_msg_block = input.parse()?;
        input.parse::<Token![,]>()?;
        let is_self_block = input.parse()?;
        input.parse::<Token![,]>()?;
        let into_raw_block = input.parse()?;
        // input.parse::<Token![,]>()?;
        Ok(DefineControlArgs {
            control_name,
            class_name,
            from_msg_block,
            is_self_block,
            into_raw_block,
        })
    }
}

#[proc_macro]
pub fn define_control(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DefineControlArgs);
    let control_name = &args.control_name;
    let class_name = &args.class_name;
    let from_msg_block = &args.from_msg_block;
    let is_self_block = &args.is_self_block;
    let into_raw_block = &args.into_raw_block;

    let msg_name = format!("{control_name}Msg");
    let msg_name_ident = Ident::new(&msg_name, control_name.span());

    let msg_type_name = format!("{control_name}MsgType");
    let msg_type_name_ident = Ident::new(&msg_type_name, control_name.span());

    let expanded = quote! {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct #control_name(Window);
        unsafe impl Send for #control_name {}
        unsafe impl Sync for #control_name {}

        impl #control_name {
            #[inline]
            pub fn neednot(self){
                let _ = std::mem::ManuallyDrop::new(self);
            }
        }

        impl Control for #control_name {
            type MsgType = #msg_name_ident;
            const CLASS_NAME: &'static str = #class_name;
            const CLASS_NAME_WIDE: &'static widestr = L!(#class_name);
            #[inline]
            unsafe fn force_from_window(wnd: Window) -> Self {
                Self(wnd)
            }
            #[inline]
            fn to_window(mut self) -> Window {
                unsafe {self.0.move_out()}
            }
            #[inline]
            fn is_self(wnd: &Window) -> Result<bool> {
                #[allow(unused_unsafe)]
                unsafe #is_self_block
            }
        }

        impl AsRef<Window> for #control_name {
            #[inline]
            fn as_ref(&self) -> &Window {
                &self.0
            }
        }
        impl AsMut<Window> for #control_name {
            #[inline]
            fn as_mut(&mut self) -> &mut Window {
                &mut self.0
            }
        }
        unsafe impl RawHwndControl for #control_name{
            unsafe fn from_hwnd_ref_unchecked(wnd:&HWND)->&Self{
                unsafe { std::mem::transmute(wnd) }
            }
            unsafe fn from_hwnd_ref_mut_unchecked(wnd:&mut HWND)->&mut Self{
                unsafe { std::mem::transmute(wnd) }
            }
        }

        pub struct #msg_name_ident {
            control: HWND,
            msg_type: #msg_type_name_ident,
        }

        unsafe impl Send for #msg_name_ident {}
        unsafe impl Sync for #msg_name_ident {}
        unsafe impl StaticMsg for #msg_name_ident {}

        impl #msg_name_ident {
            #[inline]
            pub fn get_type(&self) -> & #msg_type_name_ident {
                &self.msg_type
            }
            #[inline]
            pub unsafe fn get_type_mut(&mut self) -> &mut #msg_type_name_ident {
                &mut self.msg_type
            }
        }

        impl ControlMsgType for #msg_name_ident {
            type ControlType = #control_name;
            #[inline]
            fn get_control(&self) -> &Self::ControlType{
                unsafe { std::mem::transmute(&self.control) }
            }
            #[inline]
            fn get_control_mut(&mut self) -> &mut Self::ControlType{
                unsafe { std::mem::transmute(&mut self.control) }
            }
        }

        unsafe impl UnsafeControlMsg for #msg_name_ident {
            type NotifyType = NMHDR;
            unsafe fn into_raw(self) -> Result<Either<u16, Self::NotifyType>>{
                #[allow(unused_unsafe)]
                unsafe #into_raw_block
            }

            unsafe fn from_msg(ptr: usize, _command: bool) -> Result<Self> {
                #[allow(unused_imports)]
                use #msg_type_name_ident::*;
                let nmhdr = *(ptr as *mut NMHDR);
                let code = nmhdr.code;
                let w = nmhdr.hwndFrom.clone();
                let _ = nmhdr;
                #[allow(unused_unsafe)]
                let result = unsafe #from_msg_block;
                Ok(Self {
                    control: w,
                    msg_type: result,
                })
            }
        }

        impl Drop for #control_name {
            fn drop(&mut self) {
                unsafe {
                    let hwnd = self.0.handle();
                    if hwnd as usize != 0 {
                        let hfont = SendMessageW(hwnd, WM_GETFONT, 0 as WPARAM, 0 as LPARAM);
                        let _ = DestroyWindow(hwnd);
                        if hfont != 0 {
                            let _ = DeleteObject(hfont as HGDIOBJ);
                        }
                    }
                }
            }
        }
    };

    expanded.into()
}

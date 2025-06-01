//AI辅助编写
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

    let msg_name = format!("{}Msg", control_name);
    let msg_name_ident = Ident::new(&msg_name, control_name.span());

    let msg_type_name = format!("{}MsgType", control_name);
    let msg_type_name_ident = Ident::new(&msg_type_name, control_name.span());

    let expanded = quote! {
        pub struct #control_name(Window);
        unsafe impl Send for #control_name {}
        unsafe impl Sync for #control_name {}

        impl Control for #control_name {
            type MsgType = #msg_name_ident;

            unsafe fn force_from_window(wnd: Window) -> Self {
                Self(wnd)
            }

            fn to_window(mut self) -> Window {
                self.0.move_out()
            }

            fn get_window(&self) -> &Window {
                &self.0
            }

            fn get_window_mut(&mut self) -> &mut Window {
                &mut self.0
            }

            fn is_self(wnd: &Window) -> Result<bool> {
                #[allow(unused_unsafe)]
                unsafe #is_self_block
            }

            fn get_class() -> WindowClass {unsafe{
                WindowClass::from_str(#class_name)
            }}
        }

        pub struct #msg_name_ident {
            control: #control_name,
            msg_type: #msg_type_name_ident,
        }

        impl #msg_name_ident {
            pub fn new(control: #control_name, msg_type: #msg_type_name_ident) -> Self {
                Self { control, msg_type }
            }

            pub fn get_type(&self) -> & #msg_type_name_ident {
                &self.msg_type
            }

            pub unsafe fn get_type_mut(&mut self) -> &mut #msg_type_name_ident {
                &mut self.msg_type
            }
        }

        impl ControlMsgType for #msg_name_ident {
            type ControlType = #control_name;
            fn get_control(&self) -> &Self::ControlType{
                &self.control
            }
            fn get_control_mut(&mut self) -> &mut Self::ControlType{
                &mut self.control
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
                    control: #control_name(w.into()),
                    msg_type: result,
                })
            }
        }
        impl Drop for #msg_name_ident {
            fn drop(&mut self) {
                unsafe {
                    self.control.get_window_mut().nullify()
                }
            }
        }
        impl Drop for #control_name {
            fn drop(&mut self) {
                unsafe {
                    let hwnd = self.0.handle();
                    if hwnd.0 as usize !=  0 {
                        let hfont = SendMessageW(hwnd, WM_GETFONT, None, None).0;
                        let _ = DestroyWindow(hwnd);
                        if hfont != 0 {
                            let _ = DeleteObject(HGDIOBJ(hfont as *mut c_void));
                        }
                    }
                }
            }
        }
    };

    expanded.into()
}

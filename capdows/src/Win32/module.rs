use super::*;
pub fn get_winmain_args() -> Result<(HINSTANCE, HINSTANCE, Vec<String>, SHOW_WINDOW_CMD)> {
    let mut argsss = STARTUPINFOW::default();
    let aaaa: HINSTANCE = unsafe { GetModuleHandleW(PCWSTR::null()) }?.into();
    unsafe {
        GetStartupInfoW(&mut argsss);
    }
    let args: Vec<String> = std::env::args().collect();
    let result = (
        aaaa,
        HINSTANCE(NULL_PTR()),
        args,
        SHOW_WINDOW_CMD(argsss.wShowWindow as i32),
    );
    //println!("{:?}",result);
    Ok(result)
}

pub struct ExecutableFile {
    pub name: Option<String>,
    pub handle: Option<HMODULE>,
}
impl ExecutableFile {
    pub fn open(dir: &str) -> Self {
        let (pdir, ppdir) = str_to_pcwstr(dir);
        Self {
            name: Some(dir.to_string()),
            handle: match unsafe { GetModuleHandleW(pdir) } {
                Ok(x) => Some(x),
                Err(_) => None,
            },
        }
    }
    pub fn load_menu(&self, menu: impl IntOrName) {
        todo!() //LoadMenuW
    }
}

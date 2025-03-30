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
    handle: HMODULE,
}
impl ExecutableFile {
    pub fn from_current_file() -> Result<Self> {
        Ok(Self {
            handle: unsafe { GetModuleHandleW(PCWSTR::null()) }?.into(),
        })
    }
    pub fn open(dir: &str) -> Result<Self> {
        let (pdir, _pdir) = str_to_pcwstr(dir);
        Ok(Self {
            handle: unsafe { GetModuleHandleW(pdir) }?,
        })
    }
}
impl From<HMODULE> for ExecutableFile {
    fn from(hi: HMODULE) -> Self {
        Self { handle: hi }
    }
}
impl Into<HMODULE> for ExecutableFile {
    fn into(self) -> HMODULE {
        self.handle
    }
}
impl From<HINSTANCE> for ExecutableFile {
    fn from(hi: HINSTANCE) -> Self {
        Self { handle: hi.into() }
    }
}
impl Into<HINSTANCE> for ExecutableFile {
    fn into(self) -> HINSTANCE {
        self.handle.into()
    }
}

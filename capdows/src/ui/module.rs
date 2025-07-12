use super::*;
pub struct ExecutableFile {
    handle: HMODULE,
}
impl ExecutableFile {
    pub fn from_current_file() -> Result<Self> {
        Ok(Self {
            handle: WinError::from_win32api_ptr(unsafe { GetModuleHandleW(0 as PCWSTR) })?,
        })
    }
    pub fn open(dir: &str) -> Result<Self> {
        let (pdir, _pdir) = str_to_pcwstr(dir);
        Ok(Self {
            handle: WinError::from_win32api_ptr(unsafe { GetModuleHandleW(pdir) })?,
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

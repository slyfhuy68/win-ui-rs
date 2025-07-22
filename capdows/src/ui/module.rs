use super::*;
pub struct ExecutableFile {
    handle: HMODULE,
}
impl ExecutableFile {
    pub fn from_current_file() -> Result<Self> {
        Ok(Self {
            handle: error_from_win32!(GetModuleHandleW(0 as PCWSTR))?,
        })
    }
    pub fn open(dir: &str) -> Result<Self> {
        let (pdir, _pdir) = str_to_pcwstr(dir);
        Ok(Self {
            handle: error_from_win32!(GetModuleHandleW(pdir))?,
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

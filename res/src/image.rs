use super::*;
pub struct Bitmap(pub ResourceID, pub PathBuf);
impl Bitmap {
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(&format!(
            "{} BITMAP {:?}",
            match self.0 {
                StringId(y) => {
                    if y.parse::<f32>().is_ok() {
                        return Err(ERROR_INVALID_STRING_ID);
                    };
                    y
                }
                NumberId(x) => x.to_string(),
            },
            (std::fs::canonicalize(self.1)?).as_os_str()
        )))
    }
}
pub struct Icon(pub ResourceID, pub PathBuf);
impl Icon {
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(&format!(
            "{} ICON {:?}",
            match self.0 {
                StringId(y) => {
                    if y.parse::<f32>().is_ok() {
                        return Err(ERROR_INVALID_STRING_ID);
                    };
                    y
                }
                NumberId(x) => x.to_string(),
            },
            (std::fs::canonicalize(self.1)?).as_os_str()
        )))
    }
}
pub struct Cursor(pub ResourceID, pub PathBuf);
impl Cursor {
    pub fn pre_compile(self) -> Result<PreCompilePruduct> {
        Ok(PreCompilePruduct::from(&format!(
            "{} CURSOR {:?}",
            match self.0 {
                StringId(y) => {
                    if y.parse::<f32>().is_ok() {
                        return Err(ERROR_INVALID_STRING_ID);
                    };
                    y
                }
                NumberId(x) => x.to_string(),
            },
            (std::fs::canonicalize(self.1)?).as_os_str()
        )))
    }
}

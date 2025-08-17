use super::*;
#[derive(Clone, PartialEq, Copy, Debug)]
#[repr(transparent)]
pub struct Brush {
    pub(crate) handle: HBRUSH,
}
impl Brush {
    pub const fn handle(&self) -> HBRUSH {
        self.handle
    }
    pub fn is_invalid(&self) -> bool {
        self.handle == NULL_PTR()
    }
}
impl Default for Brush {
    fn default() -> Self {
        Self { handle: NULL_PTR() }
    }
}
impl Into<HBRUSH> for Brush {
    fn into(self) -> HBRUSH {
        self.handle
    }
}
impl From<HBRUSH> for Brush {
    fn from(handle: HBRUSH) -> Self {
        Self { handle }
    }
}

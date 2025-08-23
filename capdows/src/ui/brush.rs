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
        self.handle.is_null()
    }
}
impl Default for Brush {
    fn default() -> Self {
        Self { handle: NULL_PTR() }
    }
}
impl From<Brush> for HBRUSH {
    fn from(val: Brush) -> Self {
        val.handle
    }
}
impl From<HBRUSH> for Brush {
    fn from(handle: HBRUSH) -> Self {
        Self { handle }
    }
}

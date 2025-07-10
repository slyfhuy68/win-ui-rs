use super::*;
#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Brush(HBRUSH);

impl Brush {
    pub fn is_invalid(&self) -> bool {
        self.0 == NULL_PTR()
    }
}
impl Default for Brush {
    fn default() -> Self {
        Self(NULL_PTR())
    }
}
impl Into<HBRUSH> for Brush {
    fn into(self) -> HBRUSH {
        self.0
    }
}
impl From<HBRUSH> for Brush {
    fn from(hb: HBRUSH) -> Self {
        Self(hb)
    }
}
// impl std::fmt::Display for Brush {
//     fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"Brush({})",self.0 as usize)
//     }
// }

use super::*;
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HelpId(NonZeroI32);

impl From<HelpId> for i32 {
    fn from(val: HelpId) -> Self {
        val.0.into()
    }
}
impl HelpId {
    pub fn new(id: NonZeroI32) -> Self {
        Self(id)
    }
    pub fn get(self) -> i32 {
        self.0.get()
    }
    pub fn try_from(id: i32) -> Option<Self> {
        match id {
            0 => None,
            x => Some(unsafe { Self(NonZeroI32::new_unchecked(x)) }),
        }
    }
}
pub(crate) fn option_into(id: Option<HelpId>) -> i32 {
    match id {
        None => 0,
        Some(o) => o.into(),
    }
}

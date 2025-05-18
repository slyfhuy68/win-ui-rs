use super::*;
#[repr(transparent)]
pub struct HelpId(NonZeroI32);

impl Into<i32> for HelpId {
    fn into(self) -> i32 {
        self.0.into()
    }
}
impl HelpId {
    pub fn new(id: NonZeroI32) -> Self {
        Self(id)
    }
}
pub(crate) fn option_into(id: Option<HelpId>) -> i32 {
    match id {
        None => 0,
        Some(o) => o.into(),
    }
}

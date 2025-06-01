use std::num::NonZeroU16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LangID {
    id: NonZeroU16,
}
impl LangID {
    pub const fn new(lang_id: u16, sub_lang_id: u8) -> Option<Self> {
        if lang_id > 0x3FF || sub_lang_id > 0x3F {
            return None;
        }
        match NonZeroU16::new((sub_lang_id as u16) << 10 | lang_id) {
            Some(x) => Some(LangID { id: x }),
            None => None,
        }
    }
    pub const fn id(self) -> u16 {
        self.id.get()
    }
    pub const fn from_id(id: u16) -> Option<Self> {
        match id {
            0 => None,
            x => Some(LangID {
                id: unsafe { NonZeroU16::new_unchecked(x) },
            }),
        }
    }
    pub const fn split(self) -> (u8, u16) {
        ((self.id.get() >> 10) as u8, (self.id.get() & 0x3FF) as u16)
    }
}

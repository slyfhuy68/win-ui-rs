use std::num::NonZeroU8;
use std::num::NonZeroU16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LangID {
    id: NonZeroU16,
}
impl LangID {
    pub const fn new(lang_id: NonZeroU16, sub_lang_id: NonZeroU8) -> Self {
        LangID {
            id: unsafe {
                NonZeroU16::new_unchecked(lang_id.get() << 10 | (sub_lang_id.get() as u16))
            },
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
}

use std::borrow::*;
use std::cmp::Ordering;
use std::ffi::OsString;
use std::hash::Hash;
use std::hash::Hasher;
use std::slice;
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct widestr([u16]);
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WideString(pub(crate) Vec<u16>);
impl Borrow<widestr> for WideString {
    fn borrow(&self) -> &widestr {
        unsafe { widestr::from_utf16_unchecked(&self.0) }
    }
}
use std::ops::Deref;
use std::ops::DerefMut;
impl Deref for WideString {
    type Target = widestr;
    fn deref(&self) -> &Self::Target {
        unsafe { widestr::from_utf16_unchecked(self.0.as_slice()) }
    }
}
impl DerefMut for WideString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { widestr::from_utf16_unchecked_mut(self.0.as_mut_slice()) }
    }
}
impl Into<OsString> for &widestr {
    #[inline]
    fn into(self) -> OsString {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(self.as_wide())
    }
}
impl Ord for widestr {
    #[inline]
    fn cmp(&self, other: &widestr) -> Ordering {
        self.as_wide().cmp(other.as_wide())
    }
}
impl PartialEq for widestr {
    #[inline]
    fn eq(&self, other: &widestr) -> bool {
        self.as_wide() == other.as_wide()
    }
}
impl PartialOrd for widestr {
    #[inline]
    fn partial_cmp(&self, other: &widestr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for widestr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.as_wide() {
            state.write_u16(*i)
        }
        state.write_u16(0xffff);
    }
}
impl Eq for widestr {}
impl ToOwned for widestr {
    type Owned = WideString;

    #[inline]
    fn to_owned(&self) -> WideString {
        WideString(self.as_wide().to_owned())
    }
}
impl BorrowMut<widestr> for WideString {
    fn borrow_mut(&mut self) -> &mut widestr {
        unsafe { widestr::from_utf16_unchecked_mut(&mut self.0) }
    }
}
impl From<String> for WideString {
    #[inline]
    fn from(s: String) -> WideString {
        Self(s.encode_utf16().collect::<Vec<u16>>())
    }
}
impl Into<String> for WideString {
    #[inline]
    fn into(self) -> String {
        //创建self时已检查是否为有效utf16
        String::from_utf16_lossy(&self.0)
    }
}
impl AsRef<widestr> for WideString {
    #[inline]
    fn as_ref(&self) -> &widestr {
        //创建self时已检查self，使用unchecked提高性能
        unsafe { widestr::from_utf16_unchecked(&self.0) }
    }
}
impl AsMut<widestr> for WideString {
    #[inline]
    fn as_mut(&mut self) -> &mut widestr {
        //创建self时已检查self，使用unchecked提高性能
        unsafe { widestr::from_utf16_unchecked_mut(&mut self.0) }
    }
}
pub trait ToWideString {
    fn to_wide_string(&self) -> WideString;
}
impl<T: ToString + ?Sized> ToWideString for T {
    #[inline]
    fn to_wide_string(&self) -> WideString {
        self.to_string().into()
    }
}
use std::fmt::Display;
impl Display for widestr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in char::decode_utf16(self.as_wide().iter().cloned()) {
            f.write_char(c.unwrap_or(char::REPLACEMENT_CHARACTER))?
        }
        Ok(())
    }
}
impl Display for WideString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}
#[derive(Debug)]
pub struct Utf16Error {
    ///最大有效字符的索引
    pub(crate) valid_up_to: usize,
    ///第一个无效字符
    pub(crate) invalid_code: u16,
}

impl Utf16Error {
    #[inline]
    pub const fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }
    #[inline]
    pub const fn invalid_code(&self) -> usize {
        self.invalid_code as usize
    }
}
pub const fn check_utf16(v: &[u16]) -> Result<(), Utf16Error> {
    let mut i: usize = 0;
    while i < v.len() {
        let u = v[i];
        i += 1; //i现在是下一个code unit
        if u < 0xD800 || 0xDFFF < u {
            // 当前 code unit 不是 surrogate, 有效
            continue;
        } else if u >= 0xDC00 {
            // 当前 code unit 是没有配对的 low surrogate，无效
            return Err(Utf16Error {
                //i-1是这一个，这一个无效，有效最大到i-2
                valid_up_to: if i < 2 {
                    0 //一开始的字符就无效
                } else {
                    i - 2
                },
                invalid_code: u,
            });
        } else {
            // 当前 code unit 是 high surrogate
            if i < v.len() {
                let u2 = v[i];
                if 0xDC00 < u2 && u2 < 0xDFFF {
                    i += 1;
                    // 下一个 code unit 是 low surrogate，有效
                    continue;
                } else {
                    // 下一个 code unit 不是 low surrogate，无效
                    return Err(Utf16Error {
                        //i是下一个，下一个不是 low surrogate，
                        //所以下一个和这一个无效，有效最大到这一个的上一个i-2
                        valid_up_to: if i < 2 {
                            0 //一开始的字符就无效
                        } else {
                            i - 2
                        },
                        invalid_code: u,
                    });
                }
            } else {
                // 当前 code unit 是最后的 high surrogate，无配对，无效
                return Err(Utf16Error {
                    //i-1是这一个，这一个无效，有效最大到i-2
                    valid_up_to: if i < 2 {
                        0 //一开始的字符就无效
                    } else {
                        i - 2
                    },
                    invalid_code: u,
                });
            }
        }
    }
    Ok(())
}

//__________________________________________________________________________-
const ASCII_CASE_MASK: u16 = 0x20;
const ASCII_A: u16 = b'a' as u16;
const ASCII_Z: u16 = b'z' as u16;
const ASCIIU_A: u16 = b'A' as u16;
const ASCIIU_Z: u16 = b'Z' as u16;
impl widestr {
    #[inline]
    pub const fn to_pcwstr(&self) -> windows_sys::core::PCWSTR {
        self.as_ptr() as windows_sys::core::PCWSTR
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_wide().len()
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub const fn from_utf16(v: &[u16]) -> Result<&widestr, Utf16Error> {
        match check_utf16(v) {
            Ok(_) => Ok(unsafe { std::mem::transmute(v) }),
            Err(e) => Err(e),
        }
    }
    #[inline]
    pub const fn from_utf16_mut(v: &mut [u16]) -> Result<&mut widestr, Utf16Error> {
        match check_utf16(v) {
            Ok(_) => Ok(unsafe { std::mem::transmute(v) }),
            Err(e) => Err(e),
        }
    }
    #[inline]
    pub const unsafe fn from_utf16_unchecked(v: &[u16]) -> &widestr {
        unsafe { std::mem::transmute(v) }
    }
    #[inline]
    pub const unsafe fn from_utf16_unchecked_mut(v: &mut [u16]) -> &mut widestr {
        unsafe { std::mem::transmute(v) }
    }
    #[inline]
    pub const fn is_char_boundary(&self, index: usize) -> bool {
        //self总是有效的utf16字符串，不存在未配对的代理
        if index == 0 {
            return true;
        }

        if index >= self.len() {
            return index == self.len();
        }
        let u = self.as_wide()[index];
        if 0xD800 <= u && u <= 0xDFFF {
            u <= 0xDBFF
        } else {
            true //普通字符
        }
    }

    #[inline]
    pub const fn as_wide(&self) -> &[u16] {
        unsafe { std::mem::transmute(self) }
    }
    #[inline]
    pub const unsafe fn as_wide_mut(&mut self) -> &mut [u16] {
        unsafe { &mut *(self as *mut widestr as *mut [u16]) }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const u16 {
        self as *const widestr as *const u16
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u16 {
        self as *mut widestr as *mut u16
    }
    #[inline]
    pub const fn split_at(&self, mid: usize) -> (&widestr, &widestr) {
        if self.is_char_boundary(mid) {
            unsafe { self.split_at_unchecked(mid) }
        } else {
            panic!("failed to slice widestr"); //等到intrinsics::const_eval_select稳定改用标准库写法
        }
    }
    #[inline]
    pub const fn split_at_mut(&mut self, mid: usize) -> (&mut widestr, &mut widestr) {
        if self.is_char_boundary(mid) {
            unsafe { self.split_at_mut_unchecked(mid) }
        } else {
            panic!("failed to slice widestr"); //等到intrinsics::const_eval_select稳定改用标准库写法
        }
    }
    #[inline]
    pub const fn split_at_checked(&self, mid: usize) -> Option<(&widestr, &widestr)> {
        if self.is_char_boundary(mid) {
            Some(unsafe { self.split_at_unchecked(mid) })
        } else {
            None
        }
    }
    #[inline]
    pub const fn split_at_mut_checked(
        &mut self,
        mid: usize,
    ) -> Option<(&mut widestr, &mut widestr)> {
        if self.is_char_boundary(mid) {
            Some(unsafe { self.split_at_mut_unchecked(mid) })
        } else {
            None
        }
    }
    #[inline]
    pub const unsafe fn split_at_unchecked(&self, mid: usize) -> (&widestr, &widestr) {
        let ptr = self.as_ptr();
        unsafe {
            (
                Self::from_utf16_unchecked(slice::from_raw_parts(ptr, mid)),
                Self::from_utf16_unchecked(slice::from_raw_parts(ptr.add(mid), self.len() - mid)),
            )
        }
    }
    #[inline]
    pub const unsafe fn split_at_mut_unchecked(
        &mut self,
        mid: usize,
    ) -> (&mut widestr, &mut widestr) {
        let ptr = self.as_mut_ptr();
        unsafe {
            (
                Self::from_utf16_unchecked_mut(slice::from_raw_parts_mut(ptr, mid)),
                Self::from_utf16_unchecked_mut(slice::from_raw_parts_mut(
                    ptr.add(mid),
                    self.len() - mid,
                )),
            )
        }
    }
    #[inline]
    pub const fn is_ascii(&self) -> bool {
        let mut i = 0;
        while i < self.len() {
            if self.as_wide()[i] > 127 {
                return false;
            }
            i = i + 1;
        }
        true
    }
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &widestr) -> bool {
        let mut a = self.as_wide();
        let mut b = other.as_wide();

        if a.len() != b.len() {
            return false;
        }
        while let ([first_a, rest_a @ ..], [first_b, rest_b @ ..]) = (a, b) {
            if (first_a | (matches!(first_a, ASCIIU_A..=ASCIIU_Z) as u16 * ASCII_CASE_MASK))
                == (first_b | (matches!(first_b, ASCIIU_A..=ASCIIU_Z) as u16 * ASCII_CASE_MASK))
            {
                a = rest_a;
                b = rest_b;
            } else {
                return false;
            }
        }

        true
    }
    #[inline]
    pub const fn make_ascii_lowercase(&mut self) {
        let mut i = 0;
        let wide = unsafe { self.as_wide_mut() };
        while i < wide.len() {
            wide[i] = wide[i] | (matches!(wide[i], ASCIIU_A..=ASCIIU_Z) as u16 * ASCII_CASE_MASK);
            i += 1;
        }
    }
    #[inline]
    pub const fn make_ascii_uppercase(&mut self) {
        let mut i = 0;
        let wide = unsafe { self.as_wide_mut() };
        while i < wide.len() {
            wide[i] = wide[i] & !(matches!(wide[i], ASCII_A..=ASCII_Z) as u16 * ASCII_CASE_MASK);
            i += 1;
        }
    }
}
impl widestr {
    #[inline]
    pub fn into_boxed_wide(self: Box<widestr>) -> Box<[u16]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [u16]) }
    }
    #[inline]
    pub fn into_wide_string(self: Box<widestr>) -> WideString {
        WideString(self.into_boxed_wide().into_vec())
    }
    #[inline]
    pub fn repeat(&self, n: usize) -> WideString {
        WideString(self.as_wide().repeat(n))
    }
    #[inline]
    pub fn to_ascii_uppercase(&self) -> WideString {
        let mut s = self.to_owned();
        s.make_ascii_lowercase();
        s
    }
    #[inline]
    pub fn to_ascii_lowercase(&self) -> WideString {
        let mut s = self.to_owned();
        s.make_ascii_lowercase();
        s
    }
}
impl AsRef<[u16]> for widestr {
    #[inline]
    fn as_ref(&self) -> &[u16] {
        self.as_wide()
    }
}

impl Default for &widestr {
    #[inline]
    fn default() -> Self {
        unsafe { widestr::from_utf16_unchecked(&[]) }
    }
}

impl Default for &mut widestr {
    #[inline]
    fn default() -> Self {
        unsafe { widestr::from_utf16_unchecked_mut(&mut []) }
    }
}
//_____________________________________________________________________________
#[doc(hidden)]
pub const fn decode_utf8_char(bytes: &[u8], mut pos: usize) -> Option<(u32, usize)> {
    if bytes.len() == pos {
        return None;
    }
    let ch = bytes[pos] as u32;
    pos += 1;
    if ch <= 0x7f {
        return Some((ch, pos));
    }
    if (ch & 0xe0) == 0xc0 {
        if bytes.len() - pos < 1 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 {
            return None;
        }
        let result: u32 = ((ch & 0x1f) << 6) | (ch2 & 0x3f);
        if result <= 0x7f {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf0) == 0xe0 {
        if bytes.len() - pos < 2 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 {
            return None;
        }
        let result = ((ch & 0x0f) << 12) | ((ch2 & 0x3f) << 6) | (ch3 & 0x3f);
        if result <= 0x7ff || (0xd800 <= result && result <= 0xdfff) {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf8) == 0xf0 {
        if bytes.len() - pos < 3 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        let ch4 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 || (ch4 & 0xc0) != 0x80 {
            return None;
        }
        let result =
            ((ch & 0x07) << 18) | ((ch2 & 0x3f) << 12) | ((ch3 & 0x3f) << 6) | (ch4 & 0x3f);
        if result <= 0xffff || 0x10ffff < result {
            return None;
        }
        return Some((result, pos));
    }
    None
}
#[macro_export]
macro_rules! L {
    ($s:literal) => {{
        const LEN: usize = {
            let mut pos = 0;
            let mut len = 0;
            while let Some((code_point, new_pos)) =
                $crate::strings::decode_utf8_char($s.as_bytes(), pos)
            {
                pos = new_pos;
                len += if code_point <= 0xffff { 1 } else { 2 };
            }
            len + 1
        };
        const WIDE: &[u16; LEN] = {
            let mut buffer = [0; LEN];
            $crate::strings::do_input($s.as_bytes(), &mut buffer);
            &{ buffer }
        };
        unsafe { widestr::from_utf16_unchecked(WIDE[0..LEN - 1]) }
    }};
}
#[doc(hidden)]
pub const fn do_input(input: &[u8], buffer: &mut [u16]) {
    let mut input_pos = 0;
    let mut output_pos = 0;
    while let Some((mut code_point, new_pos)) = decode_utf8_char(input, input_pos) {
        input_pos = new_pos;
        if code_point <= 0xffff {
            buffer[output_pos] = code_point as u16;
            output_pos += 1;
        } else {
            code_point -= 0x10000;
            buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
            output_pos += 1;
            buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
            output_pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    //测试由AI生成
    use super::*;
    #[test]
    fn test_widestr_eq_ignore_ascii_case() {
        let a = L!("Hello中文不受影响❤اللغة العربية");
        let b = L!("heLlO中文不受影响❤اللغة العربية");
        assert!(a.eq_ignore_ascii_case(b));
    }
    #[test]
    fn test_widestr_make_ascii_lowercase() {
        let wstr = L!("heLlO中文不受影响❤اللغة العربية");
        assert_eq!(
            wstr.to_ascii_lowercase(),
            L!("hello中文不受影响❤اللغة العربية").to_wide_string()
        );
    }
    #[test]
    fn test_widestr_make_ascii_uppercase() {
        let wstr = L!("HEllo中文不受影响❤اللغة العربية");
        assert_eq!(
            wstr.to_ascii_uppercase(),
            L!("hello中文不受影响❤اللغة العربية").to_wide_string()
        );
    }
    #[test]
    #[should_panic(expected = "failed to slice widestr")]
    fn test_split_at_surrogate_pair_middle() {
        let data = L!("👨👩👧👦");
        let _ = data.split_at(1);
    }
    #[test]
    fn test_split_at_valid_boundary() {
        let ws = L!("👨👩👧👦");
        let (left, right) = ws.split_at(2);
        assert_eq!(left, L!("👨"));
        assert_eq!(right, L!("👩👧👦"));
    }
    #[test]
    #[should_panic]
    fn test_split_at_invalid_index_out_of_bounds() {
        let ws = L!("abc");
        ws.split_at(100);
    }
    #[test]
    fn test_widestr_to_wide_string() {
        let s = "Hello, World!中文اللغة العربية❤";
        let wstr = s.to_wide_string();
        assert_eq!(
            wstr.as_ref().as_wide(),
            s.encode_utf16().collect::<Vec<_>>()
        );
    }
    #[test]
    fn test_widestr_display() {
        let wstr = L!("HEllo中文اللغة العربية❤");
        assert_eq!(format!("{}", wstr), "HEllo中文اللغة العربية❤");
    }
    #[test]
    fn test_widestr_hash() {
        let a = L!("abc");
        let b = L!("abc");
        let mut hasher_a = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher_a);
        let mut hasher_b = std::collections::hash_map::DefaultHasher::new();
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }
    #[test]
    fn test_widestr_const_methods() {
        const WSTR: &widestr = L!("Hello");
        assert_eq!(WSTR.len(), 5);
        assert!(WSTR.is_char_boundary(0));
        assert!(WSTR.is_char_boundary(5));
    }
    #[test]
    fn test_valid_ascii() {
        let valid: &[u16] = &[0x0048, 0x0065, 0x006C, 0x006C, 0x006F];
        assert!(widestr::from_utf16(valid).is_ok());
        assert!(widestr::from_utf16(valid).unwrap().is_ascii());
    }
    #[test]
    fn test_valid_non_ascii_no_surrogate() {
        let valid: &[u16] = &[0x03A9]; // GREEK CAPITAL LETTER OMEGA
        assert!(widestr::from_utf16(valid).is_ok());
        assert!(!widestr::from_utf16(valid).unwrap().is_ascii());
    }
    #[test]
    fn test_valid_surrogate_pair() {
        let valid: &[u16] = &[0xD83D, 0xDE00]; // 😄
        assert!(widestr::from_utf16(valid).is_ok());
    }
    #[test]
    fn test_multiple_valid_surrogate_pairs() {
        let valid: &[u16] = &[0xD83D, 0xDE00, 0xD83D, 0xDE0A]; // 😄😊
        assert!(widestr::from_utf16(valid).is_ok());
    }
    #[test]
    fn test_invalid_single_high_surrogate() {
        let invalid: &[u16] = &[0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xD800);
    }
    #[test]
    fn test_invalid_single_low_surrogate() {
        let invalid: &[u16] = &[0xDC00];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xDC00);
    }
    #[test]
    fn test_invalid_high_surrogate_followed_by_invalid() {
        let invalid: &[u16] = &[0xD800, 0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xD800);
    }
    #[test]
    fn test_invalid_low_surrogate_first() {
        let invalid: &[u16] = &[0xDC00, 0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xDC00);
    }
    #[test]
    fn test_empty_string() {
        let empty: &[u16] = &[];
        let res = widestr::from_utf16(empty);
        assert!(res.is_ok());
        let ws = res.unwrap();
        assert!(ws.is_empty());
        assert!(ws.is_ascii());
    }
    #[test]
    fn test_is_char_boundary() {
        let s: &[u16] = &[0x0048, 0xD83D, 0xDE00, 0x0041]; // H😄A
        let ws = widestr::from_utf16(s).unwrap();

        assert!(ws.is_char_boundary(0)); // 开头
        assert!(ws.is_char_boundary(1)); // 😄开头
        assert!(!ws.is_char_boundary(2)); // 😄 结束位置
        assert!(ws.is_char_boundary(3)); // 'A' 位置
    }
    #[test]
    fn test_macro_l() {
        let ws = L!("Hello 😄");
        assert_eq!(ws.len(), 8); // H e l l o  (空格) 😄 -> 8 u16s
        assert!(ws.as_wide()[5] == 0x0020); // 空格
        assert!(ws.as_wide()[6] == 0xD83D); // 😄 的 high surrogate
    }
    #[test]
    fn test_default() {
        let default_widestr: &widestr = Default::default();
        assert!(default_widestr.is_empty());
        let default_mut_widestr: &mut widestr = Default::default();
        assert!(default_mut_widestr.is_empty());
    }
}

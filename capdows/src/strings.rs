#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct widestr([u16]);
#[repr(transparent)]
pub struct WideString(Vec<u16>);
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
        if 0xD800 > u || u > 0xDFFF {
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
impl widestr {
    #[inline]
    pub const fn to_pcwstr(&self) -> windows::core::PCWSTR {
        windows::core::PCWSTR(self.as_ptr())
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_bytes().len()
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
            index == self.len()
        } else {
            // 判断code unit 是不是 low surrogate
            0xDC00 <= self.as_bytes()[index] && self.as_bytes()[index] <= 0xDFFF
        }
    }
    #[inline]
    pub const fn is_ascii(&self) -> bool {
        let i = 0;
        while i < self.len() {
            if self.as_bytes()[i] > 127 {
                return false;
            }
        }
        true
    }
    #[inline]
    pub const fn as_bytes(&self) -> &[u16] {
        unsafe { std::mem::transmute(self) }
    }
    #[inline]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u16] {
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
}

impl AsRef<[u16]> for widestr {
    #[inline]
    fn as_ref(&self) -> &[u16] {
        self.as_bytes()
    }
}

impl Default for &widestr {
    /// Creates an empty widestr
    #[inline]
    fn default() -> Self {
        unsafe { widestr::from_utf16_unchecked(&[]) }
    }
}

impl Default for &mut widestr {
    /// Creates an empty mutable widestr
    #[inline]
    fn default() -> Self {
        // SAFETY: The empty widestring is valid UTF-8.
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
        const WIDESTR: &[u16; LEN] = {
            let mut buffer = [0; LEN];
            $crate::strings::do_input($s.as_bytes(), &mut buffer);
            &{ buffer }
        };
        unsafe { widestr::from_utf16_unchecked(WIDESTR) }
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
    //AI
    use super::*;

    // 有效的 ASCII 字符串
    #[test]
    fn test_valid_ascii() {
        let valid: &[u16] = &[0x0048, 0x0065, 0x006C, 0x006C, 0x006F]; // "Hello"
        assert!(widestr::from_utf16(valid).is_ok());
        assert!(widestr::from_utf16(valid).unwrap().is_ascii());
    }

    // 有效的非 ASCII 字符串（不含代理）
    #[test]
    fn test_valid_non_ascii_no_surrogate() {
        let valid: &[u16] = &[0x03A9]; // GREEK CAPITAL LETTER OMEGA
        assert!(widestr::from_utf16(valid).is_ok());
        assert!(!widestr::from_utf16(valid).unwrap().is_ascii());
    }

    // 有效的代理对（High + Low）
    #[test]
    fn test_valid_surrogate_pair() {
        let valid: &[u16] = &[0xD83D, 0xDE00]; // 😄
        assert!(widestr::from_utf16(valid).is_ok());
    }

    // 多个有效的代理对
    #[test]
    fn test_multiple_valid_surrogate_pairs() {
        let valid: &[u16] = &[0xD83D, 0xDE00, 0xD83D, 0xDE0A]; // 😄😊
        assert!(widestr::from_utf16(valid).is_ok());
    }

    // 单个 High surrogate，无效
    #[test]
    fn test_invalid_single_high_surrogate() {
        let invalid: &[u16] = &[0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xD800);
    }

    // 单个 Low surrogate，无效
    #[test]
    fn test_invalid_single_low_surrogate() {
        let invalid: &[u16] = &[0xDC00];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xDC00);
    }

    // High surrogate 后跟非 Low surrogate
    #[test]
    fn test_invalid_high_surrogate_followed_by_invalid() {
        let invalid: &[u16] = &[0xD800, 0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xD800);
    }

    // 高低顺序错误：Low surrogate 在前
    #[test]
    fn test_invalid_low_surrogate_first() {
        let invalid: &[u16] = &[0xDC00, 0xD800];
        let res = widestr::from_utf16(invalid);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.valid_up_to(), 0);
        assert_eq!(err.invalid_code(), 0xDC00);
    }

    // 空字符串
    #[test]
    fn test_empty_string() {
        let empty: &[u16] = &[];
        let res = widestr::from_utf16(empty);
        assert!(res.is_ok());
        let ws = res.unwrap();
        assert!(ws.is_empty());
        assert!(ws.is_ascii());
    }

    // char boundary 检查
    #[test]
    fn test_is_char_boundary() {
        let s: &[u16] = &[0x0048, 0xD83D, 0xDE00, 0x0041]; // H 😄 A
        let ws = widestr::from_utf16(s).unwrap();

        assert!(ws.is_char_boundary(0)); // 开头
        assert!(ws.is_char_boundary(1)); // 第一个字符后
        assert!(ws.is_char_boundary(3)); // 😄 结束位置
        assert!(ws.is_char_boundary(4)); // 'A' 位置
        assert!(!ws.is_char_boundary(2)); // 中间 low surrogate 不是边界
    }

    // 使用宏 L! 构造宽字符串
    #[test]
    fn test_macro_L() {
        let ws = L!("Hello 😄");
        assert_eq!(ws.len(), 7); // H e l l o  (空格) 😄 -> 7 code units
        assert!(ws.as_bytes()[5] == 0x0020); // 空格
        assert!(ws.as_bytes()[6] == 0xDE00); // 😄 的 low surrogate
    }

    // 默认值测试
    #[test]
    fn test_default() {
        let default_str: &widestr = &widestr::default();
        assert!(default_str.is_empty());

        let default_mut_str: &mut widestr = &mut widestr::default();
        assert!(default_mut_str.is_empty());
    }
}

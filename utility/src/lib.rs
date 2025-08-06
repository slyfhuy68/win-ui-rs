pub fn runtime_fmt(temple: &str, list: &[&str]) -> Option<String> {
    let mut iter = temple.chars().peekable();
    let mut result = String::with_capacity(temple.len());
    while let Some(one_char) = iter.next() {
        match one_char {
            '{' => {
                if let Some('{') = iter.peek() {
                    result.push('{');
                    iter.next();
                } else if let Some('}') = iter.peek() {
                    return None; //不支持{}语法
                } else if let Some(next) = iter.peek() {
                    if next.is_numeric() {
                        let mut numbers = String::new();
                        let mut next2 = iter.next()?;
                        while next2 != '}' {
                            if next2.is_numeric() {
                                numbers.push(next2);
                                next2 = iter.next()?;
                                //此处检查了{是否关闭，如果没有关闭，
                                //则会一直循环到字符串结束，导致next返回None，
                                //然后后面的?直接返回函数
                            } else if next2 == '{' {
                                return None; //不支持{1ab{语法
                            } else {
                                return None; //不支持{1ab}语法
                            }
                        }
                        let num = numbers.parse::<usize>().ok()?;
                        if num < list.len() {
                            result.push_str(list[num]);
                        } else {
                            return None; //index out of bounds
                        }
                    } else {
                        return None; //不支持{abc}语法
                    }
                }
            }
            '}' => {
                if let Some('}') = iter.peek() {
                    result.push('}');
                    iter.next();
                } else {
                    return None;
                }
            }
            x => result.push(x),
        }
    }
    // result.shrink_to_fit();
    Some(result)
}
#[inline]
pub const fn ucontain(some: u32, other: u32) -> bool {
    some & other == other
}
#[inline]
pub const fn icontain(some: i32, other: i32) -> bool {
    some & other == other
}
#[inline]
pub fn set_style(style: &mut u32, flag: u32, condition: bool) {
    *style |= flag * condition as u32;
}
#[inline]
pub fn set_istyle(style: &mut i32, flag: i32, condition: bool) {
    *style |= flag * condition as i32;
}
pub fn do_escapes(string: &str) -> String {
    let iter = string.chars();
    let mut result = String::with_capacity(string.len());
    for i in iter {
        // \\, \0, \', \", \n, \t, \xFF, \uFFFF
        result.push_str(match i {
            '\\' => "\\\\",
            '\'' => "\\'",
            '\"' => r#""""#,
            '\n' => "\\n",
            '\t' => "\\t",
            other => {
                result.push(other); //result.push_str(&format!("\\u{:X}", other as u32));
                continue;
            }
        });
    }
    // result.shrink_to_fit();
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_basic_replacement() {
        let template = "Hello {0}, welcome to {1}.";
        let list = ["Alice", "Rust"];
        assert_eq!(
            runtime_fmt(template, &list),
            Some("Hello Alice, welcome to Rust.".to_string())
        );
    }

    #[test]
    fn fmt_multiple_digit_index() {
        let template = "{0}{1}{2}";
        let list = ["A", "B", "C"];
        assert_eq!(runtime_fmt(template, &list), Some("ABC".to_string()));
    }

    #[test]
    fn fmt_invalid_empty_braces() {
        let template = "This is invalid {}";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_invalid_non_numeric_key() {
        let template = "Invalid {abc} key";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_invalid_mixed_characters() {
        let template = "Invalid {1a} format";
        let list = ["value"];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_escaped_braces() {
        let template = "Use double braces to show single: {{ and }}";
        let list: [&str; 0] = [];
        assert_eq!(
            runtime_fmt(template, &list),
            Some("Use double braces to show single: { and }".to_string())
        );
    }

    #[test]
    fn fmt_mixed_valid_and_escaped() {
        let template = "Hello {0}, this is {{0}}.";
        let list = ["World"];
        assert_eq!(
            runtime_fmt(template, &list),
            Some("Hello World, this is {0}.".to_string())
        );
    }

    #[test]
    fn fmt_index_out_of_bounds() {
        let template = "Index {5} should fail.";
        let list = ["Too", "few"];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_unmatched_right_brace() {
        let template = "This is invalid }";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_nested_left_brace() {
        let template = "Invalid {1{2} syntax";
        let list = ["val1", "val2"];
        assert_eq!(runtime_fmt(template, &list), None);
    }
    #[test]
    fn fmt_nested_right_brace() {
        let template = "Invalid {2}1} syntax";
        let list = ["val1", "val2"];
        assert_eq!(runtime_fmt(template, &list), None);
    }

    #[test]
    fn fmt_no_replacement() {
        let template = "Just a normal string.";
        let list: [&str; 0] = [];
        assert_eq!(
            runtime_fmt(template, &list),
            Some("Just a normal string.".to_string())
        );
    }
    #[test]
    fn fmt_leading_zero_index() {
        let template = "{00}";
        let list = ["Zero"];
        assert_eq!(runtime_fmt(template, &list), Some("Zero".to_string()));
    }
    #[test]
    fn fmt_unclosed_brace() {
        let template = "{0";
        let list = ["Value"];
        assert_eq!(runtime_fmt(template, &list), None);
    }
    #[test]
    fn fmt_multiple_escaped_braces() {
        let template = "Escaped {{{}}}";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template, &list), None);
    }
    #[test]
    fn fmt_invalid_index_with_space_or_negative() {
        let template1 = "{ 0}";
        let template2 = "{-1}";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template1, &list), None);
        assert_eq!(runtime_fmt(template2, &list), None);
    }
    #[test]
    fn fmt_oversized_index() {
        let template = "{18446744073709551615}";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template, &list), None);
    }
    #[test]
    fn fmt_nested_brace_errors() {
        let template1 = "{0{1}}";
        let template2 = "{0}1}";
        let list: [&str; 0] = [];
        assert_eq!(runtime_fmt(template1, &list), None);
        assert_eq!(runtime_fmt(template2, &list), None);
    }
    #[test]
    fn fmt_unclose_brace() {
        let list: [&str; 0] = [];
        let template1 = "}}}}}}}}}}}}}}}}}}}{{{{{{{{{{";
        assert_eq!(runtime_fmt(template1, &list), None);
    }
    #[test]
    fn escape_backslash() {
        let input = "\\";
        let expected = "\\\\";
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_single_quote() {
        let input = "'";
        let expected = "\\'";
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_double_quote() {
        let input = "\"";
        let expected = "\\\"";
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_newline() {
        let input = "\n";
        let expected = "\\n";
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_tab() {
        let input = "\t";
        let expected = "\\t";
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_regular_char() {
        let input = "a";
        let expected = r#"a"#; // 'a' 的 Unicode 码点是 U+0061
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_unicode_char() {
        let input = "�"; // Unicode: U+FFFD
        let expected = r#"�"#;
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_mixed_string() {
        let input = "Hello\nWorld\t!";
        let expected = r#"Hello\nWorld\t!"#;
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_multiple_chars() {
        let input = "\\\"'\n\t";
        let expected = r#"\\\"\'\n\t"#;
        assert_eq!(do_escapes(input), expected);
    }

    #[test]
    fn escape_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(do_escapes(input), expected);
    }
}
#[macro_export]
macro_rules! error_from_win32_zero {
    ($l:expr) => {
        unsafe {
            windows_sys::Win32::Foundation::SetLastError(0);
            match $l as usize {
                0 => {
                    let err = windows_sys::Win32::Foundation::GetLastError();
                    if err == 0 {
                        Ok(0 as *mut std::ffi::c_void)
                    } else {
                        Err(WinError::from_win32(err))
                    }
                }
                n => Ok(n as *mut std::ffi::c_void),
            }
        }
    };
}
#[macro_export]
macro_rules! error_from_win32_zero_num {
    ($l:expr) => {
        unsafe {
            windows_sys::Win32::Foundation::SetLastError(0);
            match $l {
                0 => {
                    let err = windows_sys::Win32::Foundation::GetLastError();
                    if err == 0 {
                        Ok(0)
                    } else {
                        Err(WinError::from_win32(err))
                    }
                }
                n => Ok(n),
            }
        }
    };
}
#[macro_export]
macro_rules! error_from_win32 {
    ($l:expr) => {
        unsafe {
            match $l as usize {
                0 => Err(WinError::from_win32(
                    windows_sys::Win32::Foundation::GetLastError(),
                )),
                n => Ok(n as *mut std::ffi::c_void),
            }
        }
    };
}
#[macro_export]
macro_rules! error_from_win32_or_invalid {
    ($l:expr) => {
        unsafe {
            match $l {
                windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE => Err(WinError::from_win32(
                    windows_sys::Win32::Foundation::GetLastError(),
                )),
                n => Ok(n),
            }
        }
    };
}
#[macro_export]
macro_rules! error_from_win32_num {
    ($l:expr) => {
        unsafe {
            match $l {
                0 => Err(WinError::from_win32(
                    windows_sys::Win32::Foundation::GetLastError(),
                )),
                n => Ok(n),
            }
        }
    };
}
#[macro_export]
macro_rules! error_from_win32_bool {
    ($l:expr) => {
        unsafe {
            match $l {
                0 => Err(WinError::from_win32(
                    windows_sys::Win32::Foundation::GetLastError(),
                )),
                _ => Ok(()),
            }
        }
    };
}
#[macro_export]
macro_rules! Lc {
    ($s:literal) => {{
        const LEN: usize = {
            let mut pos = 0;
            let mut len = 0;
            while let Some((code_point, new_pos)) =
                capdows::strings::decode_utf8_char($s.as_bytes(), pos)
            {
                pos = new_pos;
                len += if code_point <= 0xffff { 1 } else { 2 };
            }
            len + 1
        };
        const WIDE: &[u16; LEN] = {
            let mut buffer = [0; LEN];
            capdows::strings::do_input($s.as_bytes(), &mut buffer);
            &{ buffer }
        };
        unsafe { CWideStr::from_utf16_unchecked(WIDE.as_slice()) }
    }};
}
#[macro_export]
macro_rules! L {
    ($s:literal) => {{
        const LEN: usize = {
            let mut pos = 0;
            let mut len = 0;
            while let Some((code_point, new_pos)) =
                capdows::strings::decode_utf8_char($s.as_bytes(), pos)
            {
                pos = new_pos;
                len += if code_point <= 0xffff { 1 } else { 2 };
            }
            len
        };
        const WIDE: &[u16; LEN] = {
            let mut buffer = [0; LEN];
            capdows::strings::do_input($s.as_bytes(), &mut buffer);
            &{ buffer }
        };
        unsafe { widestr::from_utf16_unchecked(WIDE.as_slice()) }
    }};
}

#[macro_export] //AI宏
macro_rules! msg_box {
    ($text:expr) => {
        capdows::ui::tools::msg_box($text, None, Default::default(), Default::default())
    };
    ($text:expr, $caption:expr) => {
        capdows::ui::tools::msg_box(
            $text,
            Some($caption),
            Default::default(),
            Default::default(),
        )
    };
    ($text:expr, $caption:expr, $owner:expr) => {
        capdows::ui::tools::msg_box($text, Some($caption), $owner, Default::default())
    };
    ($text:expr, $caption:expr, $owner:expr, $options:expr) => {
        capdows::ui::tools::msg_box($text, Some($caption), $owner, $options)
    };
}

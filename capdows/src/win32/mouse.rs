use super::*;
use std::time::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
impl Window {
    ///获取***当前线程***中的捕获鼠标的窗口。
    ///如果当前线程中没有窗口捕获鼠标，则返回 None。
    pub fn from_mouse_capture() -> Option<Self> {
        unsafe {
            let result = GetCapture();
            if result.is_invalid() {
                None
            } else {
                Some(result.into())
            }
        }
    }
    ///捕获鼠标
    pub fn capture_mouse(&self) {
        unsafe {
            SetCapture(self.handle());
        }
    }
}
pub const MAX_DOUBLE_CLICK_TIME: Duration = Duration::from_millis(5000);
///获取第一次单击和第二次单击之间能被判定为双击的最大时间间隔。
///最长时间为 5000 毫秒。
pub fn get_double_click_time() -> Duration {
    unsafe { Duration::from_millis(GetDoubleClickTime() as u64) }
}
///设置第一次单击和第二次单击之间能被判定为双击的最大时间间隔。
///最长时间为 5000 毫秒。
///使用Duration.as_millis, 不足一毫秒将被忽略
pub fn set_double_click_time(time: Duration) -> Result<()> {
    if time > MAX_DOUBLE_CLICK_TIME {
        return Err(ERROR_TIME_TOO_LONG);
    }
    unsafe { Ok(SetDoubleClickTime(time.as_millis().try_into()?)?) }
}
///释放***当前线程***中的捕获的鼠标。
pub fn release_mouse() -> Result<()> {
    unsafe { Ok(ReleaseCapture()?) }
}

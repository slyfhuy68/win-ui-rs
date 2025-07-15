use super::*;
// use crate::error::Result as eResult;
// use std::sync::mpsc::*;
// use std::thread::*;
// use windows::Win32::UI::WindowsAndMessaging::*;
// pub struct PropCounter {
//     sender: Option<Sender<bool>>,
//     thread: Option<JoinHandle<eResult<()>>>,
//     result_receiver: Receiver<Box<(std::result::Result<String, FromUtf16Error>, usize)>>, //wHANDLE)>>
// }
// impl Iterator for PropCounter {
//     type Item = (String, usize);
//     fn next(&mut self) -> Option<Self::Item> {
//         match &self.sender {
//             Some(x) => x.send(true).ok()?,
//             None => return None,
//         }
//         let result = self.result_receiver.recv().ok()?;
//         Some(((result.0).ok()?, result.1))
//     }
// }
impl Window {
    pub unsafe fn get_prop(&self, key: &'static CWideStr) -> Result<HANDLE> {
        unsafe {WinError::from_win32api_maybe_zero(GetPropW(
            self.handle(),
            key.to_pcwstr(),
        ))}
    }
    pub unsafe fn set_prop(&mut self, key: &'static CWideStr, value: HANDLE) -> Result<()> {
            unsafe {Ok(WinError::from_win32api_result(SetPropW(
                self.handle(),
                key.to_pcwstr(),
                value,
            ))?)}
        }
    pub unsafe fn remove_prop(&mut self, key: &'static CWideStr) -> Result<HANDLE> {
        unsafe {WinError::from_win32api_maybe_zero(RemovePropW(
            self.handle(),
            key.to_pcwstr(),
        ))}
    }
    // pub fn prop_iter(&self) -> PropCounter {
    //     let (send_s, recv) = channel();
    //     let self_handle = unsafe { self.handle().0 as usize };
    //     let (result_sender, result_receiver) = channel();
    //     let join = std::thread::spawn(move || {
    //         let boxed_recv = Box::new((recv, result_sender));
    //         let receiver_ptr: *mut (
    //             Receiver<bool>,
    //             Sender<Box<(std::result::Result<String, FromUtf16Error>, usize)>>,
    //         ) = Box::into_raw(boxed_recv);
    //         let receiver_usize: usize = receiver_ptr as usize;
    //         unsafe extern "system" fn callback(
    //             hwnd: HWND,
    //             pcwstr: PCWSTR,
    //             handle: wHANDLE,
    //             ptr: usize,
    //         ) -> BOOL {
    //             unsafe {
    //                 let box_data = Box::from_raw(
    //                     ptr as *mut (
    //                         Receiver<bool>,
    //                         Sender<Box<(std::result::Result<String, FromUtf16Error>, usize)>>,
    //                     ),
    //                 );
    //                 match ((*box_data).0).recv() {
    //                     Ok(x) => {
    //                         if x {
    //                             let boxed_result =
    //                                 Box::new((pcwstr.to_string(), handle.0 as usize));
    //                             let _ = ((*box_data).1).send(boxed_result);
    //                             let _ = Box::into_raw(box_data); //防止rust释放
    //                             return BOOL(1);
    //                         } else {
    //                             drop(box_data);
    //                             return BOOL(0);
    //                         }
    //                     }
    //                     Err(y) => {
    //                         drop(box_data);
    //                         return BOOL(0);
    //                     }
    //                 }
    //             }
    //         }
    //         match unsafe {
    //             EnumPropsExW(
    //                 self_handle as HWND,
    //                 Some(callback),
    //                 receiver_usize as LPARAM,
    //             )
    //         } {
    //             -1 => Err(correct_error()),
    //             _ => Ok(()),
    //         }
    //     });
    //     PropCounter {
    //         sender: Some(send_s),
    //         thread: Some(join),
    //         result_receiver: result_receiver,
    //     }
    // }
}
// impl Drop for PropCounter {
//     fn drop(&mut self) {
//         self.sender.take();
//         if let Some(x) = self.thread.take() {
//             let _ = x.join();
//         };
//     }
// }

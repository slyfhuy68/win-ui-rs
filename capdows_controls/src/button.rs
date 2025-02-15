use super::*;
//-----------------------------------按钮-----------------------------------------
pub struct Button (HWND);//PUSHBUTTON
#[derive(Default)]
pub enum BottonContentPos {
	#[default] Center, //BS_CENTER | BS_VCENTER
	Left, //BS_LEFT | BS_VCENTER
	Right, //BS_RIGHT | BS_VCENTER
	Top, //BS_TOP | BS_CENTER
	Bottom, //BS_BOTTOM | BS_CENTER
	TopLeft, //BS_TOP | BS_LEFT
	TopRight, //BS_TOP | BS_RIGHT
	BottomLeft, //BS_BOTTOM | BS_LEFT
	BottomRight, //BS_BOTTOM | BS_RIGHT
}
pub struct ButtonStyle{
	pub extra_msg:bool, //BS_NOTIFY
	pub light:bool, //if light BS_DEFPUSHBUTTON else BS_PUSHBUTTON
	pub flat:bool, //BS_FLAT
}
impl Into<WINDOW_STYLE> for ButtonStyle {
	fn into(self) -> WINDOW_STYLE {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.extra_msg {ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);};
		if self.flat {ms_style |=  WINDOW_STYLE(BS_FLAT as u32);};
		if self.light {ms_style |=  WINDOW_STYLE(BS_DEFPUSHBUTTON as u32);} else {ms_style |=  WINDOW_STYLE(BS_PUSHBUTTON as u32);};
		ms_style
	}
}
impl Default for ButtonStyle {
	fn default() -> Self {
		Self {
			extra_msg:false,
			light:false,
			flat:true, 
		} 
	} 
}
pub enum ButtonAutoDrawType {
	IconOnly(BitmapOrIcon), //BS_ICON
	TextOnly(bool), //bool:multiple_lines BS_TEXT
	IconAndText(BitmapOrIcon, bool)//bool:BS_MULTILINE, BS_TEXT BS_ICON
}
pub enum ButtonMsgType {
	MouseEntering, 
	MouseLaveing, 
	Clicked, 
	DoubleClicked, 
	LoseKeyboardFocus, 
	GetKeyboardFocus, 
	Draw(usize),
}
pub struct ButtonMsg{
	hwnd:HWND, 
	pub bm_type:ButtonMsgType, 
}
impl Control for Button{
	type MsgType = ButtonMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for ButtonMsg{ 
	type ControlType = Button;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		let _ = nmhdr;
		use ButtonMsgType::*;
		let bmtype = match code {
			BCN_HOTITEMCHANGE => {
				let data = *(ptr as *mut NMBCHOTITEM);
				if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
					MouseEntering
				} else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
					MouseLaveing
				} else {
					return None;
				}
			}, 
			BN_CLICKED => {
				Clicked
			}, 
			BN_DBLCLK => {
				DoubleClicked
			}, 
			BN_KILLFOCUS => {
				LoseKeyboardFocus
			}, 
			BN_SETFOCUS => {
				GetKeyboardFocus
			}, 
			NM_CUSTOMDRAW => {
				Draw(ptr)
			}
			_ => return None, 
		};
		Some(Box::new(Self {
			hwnd:w, 
			bm_type:bmtype
		}))
	}
	fn get_control(&self) -> Self::ControlType{
		Button(self.hwnd)
	}
}
pub enum ButtonDrawType {
	ParentDraw, //BS_OWNERDRAW
	AutoDraw(ButtonAutoDrawType, ButtonStyle), //NULL
}
impl Default for ButtonDrawType {
	fn default() -> Self {
		Self::AutoDraw(ButtonAutoDrawType::TextOnly(false), Default::default())
	} 
}
impl Into<(WINDOW_STYLE, Option<BitmapOrIcon>)> for ButtonDrawType {
	fn into(self) -> (WINDOW_STYLE, Option<BitmapOrIcon>) {
		match self {
			ButtonDrawType::ParentDraw => (WINDOW_STYLE(BS_OWNERDRAW as u32), None), 
			ButtonDrawType::AutoDraw(dtype, bstyle) => {
				let mut wstyle = WINDOW_STYLE(0);
				let ditype = match dtype {
					ButtonAutoDrawType::IconOnly(boi) => Some(boi), 
					ButtonAutoDrawType::TextOnly(a) => {if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);}; None}, 
					ButtonAutoDrawType::IconAndText(boi, a) => {
						if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)};
						Some(boi)
					}
				};
				wstyle |= bstyle.into();
				(wstyle, ditype)
			}
		}
	}
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for ButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
// 		
// 	}
// }
impl Button {
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		control_style:ButtonDrawType, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, no_notify: bool) -> Result<Self> {
			let (control_style_ms, draw) = control_style.into();
			let hwnd = new_button(wnd, name, pos, identifier, style, style_ex, control_style_ms, font, no_notify, draw)?;
			Ok(Button(hwnd))
		}
}
//------------------------------------------分隔按钮----------------------------------
pub struct SplitButton (HWND);//SPLITBUTTON
pub struct SplitButtonStyle{
	pub extra_msg:bool, //BS_NOTIFY
	pub light:bool, //if light BS_DEFSPLITBUTTON else BS_SPLITBUTTON
	pub flat:bool, //BS_FLAT
}
impl Into<WINDOW_STYLE> for SplitButtonStyle {
	fn into(self) -> WINDOW_STYLE {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.extra_msg {ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);};
		if self.flat {ms_style |=  WINDOW_STYLE(BS_FLAT as u32);};
		if self.light {ms_style |=  WINDOW_STYLE(BS_DEFSPLITBUTTON as u32);} else {ms_style |=  WINDOW_STYLE(BS_SPLITBUTTON as u32);};
		ms_style
	}
}
impl Default for SplitButtonStyle {
	fn default() -> Self {
		Self {
			extra_msg:false,
			light:false,
			flat:true, 
		} 
	} 
}
pub enum SplitButtonMsgType {
	MouseEntering, 
	MouseLaveing, 
	Clicked, 
	DoubleClicked, 
	LoseKeyboardFocus, 
	GetKeyboardFocus, 
	DropDown(Rectangle), 
	Draw(usize),
}
pub struct SplitButtonMsg{
	hwnd:HWND, 
	pub bm_type:SplitButtonMsgType, 
}
impl Control for SplitButton{
	type MsgType = SplitButtonMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for SplitButtonMsg{ 
	type ControlType = SplitButton;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		let _ = nmhdr;
		use SplitButtonMsgType::*;
		let bmtype = match code {
			BCN_HOTITEMCHANGE => {
				let data = *(ptr as *mut NMBCHOTITEM);
				if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
					MouseEntering
				} else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
					MouseLaveing
				} else {
					return None;
				}
			}, 
			BN_CLICKED => {
				Clicked
			}, 
			BN_DOUBLECLICKED => {
				DoubleClicked
			}, 
			BN_KILLFOCUS => {
				LoseKeyboardFocus
			}, 
			BN_SETFOCUS => {
				GetKeyboardFocus
			}, 
			BCN_DROPDOWN => {
				let data = (*(ptr as *mut NMBCDROPDOWN)).rcButton;
				DropDown(((data.left, data.top), (data.right, data.bottom)))
			}, 
			NM_CUSTOMDRAW => {
				Draw(ptr)
			}
			_ => return None, 
		};
		Some(Box::new(Self {
			hwnd:w, 
			bm_type:bmtype
		}))
	}
	fn get_control(&self) -> Self::ControlType{
		SplitButton(self.hwnd)
	}
}
pub enum SplitButtonDrawType {
	ParentDraw, //BS_OWNERDRAW
	AutoDraw(ButtonAutoDrawType, SplitButtonStyle), //NULL
}
impl Default for SplitButtonDrawType {
	fn default() -> Self {
		Self::AutoDraw(ButtonAutoDrawType::TextOnly(false), Default::default())
	} 
}
impl Into<(WINDOW_STYLE, Option<BitmapOrIcon>)> for SplitButtonDrawType {
	fn into(self) -> (WINDOW_STYLE, Option<BitmapOrIcon>) {
		match self {
			SplitButtonDrawType::ParentDraw => (WINDOW_STYLE(BS_OWNERDRAW as u32), None), 
			SplitButtonDrawType::AutoDraw(dtype, bstyle) => {
				let mut wstyle = WINDOW_STYLE(0);
				let ditype = match dtype {
					ButtonAutoDrawType::IconOnly(boi) => Some(boi), 
					ButtonAutoDrawType::TextOnly(a) => {if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);}; None}, 
					ButtonAutoDrawType::IconAndText(boi, a) => {
						if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)};
						Some(boi)
					}
				};
				wstyle |= bstyle.into();
				(wstyle, ditype)
			}
		}
	}
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for SplitButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
// 		
// 	}
// }
impl SplitButton {
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		control_style:SplitButtonDrawType, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, no_notify: bool) -> Result<Self> {
			let (control_style_ms, draw) = control_style.into();
			let hwnd = new_button(wnd, name, pos, identifier, style, style_ex, control_style_ms, font, no_notify, draw)?;
			Ok(SplitButton(hwnd))
		}
}
//------------------------------------------链接按钮----------------------------------
pub struct LinkButton (HWND);//COMMANDLINK
pub struct LinkButtonStyle{
	pub extra_msg:bool, //BS_NOTIFY
	pub light:bool, //if light BS_DEFCOMMANDLINK else BS_COMMANDLINK
	pub flat:bool, //BS_FLAT
}
impl Into<WINDOW_STYLE> for LinkButtonStyle {
	fn into(self) -> WINDOW_STYLE {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.extra_msg {ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);};
		if self.flat {ms_style |=  WINDOW_STYLE(BS_FLAT as u32);};
		if self.light {ms_style |=  WINDOW_STYLE(BS_DEFCOMMANDLINK as u32);} else {ms_style |=  WINDOW_STYLE(BS_COMMANDLINK as u32);};
		ms_style
	}
}
impl Default for LinkButtonStyle {
	fn default() -> Self {
		Self {
			extra_msg:false,
			light:false,
			flat:true, 
		} 
	} 
}
pub struct LinkButtonMsg{
	hwnd:HWND, 
	pub bm_type:ButtonMsgType, 
}
impl Control for LinkButton{
	type MsgType = LinkButtonMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for LinkButtonMsg{ 
	type ControlType = LinkButton;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		let _ = nmhdr;
		use ButtonMsgType::*;
		let bmtype = match code {
			BCN_HOTITEMCHANGE => {
				let data = *(ptr as *mut NMBCHOTITEM);
				if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
					MouseEntering
				} else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
					MouseLaveing
				} else {
					return None;
				}
			}, 
			BN_CLICKED => {
				Clicked
			}, 
			BN_DOUBLECLICKED => {
				DoubleClicked
			}, 
			BN_KILLFOCUS => {
				LoseKeyboardFocus
			}, 
			BN_SETFOCUS => {
				GetKeyboardFocus
			},
			NM_CUSTOMDRAW => {
				Draw(ptr)
			}
			_ => return None, 
		};
		Some(Box::new(Self {
			hwnd:w, 
			bm_type:bmtype
		}))
	}
	fn get_control(&self) -> Self::ControlType{
		LinkButton(self.hwnd)
	}
}
pub enum LinkButtonDrawType {
	ParentDraw, //BS_OWNERDRAW
	AutoDraw(ButtonAutoDrawType, LinkButtonStyle), //NULL
}
impl Default for LinkButtonDrawType {
	fn default() -> Self {
		Self::AutoDraw(ButtonAutoDrawType::TextOnly(false), Default::default())
	} 
}
impl Into<(WINDOW_STYLE, Option<BitmapOrIcon>)> for LinkButtonDrawType {
	fn into(self) -> (WINDOW_STYLE, Option<BitmapOrIcon>) {
		match self {
			LinkButtonDrawType::ParentDraw => (WINDOW_STYLE(BS_OWNERDRAW as u32), None), 
			LinkButtonDrawType::AutoDraw(dtype, bstyle) => {
				let mut wstyle = WINDOW_STYLE(0);
				let ditype = match dtype {
					ButtonAutoDrawType::IconOnly(boi) => Some(boi), 
					ButtonAutoDrawType::TextOnly(a) => {if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);}; None}, 
					ButtonAutoDrawType::IconAndText(boi, a) => {
						if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)};
						Some(boi)
					}
				};
				wstyle |= bstyle.into();
				(wstyle, ditype)
			}
		}
	}
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for LinkButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
// 		
// 	}
// }
impl LinkButton {
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		control_style:LinkButtonDrawType, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, no_notify: bool) -> Result<Self> {
			let (control_style_ms, draw) = control_style.into();
			let hwnd = new_button(wnd, name, pos, identifier, style, style_ex, control_style_ms, font, no_notify, draw)?;
			Ok(LinkButton(hwnd))
		}
}
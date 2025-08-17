use super::*;
pub struct ComboBoxStyle {
    pub style: ChildWindowStyles,
    pub name: String,
    pub auto_hide_scroll: bool, //取反后的LBS_DISABLENOSCROLL
    pub delayed_rendering: bool, //LBS_NOREDRAW
    pub extra_nofity:bool,//LBS_NOTIFY
    pub extra_keyboard_nofity:bool,//LBS_WANTKEYBOARDINPUT
    pub draw_type: DrawType,
    pub sel_type: SelType, 
}
impl ComboBoxStyle{
	fn new(name:&str) -> Self{
		ComboBoxStyle {
			style: ChildWindowStyles::default(),
			name: name.to_string(),
			auto_hide_scroll: true, 
			delayed_rendering: false,
			extra_nofity:false,
			extra_keyboard_nofity:false,
			draw_type: DrawType::default(),
			sel_type: SelType::default(), 
		}
	}
}
pub enum ListBoxMsgType {
    DoubleClick,
    NoEnoughMemory,
    LoseKeyboardFocus,
    GetKeyboardFocus,
    SelectionCanceled, 
    SelectionChanged,
    
    // 需要主函数支持：
    // WM_CHARTOITEM
    // WM_CTLCOLORLISTBOX
    // WM_DELETEITEM
    // WM_VKEYTOITEM 
	// DL_BEGINDRAG
	// DL_CANCELDRAG
	// DL_DRAGGING
	// DL_DROPPED
}
pub enum SelType{
	Allow{
		multiple_selection: bool, //LBS_MULTICOLUMN
		ext_selection: bool, //LBS_EXTENDEDSEL
	}, 
	Forbid//LBS_NOSEL
}
impl Default for SelType{
	fn default() -> Self{
		SelType::Allow{
			multiple_selection: false, 
			ext_selection: false, 
		}
	}
}
pub enum OwnerSaveDataType{
	Yes, //LBS_NODATA | LBS_OWNERDRAWFIXED
	No{
		owner_save_list: bool, //取反的LBS_HASSTRINGS
		auto_sort: bool,        //CBS_SORT
		//None: LBS_OWNERDRAWVARIABLE, true: LBS_OWNERDRAWFIXED | LBS_NOINTEGRALHEIGHT false: LBS_OWNERDRAWFIXED
		fixed_height: Option<bool>, 
	}
}
impl Default for OwnerSaveDataType{
	fn default() -> Self{
		OwnerSaveDataType::No{
			owner_save_list: true, 
			auto_sort: false,  
			fixed_height: None
		}
	}
}
pub enum DrawType{
	OwnerDraw(OwnerSaveDataType), 
	AutoDraw{
		auto_size: bool,        //取反后的LBS_NOINTEGRALHEIGHT
		auto_sort: bool,        //CBS_SORT
		costom_tab_size:bool, 
	}
}
impl Default for DrawType{
	fn default() -> Self{
		DrawType::AutoDraw{
			auto_size:  true, 
			auto_sort: false,  
			costom_tab_size: true
		}
	}
}
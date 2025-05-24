use super::*; //部分触发代码使用AI编写
use std::any::Any;
pub type MenuItemID = u16;
#[derive(Clone, PartialEq)]
pub enum MenuItemBitmapIcon {
    ///位图
    Bitmap(super::image::Bitmap /*位图的句柄*/),
    ///Windows 系统图标
    SystemIcon(usize), //HBMMENU_SYSTEM
    ///在消息接收器的回调中手动绘制。
    CallBack, //HBMMENU_CALLBACK
    ///弹出菜单的“关闭”按钮。
    CloseP, //HBMMENU_POPUP_CLOSE
    ///菜单栏的“关闭”按钮。
    CloseB, //HBMMENU_MBAR_CLOSE
    ///菜单栏的“已禁用关闭”按钮。
    CloseBD, //HBMMENU_MBAR_CLOSE_D
    ///弹出菜单的最小化按钮。
    MimimizeP, //HBMMENU_POPUP_MINIMIZE
    ///菜单栏的最小化按钮。
    MimimizeB, //HBMMENU_MBAR_MINIMIZE
    ///已禁用菜单栏的最小化按钮。
    MimimizeBD, //HBMMENU_MBAR_MINIMIZE_D
    ///弹出菜单的最大化按钮。
    MaximizeP, //HBMMENU_POPUP_MAXIMIZE
    ///弹出菜单的“还原”按钮。
    RestoreP, //HBMMENU_POPUP_RESTORE
    ///菜单栏的“还原”按钮。
    RestoreB, //HBMMENU_MBAR_RESTORE
}
impl From<(HBITMAP, usize)> for MenuItemBitmapIcon {
    fn from(style: (HBITMAP, usize)) -> Self {
        use MenuItemBitmapIcon::*;
        match style.0 {
            HBMMENU_SYSTEM => SystemIcon(style.1),
            HBMMENU_CALLBACK => CallBack,
            HBMMENU_POPUP_CLOSE => CloseP,
            HBMMENU_MBAR_CLOSE => CloseB,
            HBMMENU_MBAR_CLOSE_D => CloseBD,
            HBMMENU_POPUP_MINIMIZE => MimimizeP,
            HBMMENU_MBAR_MINIMIZE => MimimizeB,
            HBMMENU_MBAR_MINIMIZE_D => MimimizeBD,
            HBMMENU_POPUP_MAXIMIZE => MaximizeP,
            HBMMENU_POPUP_RESTORE => RestoreP,
            HBMMENU_MBAR_RESTORE => RestoreB,
            _ => Bitmap(style.0.into()),
        }
    }
}
impl Into<(HBITMAP, usize)> for MenuItemBitmapIcon {
    fn into(self) -> (HBITMAP, usize) {
        use MenuItemBitmapIcon::*;
        (
            match self {
                Bitmap(b) => b.into(),
                SystemIcon(icon_id) => return (HBMMENU_SYSTEM, icon_id),
                CallBack => HBMMENU_CALLBACK,
                CloseP => HBMMENU_POPUP_CLOSE,
                CloseB => HBMMENU_MBAR_CLOSE,
                CloseBD => HBMMENU_MBAR_CLOSE_D,
                MimimizeP => HBMMENU_POPUP_MINIMIZE,
                MimimizeB => HBMMENU_MBAR_MINIMIZE,
                MimimizeBD => HBMMENU_MBAR_MINIMIZE_D,
                MaximizeP => HBMMENU_POPUP_MAXIMIZE,
                RestoreP => HBMMENU_POPUP_RESTORE,
                RestoreB => HBMMENU_MBAR_RESTORE,
            },
            0,
        )
    }
}
#[derive(Clone, PartialEq, Default)]
pub enum MenuItemDisabledState {
    ///启用
    #[default]
    Enabled, //MFS_ENABLED
    #[doc(hidden)]
    DisabledNoGrayed, //MF_DISABLED
    #[doc(hidden)]
    Grayed, //MF_GRAYED
    ///禁用
    Disabled, //MFS_DISABLED
}
#[derive(Clone, PartialEq, Default)]
pub struct MenuItemState {
    ///启用状态和灰显状态
    pub state: MenuItemDisabledState,
    ///是否高亮显示
    pub hilite: bool, //true MFS_HILITE,false MFS_UNHILITE
    pub checked: bool, //true MFS_CHECKED,false MFS_UNCHECKED
}
impl Into<MENU_ITEM_STATE> for MenuItemState {
    fn into(self) -> MENU_ITEM_STATE {
        let mut mtype = MENU_ITEM_STATE(0);
        if self.hilite {
            mtype |= MFS_HILITE;
        }
        if self.checked {
            mtype |= MFS_CHECKED;
        }
        match self.state {
            MenuItemDisabledState::Enabled => (),
            MenuItemDisabledState::Disabled => mtype |= MFS_DISABLED,
            MenuItemDisabledState::DisabledNoGrayed => mtype |= MENU_ITEM_STATE(MF_DISABLED.0),
            MenuItemDisabledState::Grayed => mtype |= MENU_ITEM_STATE(MF_GRAYED.0),
        };
        mtype
    }
}
impl From<MENU_ITEM_STATE> for MenuItemState {
    fn from(style: MENU_ITEM_STATE) -> Self {
        let hilite = style.contains(MFS_HILITE);
        let checked = style.contains(MFS_CHECKED);
        let state = if style.contains(MFS_DISABLED) {
            MenuItemDisabledState::Disabled
        } else if style.contains(MENU_ITEM_STATE(MF_GRAYED.0)) {
            MenuItemDisabledState::Grayed
        } else if style.contains(MENU_ITEM_STATE(MF_DISABLED.0)) {
            MenuItemDisabledState::DisabledNoGrayed
        } else {
            MenuItemDisabledState::Enabled
        };
        MenuItemState { state, hilite, checked }
    }
}
#[derive(Clone, PartialEq, Default)]
pub enum MenuCheckedIcon {
    #[default]
    CheckMark,
    Cullet, //MFT_RADIOCHECK
    Costom(Bitmap),
}
#[derive(Default)]
pub struct MenuCheckIcon {
    pub checked: MenuCheckedIcon,
    pub unchecked: Option<Bitmap>,
}
impl From<(MENU_ITEM_TYPE, (HBITMAP, HBITMAP))> for MenuCheckIcon {
    fn from(style: (MENU_ITEM_TYPE, (HBITMAP, HBITMAP))) -> Self {
        let checked = if style.1 .0.is_null() {if style.0.contains(MFT_RADIOCHECK) {
            MenuCheckedIcon::Cullet
        } else {
            MenuCheckedIcon::CheckMark
        }} else { MenuCheckedIcon::Costom(style.1 .0.into()) };
        MenuCheckIcon {
            checked,
            unchecked: if style.1 .1.is_null() { None } else { Some(style.1 .1.into()) },
        }
    }
}
impl Into<(MENU_ITEM_TYPE, (HBITMAP, HBITMAP))> for MenuCheckIcon {
    fn into(self) -> (MENU_ITEM_TYPE, (HBITMAP, HBITMAP)) {
        use MenuCheckedIcon::*;
        let (itype, checked) = match self.checked {
            CheckMark => (MENU_ITEM_TYPE(0), HBITMAP(NULL_PTR())),
            Cullet => (MFT_RADIOCHECK, HBITMAP(NULL_PTR())),
            Costom(bitmap) => (MENU_ITEM_TYPE(0), bitmap.into()),
        };
        let unchecked = match self.unchecked {
            None => HBITMAP(NULL_PTR()),
            Some(bitmap) => bitmap.into(),
        };
        (itype, (checked, unchecked))
    }
}
pub enum MenuItemShow {
    Bitmap(MenuCheckIcon, MenuItemBitmapIcon), //MFT_BITMAP? | MIIM_BITMAP
    String(
        MenuCheckIcon,
        String, /* dwTypeData 成员是指向以 null 结尾的字符串的指针*/
    ), //MFT_STRING
    OwnDraw(MenuCheckIcon, Option<Box<dyn Any>>), //MFT_OWNERDRAW
}
impl
    From<(
        MENU_ITEM_TYPE,
        PWSTR,
        (HBITMAP, usize),
        Option<Vec<u16>>,
        u32,
        (HBITMAP, HBITMAP),
    )> for MenuItemShow
{
    fn from(
        style: (
            MENU_ITEM_TYPE,
            PWSTR,
            (HBITMAP, usize),
            Option<Vec<u16>>,
            u32,
            (HBITMAP, HBITMAP),
        ),
    ) -> Self {
        todo!()
    }
}
impl
    Into<(
        MENU_ITEM_TYPE,
        PWSTR,
        (HBITMAP, usize),
        Option<Vec<u16>>,
        u32,
        (HBITMAP, HBITMAP),
    )> for MenuItemShow
{
    fn into(
        self,
    ) -> (
        MENU_ITEM_TYPE,
        PWSTR,
        (HBITMAP, usize),
        Option<Vec<u16>>,
        u32,
        (HBITMAP, HBITMAP),
    ) {
        use MenuItemShow::*;
        match self {
            Bitmap(check, bitmap) => {
                let (type2, check) = check.into();
                let data: (HBITMAP, usize) = bitmap.into();
                (
                    type2 | MFT_BITMAP,
                    PWSTR(data.0.0 as *mut u16),
                    data,
                    None,
                    0,
                    check,
                )
            }
            String(check, string) => {
                let (type2, data) = check.into();
                let (pwstr, buffer) = str_to_pwstr(&string);
                let len = buffer.len() as u32 -1;
                (
                    type2 | MFT_STRING,
                    pwstr,
                    (HBITMAP(std::ptr::null_mut()), 0),
                    Some(buffer),
                    len, 
                    data,
                )
            }
            OwnDraw(_, _) => todo!(),
        }
    }
}
#[derive(Clone, PartialEq, Default)]
pub enum MenuItemBreakType {
    #[default]
    No, //NULL
    NewBreakLine, //MFT_MENUBARBREAK
    NewBreak,     //MFT_MENUBREAK
}
impl From<MENU_ITEM_TYPE> for MenuItemBreakType {
    fn from(style: MENU_ITEM_TYPE) -> Self {
        if style.contains(MFT_MENUBARBREAK) {
            MenuItemBreakType::NewBreakLine
        } else if style.contains(MFT_MENUBREAK) {
            MenuItemBreakType::NewBreak
        } else {
            MenuItemBreakType::No
        }
    }
}
impl Into<MENU_ITEM_TYPE> for MenuItemBreakType {
    fn into(self) -> MENU_ITEM_TYPE {
        match self {
            MenuItemBreakType::No => MENU_ITEM_TYPE(0),
            MenuItemBreakType::NewBreakLine => MFT_MENUBARBREAK,
            MenuItemBreakType::NewBreak => MFT_MENUBREAK,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct MenuItemStyle {
    pub new_break: MenuItemBreakType,
    pub righ_to_left: bool,          //MFT_RIGHTORDER
    pub right_align_from_this: bool, //MFT_RIGHTJUSTIFY
    pub state: MenuItemState,        //MIIM_STATE
}
impl From<(MENU_ITEM_TYPE, MENU_ITEM_STATE)> for MenuItemStyle {
    fn from(style: (MENU_ITEM_TYPE, MENU_ITEM_STATE)) -> Self {
        let new_break = MenuItemBreakType::from(style.0);
        let righ_to_left = style.0.contains(MFT_RIGHTORDER);
        let right_align_from_this = style.0.contains(MFT_RIGHTJUSTIFY);
        let state = MenuItemState::from(style.1);
        MenuItemStyle { new_break, righ_to_left, right_align_from_this, state }
    }
}
impl Into<(MENU_ITEM_TYPE, MENU_ITEM_STATE)> for MenuItemStyle {
    fn into(self) -> (MENU_ITEM_TYPE, MENU_ITEM_STATE) {
        // let (fType, dwTypeData, (hbmpItem, dwItemData), buffer, cch) = self.mtype.into();
        let mut fType = MENU_ITEM_TYPE(0);
        fType |= self.new_break.into();
        if self.righ_to_left {
            fType |= MFT_RIGHTORDER;
        }
        if self.right_align_from_this {
            fType |= MFT_RIGHTJUSTIFY;
        }
        let fstate = self.state.into();
        (fType, fstate)
    }
}
pub enum MenuItemInfo {
    Normal(
        MenuItemStyle,
        MenuItemShow,
        Option<MenuItemID>, /*MIIM_ID*/
    ),
    Child(MenuItemStyle, MenuItemShow, Menu /*MIIM_SUBMENU*/),
    Separator, //MFT_SEPARATOR
}
impl TryFrom<MENUITEMINFOW> for MenuItemInfo {
    type Error = FromUtf16Error;

    fn try_from(style: MENUITEMINFOW) -> Result<Self, Self::Error> {
        if style.fType & MFT_SEPARATOR == MFT_SEPARATOR {
            // 分隔符情况
            return Ok(MenuItemInfo::Separator);
        }
        let show = if style.fType.contains(MFT_STRING) {
            // 将 PWSTR 转换为 Rust 字符串，并处理可能出现的错误
            unsafe {
                MenuItemShow::String(MenuCheckIcon::default(), style.dwTypeData.to_string()?)
            }
        }
        else if style.fType.contains(MFT_BITMAP) {
            let bitmap_icon = MenuItemBitmapIcon::from((style.hbmpItem, style.dwItemData as usize));
            MenuItemShow::Bitmap(MenuCheckIcon::default(), bitmap_icon)
        } else {todo!()};
        // 公共字段提取
        let state = MenuItemState::from(style.fState);
        let menu_item_style = MenuItemStyle::from(style.fType);

        // 根据 hSubMenu 是否为空来确定是普通项还是子菜单项
        if style.hSubMenu.is_null() {
            Ok(MenuItemInfo::Normal(
                menu_item_style,
                show,
                if style.wID != 0 { Some(style.wID as u16) } else { None }, // 转换为 Option<MenuItemID>
            ))
        } else {
            // 创建 Menu 实例（根据实际应用逻辑）
            let menu = Menu::from_handle(style.hSubMenu);
            Ok(MenuItemInfo::Child(
                menu_item_style,
                show,
                menu,
            ))
        }
    }
}
impl Into<(MENUITEMINFOW, Option<Vec<u16>>)> for MenuItemInfo {
    fn into(self) -> (MENUITEMINFOW, Option<Vec<u16>>) {
        use MenuItemInfo::*;
        match self {
            Normal(style, show, id) => {
                let (mtype, fState) = style.into();
                let (
                    mtype2,
                    dwTypeData,
                    (hbmpItem, dwItemData),
                    buffer,
                    cch,
                    (hbmpChecked, hbmpUnchecked),
                ) = show.into();
                let wID = match id {
                    None => 0,
                    Some(num) => num as u32,
                };
                (
                    MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_BITMAP
                            | MIIM_CHECKMARKS
                            | MIIM_DATA
                            | MIIM_FTYPE
                            | MIIM_ID
                            | MIIM_STATE
                            | MIIM_STRING
                            | MIIM_SUBMENU
                            | MIIM_TYPE,
                        fType: mtype | mtype2,
                        fState,
                        wID,
                        hSubMenu: HMENU(0 as *mut c_void),
                        hbmpChecked,
                        hbmpUnchecked,
                        dwItemData,
                        dwTypeData,
                        cch,
                        hbmpItem,
                    },
                    buffer,
                )
            }
            Child(style, show, menu) => {
                let (mtype, fState) = style.into();
                let (
                    mtype2,
                    dwTypeData,
                    (hbmpItem, dwItemData),
                    buffer,
                    cch,
                    (hbmpChecked, hbmpUnchecked),
                ) = show.into();
                let hSubMenu = unsafe { menu.handle() };
                (
                    MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_BITMAP
                            | MIIM_CHECKMARKS
                            | MIIM_DATA
                            | MIIM_FTYPE
                            | MIIM_ID
                            | MIIM_STATE
                            | MIIM_STRING
                            | MIIM_SUBMENU
                            | MIIM_TYPE,
                        fType: mtype | mtype2,
                        fState,
                        wID: 0,
                        hSubMenu,
                        hbmpChecked,
                        hbmpUnchecked,
                        dwItemData,
                        dwTypeData,
                        cch,
                        hbmpItem,
                    },
                    buffer,
                )
            }
            Separator => (
                MENUITEMINFOW {
                    cbSize: size_of::<MENUITEMINFOW>() as u32,
                    fMask: MIIM_BITMAP
                        | MIIM_CHECKMARKS
                        | MIIM_DATA
                        | MIIM_FTYPE
                        | MIIM_ID
                        | MIIM_STATE
                        | MIIM_STRING
                        | MIIM_SUBMENU
                        | MIIM_TYPE,
                    fType: MFT_SEPARATOR,
                    fState: Default::default(),
                    wID: 0,
                    hSubMenu: Default::default(),
                    hbmpChecked: HBITMAP::default(),
                    hbmpUnchecked: HBITMAP::default(),
                    dwItemData: 0,
                    dwTypeData: PWSTR::null(),
                    cch: 0,
                    hbmpItem: HBITMAP::default(),
                },
                None,
            ),
        }
    }
}
//----------------------------------------------
pub enum MenuItemPos {
    CostomId(MenuItemID),
    Position(u16),
}
#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct Menu {
    handle: HMENU,
}
pub enum MenuCheckShow {
    Normal,
    AlignToBmp,
    NoCheck,
}
use MenuItemPos::*;
pub struct MenuStyle {
    check_show: MenuCheckShow,
    auto_dismiss: bool,
    drag_dorp: bool,
    mode_less: bool,
    notify_by_id: bool,
    max_height: Option<NonZeroU32>,
    help_id: Option<HelpId>,
}
impl Into<MENUINFO> for MenuStyle {
    fn into(self) -> MENUINFO {
        let mut dwStyle = windows::Win32::UI::WindowsAndMessaging::MENUINFO_STYLE(0);
        if self.auto_dismiss {
            dwStyle |= MNS_AUTODISMISS;
        };
        if self.drag_dorp {
            dwStyle |= MNS_DRAGDROP;
        };
        if self.mode_less {
            dwStyle |= MNS_MODELESS;
        };
        if self.notify_by_id {
            dwStyle |= MNS_NOTIFYBYPOS;
        };
        match self.check_show {
            MenuCheckShow::Normal => (),
            MenuCheckShow::AlignToBmp => dwStyle |= MNS_CHECKORBMP,
            MenuCheckShow::NoCheck => dwStyle |= MNS_NOCHECK,
        };
        MENUINFO {
            cbSize: size_of::<MENUINFO>() as u32,
            fMask: MIM_HELPID | MIM_MAXHEIGHT | MIM_STYLE,
            dwStyle,
            cyMax: match self.max_height {
                None => 0,
                Some(s) => s.into(),
            },
            dwContextHelpID: option_into(self.help_id) as u32,
            dwMenuData: 0,
            hbrBack: HBRUSH(0 as *mut c_void),
        }
    }
}
impl Menu {
    pub unsafe fn from_handle(handle: HMENU) -> Self {
        Menu { handle }
    }
    pub fn new() -> Result<Self> {
        Ok(Menu {
            handle: unsafe { CreatePopupMenu()? },
        })
    }
    pub fn from_mut_ref(handle: &mut HMENU) -> &mut Self {
        unsafe { &mut *(handle as *mut HMENU as *mut Self) }
    }
    pub unsafe fn handle(mut self) -> HMENU {
        let handle = self.handle;
        self.handle = HMENU(NULL_PTR());
        handle
    }
    pub fn is_invalid(&self) -> bool {
        self.handle.0 == NULL_PTR()
    }
    pub fn item_info_list(&self) -> Result<Vec<MenuItemInfo>> {
        let mut num = self.item_count();
        let result = if num == 0 {
            return result;
        } else {
            Vec::with_capacity(num)
        };
        for i in 0..num {
            Vec.push(self.get_item_info(MenuItemPos::Position(i))?)
        } 
    }
    pub fn get_item_info(&self, pos: MenuItemPos) -> Result<MenuItemInfo> {
        let (id, bp) = match pos {
            MenuItemPos::Position(i) => (i as u32, true), 
            MenuItemPos::CostomId(i) => (i as u32, false)
        };
        let mut lpmii = MENUITEMINFOW::default();
        lpmii.cbSize = size_of<MENUITEMINFOW> as u32;
        lpmii.fMask = MIIM_BITMAP
            | MIIM_CHECKMARKS
            | MIIM_DATA
            | MIIM_FTYPE
            | MIIM_ID
            | MIIM_STATE
            | MIIM_STRING
            | MIIM_SUBMENU
            | MIIM_TYPE;
        unsafe {
            GetMenuItemInfoW(self.handle, id, bp, &mut lpmii)?
        }
        if lpmii.fType.conncttts(MFT_STRING) {
            let buffer = vec![0u16; lpmii.cch+1];
            lpmii.fMask = MIIM_STRING;
            lpmii.cch +=1;
            lpmii.dwTypeData=PWSTR(buffer.as_mut_ptr())
            unsafe {
                GetMenuItemInfoW(self.handle, id, bp, &mut lpmii)?
            }
        }
        Ok(MenuItemInfo::try_from(lpmii)?)
    }
    pub fn item_count(&self) -> Result<u16> {
        let num: Result<u16, _> = unsafe {GetMenuItemCount()}.try_into();
        match num {
            Ok(num) => Ok(num), 
            Err(e) => Err(correct_error())
        }
    }
    pub fn insert_item(&mut self, before_item: Option<MenuItemPos>, item: MenuItemInfo) -> Result<()> {
        let (menu_item_info, _buffer) = item.into();
        let (id, flag) = match before_item {
            None => {
                
                return; 
            },
            Some(CostomId(id)) => (id, false),
            Some(Position(pos)) => (pos, true),
        };
        unsafe {
            Ok(InsertMenuItemW(
                self.handle,
                id as u32,
                flag,
                &menu_item_info,
            )?)
        }
    }
}
impl Drop for Menu {
    fn drop(&mut self) {
        //DestroyMenu
        if !self.is_invalid() {
            unsafe {
                let _ = DestroyMenu(self.handle);
            }
        }
    }
}

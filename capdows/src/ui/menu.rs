use super::*; //部分触发代码使用AI编写
use std::any::Any;
pub type MenuItemID = u16;
use std::ops::Deref;
use std::ops::DerefMut;
#[derive(Debug)]
#[repr(transparent)]
pub struct Menu {
    handle: HMENU,
}
#[derive(Debug)]
#[repr(transparent)]
pub struct MenuBar {
    handle: Menu,
}
impl Deref for MenuBar {
    type Target = Menu;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
impl DerefMut for MenuBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}
impl MenuBar {
    #[inline]
    pub const unsafe fn from_menu(menu: Menu) -> Self {
        MenuBar { handle: menu }
    }
    #[inline]
    pub const unsafe fn from_handle(menu: HMENU) -> Self {
        unsafe {
            MenuBar {
                handle: Menu::from_handle(menu),
            }
        }
    }
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(unsafe {
            Self::from_menu(Menu::from_handle(match CreateMenu() as usize {
                0 => return Error::correct_error(),
                x => x as *mut c_void,
            }))
        })
    }
    #[inline]
    pub const unsafe fn null() -> Self {
        unsafe {
            Self {
                handle: Menu::null(),
            }
        }
    }
    #[inline]
    pub unsafe fn handle(self) -> HMENU {
        unsafe { self.handle.handle() }
    }
}

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
    fn from(_: (HBITMAP, usize)) -> Self {
        todo!()
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
impl Into<MENU_ITEM_STATE> for MenuItemDisabledState {
    fn into(self) -> MENU_ITEM_STATE {
        let mut mtype = MENU_ITEM_STATE(0);
        match self {
            MenuItemDisabledState::Enabled => (),
            MenuItemDisabledState::Disabled => mtype |= MFS_DISABLED,
            MenuItemDisabledState::DisabledNoGrayed => mtype |= MENU_ITEM_STATE(MF_DISABLED.0),
            MenuItemDisabledState::Grayed => mtype |= MENU_ITEM_STATE(MF_GRAYED.0),
        };
        mtype
    }
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
        mtype | self.state.into()
    }
}
impl From<MENU_ITEM_STATE> for MenuItemState {
    fn from(_: MENU_ITEM_STATE) -> Self {
        todo!()
    }
}
#[derive(Default)]
pub struct MenuCheckIcon {
    pub checked: MenuCheckedIcon,
    pub unchecked: Option<Bitmap>,
}
impl From<(MENU_ITEM_TYPE, (HBITMAP, HBITMAP))> for MenuCheckIcon {
    fn from(_: (MENU_ITEM_TYPE, (HBITMAP, HBITMAP))) -> Self {
        todo!()
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
        u32,
        (HBITMAP, HBITMAP),
    )> for MenuItemShow
{
    fn from(
        _: (
            MENU_ITEM_TYPE,
            PWSTR,
            (HBITMAP, usize),
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
                let len = buffer.len() as u32 - 1;
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
    fn from(_: MENU_ITEM_TYPE) -> Self {
        todo!()
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
pub enum MenuCheckedIcon {
    #[default]
    CheckMark,
    Cullet, //MFT_RADIOCHECK
    Costom(Bitmap),
}
#[derive(Clone, PartialEq, Default)]
pub struct MenuItemStyle {
    pub new_break: MenuItemBreakType,
    pub righ_to_left: bool,          //MFT_RIGHTORDER
    pub right_align_from_this: bool, //MFT_RIGHTJUSTIFY
    pub state: MenuItemState,        //MIIM_STATE
}
impl From<(MENU_ITEM_TYPE, MENU_ITEM_STATE)> for MenuItemStyle {
    fn from(_: (MENU_ITEM_TYPE, MENU_ITEM_STATE)) -> Self {
        todo!()
    }
}
impl Into<(MENU_ITEM_TYPE, MENU_ITEM_STATE)> for MenuItemStyle {
    fn into(self) -> (MENU_ITEM_TYPE, MENU_ITEM_STATE) {
        // let (fType, dwTypeData, (hbmpItem, dwItemData), buffer, cch) = self.mtype.into();
        #[allow(non_snake_case)]
        let mut fType = MENU_ITEM_TYPE(0);
        fType |= self.new_break.into();
        if self.righ_to_left {
            fType |= MFT_RIGHTORDER;
        }
        if self.right_align_from_this {
            fType |= MFT_RIGHTJUSTIFY;
        }
        #[allow(non_snake_case)]
        let fstate = self.state.into();
        (fType, fstate)
    }
}
pub enum MenuItem<'a> {
    Normal(
        MenuItemStyle,
        MenuItemShow,
        Option<MenuItemID>, /*MIIM_ID*/
    ),
    Child(
        MenuItemStyle,
        MenuItemShow,
        &'a mut Menu, /*MIIM_SUBMENU*/
    ),
    Separator, //MFT_SEPARATOR
}
impl From<MENUITEMINFOW> for MenuItem<'_> {
    fn from(_: MENUITEMINFOW) -> Self {
        todo!()
    }
}
impl Into<(MENUITEMINFOW, Option<Vec<u16>>)> for MenuItem<'_> {
    fn into(self) -> (MENUITEMINFOW, Option<Vec<u16>>) {
        use MenuItem::*;
        match self {
            Normal(style, show, id) => {
                #[allow(non_snake_case)]
                let (mtype, fState) = style.into();
                #[allow(non_snake_case)]
                let (
                    mtype2,
                    dwTypeData,
                    (hbmpItem, dwItemData),
                    buffer,
                    cch,
                    (hbmpChecked, hbmpUnchecked),
                ) = show.into();
                #[allow(non_snake_case)]
                let wID = match id {
                    None => 0,
                    Some(num) => num as u32,
                };

                (
                    MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_TYPE
                            | MIIM_CHECKMARKS
                            | MIIM_DATA
                            | MIIM_ID
                            | MIIM_STATE
                            | (if hbmpItem.is_invalid() {
                                MENU_ITEM_MASK(0)
                            } else {
                                MIIM_BITMAP
                            }),
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
                #[allow(non_snake_case)]
                let (mtype, fState) = style.into();
                #[allow(non_snake_case)]
                let (
                    mtype2,
                    dwTypeData,
                    (hbmpItem, dwItemData),
                    buffer,
                    cch,
                    (hbmpChecked, hbmpUnchecked),
                ) = show.into();
                #[allow(non_snake_case)]
                let hSubMenu = unsafe { menu.get_handle() };
                (
                    MENUITEMINFOW {
                        cbSize: size_of::<MENUITEMINFOW>() as u32,
                        fMask: MIIM_TYPE
                            | MIIM_CHECKMARKS
                            | MIIM_DATA
                            | MIIM_ID
                            | MIIM_STATE
                            | (if hbmpItem.is_invalid() {
                                MENU_ITEM_MASK(0)
                            } else {
                                MIIM_BITMAP
                            }),
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
                    fMask: MIIM_FTYPE,
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
        #[allow(non_snake_case)]
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
    ///使用此函数Menu会自动释放，需要注意HMENU是否为Windows管理释放
    #[inline]
    pub const unsafe fn from_handle(handle: HMENU) -> Self {
        Menu { handle }
    }
    #[inline]
    pub const unsafe fn null() -> Self {
        Menu {
            handle: HMENU(0 as *mut c_void),
        }
    }
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Menu {
            handle: unsafe { CreatePopupMenu()? },
        })
    }
    pub fn nullify(&mut self) {
        self.handle = HMENU(NULL_PTR());
    }
    #[inline]
    pub fn from_mut_ref<'a>(handle: &'a mut HMENU) -> &'a mut Menu {
        unsafe { std::mem::transmute(handle) }
    }
    #[inline]
    pub unsafe fn handle(mut self) -> HMENU {
        let handle = self.handle;
        self.handle = HMENU(NULL_PTR());
        handle
    }
    #[inline]
    pub const unsafe fn get_handle(&self) -> HMENU {
        self.handle
    }
    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.handle.0 as usize == 0
    }
    pub fn item_count(&self) -> Result<MenuItemID> {
        match unsafe { GetMenuItemCount(Some(self.handle)) } {
            -1 => Err(WinError::correct_error()),
            x => Ok(x as MenuItemID),
        }
    }
    /// 如果菜单栏在创建窗口后发生更改，则需要调用window.redraw_menu_bar()来绘制更改后的菜单栏。
    pub fn insert_item(&mut self, before_item: Option<MenuItemPos>, item: MenuItem) -> Result<()> {
        let (menu_item_info, _buffer) = item.into();
        let (id, flag) = match before_item {
            None => (
                self.item_count()?, //在最后一项追加
                true,
            ),
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
    pub fn set_item_state(
        &mut self,
        item: MenuItemPos,
        state: MenuItemDisabledState,
    ) -> Result<()> {
        let (id, flag) = match item {
            CostomId(id) => (id, MF_BYCOMMAND),
            Position(pos) => (pos, MF_BYPOSITION),
        };
        let state: MENU_ITEM_STATE = state.into();
        if unsafe { EnableMenuItem(self.handle, id as u32, MENU_ITEM_FLAGS(state.0) | flag).0 }
            == -1
        {
            Err(ERROR_NOT_FOUND)
        } else {
            Ok(())
        }
    }
    pub fn remove_item(&mut self, item: MenuItemPos) -> Result<()> {
        let (id, flag) = match item {
            CostomId(id) => (id, MF_BYCOMMAND),
            Position(pos) => (pos, MF_BYPOSITION),
        };
        match unsafe { DeleteMenu(self.handle, id as u32, flag) } {
            Ok(()) => Ok(()),
            Err(_) => Err(WinError::correct_error()),
        }
    }
    pub fn clear(&mut self) -> Result<()> {
        for _ in 0..self.item_count()? {
            self.remove_item(MenuItemPos::Position(0))?;
        }
        Ok(())
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

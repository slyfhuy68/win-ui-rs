use super::*;
//--------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
pub struct Menu {
    pub handle: HMENU,
}
#[derive(Clone, PartialEq)]
pub enum MenuItemBitmapIcon {
    ///位图
    Bitmap(super::image::Bitmap /*位图的句柄*/),
    ///Windows 系统图标
    SystemIcon(usize), //HBMMENU_SYSTEM
    ///在消息接收器的回调中手动绘制。
    CallBack,        //HBMMENU_CALLBACK
    ///弹出菜单的“关闭”按钮。
    CloseP,          //HBMMENU_POPUP_CLOSE
    ///菜单栏的“关闭”按钮。
    CloseB,          //HBMMENU_MBAR_CLOSE
    ///菜单栏的“已禁用关闭”按钮。
    CloseBD,         //HBMMENU_MBAR_CLOSE_D
    ///弹出菜单的最小化按钮。
    MimimizeP,       //HBMMENU_POPUP_MINIMIZE
    ///菜单栏的最小化按钮。
    MimimizeB,       //HBMMENU_MBAR_MINIMIZE
    ///已禁用菜单栏的最小化按钮。
    MimimizeBD,      //HBMMENU_MBAR_MINIMIZE_D
    ///弹出菜单的最大化按钮。
    MaximizeP,       //HBMMENU_POPUP_MAXIMIZE
    ///弹出菜单的“还原”按钮。
    RestoreP,        //HBMMENU_POPUP_RESTORE
    ///菜单栏的“还原”按钮。
    RestoreB,        //HBMMENU_MBAR_RESTORE
}
impl From<(HBITMAP, usize)> for MenuItemBitmapIcon {
    fn from(style: (HBITMAP, usize)) -> Self {
        todo!()
    }
}
impl Into<(HBITMAP, usize)> for MenuItemBitmapIcon {
    fn into(self) -> (HBITMAP, usize) {
        (match self {
            Bitmap(b) => b.into(),
            SystemIcon(icon_id) => return (HBMMENU_SYSTEM, icon_id), 
            CallBack => HBMMENU_CALLBACK, 
            CloseP => HBMMENU_POPUP_CLOSE,  
            CloseB => BMMENU_MBAR_CLOSE, 
            CloseBD => HBMMENU_MBAR_CLOSE_D,  
            MimimizeP => HBMMENU_POPUP_MINIMIZE,   
            MimimizeB => HBMMENU_MBAR_MINIMIZE,  
            MimimizeBD => HBMMENU_MBAR_MINIMIZE_D,
            MaximizeP => HBMMENU_POPUP_MAXIMIZE,   
            RestoreP => HBMMENU_POPUP_RESTORE,  
            RestoreB => HBMMENU_MBAR_RESTORE,       
        }, 0)
    }
}
#[derive(Clone, PartialEq)]
pub enum MenuItemDisabledState {
    ///启用
    Enabled,  //MFS_ENABLED
    #[doc(hidden)]
    DisabledNoGrayed, //MF_DISABLED
    #[doc(hidden)]
    Grayed,   //MF_GRAYED
    ///禁用
    Disabled, //MFS_DISABLED
}

#[derive(Clone, PartialEq)]
pub struct MenuItemState {
    ///启用状态和灰显状态
    pub state: MenuItemDisabledState,
    ///是否高亮显示
    pub hilite: bool,  //true MFS_HILITE,false MFS_UNHILITE
    pub checked: bool, //true MFS_CHECKED,false MFS_UNCHECKED
}
impl Into<MENU_ITEM_STATE> for MenuItemState {
    fn into(self) -> MENU_ITEM_STATE {
        let mtype = MENU_ITEM_TYPE(0);
        if self.hilite{
            mtype |= MFS_HILITE;
        }
        if self.checked{
            mtype |= MFS_CHECKED;
        }
        match self.state {
            Enabled => (), 
            Disabled => mtype |= MFS_DISABLED, 
            DisabledNoGrayed => mtype |= MENU_ITEM_TYPE(MF_DISABLED.0), 
            Grayed => mtype |= MENU_ITEM_TYPE(MF_GRAYED.0), 
        }
    }
}
impl From<MENU_ITEM_STATE> for MenuItemState {
    fn from(style: MENU_ITEM_STATE) -> Self {
        todo!()
    }
}
#[derive(Clone, PartialEq)]
pub enum MenuItemShow {
    Bitmap(
        MenuItemBitmapIcon,
    ), //MFT_BITMAP? | MIIM_BITMAP
    String(
        String, /* dwTypeData 成员是指向以 null 结尾的字符串的指针*/
    ), //MFT_STRING
    OwnDraw(Option<Box<dyn Any>>), //MFT_OWNERDRAW
}
impl From<(MENU_ITEM_TYPE, PWSTR, (HBITMAP, usize), u32)> for MenuItemShow {
    fn from(style: (MENU_ITEM_TYPE, PWSTR, (HBITMAP, usize), u32)) -> Self {
        todo!()
    }
}
impl Into<(MENU_ITEM_TYPE, PWSTR, (HBITMAP, usize), Option<Vec<u16>>, u32)> for MenuItemShow {
    fn into(self) -> (MENU_ITEM_TYPE, PWSTR, (HBITMAP, usize), Option<Vec<u16>>, u32) {
        match self{
            Bitmap(bitmap) => {
                let data = bitmap.into(), 
                (MFT_BITMAP, PWSTR(data.0.0), data, None)
            }
            String(string) => {
                let (pcwstr, buffer) = str_to_pcwstr(string);
                (MFT_STRING, pcwstr, (HBITMAP(0), 0), Some(buffer), string.len())
            }
            OwnDraw(data) => todo!()
        }
    }
}
pub enum MenuItemBreakType{
    No, //NULL
    NewBreakLine, //MFT_MENUBARBREAK
    NewBreak, //MFT_MENUBREAK
    
}
impl From<(MENU_ITEM_TYPE)> for MenuItemBreakType {
    fn from(style: MENUITEMINFOW) -> Self {
        todo!()
    }
}
impl Into<MENU_ITEM_TYPE> for MenuItemBreakType {
    fn into(self) -> MENU_ITEM_TYPE {
        match self {
            No => MENU_ITEM_TYPE(0), 
            NewBreakLine => MFT_MENUBARBREAK, 
            NewBreak => MFT_MENUBREAK
        }
    }
}
pub enum MenuCheckedIcon{
    CheckMark, 
    Cullet, //MFT_RADIOCHECK
    Costom(Bitmap)
}
pub struct MenuCheckIcon{
    pub checked: MenuCheckedIcon, 
    pub unchecked: Option<Bitmap>, 
}
impl From<(MENU_ITEM_TYPE, (HBITMAP, HBITMAP))> for MenuCheckIcon {
    fn from(style: MENUITEMINFOW) -> Self {
        todo!()
    }
}
impl Into<(MENU_ITEM_TYPE, (HBITMAP, HBITMAP))> for MenuCheckIcon {
    fn into(self) -> (MENU_ITEM_TYPE, (HBITMAP, HBITMAP)) {
        let (itype, checked) = match self.ckecked {
            CheckMark => (MENU_ITEM_TYPE(0), HBITMAP(NULL_PTR())), 
            Cullet => (MFT_RADIOCHECK, HBITMAP(NULL_PTR())), 
            Costom(bitmap) => (MENU_ITEM_TYPE(0), bitmap.into())
        };
        let unchecked = match self.unckecked {
            None => HBITMAP(NULL_PTR()), 
            Some(bitmap) => bitmap.into()
        }
        (itype, (checked, unchecked))
    }
}
#[derive(Clone, PartialEq)]
pub struct MenuItemStyle {
    pub mtype: MenuItemShow,
    pub new_break: MenuItemBreakType,
    pub righ_to_left: bool,                            //MFT_RIGHTORDER
    pub right_align_from_this: bool,                    //MFT_RIGHTJUSTIFY
    pub state: MenuItemState,                          //MIIM_STATE
    pub checks: MenuCheckIcon,            //MIIM_CHECKMARKS
}
impl From<MENUITEMINFOW> for MenuStyle {
    fn from(style: MENUITEMINFOW) -> Self {
        todo!()
    }
}
impl Into<(MENUITEMINFOW, Option<Vec<u16>>)> for MenuStyle {
    fn into(self) -> (MENUITEMINFOW, Option<Vec<u16>>) {
        //(MENU_ITEM_TYPE, PWSTR, (HBITMAP, usize))
        let (fType, dwTypeData, (hbmpItem, dwItemData), buffer, cch) = self.mtype.into();
        fType |= self.new_break.into();
        if self.righ_to_left {
            fType |= MFT_RIGHTORDER;
        }
        if self.right_align_from_this {
            fType |= MFT_RIGHTJUSTIFY;
        }
        let fState = self.state.into();
        let (ftype2, (hbmpChecked, hbmpUnchecked)) = self.checks.into();
        fType |= ftype2;
        (MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32, 
            fMask: MIIM_FULL, 
            fType, 
            fState, 
            wID: 0, 
            hSubMenu: Default::default(), 
            hbmpChecked,
            hbmpUnchecked,
            dwItemData,
            dwTypeData,
            cch,
            hbmpItem,
        }, buffer)
    }
}
#[derive(Clone, PartialEq)]
pub enum MenuItem {
    Normal(
        MenuItemStyle,
        Option<u32 /*自定义菜单标识符*/>, /*MIIM_ID*/
    ),
    Child(MenuItemStyle, Menu /*MIIM_SUBMENU*/),
    Separator, //MFT_SEPARATOR
}
const MIIM_FULL: MENU_ITEM_MASK = MIIM_BITMAP|
MIIM_CHECKMARKS|
MIIM_DATA|
MIIM_FTYPE|
MIIM_ID|
MIIM_STATE|
MIIM_STRING|
MIIM_SUBMENU|
MIIM_TYPE;
impl From<MENUITEMINFOW> for MenuStyle {
    fn from(style: MENUITEMINFOW) -> Self {
        todo!()
    }
}
impl Into<(MENUITEMINFOW, Option<Vec<u16>>)> for MenuItem {
    fn into(self) -> (MENUITEMINFOW, Option<Vec<u16>>) {
        match self {
            Normal(style, id) => {
                let mut result = self.style.into();
                result.0.wID = match id {
                    None => 0, 
                    Some(num) => num
                };
                result
            }
            Child(style, menu) => {
                let mut result = self.style.into();
                result.0.hSubMenu = menu.into();
                result
            }
            Separator => (MENUITEMINFOW{
                cbSize: size_of::<MENUITEMINFOW>() as u32, 
                fMask: MIIM_FULL, 
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
            }, None), 
        }
    }
}
//----------------------------------------------
#[derive(Clone, PartialEq)]
pub struct MenuStyle {
    auto_dismiss: bool,   //MNS_AUTODISMISS
    checked_or_bmp: bool, //MNS_CHECKORBMP
    drag_drop: bool,      //MNS_DRAGDROP
    modleless: bool,      //MNS_MODELESS
    no_check: bool,       //MNS_NOCHECK
    notify_by_pos: bool,  //MNS_NOTIFYBYPOS
}
impl From<MENUINFO_STYLE> for MenuStyle {
    fn from(style: MENUINFO_STYLE) -> Self {
        todo!()
    }
}

impl Into<MENUINFO_STYLE> for MenuStyle {
    fn into(self) -> MENUINFO_STYLE {
        todo!()
    }
}
#[derive(Clone, PartialEq)]
pub struct MenuShowAnimation {
    horneg: bool,
    horpos: bool,
    verneg: bool,
    verpos: bool,
}
#[derive(Clone, PartialEq, Default)]
pub enum MenuShowHorizontallyPosition {
    Center,
    #[default]
    Left,
    Right,
}
#[derive(Clone, PartialEq, Default)]
pub enum MenuShowVerticalPosition {
    Center,
    #[default]
    Top,
    Bottom,
}
#[derive(Clone, PartialEq)]
pub struct MenuShowStyle {
    horizontal_position: MenuShowHorizontallyPosition,
    vertical_position: MenuShowVerticalPosition,
    only_left_mouse_button: bool,
    animation: Option<MenuShowAnimation>,
}
impl Default for MenuShowStyle {
    fn default() -> Self {
        Self {
            horizontal_position: Default::default(),
            vertical_position: Default::default(),
            only_left_mouse_button: true,
            animation: None,
        }
    }
}
impl Menu {
    //     pub fn create() -> Result<Self> {
    //         todo!() //CreateMenu
    //     }
    //     pub fn create_popup() -> Result<Self> {
    //         todo!() //CreatePopupMenu
    //     }
    //     pub fn from_list(itype: MenuType, items: Vec<MenuItem>) -> Result<Self> {
    //         let mut result_menu = match itype {
    //             MenuType::Menu => Self::create()?,
    //             MenuType::PopupMenu => Self::create_popup()?,
    //         };
    //         if items.len() > u32::MAX as usize {
    //             return Err(Error::new(ERROR_SECRET_TOO_LONG.into(), ""));
    //         }
    //         for (index, item) in items.into_iter().enumerate() {
    //             result_menu.insert(Some(PosOrID::Position(index.try_into()?)), item)?;
    //         }
    //         Ok(result_menu)
    //     }
    //     // pub fn load_from(ef: ExecutableFile, menu: Either<&str, usize>) -> Result<Self> {
    //     //     todo!() //LoadMenuW
    //     // }
    //     // pub fn append(&mut self, new_item: MenuItem) -> Result<()> {
    //     //     todo!() //AppendMenu
    //     // }
    //     pub fn insert(
    //         &mut self,
    //         _next_item: Option<PosOrID>, /*None: 给api提供0、true，表示在第一个位置插入*/
    //         _new_item: MenuItem,
    //     ) -> Result<()> {
    //         todo!() //InsertMenuItemW
    //     }
    //     pub fn set_radio(&mut self, items: PosesOrIDs<(u32, u32, u32)>) -> Result<()> {
    //         todo!() //CheckMenuRadioItem
    //     }
    //     pub fn delete_item(&mut self, item_pos: PosOrID) -> Result<()> {
    //         todo!() //DeleteMenu
    //     }
    //     pub fn enable_item(&mut self, item_pos: PosOrID, state: MenuItemDisabledState) -> Result<()> {
    //         todo!() //EnableMenuItem
    //     }
    //     pub fn end_menu(/*needn't self*/) -> Result<()> {
    //         todo!() //EndMenu
    //     }
    //     pub fn get_default_item(
    //         &self,
    //         by_pos: bool,
    //         go_into_popups: bool, /*GMDI_GOINTOPOPUPS*/
    //         use_disabled: bool,   /*GMDI_USEDISABLED*/
    //     ) -> MenuItem {
    //         todo!() //GetMenuDefaultItem
    //     }
    //     //--------------------------------
    //     pub fn get_style(&self) -> Result<MenuStyle> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_STYLE/*}里*/,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(MenuStyle::from(menu_info./*这{*/dwStyle /*}里*/))
    //     }
    //     pub fn get_max_high(&self) -> Result<u32> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_MAXHEIGHT/*}里*/,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(menu_info./*这{*/cyMax /*}里*/)
    //     }
    //     pub fn get_backround_brush(&self) -> Result<Brush> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_BACKGROUND/*}里*/,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(Brush::from(menu_info./*这{*/hbrBack /*}里*/))
    //     }
    //     pub fn get_data(&self) -> Result<usize> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_MENUDATA/*}里*/,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(menu_info./*这{*/dwMenuData /*}里*/)
    //     }
    //     pub fn set_style(&mut self, style: MenuStyle) -> Result<()> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_STYLE/*}里*/,
    //             dwStyle: style.into(),
    //             ..MENUINFO::default()
    //         };
    //         unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(())
    //     }
    //     pub fn set_max_high(&mut self, max_high: u32) -> Result<()> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_MAXHEIGHT/*}里*/,
    //             cyMax: max_high,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(())
    //     }
    //     pub fn set_backround_brush(&mut self, cbrush: Brush) -> Result<()> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_BACKGROUND/*}里*/,
    //             hbrBack: cbrush.into(),
    //             ..MENUINFO::default()
    //         };
    //         unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(())
    //     }
    //     pub fn set_data(&mut self, data: usize) -> Result<()> {
    //         let mut menu_info = MENUINFO {
    //             cbSize: std::mem::size_of::<MENUINFO>() as u32,
    //             fMask: /*这{*/MIM_MENUDATA/*}里*/,
    //             dwMenuData: data,
    //             ..MENUINFO::default()
    //         };
    //         unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
    //         Ok(())
    //     }
    //     pub fn get_context_help(&self) -> Result<Option<Help>> {
    //         Ok(match unsafe { GetMenuContextHelpId(self.handle) } {
    //             0 => None,
    //             x => Some(Help { handle: x }),
    //         })
    //     }
    //     pub fn set_context_help(&mut self, help_id: Option<u32>) -> Result<()> {
    //         let help = match help_id {
    //             None => 0,
    //             Some(x) => x,
    //         };
    //         unsafe { SetMenuContextHelpId(self.handle, help) }
    //     }
    //     pub fn get_sub_menu(&self, position: u32) -> Option<Self> {
    //         todo!() //GetSubMenu
    //     }
    //     pub fn count(&self) -> Result<u16> {
    //         match { unsafe { GetMenuItemCount(Some(self.handle)) } } {
    //             -1 => Err(Error::from_win32()),
    //             x => Ok(x.try_into()?),
    //         }
    //     }
    //     pub fn get_items_clone(&self) -> Result<Vec<MenuItem>> {
    //         let mut result: Vec<MenuItem> = Vec::new();
    //         for i in 0..self.count()? {
    //             result.push(self.get_item_clone(PosOrID::Position(i.into()))?)
    //         }
    //         Ok(result)
    //     }
    //     pub fn get_item_id(&self, pos: u32) -> Option<u32> {
    //         todo!() //GetMenuItemID
    //     }
    //     pub fn get_item_clone(&self, pos: PosOrID) -> Result<MenuItem> {
    //         todo!() //GetMenuItemInfo
    //     }
    //     pub fn get_menu_item_coordinates(&self, pos: PosOrID) -> Result<Rectangle> {
    //         todo!() //GetMenuItemRect
    //     }
    pub fn is_invalid(&self) -> bool {
        self.handle.0 == NULL_PTR()
    }
    //     pub fn sure_invalid(&self) -> bool {
    //         unsafe { (!IsMenu(self.handle)).into() }
    //     }
    //     pub fn get_menu_item_pos_from_point(&self, window: Option<Window>, x: i32, y: i32) -> u32 {
    //         //固定返回Pos
    //         todo!() //MenuItemFromPoint
    //     }
    //     pub fn set_defult_item(&mut self, item: Option<PosOrID>) -> Result<()> {
    //         todo!() //SetMenuDefaultItem
    //     }
    //     pub fn remove_item(&mut self, item: PosOrID) -> Result<Option<Self>> {
    //         let result = match item {
    //             PosOrID::Position(y) => Ok(self.get_sub_menu(y)),
    //             _ => Ok(None),
    //         };
    //         let (flag, ddata) = item.to_u32_f();
    //         unsafe {
    //             RemoveMenu(self.handle, ddata, flag)?;
    //         };
    //         result
    //     }
    //     pub fn updeate_item(&mut self, item: PosOrID, new_item: MenuItem) -> Result<()> {
    //         todo!() //SetMenuItemInfoW
    //     }
    //     pub fn show(
    //         &self,
    //         style: MenuShowStyle,
    //         owner: Window,
    //         x: i32,
    //         y: i32,
    //         tpmp: Option<Rectangle>,
    //         notify /*记得取反*/: bool,
    //     ) -> Result<()> {
    //         todo!() //TrackPopupMenuEx,不指定TPM_RETURNCMD
    //     }
    //     pub fn show_retcmd(
    //         &self,
    //         style: MenuShowStyle,
    //         owner: Window,
    //         x: i32,
    //         y: i32,
    //         tpmp: Option<Rectangle>,
    //         notify /*记得取反*/: bool,
    //     ) -> Result<i32> {
    //         todo!() //TrackPopupMenuEx,指定TPM_RETURNCMD
    //     }
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

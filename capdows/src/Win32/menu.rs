use super::*;
//--------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
pub enum MenuType {
    Menu, //为什么不给滑稽申请uincode这个🤪太不像了
    PopupMenu,
}
#[derive(Clone, PartialEq)]
pub struct Menu {
    pub mtype: MenuType,
    pub handle: HMENU,
}
pub enum PosOrID {
    Identifier(u32), //MF_BYCOMMAND
    Position(u32),   //MF_BYPOSITION
}
pub enum PosesOrIDs<A> {
    Identifier(A), //MF_BYCOMMAND
    Position(A),   //MF_BYPOSITION
}
impl PosOrID {
    fn to_u32(self) -> u32 {
        match self {
            PosOrID::Identifier(x) => return x,
            PosOrID::Position(y) => return y,
        }
    }
    fn to_u32F(self) -> (MENU_ITEM_FLAGS, u32) {
        match self {
            PosOrID::Identifier(x) => return (MF_BYCOMMAND, x),
            PosOrID::Position(y) => return (MF_BYPOSITION, y),
        }
    }
    fn to_u32B(self) -> (bool, u32) {
        match self {
            PosOrID::Identifier(x) => return (false, x),
            PosOrID::Position(y) => return (true, y),
        }
    }
}
//--------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
pub enum MenuItemBitmapIcon {
    Bitmap(crate::Win32::image::Bitmap /*位图的句柄*/),
    SystemIcon(u32), //HBMMENU_SYSTEM
    CallBack,        //HBMMENU_CALLBACK
    CloseP,          //HBMMENU_POPUP_CLOSE
    CloseB,          //HBMMENU_MBAR_CLOSE
    CloseBD,         //HBMMENU_MBAR_CLOSE_D
    MimimizeP,       //HBMMENU_POPUP_MINIMIZE
    MimimizeB,       //HBMMENU_MBAR_MINIMIZE
    MimimizeBD,      //HBMMENU_MBAR_MINIMIZE_D
    MaximizeP,       //HBMMENU_POPUP_MAXIMIZE
    RestoreP,        //HBMMENU_POPUP_RESTORE
    RestoreB,        //HBMMENU_MBAR_RESTORE
}
#[derive(Clone, PartialEq)]
pub enum MenuItemDisabledState {
    Grayed,   //0x00000001L
    Disabled, //0x00000002L
    Enabled,  //0x00000000L
}
#[derive(Clone, PartialEq)]
pub struct MenuItemState {
    state: MenuItemDisabledState,
    hilite: bool,  //true MFS_HILITE,false MFS_UNHILITE
    checked: bool, //true MFS_CHECKED,false MFS_UNCHECKED
}
#[derive(Clone, PartialEq)]
pub enum MenuItemShow {
    Bitmap(
        crate::Win32::image::Bitmap, /*dwTypeData地位是位图句柄*/
    ), //MFT_BITMAP
    String(
        String, /* dwTypeData 成员是指向以 null 结尾的字符串的指针*/
    ), //MFT_STRING
    OwnDraw, //MFT_OWNERDRAW
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
#[derive(Clone, PartialEq)]
pub enum MenuItemBreakType {
    NoBreak,   //NULL
    BarBreak,  //MFT_MENUBARBREAK
    MenuBreak, //MFT_MENUBREAK
}
#[derive(Clone, PartialEq)]
pub struct MenuItemCheckStyle {
    radio_check: bool,         //MFT_RADIOCHECK
    unchecked: Option<Bitmap>, //hbmpUnchecked
    checked: Option<Bitmap>,   //hbmpChecked
}
#[derive(Clone, PartialEq)]
pub struct MenuItemStyle {
    from: Menu,
    mtype: MenuItemShow,
    break_type: MenuItemBreakType,
    righ_to_rder: bool,                            //MFT_RIGHTORDER
    right_justify_when_in_menu_bar: bool,          //MFT_RIGHTJUSTIFY
    state: MenuItemState,                          //MIIM_STATE
    checks: Option<MenuItemCheckStyle>,            //MIIM_CHECKMARKS
    icon: Option<MenuItemBitmapIcon /*hbmpItem*/>, //MIIM_BITMAP
    data: Option<u32 /*dwItemData*/>,              //MIIM_DATA
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
    pub fn create() -> Result<Self> {
        todo!() //CreateMenu
    }
    pub fn create_popup() -> Result<Self> {
        todo!() //CreatePopupMenu
    }
    pub fn from_list(itype: MenuType, items: Vec<MenuItem>) -> Result<Self> {
        let mut result_menu = match itype {
            MenuType::Menu => Self::create()?,
            MenuType::PopupMenu => Self::create_popup()?,
        };
        if items.len() > 4_294_967_295usize {
            return Err(Error::new(ERROR_SECRET_TOO_LONG.into(), ""));
        }
        for (index, item) in items.into_iter().enumerate() {
            result_menu.insert(Some(PosOrID::Position(index.try_into()?)), item)?;
        }
        Ok(result_menu)
    }
    pub fn load_from(ef: ExecutableFile, menu: Either<&str, usize>) -> Result<Self> {
        todo!() //LoadMenuW
    }
    pub fn append(&mut self, new_item: MenuItem) -> Result<()> {
        todo!() //AppendMenu
    }
    pub fn insert(
        &mut self,
        next_item: Option<PosOrID>, /*None: 给api提供0、true，表示在第一个位置插入*/
        new_item: MenuItem,
    ) -> Result<()> {
        todo!() //InsertMenuItemW
    }
    pub fn set_radio(&mut self, items: PosesOrIDs<(u32, u32, u32)>) -> Result<()> {
        todo!() //CheckMenuRadioItem
    }
    pub fn delete_item(&mut self, item_pos: PosOrID) -> Result<()> {
        todo!() //DeleteMenu
    }
    pub fn enable_item(&mut self, item_pos: PosOrID, state: MenuItemDisabledState) -> Result<()> {
        todo!() //EnableMenuItem
    }
    pub fn end_menu(/*needn't self*/) -> Result<()> {
        todo!() //EndMenu
    }
    pub fn get_default_item(
        &self,
        by_pos: bool,
        go_into_popups: bool, /*GMDI_GOINTOPOPUPS*/
        use_disabled: bool,   /*GMDI_USEDISABLED*/
    ) -> MenuItem {
        todo!() //GetMenuDefaultItem
    }
    //--------------------------------
    pub fn get_style(&self) -> Result<MenuStyle> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_STYLE/*}里*/,
            ..MENUINFO::default()
        };
        unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
        Ok(MenuStyle::from(menu_info./*这{*/dwStyle /*}里*/))
    }
    pub fn get_max_high(&self) -> Result<u32> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_MAXHEIGHT/*}里*/,
            ..MENUINFO::default()
        };
        unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
        Ok(menu_info./*这{*/cyMax /*}里*/)
    }
    pub fn get_backround_brush(&self) -> Result<Brush> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_BACKGROUND/*}里*/,
            ..MENUINFO::default()
        };
        unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
        Ok(Brush::from(menu_info./*这{*/hbrBack /*}里*/))
    }
    pub fn get_data(&self) -> Result<usize> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_MENUDATA/*}里*/,
            ..MENUINFO::default()
        };
        unsafe { GetMenuInfo(self.handle, &mut menu_info)? };
        Ok(menu_info./*这{*/dwMenuData /*}里*/)
    }
    pub fn set_style(&mut self, style: MenuStyle) -> Result<()> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_STYLE/*}里*/,
            dwStyle: style.into(),
            ..MENUINFO::default()
        };
        unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
        Ok(())
    }
    pub fn set_max_high(&mut self, max_high: u32) -> Result<()> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_MAXHEIGHT/*}里*/,
            cyMax: max_high,
            ..MENUINFO::default()
        };
        unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
        Ok(())
    }
    pub fn set_backround_brush(&mut self, cbrush: Brush) -> Result<()> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_BACKGROUND/*}里*/,
            hbrBack: cbrush.into(),
            ..MENUINFO::default()
        };
        unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
        Ok(())
    }
    pub fn set_data(&mut self, data: usize) -> Result<()> {
        let mut menu_info = MENUINFO {
            cbSize: std::mem::size_of::<MENUINFO>() as u32,
            fMask: /*这{*/MIM_MENUDATA/*}里*/,
            dwMenuData: data,
            ..MENUINFO::default()
        };
        unsafe { SetMenuInfo(self.handle, &mut menu_info)? };
        Ok(())
    }
    pub fn get_context_help(&self) -> Result<Option<Help>> {
        Ok(match unsafe { GetMenuContextHelpId(self.handle) } {
            0 => None,
            x => Some(Help { handle: x }),
        })
    }
    pub fn set_context_help(&mut self, help_id: Option<u32>) -> Result<()> {
        let help = match help_id {
            None => 0,
            Some(x) => x,
        };
        unsafe { SetMenuContextHelpId(self.handle, help) }
    }
    pub fn get_sub_menu(&self, position: u32) -> Option<Self> {
        todo!() //GetSubMenu
    }
    pub fn count(&self) -> Result<u16> {
        match { unsafe { GetMenuItemCount(Some(self.handle)) } } {
            -1 => Err(Error::from_win32()),
            x => Ok(x.try_into()?),
        }
    }
    pub fn get_items_clone(&self) -> Result<Vec<MenuItem>> {
        let mut result: Vec<MenuItem> = Vec::new();
        for i in 0..self.count()? {
            result.push(self.get_item_clone(PosOrID::Position(i.into()))?)
        }
        Ok(result)
    }
    pub fn get_item_id(&self, Pos: u32) -> Option<u32> {
        todo!() //GetMenuItemID
    }
    pub fn get_item_clone(&self, Pos: PosOrID) -> Result<MenuItem> {
        todo!() //GetMenuItemInfo
    }
    pub fn get_menu_item_coordinates(&self, Pos: PosOrID) -> Result<Rectangle> {
        todo!() //GetMenuItemRect
    }
    pub fn is_invalid(&self) -> bool {
        self.handle.0 == NULL_PTR()
    }
    pub fn sure_invalid(&self) -> bool {
        unsafe { (!IsMenu(self.handle)).into() }
    }
    pub fn get_menu_item_pos_from_point(&self, window: Option<Window>, x: i32, y: i32) -> u32 {
        //固定返回Pos
        todo!() //MenuItemFromPoint
    }
    pub fn set_defult_item(&mut self, item: Option<PosOrID>) -> Result<()> {
        todo!() //SetMenuDefaultItem
    }
    pub fn remove_item(&mut self, item: PosOrID) -> Result<Option<Self>> {
        let result = match item {
            PosOrID::Position(y) => Ok(self.get_sub_menu(y)),
            _ => Ok(None),
        };
        let (flag, ddata) = item.to_u32F();
        unsafe {
            RemoveMenu(self.handle, ddata, flag)?;
        };
        result
    }
    pub fn updeate_item(&mut self, item: PosOrID, new_item: MenuItem) -> Result<()> {
        todo!() //SetMenuItemInfoW
    }
    pub fn show(
        &self,
        style: MenuShowStyle,
        owner: Window,
        x: i32,
        y: i32,
        tpmp: Option<Rectangle>,
        notify /*记得取反*/: bool,
    ) -> Result<()> {
        todo!() //TrackPopupMenuEx,不指定TPM_RETURNCMD
    }
    pub fn show_retcmd(
        &self,
        style: MenuShowStyle,
        owner: Window,
        x: i32,
        y: i32,
        tpmp: Option<Rectangle>,
        notify /*记得取反*/: bool,
    ) -> Result<i32> {
        todo!() //TrackPopupMenuEx,指定TPM_RETURNCMD
    }
}
impl Drop for Menu {
    fn drop(&mut self) {
        //DestroyMenu
        if !self.is_invalid() {
            unsafe {
                DestroyMenu(self.handle);
            }
        }
    }
}

use windows_sys::Win32::{
    Foundation::{POINT, RECT, SIZE},
    Graphics::Gdi::ClientToScreen,
};
pub trait Win32Unit: Copy {}
pub trait Win32Point: Copy {
    type Unit: Win32Unit;
    fn new(x: i32, y: i32) -> Self;
    fn to_tuple(self) -> (i32, i32);
    ///以窗口左上角为原点  
    ///以屏幕右、上为正方向，如果创建窗口时指定[`crate::ui::style::NormalWindowStyles::right_layout`]为false，则与系统语言方向***无关***
    fn window_to_screen(&mut self, wnd: &crate::ui::window::Window) -> crate::error::Result<()> {
        let (x, y) = self.to_tuple();
        let mut point = POINT { x, y };
        crate::error::WinError::from_win32api_result(unsafe {
            ClientToScreen(wnd.handle(), &mut point)
        })?;
        *self = Self::new(point.x, point.y);
        Ok(())
    }
}
pub trait Win32Size: Copy {
    type Unit: Win32Unit;
    fn new(width: i32, height: i32) -> Self;
    fn to_tuple(self) -> (i32, i32);
}

pub trait Win32Rect: Copy {
    type Point: Win32Point;
    type Size: Win32Size<Unit = <<Self as Win32Rect>::Point as Win32Point>::Unit>;
    fn new(origin: Self::Point, size: Self::Size) -> Self;
    fn to_tuple(self) -> (Self::Point, Self::Size);
}
/// 没有经过任何dpi、逻辑变换的单位，原点为屏幕左上角(0, 0)，x正方向向右，y正方向向下，
/// 表示像素（屏幕）或点（打印机）等
/// 通常对应一个屏幕上的物理像素。所有实际的绘图操作最终都使用设备单位进行。
/// 此单位是静态单位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceUnit;
impl Win32Unit for DeviceUnit {}
///逻辑单位, 静态单位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalUnit<
    const Y_SCALING: i32,
    const X_SCALING: i32,
    const X_ORG: i32,
    const Y_ORG: i32,
>;
impl<const Y_SCALING: i32, const X_SCALING: i32, const X_ORG: i32, const Y_ORG: i32> Win32Unit
    for LogicalUnit<Y_SCALING, X_SCALING, X_ORG, Y_ORG>
{
}
///对话框单位，已考虑DPI, 分别是字符宽度和高度, 动态单位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DialogTemplateUnit(i32, i32);
impl Win32Unit for DialogTemplateUnit {}
///经DPI缩放的设备单位, 动态单位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DPIAwareDeviceUnit;
impl Win32Unit for DPIAwareDeviceUnit {}
///经DPI缩放的逻辑单位, 动态单位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DPIAwareLogicalUnit<
    const Y_SCALING: i32,
    const X_SCALING: i32,
    const X_ORG: i32,
    const Y_ORG: i32,
>;
impl<const Y_SCALING: i32, const X_SCALING: i32, const X_ORG: i32, const Y_ORG: i32> Win32Unit
    for DPIAwareLogicalUnit<Y_SCALING, X_SCALING, X_ORG, Y_ORG>
{
}
pub type Point = euclid::Point2D<i32, DPIAwareDeviceUnit>;
pub type Size = euclid::Size2D<i32, DPIAwareDeviceUnit>;
pub type Rect = euclid::Rect<i32, DPIAwareDeviceUnit>;
pub type DialogPonit = euclid::Point2D<i32, DialogTemplateUnit>;
pub type DialogSize = euclid::Size2D<i32, DialogTemplateUnit>;
pub type DialogRect = euclid::Rect<i32, DialogTemplateUnit>;
impl<U> Win32Point for euclid::Point2D<i32, U>
where
    U: Win32Unit,
{
    type Unit = U;
    #[inline]
    fn new(x: i32, y: i32) -> Self {
        euclid::Point2D::new(x, y)
    }
    #[inline]
    fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}
impl<U> Win32Size for euclid::Size2D<i32, U>
where
    U: Win32Unit,
{
    type Unit = U;
    #[inline]
    fn new(width: i32, height: i32) -> Self {
        euclid::Size2D::new(width, height)
    }
    #[inline]
    fn to_tuple(self) -> (i32, i32) {
        (self.width, self.height)
    }
}
impl<U> Win32Rect for euclid::Rect<i32, U>
where
    U: Win32Unit,
{
    type Point = euclid::Point2D<i32, U>;
    type Size = euclid::Size2D<i32, U>;
    #[inline]
    fn new(origin: Self::Point, size: Self::Size) -> Self {
        euclid::Rect::new(origin, size)
    }
    #[inline]
    fn to_tuple(self) -> (Self::Point, Self::Size) {
        (self.origin, self.size)
    }
}
impl<U> Win32Rect for euclid::Box2D<i32, U>
where
    U: Win32Unit,
{
    type Point = euclid::Point2D<i32, U>;
    type Size = euclid::Size2D<i32, U>;
    #[inline]
    fn new(origin: Self::Point, size: Self::Size) -> Self {
        euclid::Box2D::from_origin_and_size(origin, size)
    }
    #[inline]
    fn to_tuple(self) -> (Self::Point, Self::Size) {
        (self.min, (self.max - self.min).to_size())
    }
}
pub mod ext_methods {
    use super::*;
    mod sealed {
        use super::*;
        pub trait SealedPointInto {}
        impl SealedPointInto for (i32, i32) {}
        impl SealedPointInto for POINT {}
        pub trait SealedSizeInto {}
        impl SealedSizeInto for (i32, i32) {}
        impl SealedSizeInto for SIZE {}
        pub trait SealedRectInto {}
        impl SealedRectInto for RECT {}
        pub trait SealedPointFrom {}
        impl<U: Win32Unit> SealedPointFrom for euclid::Point2D<i32, U> {}
        pub trait SealedSizeFrom {}
        impl<U: Win32Unit> SealedSizeFrom for euclid::Size2D<i32, U> {}
        pub trait SealedRectFrom {}
        impl<U: Win32Unit> SealedRectFrom for euclid::Rect<i32, U> {}
    }
    pub trait PointExtInto: sealed::SealedPointInto {
        fn to_point_with_unit<U: Win32Unit>(self) -> euclid::Point2D<i32, U>;
    }
    pub trait SizeExtInto: sealed::SealedSizeInto {
        fn to_size_with_unit<U: Win32Unit>(self) -> euclid::Size2D<i32, U>;
    }
    pub trait RectExtInto: sealed::SealedRectInto {
        fn to_rect_with_unit<U: Win32Unit>(self) -> euclid::Rect<i32, U>;
    }
    pub trait PointExtFrom: sealed::SealedPointFrom {
        fn to_win32_point(self) -> POINT;
    }
    pub trait SizeExtFrom: sealed::SealedSizeFrom {
        fn to_win32_size(self) -> SIZE;
    }
    pub trait RectExtFrom: sealed::SealedRectFrom {
        fn to_win32_rect(self) -> RECT;
    }

    impl PointExtInto for (i32, i32) {
        #[inline]
        fn to_point_with_unit<U: Win32Unit>(self) -> euclid::Point2D<i32, U> {
            let (x, y) = self;
            euclid::Point2D::new(x, y)
        }
    }

    impl PointExtInto for POINT {
        #[inline]
        fn to_point_with_unit<U: Win32Unit>(self) -> euclid::Point2D<i32, U> {
            euclid::Point2D::new(self.x, self.y)
        }
    }

    impl SizeExtInto for (i32, i32) {
        #[inline]
        fn to_size_with_unit<U: Win32Unit>(self) -> euclid::Size2D<i32, U> {
            let (width, height) = self;
            euclid::Size2D::new(width, height)
        }
    }

    impl SizeExtInto for SIZE {
        #[inline]
        fn to_size_with_unit<U: Win32Unit>(self) -> euclid::Size2D<i32, U> {
            euclid::Size2D::new(self.cx, self.cy)
        }
    }

    impl RectExtInto for RECT {
        #[inline]
        fn to_rect_with_unit<U: Win32Unit>(self) -> euclid::Rect<i32, U> {
            euclid::Rect::new(
                euclid::Point2D::new(self.left, self.top),
                euclid::Size2D::new(self.right - self.left, self.bottom - self.top),
            )
        }
    }
    impl<U: Win32Unit> PointExtFrom for euclid::Point2D<i32, U> {
        #[inline]
        fn to_win32_point(self) -> POINT {
            POINT {
                x: self.x,
                y: self.y,
            }
        }
    }
    impl<U: Win32Unit> SizeExtFrom for euclid::Size2D<i32, U> {
        #[inline]
        fn to_win32_size(self) -> SIZE {
            SIZE {
                cx: self.width,
                cy: self.height,
            }
        }
    }
    impl<U: Win32Unit> RectExtFrom for euclid::Rect<i32, U> {
        #[inline]
        fn to_win32_rect(self) -> RECT {
            RECT {
                left: self.min_x(),
                top: self.min_y(),
                right: self.max_x(),
                bottom: self.max_y(),
            }
        }
    }
}

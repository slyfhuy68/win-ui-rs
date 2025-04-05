static EPT_ICON: LazyLock<Icon> = LazyLock::new(|| {
	Icon::load_from_module(
		ExecutableFile::from_current_file().unwrap(),
		Right(2),
		None,
		true,
	)
	.unwrap()
});
static FULL_ICON: LazyLock<Icon> = LazyLock::new(|| {
		Icon::load_from_module(
		ExecutableFile::from_current_file().unwrap(),
		Right(3),
		None,
		true,
	)
	.unwrap()

});
static EPT_CURSOR: LazyLock<Cursor> = LazyLock::new(|| {
	Cursor::load_from_module(
		ExecutableFile::from_current_file().unwrap(),
		Right(4),
		None,
		true,
	)
	.unwrap()
});
pub fn new(wnd: &mut Window,pos: Option<Rectangle>,identifier: WindowID,) -> Windows {
	let view = ImageTextView::new(
		&mut Window,
		pos: Option<Rectangle>,
		identifier: WindowID,
		control_style: ImageTextViewStyle::,
		style: ChildWindowStyles::default(),
		style_ex: NormalWindowExStyles::default(),
		font: false,
		no_notify: false,
	) -> Result<Self>
}
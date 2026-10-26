#[macro_export]
macro_rules! phosphor {
    ($icon:ident) => {
        phosphor!($icon, REGULAR)
    };
    ($icon:ident, $style:ident) => {
        Icon::default().data(phosphor_svgs::icon::$icon::$style.as_bytes())
    };
}

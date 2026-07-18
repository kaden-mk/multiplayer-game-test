#[macro_export]
macro_rules! bind_func {
    ($lua:expr, $table:expr, $luau_name:expr, $instance:expr, $method:ident, ($($arg:ident : $ty:ty),*)) => {{
        let ctx = $instance.clone();
        let func = $lua.create_function(move |_, ($($arg,)*): ($($ty,)*)| {
            ctx.$method($($arg),*)
        })?;
        $table.set($luau_name, func)?;
    }};
}

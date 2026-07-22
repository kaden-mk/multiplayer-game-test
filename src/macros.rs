#[macro_export]
macro_rules! bind_func {
    ($lua:expr, $table:expr, $luau_name:expr, $instance:expr, $method:ident, ($($arg:ident : $ty:ty),*) -> $ret:ty) => {{
        inventory::submit! {
            $crate::core::typegen::FnSig {
                module: module_path!(),
                name: $luau_name,
                args: &[ $( (stringify!($arg), stringify!($ty)) ),* ],
                ret: stringify!($ret),
            }
        };

        let ctx = $instance.clone();
        let func = $lua.create_function(move |_, ($($arg,)*): ($($ty,)*)| {
            ctx.$method($($arg),*)
        })?;
        $table.set($luau_name, func)?;
    }};
}

#[macro_export]
macro_rules! colors {
    ($($name:ident),* $(,)?) => {
        pub const COLORS: &[(&str, Color)] = &[ $( (stringify!($name), Color::$name) ),* ];
    };
}

#[macro_export]
macro_rules! mouse_buttons {
    ($($name:ident),* $(,)?) => {
        paste! {
            pub const MOUSE_BUTTONS: &[(&str, MouseButton)] = &[
                $( (stringify!($name), MouseButton::[<MOUSE_ $name>]) ),*
            ];
        }
    };
}
#[macro_export]
macro_rules! keys {
    ($($name:ident),* $(,)?) => {
        paste! {
            pub const KEYS: &[(&str, KeyboardKey)] = &[
                $( (stringify!($name), KeyboardKey::[<KEY_ $name>]) ),*
            ];
        }
    };
}

#[macro_export]
macro_rules! npatch_layouts {
    ($($name:ident),* $(,)?) => {
        paste! {
            pub const NPATCH_LAYOUTS: &[(&str, NPatchLayout)] = &[
                $( (stringify!($name), NPatchLayout::[<NPATCH_ $name>]) ),*
            ];
        }
    };
}

#[macro_use]
#[macro_export]
macro_rules! register_modules {
    ($app:expr,[$($m:ident),*]) => {{
        let mut app = $app;
        $(
        app = app.configure($m::config);
        )*
        app
    }};
}


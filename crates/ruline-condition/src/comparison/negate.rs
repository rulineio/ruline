#[macro_export(local_inner_macros)]
macro_rules! negate {
    ($fn_name:path, $arg:expr) => {{
        $fn_name($arg).map(|result| !result)
    }};
}

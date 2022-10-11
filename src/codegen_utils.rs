/// Conditional compilation. If the variable's value is equal to the expected value then it calls the functon, otherwise it does nothing.
#[macro_export]
macro_rules! compile_if_eq {
    ($var:expr,$expected:expr,$exec:expr) => {
        match $var {
            $expected => $exec(),
            _ => (),
        }
    };
}

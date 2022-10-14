/// Conditional compilation. If the variable's value is equal to the expected value then it calls the functon, otherwise it does nothing.
#[macro_export]
macro_rules! compile_if_eq {
    ($var:expr,$expected:expr,$code:block) => {
        match $var {
            $expected => $code,
            _ => (),
        }
    };
}

#[macro_export]
macro_rules! license_info {
    () => {
        println!(
            " 
Copyright (c) 2022 PeterPierinakos & the Contributors

This is free software. You may find details for the license of the project in any file that starts with \"LICENSE\" in the root directory of the project.
            "
        );
    }
}

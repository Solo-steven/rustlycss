#[macro_export]
macro_rules! syntax_error {
    ($message: expr) => {
        panic!("[Syntax Error]: {:?}", $message);
    };
}
#![allow(unused)]
#![warn(clippy::useless_default_generic_parameters)]

macro_rules! option_default {
    ($default:ty) => {
        type Option<T = $default> = core::option::Option<T>;
    }
}

struct MyError;
type Result<T = ()> = core::result::Result<T, MyError>;

// Should lint
fn foo() -> Result<()> {
    Ok(())
}

// Should not lint
option_default!(i32);
fn bar() -> Result {
    Ok(())
}


#[macro_use]
extern crate epitaph;

use std::error::Error as StdError;

use epitaph::DetailedError;


fn success_func () -> Result<i32, Error>
{
    Ok(5)
}
fn fail_func () -> Result<i32, Error>
{
    err!(ErrorKind::ErrorOne; "Error One");
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ErrorKind
{
    ErrorOne,
    ErrorTwo,
    ErrorThree,
}

error_items!{Error<Kind = ErrorKind> "Test Error"}

#[test]
fn funcs ()
{
    fn funcs_impl <F> (f : F)
        where F : FnOnce(Result<i32, Error>)
    {
        f_attempt!((f) success_func());
        f_attempt!((f) success_func() => "Description");
        f_attempt!((f) success_func() => "Description with arg {}", 5);
        f_attempt!((f; ()) success_func());
        f_attempt!((f; ()) success_func() => "Description");
        f_attempt!((f; ()) success_func() => "Description with arg {}", 5);

        f_attempt_err!((f) success_func() => |_| ErrorKind::ErrorOne; "Description");
        f_attempt_err!((f) success_func() => |_| ErrorKind::ErrorOne; "Description with arg {}", 5);
        f_attempt_err!((f) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description");
        f_attempt_err!((f) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description with arg {}", 5);
        f_attempt_err!((f; ()) success_func() => |_| ErrorKind::ErrorOne; "Description");
        f_attempt_err!((f; ()) success_func() => |_| ErrorKind::ErrorOne; "Description with arg {}", 5);
        f_attempt_err!((f; ()) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description");
        f_attempt_err!((f; ()) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description with arg {}", 5);
    }

    funcs_impl(|_| ());
}





#![cfg(feature = "alternate-future")]

#[macro_use]
extern crate epitaph;
extern crate alternate_future;

use std::error::Error as StdError;
use alternate_future::{promise_future, Promise, Future};

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
fn promises ()
{
    fn promises_impl (p : Promise<Result<i32, Error>>)
    {
        p_attempt!((p) success_func());
        p_attempt!((p) success_func() => "Description");
        p_attempt!((p) success_func() => "Description with arg {}", 5);
        p_attempt!((p; ()) success_func());
        p_attempt!((p; ()) success_func() => "Description");
        p_attempt!((p; ()) success_func() => "Description with arg {}", 5);

        p_attempt_err!((p) success_func() => |_| ErrorKind::ErrorOne; "Description");
        p_attempt_err!((p) success_func() => |_| ErrorKind::ErrorOne; "Description with arg {}", 5);
        p_attempt_err!((p) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description");
        p_attempt_err!((p) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description with arg {}", 5);
        p_attempt_err!((p; ()) success_func() => |_| ErrorKind::ErrorOne; "Description");
        p_attempt_err!((p; ()) success_func() => |_| ErrorKind::ErrorOne; "Description with arg {}", 5);
        p_attempt_err!((p; ()) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description");
        p_attempt_err!((p; ()) success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description with arg {}", 5);
    }

    let (p, f) = promise_future();
    promises_impl(p);
}





#[macro_use]
extern crate epitaph;

use std::error::Error as StdError;

use epitaph::DetailedError;


fn success_func () -> Result<(), Error>
{
    Ok(())
}
fn fail_func () -> Result<(), Error>
{
    err!(ErrorKind::ErrorOne; "Error One");
}
fn proxy_fail () -> Result<(), Error>
{
    attempt!(fail_func());

    Ok(())
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
fn trace ()
{
    let err = proxy_fail().err().unwrap();

    assert!(err.trace()[0].0.ends_with("tests.rs"));
    assert_eq!(err.trace()[0].1, 17);
    assert!(err.trace()[0].0.ends_with("tests.rs"));
    assert_eq!(err.trace()[1].1, 21);
}


#[test]
fn attempts ()
{
    fn attempts_impl () -> Result<(), Error>
    {
        attempt!(success_func());
        attempt!(success_func() => "Description");
        attempt!(success_func() => "Description with arg {}", 5);

        attempt_err!(success_func() => |_| ErrorKind::ErrorOne; "Description");
        attempt_err!(success_func() => |_| ErrorKind::ErrorOne; "Description with arg {}", 5);
        attempt_err!(success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description");
        attempt_err!(success_func() => |_| ErrorKind::ErrorOne, |_| epitaph::StringErr::from_str("inner error"); "Description with arg {}", 5);

        Ok(())
    }

    assert!(attempts_impl().is_ok());
}



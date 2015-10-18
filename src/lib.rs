

use std::any::Any;
use std::borrow::ToOwned;
use std::fmt::{self, Debug, Display, Formatter};


pub trait DetailedError : std::error::Error
{
    type Kind;
    
    fn new (kind : <Self as DetailedError>::Kind, cause : Option<Box<std::error::Error + Send + Sync>>, desc : String, file : &'static str, line : u32) -> Self;
    fn kind (&self) -> &<Self as DetailedError>::Kind;
    fn trace (&self) -> &[(&'static str, u32)];
}


pub trait DetailedFromError <E>
{
    fn from_error (cause : E, msg : Option<String>, file : &'static str, line : u32) -> Self;
}


#[macro_export]
macro_rules! error_items
{
    (
        $err_name:ident <Kind = $kind_name:ty> $desc_str:expr
    ) => (
        pub struct $err_name
        {
            kind   : $kind_name,
            cause  : Option<Box<::std::error::Error + Send + Sync>>,
            msg    : String,
            trace  : Vec<(&'static str, u32)>,
        }
        impl $crate::DetailedError for $err_name
        {
            type Kind = $kind_name;
            
            
            #[inline]
            fn new (
                kind  : $kind_name,
                cause : Option<Box<::std::error::Error + Send + Sync>>,
                msg   : String,
                file  : &'static str,
                line  : u32
            ) -> $err_name
            {
                $err_name{kind: kind, cause: cause, msg: msg, trace: vec!((file, line))}
            }
            
            
            #[inline]
            fn kind (&self) -> &$kind_name { &self.kind }
            #[inline]
            fn trace (&self) -> &[(&'static str, u32)] { &self.trace }
        }
        impl ::std::error::Error for $err_name
        {
            #[inline]
            fn description (&self) -> &str { &$desc_str }
            
            
            #[inline]
            fn cause (&self) -> Option<&::std::error::Error> {
                match self.cause {
                    Some(ref b) => Some(&**b),
                    None        => None,
                }
            }
        }
        impl $crate::DetailedFromError<$err_name> for $err_name
        {
            fn from_error (mut e : $err_name, _ : Option<String>, file : &'static str, line : u32) -> $err_name
            {
                e.trace.push((file, line));
                e
            }
        }
        //~ impl $crate::DetailedFromError<$err_name> for Box<::std::error::Error>
        //~ {
            //~ fn from_error (mut e : $err_name, _ : Option<String>, file : &'static str, line : u32) -> Box<::std::error::Error>
            //~ {
                //~ e.trace.push((file, line));
                //~ Box::new(e)
            //~ }
        //~ }
        //~ impl ::std::error::FromError<$err_name> for Box<::std::error::Error>
        //~ {
            //~ fn from_error (e : $err_name) -> Box<::std::error::Error> { Box::new(e) }
        //~ }
        impl ::std::fmt::Debug for $err_name
        {
            fn fmt (&self, fmt : &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>
            {
                use ::std::fmt::Display;
                
                Display::fmt(self, fmt)
            }
        }
        impl ::std::fmt::Display for $err_name
        {
            fn fmt (&self, fmt : &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>
            {
                use ::std::fmt::Display;
                
                try!(writeln!(fmt, "{}: {}", $desc_str, self.msg));
                for &(f, l) in self.trace.iter().rev() {
                    try!(writeln!(fmt, "    @ {}:{}", f, l));
                }
                if let Some(ref cause) = self.cause {
                    try!(Display::fmt(cause, fmt));
                }
                
                Ok(())
            }
        }
    )
}


#[macro_export]
macro_rules! new_err
{
    (
        $kind:expr; $desc:expr
    ) => (
        $crate::DetailedError::new($kind, None, ::std::borrow::ToOwned::to_owned($desc), file!(), line!())
    );
    (
        $kind:expr; $desc:expr, $($arg:tt)*
    ) => (
        $crate::DetailedError::new($kind, None, format!($desc, $($arg)*), file!(), line!())
    );
    (
        $kind:expr, $cause:expr; $desc:expr
    ) => (
        $crate::DetailedError::new($kind, Some($cause.into()), ::std::borrow::ToOwned::to_owned($desc), file!(), line!())
    );
    (
        $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => (
        $crate::DetailedError::new($kind, Some($cause.into()), format!($desc, $($arg)*), file!(), line!())
    );
}


#[macro_export]
macro_rules! from_err
{
    (
        $err:expr
    ) => (
        $crate::DetailedFromError::from_error($err, None, file!(), line!())
    );
    (
        $err:expr; $detail:expr
    ) => (
        $crate::DetailedFromError::from_error($err, Some(format!("{}", $detail)), file!(), line!())
    );
    (
        $err:expr; $detail:expr, $($arg:tt)*
    ) => (
        $crate::DetailedFromError::from_error($err, Some(format!($detail, $($arg)*)), file!(), line!())
    )
}


#[macro_export]
macro_rules! err
{
    (
        $kind:expr; $desc:expr
    ) => (
        return Err(new_err!($kind; $desc));
    );
    (
        $kind:expr; $desc:expr, $($arg:tt)*
    ) => (
        return Err(new_err!($kind; $desc, $($arg)*));
    );
    (
        $kind:expr, $cause:expr; $desc:expr
    ) => (
        return Err(new_err!($kind, $cause; $desc));
    );
    (
        $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => (
        return Err(new_err!($kind, $cause; $desc, $($arg)*));
    );
}


#[macro_export]
macro_rules! attempt
{
    (
        $expr:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => return Err(from_err!(e)),
        }
    );
    (
        $expr:expr => $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => return Err(from_err!(e; $desc)),
        }
    );
    (
        $expr:expr => $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => return Err(from_err!(e; $desc, $($arg)*)),
        }
    );
}


#[macro_export]
macro_rules! attempt_err
{
    (
        $expr:expr => $kind_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => err!($kind_fn(&e), e; $desc),
        }
    );
    (
        $expr:expr => $kind_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => err!($kind_fn(&e), e; $desc, $($arg)*),
        }
    );
    (
        $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                err!(kind, cause; $desc)
            },
        }
    );
    (
        $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                err!(kind, cause; $desc, $($arg)*)
            },
        }
    );
}




#[macro_export]
macro_rules! f_ok
{
    (
        ($func:expr) $expr:expr
    ) => ({
        $func(Ok($expr));
        return;
    });
    (
        ($func:expr; $ret:expr) $expr:expr
    ) => ({
        $func(Ok($expr));
        return $ret;
    });
}
#[macro_export]
macro_rules! f_err
{
    (
        ($func:expr) $kind:expr; $desc:expr
    ) => ({
        $func(Err(new_err!($kind; $desc)));
        return;
    });
    (
        ($func:expr) $kind:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $func(Err(new_err!($kind; $desc, $($arg)*)));
        return;
    });
    (
        ($func:expr) $kind:expr, $cause:expr; $desc:expr
    ) => ({
        $func(Err(new_err!($kind, $cause; $desc)));
        return;
    });
    (
        ($func:expr) $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $func(Err(new_err!($kind, $cause; $desc, $($arg)*)));
        return;
    });

    (
        ($func:expr; $ret:expr) $kind:expr; $desc:expr
    ) => ({
        $func(Err(new_err!($kind; $desc)));
        return $ret;
    });
    (
        ($func:expr; $ret:expr) $kind:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $func(Err(new_err!($kind; $desc, $($arg)*)));
        return $ret;
    });
    (
        ($func:expr; $ret:expr) $kind:expr, $cause:expr; $desc:expr
    ) => ({
        $func(Err(new_err!($kind, $cause; $desc)));
        return $ret;
    });
    (
        ($func:expr; $ret:expr) $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $func(Err(new_err!($kind, $cause; $desc, $($arg)*)));
        return $ret;
    });
}


#[macro_export]
macro_rules! f_attempt
{
    (
        ($func:expr) $expr:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e)));
                return;
            },
        }
    );
    (
        ($func:expr) $expr:expr => $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e; $desc)));
                return;
            },
        }
    );
    (
        ($func:expr) $expr:expr => $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e; $desc, $($arg)*)));
                return;
            },
        }
    );

    (
        ($func:expr; $ret:expr) $expr:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e)));
                return $ret;
            },
        }
    );
    (
        ($func:expr; $ret:expr) $expr:expr => $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e; $desc)));
                return $ret;
            },
        }
    );
    (
        ($func:expr; $ret:expr) $expr:expr => $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $func(Err(from_err!(e; $desc, $($arg)*)));
                return $ret;
            },
        }
    );
}


#[macro_export]
macro_rules! f_attempt_err
{
    (
        ($func:ident) $expr:expr => $kind_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => f_err!(($func) $kind_fn(&e), e; $desc),
        }
    );
    (
        ($func:ident) $expr:expr => $kind_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => f_err!(($func) $kind_fn(&e), e; $desc, $($arg)*),
        }
    );
    (
        ($func:ident) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                f_err!(($func) kind, cause; $desc)
            },
        }
    );
    (
        ($func:ident) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                f_err!(($func) kind, cause; $desc, $($arg)*)
            },
        }
    );

    (
        ($func:ident; $ret:expr) $expr:expr => $kind_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => f_err!(($func; $ret) $kind_fn(&e), e; $desc),
        }
    );
    (
        ($func:ident; $ret:expr) $expr:expr => $kind_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => f_err!(($func; $ret) $kind_fn(&e), e; $desc, $($arg)*),
        }
    );
    (
        ($func:ident; $ret:expr) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                f_err!(($func; $ret) kind, cause; $desc)
            },
        }
    );
    (
        ($func:ident; $ret:expr) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                f_err!(($func; $ret) kind, cause; $desc, $($arg)*)
            },
        }
    );
}






#[cfg(feature = "alternate-future")]
#[macro_export]
macro_rules! p_ok
{
    (
        ($prom:expr) $expr:expr
    ) => ({
        $prom.ok($expr);
        return;
    });
    (
        ($prom:expr; $ret:expr) $expr:expr
    ) => ({
        $prom.ok($expr);
        return $ret;
    });
}
#[cfg(feature = "alternate-future")]
#[macro_export]
macro_rules! p_err
{
    (
        ($prom:expr) $kind:expr; $desc:expr
    ) => ({
        $prom.err(new_err!($kind; $desc));
        return;
    });
    (
        ($prom:expr) $kind:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $prom.err(new_err!($kind; $desc, $($arg)*));
        return;
    });
    (
        ($prom:expr) $kind:expr, $cause:expr; $desc:expr
    ) => ({
        $prom.err(new_err!($kind, $cause; $desc));
        return;
    });
    (
        ($prom:expr) $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $prom.err(new_err!($kind, $cause; $desc, $($arg)*));
        return;
    });

    (
        ($prom:expr; $ret:expr) $kind:expr; $desc:expr
    ) => ({
        $prom.err(new_err!($kind; $desc));
        return $ret;
    });
    (
        ($prom:expr; $ret:expr) $kind:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $prom.err(new_err!($kind; $desc, $($arg)*));
        return $ret;
    });
    (
        ($prom:expr; $ret:expr) $kind:expr, $cause:expr; $desc:expr
    ) => ({
        $prom.err(new_err!($kind, $cause; $desc));
        return $ret;
    });
    (
        ($prom:expr; $ret:expr) $kind:expr, $cause:expr; $desc:expr, $($arg:tt)*
    ) => ({
        $prom.err(new_err!($kind, $cause; $desc, $($arg)*));
        return $ret;
    });
}


#[cfg(feature = "alternate-future")]
#[macro_export]
macro_rules! p_attempt
{
    (
        ($prom:expr) $expr:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e));
                return;
            },
        }
    );
    (
        ($prom:expr) $expr:expr => $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e; $desc));
                return;
            },
        }
    );
    (
        ($prom:expr) $expr:expr => $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e; $desc, $($arg)*));
                return;
            },
        }
    );

    (
        ($prom:expr; $ret:expr) $expr:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e));
                return $ret;
            },
        }
    );
    (
        ($prom:expr; $ret:expr) $expr:expr => $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e; $desc));
                return $ret;
            },
        }
    );
    (
        ($prom:expr; $ret:expr) $expr:expr => $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => {
                $prom.err(from_err!(e; $desc, $($arg)*));
                return $ret;
            },
        }
    );
}


#[cfg(feature = "alternate-future")]
#[macro_export]
macro_rules! p_attempt_err
{
    (
        ($prom:ident) $expr:expr => $kind_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => p_err!(($prom) $kind_fn(&e), e; $desc),
        }
    );
    (
        ($prom:ident) $expr:expr => $kind_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => p_err!(($prom) $kind_fn(&e), e; $desc, $($arg)*),
        }
    );
    (
        ($prom:ident) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                p_err!(($prom) kind, cause; $desc)
            },
        }
    );
    (
        ($prom:ident) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                p_err!(($prom) kind, cause; $desc, $($arg)*)
            },
        }
    );

    (
        ($prom:ident; $ret:expr) $expr:expr => $kind_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => p_err!(($prom; $ret) $kind_fn(&e), e; $desc),
        }
    );
    (
        ($prom:ident; $ret:expr) $expr:expr => $kind_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e)  => p_err!(($prom; $ret) $kind_fn(&e), e; $desc, $($arg)*),
        }
    );
    (
        ($prom:ident; $ret:expr) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                p_err!(($prom; $ret) kind, cause; $desc)
            },
        }
    );
    (
        ($prom:ident; $ret:expr) $expr:expr => $kind_fn:expr, $err_fn:expr; $desc:expr, $($arg:tt)*
    ) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let kind = $kind_fn(&e);
                let cause = $err_fn(e);
                p_err!(($prom; $ret) kind, cause; $desc, $($arg)*)
            },
        }
    );
}


#[derive(Clone)]
pub struct StringErr (String);
impl StringErr
{
    #[inline]
    pub fn new (error : String) -> StringErr { StringErr(error) }
    
    #[inline]
    pub fn from_str (error : &str) -> StringErr { StringErr(error.to_owned()) }
}
impl std::error::Error for StringErr
{
    #[inline]
    fn description (&self) -> &str { "error string" }
}
impl std::convert::From<String> for StringErr
{
    #[inline]
    fn from (error : String) -> StringErr { StringErr(error) }
}
impl <'a> std::convert::From<&'a str> for StringErr
{
    #[inline]
    fn from (error : &'a str) -> StringErr { StringErr::from_str(error) }
}
//~ impl std::error::FromError<StringErr> for Box<std::error::Error>
//~ {
    //~ #[inline]
    //~ fn from_error (error : StringErr) -> Box<std::error::Error> { box error }
//~ }
impl Debug for StringErr
{
    #[inline]
    fn fmt (&self, fmt : &mut Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(&self.0, fmt)
    }
}
impl Display for StringErr
{
    #[inline]
    fn fmt (&self, fmt : &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.0, fmt)
    }
}


pub struct ValueErr <E> (E, String) where E : Any + Send + Display;
impl <E> ValueErr<E>
    where E : Any + Send + Display
{
    pub fn new (error : E) -> ValueErr<E>
    {
        let msg = format!("{}", error);
        ValueErr(error, msg)
    }
}
impl <E> std::error::Error for ValueErr<E>
    where E : Any + Send + Display
{
    #[inline]
    fn description (&self) -> &str { "error value" }
}
impl <E> std::convert::From<E> for ValueErr<E>
    where E : Any + Send + Display
{
    #[inline]
    fn from (error : E) -> ValueErr<E> { ValueErr::new(error) }
}
//~ impl <E : Send + Display> std::error::FromError<ValueErr<E>> for Box<std::error::Error>
//~ {
    //~ #[inline]
    //~ fn from_error (error : ValueErr<E>) -> Box<std::error::Error> { Box::new(error) }
//~ }
impl <E> Debug for ValueErr<E>
    where E : Any + Send + Display
{
    #[inline]
    fn fmt (&self, fmt : &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(self, fmt)
    }
}
impl <E> Display for ValueErr<E>
    where E : Any + Send + Display
{
    #[inline]
    fn fmt (&self, fmt : &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}
impl <E> Clone for ValueErr<E>
    where E : Any + Send + Clone + Display
{
    #[inline]
    fn clone (&self) -> ValueErr<E> { ValueErr(self.0.clone(), self.1.clone()) }
}


#[macro_export]
macro_rules! impl_from_error
{
    (
        <$src:ty, $dest:ty> $kind_fn:expr; $desc:expr
    ) => (
        impl ::std::convert::From<$src> for $dest
        {
            fn from (error : $src) -> $dest
            {
                let kind = $kind_fn(&error);
                $crate::DetailedError::new(kind, Some(Box::new(error) as Box<::std::error::Error + Send + Sync>), ::std::borrow::ToOwned::to_owned($desc), "<unknown>", 0)
            }
        }
        impl $crate::DetailedFromError<$src> for $dest
        {
            fn from_error (error : $src, desc : Option<String>, file : &'static str, line : u32) -> $dest
            {
                let kind = $kind_fn(&error);
                let desc = match desc {
                    Some(s) => format!("{}: {}", $desc, s),
                    None    => ::std::borrow::ToOwned::to_owned($desc),
                };
                $crate::DetailedError::new(kind, Some(Box::new(error) as Box<::std::error::Error + Send + Sync>), desc, file, line)
            }
        }
    )
}


#[macro_export]
macro_rules! impl_from_val_error
{
    (
        <$src:ty, $dest:ty> $kind_fn:expr; $desc:expr
    ) => (
        impl ::std::error::FromError<$src> for $dest
        {
            fn from_error (error : $src) -> $dest
            {
                let kind = $kind_fn(&error);
                $crate::DetailedError::new(kind, Some(Box::new($crate::api::error::ValueErr::new(error)) as Box<::std::error::Error + Send + Sync>), String::from_str($desc), "<unknown>", 0)
            }
        }
        impl $crate::DetailedFromError<$src> for $dest
        {
            fn from_error (error : $src, desc : Option<String>, file : &'static str, line : u32) -> $dest
            {
                let kind = $kind_fn(&error);
                let desc = match desc {
                    Some(s) => format!("{}: {}", $desc, s),
                    None    => String::from_str($desc),
                };
                $crate::DetailedError::new(kind, Some(Box::new($crate::api::error::ValueErr::new(error)) as Box<::std::error::Error + Send + Sync>), desc, file, line)
            }
        }
    )
}


#[cfg(feature = "alternate-future")]
#[macro_export]
macro_rules! impl_from_await_error
{
    (
        <$dest:ty> $aborted_kind:expr; $aborted_desc:expr
    ) => (
        impl ::std::convert::From<::alternate_future::AwaitError> for $dest
        {
            fn from (error : ::alternate_future::AwaitError) -> $dest
            {
                match error {
                    ::alternate_future::AwaitError::Broken => {
                        $crate::DetailedError::new($aborted_kind, None, ::std::borrow::ToOwned::to_owned($aborted_desc), "<unknown>", 0)
                    },
                }
            }
        }
        impl $crate::DetailedFromError<::alternate_future::AwaitError> for $dest
        {
            fn from_error (error : ::alternate_future::AwaitError, desc : Option<String>, file : &'static str, line : u32) -> $dest
            {
                match error {
                    ::alternate_future::AwaitError::Broken => {
                        let desc = match desc {
                            Some(s) => format!("{}: {}", $aborted_desc, s),
                            None    => ::std::borrow::ToOwned::to_owned($aborted_desc),
                        };
                        $crate::DetailedError::new($aborted_kind, Some(Box::new(error) as Box<::std::error::Error + Send + Sync>), desc, file, line)
                    },
                }
            }
        }
    )
}



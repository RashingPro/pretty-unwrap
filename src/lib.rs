//! Rust's built-in `unwrap()` and similar methods use the `Debug` trait when
//! panicking, although sometimes you might want `Display` instead. This crate
//! provides `unwrap_pretty`, `expect_pretty`, `unwrap_err_pretty` and
//! `expect_err_pretty` methods for this exact purpose.
//!
//! ## `no_std`
//! This crate is `no_std` compatible.

#![no_std]

use core::fmt::Display;

/// This trait contains methods for when `Err` implements `Display`
pub trait UnwrapPretty {
    type Success;

    fn unwrap_pretty(self) -> Self::Success;

    fn expect_pretty(self, msg: &str) -> Self::Success;
}

/// This trait contains methods for when `Ok` implements `Display`
pub trait UnwrapErrPretty {
    type Error;

    fn unwrap_err_pretty(self) -> Self::Error;

    fn expect_err_pretty(self, msg: &str) -> Self::Error;
}

impl<T, E> UnwrapPretty for Result<T, E>
where
    E: Display
{
    type Success = T;

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// use core::fmt::{Display, Formatter};
    /// use pretty_unwrap::UnwrapPretty;
    ///
    /// struct CustomError;
    ///
    /// impl Display for CustomError {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    ///         write!(f, "Oh nooo! Error just happened!")
    ///     }
    /// }
    ///
    /// let x: Result<u32, CustomError> = Err(CustomError);
    /// x.unwrap_pretty(); // panics with `Oh nooo! Error just happened!`
    /// ```
    #[inline(always)]
    #[track_caller]
    fn unwrap_pretty(self) -> Self::Success {
        self.unwrap_or_else(|err| {
            panic!("called `Result::unwrap_pretty()` on an `Err` value: {err}")
        })
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message including the
    /// passed message, and the content of the [`Err`].
    ///
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use core::fmt::{Display, Formatter};
    /// use pretty_unwrap::UnwrapPretty;
    ///
    /// struct CustomError;
    ///
    /// impl Display for CustomError {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    ///         write!(f, "Oh nooo! Error just happened!")
    ///     }
    /// }
    ///
    /// let x: Result<u32, CustomError> = Err(CustomError);
    /// x.expect_pretty("Testing expect"); // panics with `Testing expect: Oh nooo! Error just happened!`
    /// ```
    #[inline]
    #[track_caller]
    fn expect_pretty(self, msg: &str) -> Self::Success {
        self.unwrap_or_else(|err| panic!("{msg}: {err}"))
    }
}

impl<T, E> UnwrapErrPretty for Result<T, E>
where
    T: Display
{
    type Error = E;

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a custom panic message provided
    /// by the [`Ok`]'s value.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use core::fmt::{Display, Formatter};
    /// use pretty_unwrap::UnwrapErrPretty;
    ///
    /// struct CustomSuccess;
    ///
    /// impl Display for CustomSuccess {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    ///         write!(f, "Oh nooo! Error just happened!")
    ///     }
    /// }
    ///
    /// let x: Result<CustomSuccess, &str> = Ok(CustomSuccess);
    /// x.unwrap_err_pretty(); // panics with `Oh nooo! Error just happened!`
    /// ```
    #[inline]
    #[track_caller]
    fn unwrap_err_pretty(self) -> Self::Error {
        match self {
            Ok(val) => panic!("called `Result::unwrap_err_pretty()` on an `Ok` value: {val}"),
            Err(e) => e
        }
    }

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a panic message including the
    /// passed message, and the content of the [`Ok`].
    ///
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use core::fmt::{Display, Formatter};
    /// use pretty_unwrap::UnwrapErrPretty;
    ///
    /// struct CustomSuccess;
    ///
    /// impl Display for CustomSuccess {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    ///         write!(f, "Oh nooo! Error just happened!")
    ///     }
    /// }
    ///
    /// let x: Result<CustomSuccess, &str> = Ok(CustomSuccess);
    /// x.expect_err_pretty("Testing expect"); // panics with `Testing expect: Oh nooo! Error just happened!`
    /// ```
    #[inline]
    #[track_caller]
    fn expect_err_pretty(self, msg: &str) -> Self::Error {
        match self {
            Ok(val) => panic!("{msg}: {val}"),
            Err(e) => e
        }
    }
}

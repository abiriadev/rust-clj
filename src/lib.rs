//! this is clf crate!
//! feel free to look around, whenever you have questions, open a new issue at here :)
//!
//! # how to use:
//!
//! ```
//! use clj;
//!
//! let two = 2;
//! let three = match clj::add_one(two) {
//!     Ok(v) => v,
//!     Err(e) => panic!("error! :(")
//! };
//! # assert_eq!(three, 3);
//! ```

#![feature(backtrace)]
#![feature(error_type_id)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// desirable 바람직한

// fully qualified 자격을 갖춘
// abbreviated 축약된

// formal 형식적인

// Brevity 간결함
// succinct 간결한

// exhaustive 빠짐없는

// Ambiguous 모호한, 두가지 뜻이 있는, 경로가 겹치는

// encounter 만나다, 맞닥뜨리다, 무주치다

// venerable 오래된, 익숙한, 형식적인

// elaborate 애써 만들다, 장황하게 만들다
// accomplish 이루다, 완수하다, 달성하다, 어치브랑 비슷
// aspect 관점, 시각

#[cfg(feature = "webp")]
pub fn webp() -> String {
    String::from("hi lol")
}

#[cfg(feature = "time")]
pub fn time() -> String {
    String::from("Time to wake up!")
}

// #![allow(unused)]
#[no_std]
// fn main() {
// #[cfg(feature = "std")]
// extern crate std;
#[cfg(feature = "std")]
pub fn function_that_requires_std() {
    // ...
}
// }

#[cfg(all(feature = "foo", feature = "bar"))]
compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");

// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn macos_only() {
    // ...
}

#[cfg(any(foo, bar))]
fn needs_foo_or_bar() {
    // ...
}

// This function is only included when compiling for a unixish OS with a 32-bit
// architecture
#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {
    // ...
}

// This function is only included when foo is not defined
#[cfg(not(foo))]
fn needs_not_foo() {
    // ...
}

fn a() -> &'static str {
    if cfg!(windows) {
        "ss"
    } else {
        "dslksdkl"
    }
}

#[derive(Debug)]
pub struct MyErr;

impl std::fmt::Display for MyErr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", "err: aaaa")
    }
}

impl std::error::Error for MyErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// add 1 to given number and return it
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = clj::add_one(arg).unwrap();
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// it would panic when given number is lower than 1
///
/// ```should_panic
/// let negative_integer = -1;
/// clj::add_one(negative_integer); // panic!
/// ```
///
/// # Errors
///
/// it returns an error when given number is exactly 100
///
/// ```
/// let _100 = 100;
/// let result: Result<i32, clj::MyErr> = clj::add_one(_100);
/// # if let Ok(_) = result {
/// #    panic!();
/// # }
/// ```
///
/// # Safety
///
/// ...I'm weary now.
pub fn add_one(x: i32) -> Result<i32, MyErr> {
    if x < 1 {
        panic!("x is lower than 1")
    };

    if x == 100 {
        return Err(MyErr);
    };

    Ok(x + 1)
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// this is module A
pub mod A {

    /// this is module B
    pub mod B {

        /// this is enum `UsefulEnum`, which is very useful! :)
        pub enum UsefulEnum {
            /// Type1 is the first type of this enum!
            Type1,

            /// Type2 is the second type of this enum!
            Type2,
        }
    }
}

pub use crate::A::B::UsefulEnum;

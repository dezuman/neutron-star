
extern crate neutron_star_rt;
extern crate neutron_star_constants;

use core::fmt;
use neutron_star_rt::*;
use neutron_star_constants::*;
use crate::logging::*;
pub struct Printer{}

impl Printer{
    fn print_string(&self, s: &str) {
        log_info(s);
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::testing::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut p = Printer{};
    p.write_fmt(args).unwrap();
}
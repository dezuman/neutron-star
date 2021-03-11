
extern crate neutron_star_rt;
extern crate neutron_common;

use core::fmt;
use crate::logging::*;
use crate::syscalls::*;
#[derive(Default)]
pub struct Printer{
    part_count: u8
}

impl Printer{
    /*
    fn print_string(&self, s: &str) {
        log_info(s);
    }
    */
    fn flush(&mut self){
        log_info_from_costack(self.part_count);
        self.part_count = 0;
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        push_costack(s.as_bytes());
        if self.part_count == 255{
            self.flush();
        }
        self.part_count += 1;
        Ok(())
    }
}
impl Drop for Printer {
    fn drop(&mut self){
        if self.part_count > 0{
            self.flush();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::testing::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut p = Printer::default();
    p.write_fmt(args).unwrap();
    p.flush();
}
use web_sys::console;
use crate::DEBUG;

// A macro to provide `println!(..)`-style syntax for `console.debug` logging.
macro_rules! debug {
    ( $( $t:tt )* ) => {
        if DEBUG {
            web_sys::console::debug_1(&format!( $( $t )* ).into());
        }
    }
}


pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        if DEBUG {
            console::time_with_label(name);
        }
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        if DEBUG {
            console::time_end_with_label(self.name);
        }
    }
}
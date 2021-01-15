use crate::DEBUG;
use yew::services::ConsoleService;

pub fn time(s: &str) {
    if DEBUG {
        ConsoleService::time_named(s);
    }
}

pub fn time_end(s: &str) {
    if DEBUG {
        ConsoleService::time_named_end(s);
    }
}

pub fn debug<T: core::fmt::Debug>(s: T) {
    if DEBUG {
        ConsoleService::debug(&format!("{:?}", s));
    }
}

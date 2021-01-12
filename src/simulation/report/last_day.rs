use getset::{Getters}; // , Setters, MutGetters};

/// Report of the last day of a simulation of a game.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Default)]
pub struct ReportLastDay {
    /// Numbers of healthy individulas.
    #[getset(get = "pub")]
    pub(crate) healthy: Vec<usize>,
    /// Numbers of sick individulas.
    #[getset(get = "pub")]
    pub(crate) sick: Vec<usize>,
    /// Contained outbreaks, i.e. if 
    /// there are still healthy individuals 
    /// and no infected ones.
    #[getset(get = "pub")]
    pub(crate) contained: Vec<bool>,
}
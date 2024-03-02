use crate::turing_machine::turing_machine::TuringMachine;

/// Implements filter techniques for `TuringMachine`s that
/// are currently being run.
///
/// Filtering used consists of... [SOON]
pub struct FilterRuntime {}

impl FilterRuntime {
    /// Applies all filters of the `FilterRuntime` struct to the provided
    /// `TuringMachine` and returns true if they were `all` passed.
    pub fn filter_all(turing_machine: &TuringMachine) -> bool {
        return Self::filter_long_escapees(turing_machine)
            && Self::filter_short_escapees(turing_machine);
    }

    pub fn filter_long_escapees(turing_machine: &TuringMachine) -> bool {
        return true;
    }

    pub fn filter_short_escapees(turing_machine: &TuringMachine) -> bool {
        return true;
    }
}

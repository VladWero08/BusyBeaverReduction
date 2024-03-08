use crate::turing_machine::{direction::Direction, turing_machine::TuringMachine};

/// Implements filter techniques for `TuringMachine`s that
/// are currently being run.
///
/// Filtering used consists of... [SOON]
pub struct FilterRuntime {
    counter: u8,
}

impl FilterRuntime {
    pub fn new() -> Self {
        return FilterRuntime {
            counter: 0
        }
    }    

    /// Applies all filters of the `FilterRuntime` struct to the provided
    /// `TuringMachine` and returns true if they were `all` passed.
    pub fn filter_all(&mut self, turing_machine: &TuringMachine) -> bool {
        return self.filter_long_escapees(turing_machine)
            && self.filter_short_escapees(turing_machine);
    }

    /// Given the current state of a `TuringMachine`, count
    /// how many times did the tape increased ( visited a new cell )
    /// in a row.
    ///
    /// If the number counted `exceeds the number of states`
    /// of the turing machine, that means it will loop endlessly.
    fn filter_long_escapees(&mut self, turing_machine: &TuringMachine) -> bool {
        // if the tape did not increase at all,
        // the filter is considered passed
        if turing_machine.tape.len() == 0 {
            return true;
        }

        // if the tape did not increase, reset the counter
        // and the filter is considered passed
        if turing_machine.tape_increased == false {
            self.counter = 0;
            return true;
        }

        self.counter += 1;

        return self.counter <= turing_machine.transition_function.number_of_states;
    }

    /// Given the current state of a `TuringMachine`, verify if
    /// the tape increased in the last move on a transition such as:
    /// `(q_n, 0) -> (q_n, 0, R)`.
    ///
    /// If it did, it means it will loop endlessly.
    fn filter_short_escapees(&mut self, turing_machine: &TuringMachine) -> bool {
        // if the tape did not increase at all,
        // the filter is considered passed
        if turing_machine.tape.len() == 0 {
            return true;
        }

        let possible_transition = turing_machine.transition_function.transitions.get(&(
            turing_machine.current_state,
            turing_machine.tape[turing_machine.head_position],
        ));

        match possible_transition {
            Some(transition) => {
                return !(turing_machine.current_state == transition.0
                    && turing_machine.tape[turing_machine.head_position] == transition.1
                    && transition.1 == 0
                    && transition.2 == Direction::RIGHT);
            }
            None => {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_long_escapees() {
        // TO DO
    }

    #[test]
    fn filter_short_escapees() {
        // TO DO
    }
}

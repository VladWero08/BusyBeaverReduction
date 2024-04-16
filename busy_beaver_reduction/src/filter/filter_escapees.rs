use crate::turing_machine::direction::Direction;
use crate::turing_machine::turing_machine::TuringMachine;

pub struct FilterEscapees {
    counter: u8,
}

impl FilterEscapees {
    pub fn new() -> Self {
        return FilterEscapees { counter: 0 };
    }

    /// Given the current state of a `TuringMachine`, count
    /// how many times did the tape increased ( visited a new cell )
    /// in a row.
    ///
    /// If the number counted `exceeds the number of states`
    /// of the turing machine, that means it will loop endlessly.
    pub fn filter_long_escapees(&mut self, turing_machine: &TuringMachine) -> bool {
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
    /// `(q_n, 0) -> (q_n, 0, R/L)`.
    ///
    /// If it did, it means it will loop endlessly.
    pub fn filter_short_escapees(&mut self, turing_machine: &TuringMachine) -> bool {
        // if the tape did not increase at all,
        // the filter is considered passed
        if turing_machine.tape.len() == 0 {
            return true;
        }

        // if the tape did not increase in the last iteration,
        // the filer is considered passed
        if turing_machine.tape_increased == false {
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
                    && transition.1 == 0);
            }
            None => {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{delta::{transition::Transition, transition_function::TransitionFunction}, turing_machine::{direction::Direction, turing_machine::TuringMachine}};

    use super::FilterEscapees;

    #[test]
    fn filter_long_escapees() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);
        let mut filter_escapees: FilterEscapees = FilterEscapees::new();

        transition_function.add_transition(Transition::new_params(0, 0, 1, 0, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(0, 1, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(1, 0, 0, 0, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(1, 1, 1, 1, Direction::LEFT));

        // create the turing machines based on the transition function
        let mut turing_machine: TuringMachine = TuringMachine::new(transition_function);
        let maximum_steps = 1000;

        // execute the turing machine until it reaches the maximum
        // number of steps OR it gets filtered out by the escapees filter
        while turing_machine.steps < maximum_steps {
            if filter_escapees.filter_long_escapees(&turing_machine) {
                break;
            }

            turing_machine.make_transition();
        }

        assert_ne!(turing_machine.steps, maximum_steps);
    }

    #[test]
    fn filter_short_escapees() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(3, 2);
        let mut filter_escapees: FilterEscapees = FilterEscapees::new();

        transition_function.add_transition(Transition::new_params(0, 0, 1, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(0, 1, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(1, 0, 0, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(1, 1, 1, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 0, 2, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 1, 1, 1, Direction::RIGHT));

        // create the turing machines based on the transition function
        let mut turing_machine: TuringMachine = TuringMachine::new(transition_function);
        let maximum_steps = 1000;

        // execute the turing machine until it reaches the maximum
        // number of steps OR it gets filtered out by the escapees filter
        while turing_machine.steps < maximum_steps {
            if filter_escapees.filter_short_escapees(&turing_machine) {
                break;
            }

            turing_machine.make_transition();
        }

        assert_ne!(turing_machine.steps, maximum_steps);
    }
}

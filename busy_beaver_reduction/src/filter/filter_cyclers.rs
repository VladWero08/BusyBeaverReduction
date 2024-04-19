use crate::turing_machine::turing_machine::TuringMachine;

pub struct FilterCyclers {
    history: Vec<(String, usize, u8)>,
}

impl FilterCyclers {
    pub fn new() -> Self {
        return FilterCyclers {
            history: Vec::new(),
        };
    }

    /// Given the current state of a `TuringMachine`, verify if
    /// this state was seen in the past, aka it is repeated in the
    /// history of computation of the Turing Machine.
    ///
    /// The state that is verified consists of the tuple
    /// `(<hashed_tape>, <head_position>, <current logical state>)`.
    ///
    /// If the tuple was seen in the past, it means it will loop endlessly.
    pub fn filter(&mut self, turing_machine: &TuringMachine) -> bool {
        let turing_machine_encoded = turing_machine.encode();

        // if the history of computation already
        // contains the current state of the turing machine, it
        // means it is a repetition
        if self.history.contains(&turing_machine_encoded) {
            return false;
        }

        // add the current state to the history of computation
        self.history.push(turing_machine_encoded);

        // the filtered is passed
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::delta::transition::Transition;
    use crate::delta::transition_function::TransitionFunction;
    use crate::turing_machine::direction::Direction;
    use crate::turing_machine::turing_machine::TuringMachine;

    use super::FilterCyclers;

    #[test]
    fn filter_cycler() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(5, 2);
        let mut filter_cyclers: FilterCyclers = FilterCyclers::new();

        transition_function.add_transition(Transition::new_params(0, 0, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(0, 1, 101, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(1, 0, 2, 0, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(1, 1, 0, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 0, 3, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 1, 0, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(3, 0, 1, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(3, 1, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(4, 0, 1, 2, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(4, 1, 1, 2, Direction::RIGHT));

        // create the turing machines based on the transition function
        let mut turing_machine: TuringMachine = TuringMachine::new(transition_function);
        let maximum_steps = 1000;

        turing_machine.make_transition();

        // execute the turing machine until it reaches the maximum
        // number of steps OR it gets filtered out by the escapees filter
        while turing_machine.steps < maximum_steps {
            if !(filter_cyclers.filter(&turing_machine)) {
                break;
            }

            turing_machine.make_transition();
        }

        assert_ne!(turing_machine.steps, maximum_steps);
    }
}

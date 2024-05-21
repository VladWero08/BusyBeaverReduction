use std::collections::HashMap;

use crate::turing_machine::direction::Direction;
use crate::turing_machine::turing_machine::TuringMachine;

pub struct FilterTranslatedCyclers {
    // u8 -> state,
    // direction -> direction of increase
    // Vec<u8> -> tape content
    history: HashMap<(u8, Direction), Vec<u8>>,
}

impl FilterTranslatedCyclers {
    pub fn new() -> Self {
        return FilterTranslatedCyclers {
            history: HashMap::new(),
        };
    }

    /// Given the current state of a `TuringMachine`, applies the following filter:
    ///
    /// 1. When the machine's tape reaches a new cell in a certain state
    /// for the `first time`: add the  `(current_state, direction)` = `(tape)`
    /// entry in the history hashmap.
    ///
    /// 2. When the machine's tape reaches a new cell in the same state
    /// for the `second time`, it means that a translated cycle could have occurred
    /// between the first and the second appearance": add the `(current_state, direction)` = `(tape)`
    /// entry in the possible cyclers hashmap
    ///
    /// 3. When the machine's tape reaches a new cell in the same state
    /// for the `third time`, it can be verified if the cycle took place:
    /// if tape has the same content between (1st appearence, 2nd appearence) and
    /// (2nd appearece, 3rd appearence) shifted L positions, it means it is a translated cycler.
    ///
    /// 4. If the tape differs, update the history and the possible cycler's hashmap:
    /// - possible cycler -> history,
    /// - state that filtered out the possible cycler -> possible cycler.
    pub fn filter(&mut self, turing_machine: &TuringMachine) -> bool {
        // if the tape did not increase in the last iteration,
        // the filer is considered passed
        if turing_machine.tape_increased == false {
            return true;
        }

        // extract the direction in
        // which the tape increased
        let direction;
        match turing_machine.head_position {
            0 => direction = Direction::LEFT,
            _ => direction = Direction::RIGHT,
        };

        let history_entry = self.history.get(&(turing_machine.current_state, direction));

        match history_entry {
            Some(_) => {
                // if the current state was already a possible cycler,
                // check if the cycle was actually executed
                let check_cycler = self.check_possible_cycler(turing_machine, direction);

                // if it wasn't, update the history
                if check_cycler == false {
                    self.insert_history(
                        turing_machine.current_state,
                        direction,
                        turing_machine.tape.clone(),
                    );
                }

                // if the cycler exists, return false, meaning the
                // filter was not passed
                return !check_cycler;
            }
            None => {
                // if the current state was not reached before,
                // insert it in the history
                self.insert_history(
                    turing_machine.current_state,
                    direction,
                    turing_machine.tape.clone(),
                );

                return true;
            }
        }
    }

    /// Given a state, a tape position and the number of steps
    /// executed till reaching this configuration, insert the entry
    /// in the history's hashmap.
    fn insert_history(&mut self, state: u8, direction: Direction, tape: Vec<u8>) {
        self.history.insert((state, direction), tape);
    }

    /// Knowing that `state` is a possible cycler, which means
    /// that was already visited twice, it appears another time, in a new
    /// cell.
    ///
    /// This function checks if the values on the tape between (1st_appeareance, 2nd_appearence)
    /// are the same with the tape values between (2nd appearence, 3rd appearence).
    fn check_possible_cycler(
        &mut self,
        turing_machine: &TuringMachine,
        direction: Direction,
    ) -> bool {
        let history_tape = self
            .history
            .get(&(turing_machine.current_state, direction))
            .unwrap();
        let history_tape_length = (history_tape.len() - 1) as u64;
        let current_tape_length = (turing_machine.tape.len() - 1) as u64;

        match direction {
            Direction::RIGHT => {
                for i in 0..history_tape.len() {
                    // check if the tape matches in both intervals,
                    // if it doesn't, it means its not a translated cycler
                    if turing_machine.tape[(current_tape_length - (i as u64)) as usize]
                        != history_tape[(history_tape_length - (i as u64)) as usize]
                    {
                        return false;
                    }
                }
            }
            Direction::LEFT => {
                for i in 0..history_tape.len() {
                    // check if the tape matches in both intervals,
                    // if it doesn't, it means its not a translated cycler
                    if turing_machine.tape[(current_tape_length + (i as u64)) as usize]
                        != history_tape[(history_tape_length + (i as u64)) as usize]
                    {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::delta::transition::Transition;
    use crate::delta::transition_function::TransitionFunction;
    use crate::turing_machine::direction::Direction;
    use crate::turing_machine::turing_machine::TuringMachine;

    use super::FilterTranslatedCyclers;

    #[test]
    fn filter_translated_cycler() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(5, 2);
        let mut filter_translated_cyclers: FilterTranslatedCyclers = FilterTranslatedCyclers::new();

        transition_function.add_transition(Transition::new_params(0, 0, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(0, 1, 4, 0, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(1, 0, 2, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(1, 1, 0, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 0, 3, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(2, 1, 1, 1, Direction::LEFT));
        transition_function.add_transition(Transition::new_params(3, 0, 1, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(3, 1, 101, 1, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(4, 0, 4, 0, Direction::RIGHT));
        transition_function.add_transition(Transition::new_params(4, 1, 1, 1, Direction::RIGHT));

        // create the turing machines based on the transition function
        let mut turing_machine: TuringMachine = TuringMachine::new(transition_function);
        let maximum_steps = 10000;

        turing_machine.make_transition();

        // execute the turing machine until it reaches the maximum
        // number of steps OR it gets filtered out by the escapees filter
        while turing_machine.steps < maximum_steps {
            if !(filter_translated_cyclers.filter(&turing_machine)) {
                break;
            }

            turing_machine.make_transition();
        }

        assert_ne!(turing_machine.steps, maximum_steps);
    }
}

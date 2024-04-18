use std::collections::HashMap;

use crate::turing_machine::direction::{self, Direction};
use crate::turing_machine::turing_machine::TuringMachine;

pub struct FilterTranslatedCyclers {
    // u8 -> state, 
    // usize -> position on the tape
    // i64 -> step of the Turing Machine
    history: HashMap<(u8, Direction), (usize, i64)>,
    possible_cycler: HashMap<(u8, Direction), (usize, i64)>,
}

impl FilterTranslatedCyclers {
    pub fn new() -> Self {
        return FilterTranslatedCyclers {
            history: HashMap::new(),
            possible_cycler: HashMap::new(),
        };
    }

    /// Given the current state of a `TuringMachine`, applies the following filter:
    ///
    /// 1. When the machine's tape reaches a new cell in a certain state
    /// for the `first time`: add the  `(current_state, direction)` = `(head_position, steps)`
    /// entry in the history hashmap. 
    ///
    /// 2. When the machine's tape reaches a new cell in a certain state
    /// for the `second time`, it means that a translated cycle could have occurred
    /// between the first and the second appearance": add the `(current_state, direction)` = `(head_position, steps)`
    /// entry in the possible cyclers hashmap
    ///
    /// 3. When the machine's tape reaches a new cell in a certain state
    /// for the `third time`, it can be verified if the cycle took place:
    /// if the tape has the same size and same content between (1st_appearence, 2nd_appearnce) and
    /// (2nd appearance, 3rd appearance), it means it is a translated cycler.
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
            Some(_) => { }
            None => {
                // if the current state was not reached before,
                // insert it in the history
                self.insert_history(
                    turing_machine.current_state,
                    direction,
                    turing_machine.head_position,
                    turing_machine.steps,
                );

                return true;
            }
        }

        let possible_cycler_entry = self.possible_cycler.get(&(turing_machine.current_state, direction));

        match possible_cycler_entry {
            Some(_) => {
                // if the current state was already a possible cycler,
                // check if the cycle was actually executed
                let check_cycler = self.check_possible_cycler(turing_machine, direction);

                // if it wasn't, update the history and the possible cyclers
                if check_cycler == false {
                    self.update_cycler(turing_machine, direction);
                }

                // if the cycler exists, return false, meaning the
                // filter was not passed
                return !check_cycler;
            }
            None => {
                // if the current state was already computed in history,
                // mark it as a possible cycler
                self.insert_possible_cycler(
                    turing_machine.current_state,
                    direction,
                    turing_machine.head_position,
                    turing_machine.steps,
                );

                return true;
            }
        }
    }

    /// Given a state, a tape position and the number of steps
    /// executed till reaching this configuration, insert the entry
    /// in the history's hashmap.
    fn insert_history(&mut self, state: u8, direction: Direction, tape_position: usize, step: i64) {
        self.history.insert((state, direction), (tape_position, step));
    }

    /// Given a state, a tape position and the number of steps
    /// executed till reaching this configuration, insert the entry
    /// in the possible cycler's hashmap.
    ///
    /// This should be called when the entry already exists in the
    /// history, and it was computed on a cell that was never visited before.
    fn insert_possible_cycler(&mut self, state: u8, direction: Direction, tape_position: usize, step: i64) {
        self.possible_cycler.insert((state, direction), (tape_position, step));
    }

    /// Translates the value from a possibly cycler (that has been
    /// verified and it is not a cycler) to history, and creates
    /// a new possible cycler with the value that filtered out
    /// the initial possible cycler.
    fn update_cycler(&mut self, turing_machine: &TuringMachine, direction: Direction) {
        if let Some(entry) = self.possible_cycler.get(&(turing_machine.current_state, direction)) {
            self.insert_history(turing_machine.current_state, direction, entry.0, entry.1);
            self.insert_possible_cycler(
                turing_machine.current_state,
                direction,
                turing_machine.head_position,
                turing_machine.steps,
            );
        }
    }

    /// Knowing that `state` is a possible cycler, which means
    /// that was already visited twice, it appears another time, in a new
    /// cell.
    ///
    /// This function checks if the values on the tape between (1st_appeareance, 2nd_appearence)
    /// are the same with the tape values between (2nd appearence, 3rd appearence).
    fn check_possible_cycler(&mut self, turing_machine: &TuringMachine, direction: Direction) -> bool {
        let history_appearence = self.history.get(&(turing_machine.current_state, direction)).unwrap();
        let cycler_appearence = self
            .possible_cycler
            .get(&(turing_machine.current_state, direction))
            .unwrap();

        // if different number of steps were executed,
        // it means its not a translated cycler
        if turing_machine.steps - cycler_appearence.1 != cycler_appearence.1 - history_appearence.1
        {
            return false;
        }

        // if the distances between appearences are not the same,
        // it means its not a translated cycler
        if ((turing_machine.head_position - cycler_appearence.0) as isize).abs()
            != ((cycler_appearence.0 - history_appearence.0) as isize).abs()
        {
            return false;
        }

        let distance = turing_machine.head_position - cycler_appearence.0 + 1;
        // if the cycler appearence is bigger than the history appearence, 
        // it means that it could be a right translated cycler, 
        // otherwise it is a left translated cycler
        let direction_factor: i64 = if direction == Direction::RIGHT { 1 } else { -1 };

        for i in 0..distance {
            // index for interval (1st_appeareance, 2nd_appearence)
            let first_interval_index = (history_appearence.0 as i64 + i as i64 * direction_factor) as usize;
            // index for interval (2nd_appeareance, 3rd_appearence)
            let second_interval_index = (cycler_appearence.0 as i64 + i as i64 * direction_factor) as usize;

            // check if the tape matches in both intervals,
            // if it doesn't, it means its not a translated cycler
            if turing_machine.tape[first_interval_index]
                != turing_machine.tape[second_interval_index]
            {
                return false;
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
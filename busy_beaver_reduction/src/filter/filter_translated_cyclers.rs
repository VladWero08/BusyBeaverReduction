use std::collections::HashMap;

use crate::turing_machine::turing_machine::TuringMachine;

pub struct FilterTranslatedCyclers {
    history: HashMap<u8, (usize, i64)>,
    possible_cycler: HashMap<u8, (usize, i64)>,
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
    /// for the `first time`: add the  `(current_state)` = `(head_position, steps)`
    /// entry in the history hashmap.
    ///
    /// 2. When the machine's tape reaches a new cell in a certain state
    /// for the `second time`, it means that a translated cycle could have occurred
    /// between the first and the second appearance": add the `(current_state)` = `(head_position, steps)`
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
            return false;
        }

        let history_entry = self.possible_cycler.get(&turing_machine.current_state);

        match history_entry {
            Some(_) => {}
            None => {
                // if the current state was not reached before,
                // insert it in the history
                self.insert_history(
                    turing_machine.current_state,
                    turing_machine.head_position,
                    turing_machine.steps,
                );

                return true;
            }
        }

        let possible_cycler_entry = self.possible_cycler.get(&(&turing_machine.current_state));

        match possible_cycler_entry {
            Some(_) => {
                // if the current state was already a possible cycler,
                // check if the cycle was actually executed
                let check_cycler = self.check_possible_cycler(turing_machine);

                // if it wasn't, update the history and the possible cyclers
                if check_cycler == false {
                    self.update_cycler(turing_machine);
                }

                return check_cycler;
            }
            None => {
                // if the current state was already computed in history,
                // mark it as a possible cycler
                self.insert_possible_cycler(
                    turing_machine.current_state,
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
    fn insert_history(&mut self, state: u8, tape_position: usize, step: i64) {
        self.history.insert(state, (tape_position, step));
    }

    /// Given a state, a tape position and the number of steps
    /// executed till reaching this configuration, insert the entry
    /// in the possible cycler's hashmap.
    ///
    /// This should be called when the entry already exists in the
    /// history, and it was computed on a cell that was never visited before.
    fn insert_possible_cycler(&mut self, state: u8, tape_position: usize, step: i64) {
        self.possible_cycler.insert(state, (tape_position, step));
    }

    /// Translates the value from a possibly cycler (that has been
    /// verified and it is not a cycler) to history, and creates
    /// a new possible cycler with the value that filtered out
    /// the initial possible cycler.
    fn update_cycler(&mut self, turing_machine: &TuringMachine) {
        if let Some(entry) = self.possible_cycler.get(&turing_machine.current_state) {
            self.insert_history(turing_machine.current_state, entry.0, entry.1);
            self.insert_possible_cycler(
                turing_machine.current_state,
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
    fn check_possible_cycler(&mut self, turing_machine: &TuringMachine) -> bool {
        let history_appearence = self.history.get(&turing_machine.current_state).unwrap();
        let cycler_appearence = self
            .possible_cycler
            .get(&turing_machine.current_state)
            .unwrap();

        // if different number of steps were executed,
        // it means its not a translated cycler
        if turing_machine.steps - cycler_appearence.1 != cycler_appearence.1 - history_appearence.1
        {
            return false;
        }

        // if the distances between appearences are not the same,
        // it means its not a translated cycler
        if turing_machine.head_position - cycler_appearence.0
            != cycler_appearence.0 - history_appearence.0
        {
            return false;
        }

        let distance = turing_machine.head_position - cycler_appearence.0;

        for i in 0..distance {
            // index for interval (1st_appeareance, 2nd_appearence)
            let first_interval_index = history_appearence.0 + i;
            // index for interval (2nd_appeareance, 3rd_appearence)
            let second_interval_index = cycler_appearence.0 + i;

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

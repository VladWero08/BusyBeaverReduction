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

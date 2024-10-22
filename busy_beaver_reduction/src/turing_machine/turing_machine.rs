use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::{Duration, Instant};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_runtime::FilterRuntime;
use crate::filter::filter_runtime::FilterRuntimeType;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

const MAX_STEPS_TO_RUN: i64 = 21;

#[derive(Clone)]
pub struct TuringMachine {
    pub transition_function: TransitionFunction,
    pub tape: Vec<u8>,
    pub tape_increased: bool,
    pub head_position: usize,
    pub current_state: u8,
    pub halted: bool,
    pub steps: i64,
    pub score: i32,
    pub runtime: i64,
    pub filtered: FilterRuntimeType,
}

impl TuringMachine {
    pub fn new(transition_function: TransitionFunction) -> Self {
        TuringMachine {
            transition_function: transition_function,
            tape: vec![0],
            tape_increased: false,
            head_position: 0,
            current_state: SpecialStates::StateStart.value(),
            halted: false,
            steps: 0,
            score: 0,
            runtime: 0,
            filtered: FilterRuntimeType::None,
        }
    }

    /// Calculate the score from the tape, the number
    /// of 1s written on the tape.
    pub fn set_score(&mut self) {
        for &symbol in self.tape.iter() {
            if symbol == 1 {
                self.score += 1;
            }
        }
    }

    /// Sets the runtime for the execution of the
    /// turing machine, given a `core::time::Duration` object.
    pub fn set_runtime(&mut self, time: Duration) {
        self.runtime = time.as_secs() as i64;
    }

    /// Runs the turing machine until it is halted or until
    /// it is stopped by a runtime filter.
    ///
    /// Uses a `FilterRuntime` object that is watching
    /// carefully the execution of the turing machine.
    /// If at any time the filters are not passed, stop the execution.
    pub fn execute(&mut self) {
        let start_time: Instant = Instant::now();
        let mut filter_runtime: FilterRuntime = FilterRuntime::new();

        self.make_transition();

        while self.halted != true && self.steps < MAX_STEPS_TO_RUN {
            let filter_result: FilterRuntimeType = filter_runtime.filter_all(&self);

            match filter_result {
                FilterRuntimeType::ShortEscapee
                | FilterRuntimeType::LongEscapee
                | FilterRuntimeType::Cycler
                | FilterRuntimeType::TranslatedCycler => {
                    self.filtered = filter_result;
                    break;
                }
                FilterRuntimeType::None => {}
            };

            self.make_transition();
        }

        // set the metrics for the turing machine
        self.set_score();
        self.set_runtime(start_time.elapsed());
    }

    /// Tries to make a transition of the Turing Machine
    /// using the `current_state` and the symbol found on
    /// the `tape` at the `head_position` position.
    ///
    /// If the transition exists in the `transition_function`,
    /// it will be made.
    ///
    /// Return whether the transition describes is possible.
    pub fn make_transition(&mut self) -> bool {
        let possible_transition = self
            .transition_function
            .transitions
            .get(&(self.current_state, self.tape[self.head_position]));

        match possible_transition {
            Some(transition) => {
                // by default, tape is not increased
                self.tape_increased = false;
                // change the current state
                self.current_state = transition.0;
                // write the new value to the tape
                self.tape[self.head_position] = transition.1;
                // move the header of the tape
                self.move_(transition.2);

                // check if the Turing Machine reached a halting state
                self.is_halted();

                return true;
            }
            None => {
                return false;
            }
        }
    }

    /// Executes the movement of the Turing Machine's head
    /// depending on the `direction` provided.
    pub fn move_(&mut self, direction: Direction) {
        self.steps += 1;

        match direction {
            Direction::LEFT => self.move_left(),
            Direction::RIGHT => self.move_right(),
        }
    }

    /// Moves the `head` (`head_position`) of the Turing Machine
    /// to the left only if it does not exceed the
    /// left most position of the tape.
    pub fn move_left(&mut self) {
        // if the head is at the left most position,
        // insert a new element there
        if self.head_position == 0 {
            self.tape.insert(0, 0);
            self.tape_increased = true;
        } else {
            self.head_position -= 1;
        }
    }

    /// Moves the `head` (`head_position`) of the Turing Machine
    /// to the right and `extends` the tape if necessary.
    pub fn move_right(&mut self) {
        self.head_position += 1;

        // if the tape length is exceeded, add
        // a new value on the tape, where the head
        // will be pointing at
        if self.tape.len() - 1 < self.head_position {
            self.tape.push(0);
            self.tape_increased = true;
        }
    }

    /// Checks if the `state` given as parameter
    /// represents a halting state for the Turing Machine.
    ///
    /// Modifies the `halted` state accordingly.
    pub fn is_halted(&mut self) {
        let state_: SpecialStates = SpecialStates::transform(self.current_state);

        match state_ {
            SpecialStates::StateHalt => self.halted = true,
            _ => {}
        }
    }

    /// Encodes the Turing Machine's overall state as
    /// a tuple `(String, usize, u8)`, where:
    /// - String: hashed value of the tape
    /// - usize: current head position
    /// - u8: current state
    pub fn encode(&self) -> (String, usize, u8) {
        let mut hasher = Sha256::new();
        hasher.input(&self.tape);
        let hashed_tape = hasher.result_str();

        (hashed_tape, self.head_position, self.current_state)
    }
}

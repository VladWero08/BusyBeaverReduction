use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

pub struct TuringMachine {
    pub tape: Vec<u8>,
    pub tape_increased: bool,
    pub head_position: usize,
    pub current_state: u8,
    pub steps: u32,
    pub transition_function: TransitionFunction,
    pub halted: bool,
}

impl TuringMachine {
    pub fn new() -> Self {
        TuringMachine {
            tape: vec![0],
            tape_increased: false,
            head_position: 0,
            current_state: SpecialStates::STATE_START.value(),
            steps: 0,
            transition_function: TransitionFunction::new(),
            halted: false,
        }
    }

    /// Runs the turing machine until it is halted.
    pub fn execute(&mut self) {
        while self.halted != true {
            self.make_transition();
        }
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
        match direction {
            Direction::LEFT => self.move_left(),
            Direction::RIGHT => self.move_right(),
            _ => {}
        }
    }

    /// Moves the `head` (`head_position`) of the Turing Machine
    /// to the left only if it does not exceed the
    /// left most position of the tape.
    pub fn move_left(&mut self) {
        // if the head is at the left most position,
        // ignore the movement and exit
        if self.head_position == 0 {
            return;
        }

        self.head_position -= 1;
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
            SpecialStates::STATE_HALT => self.halted = true,
            _ => {}
        }
    }
}

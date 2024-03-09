use std::collections::HashMap;

use crate::delta::transition::Transition;
use crate::turing_machine::direction::Direction;

#[derive(PartialEq, Clone, Debug)]
pub struct TransitionFunction {
    pub number_of_states: u8,
    pub number_of_symbols: u8,
    pub transitions: HashMap<(u8, u8), (u8, u8, Direction)>,
}

impl TransitionFunction {
    pub fn new(number_of_states: u8, number_of_symbols: u8) -> Self {
        TransitionFunction {
            number_of_states: number_of_states,
            number_of_symbols: number_of_symbols,
            transitions: HashMap::new(),
        }
    }

    /// Given a `Transition`, inserts it into the HashMap,
    /// indexing it by (`from_state`, `from_symbol`).
    ///
    /// It means that the transition will be identified
    /// by the current state of the Turing Machine and
    /// the current symbol where the `head` is pointing at.
    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.insert(
            (transition.from_state, transition.from_symbol),
            (
                transition.to_state,
                transition.to_symbol,
                transition.direction,
            ),
        );
    }

    /// Encodes the `transitions` HashMap by firstly encoding
    /// each entry and making a `Vec<String>>` with the encodings.
    /// After that, concatenate the vector with "|".
    ///
    /// Returns the resulted `String`.
    ///
    /// EXAMPLE:
    ///
    /// Considering the following encodings of some transitions:
    ///
    /// String transition_encoding_01 = "0,0,1,1,0";
    /// String transition_encoding_02 = "0,0,1,0,0";
    /// String transition_encoding_03 = "1,1,1,0,1";
    ///
    /// transition_function.encode() = "0,0,1,1,0|0,0,1,0,0|1,1,1,0,1"
    pub fn encode(&self) -> String {
        return self
            .transitions
            .iter()
            .map(|transition| Transition::encode_from_hashmap(transition))
            .collect::<Vec<String>>()
            .join("|");
    }

    /// Given a `String`, reconstructs the self `TransitionFunction.transitions` by
    /// decoding each transition from `encoded` and adding it back in the HashMap.
    pub fn decode(&mut self, encoded: String) {
        let transitions: Vec<String> = encoded.split("|").map(|s| s.to_string()).collect();

        for transition in transitions {
            let mut transition_: Transition = Transition::new();
            transition_.decode(transition);
            self.add_transition(transition_);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::delta::transition;

    use super::*;

    #[test]
    fn encode() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        let transition_01: Transition = Transition {
            from_state: 0,
            from_symbol: 0,
            to_state: 1,
            to_symbol: 1,
            direction: Direction::RIGHT,
        };
        let transition_02: Transition = Transition {
            from_state: 0,
            from_symbol: 1,
            to_state: 1,
            to_symbol: 1,
            direction: Direction::RIGHT,
        };

        transition_function.add_transition(transition_01);
        transition_function.add_transition(transition_02);

        let transition_function_encoded = transition_function.encode();

        if transition_function_encoded == "0,0,1,1,1|0,1,1,1,1" {
            assert_eq!(true, true);
        } else if transition_function_encoded == "0,1,1,1,1|0,0,1,1,1" {
            assert_eq!(true, true);
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn decode() {
        let transition_function_encoded = "0,0,0,0,1|0,1,1,0,1|1,1,0,1,0".to_string();
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        transition_function.decode(transition_function_encoded);

        assert_eq!(transition_function.transitions.contains_key(&(0, 0)), true);
        assert_eq!(transition_function.transitions.contains_key(&(0, 1)), true);
        assert_eq!(transition_function.transitions.contains_key(&(1, 1)), true);
        assert_eq!(
            transition_function.transitions.get(&(0, 0)),
            Some(&(0 as u8, 0 as u8, Direction::RIGHT))
        );
        assert_eq!(
            transition_function.transitions.get(&(0, 1)),
            Some(&(1 as u8, 0 as u8, Direction::RIGHT))
        );
        assert_eq!(
            transition_function.transitions.get(&(1, 1)),
            Some(&(0 as u8, 1 as u8, Direction::LEFT))
        );
    }
}

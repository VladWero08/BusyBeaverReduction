use crate::turing_machine::direction::Direction;

#[derive(Clone, Copy)]
pub struct Transition {
    pub from_state: u8,
    pub from_symbol: u8,
    pub to_state: u8,
    pub to_symbol: u8,
    pub direction: Direction,
}

impl Transition {
    pub fn new() -> Self {
        Transition {
            from_state: 0,
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::LEFT,
        }
    }

    /// Returns the transition as a `Vec<u8>`;
    ///
    /// Used for encoding the transition as a `String`.
    fn get_as_vec(&self) -> Vec<u8> {
        return vec![
            self.from_state,
            self.from_symbol,
            self.to_state,
            self.to_symbol,
            self.direction.value(),
        ];
    }

    /// Given an entry in the hashmap of transitions,
    /// returns a `Transition` built from it.
    pub fn get_from_hashmap(transition: (&(u8, u8), &(u8, u8, Direction))) -> Self {
        return Transition {
            from_state: transition.0 .0,
            from_symbol: transition.0 .1,
            to_state: transition.1 .0,
            to_symbol: transition.1 .1,
            direction: transition.1 .2,
        };
    }

    /// Given a Transition, transforms it in its `Vec<u8>`
    /// version, and afterwards encodes it: transforms all u8
    /// into String and concatenates them using a `comma`.
    ///
    /// ### Return:
    /// - `String`: encoded transition  as String, from Transition object
    ///
    /// ### Example:
    ///  Transition transition = { <br/>
    ///     `from_state`: 0,    <br/>
    ///     `from_symbol`: 0,   <br/>
    ///     `to_state`: 1,      <br/>
    ///     `to_symbol`: 1,     <br/>
    ///     `direction`: LEFT,  <br/>
    /// };
    ///
    /// transition.`encode()` = "0,0,1,1,0" ( LEFT will be transformed into a 0 )
    pub fn encode(&self) -> String {
        let transition_as_vec: Vec<u8> = self.get_as_vec();

        transition_as_vec
            .iter()
            .map(|&u| u.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    /// Given an entry in a hashmap of transitions, transforms
    /// the entry into a `Transition` and returns the encoding of it.
    pub fn encode_from_hashmap(transition: (&(u8, u8), &(u8, u8, Direction))) -> String {
        let transition_from_hashmap: Transition = Transition::get_from_hashmap(transition);

        transition_from_hashmap.encode()
    }

    /// Given an encoding of a `Transition`, decodes it and
    /// reconstructs the `Transition` function within `itself`.
    pub fn decode(&mut self, transition: String) {
        let transition: Vec<u8> = transition
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        self.from_state = transition[0];
        self.from_symbol = transition[1];
        self.to_state = transition[2];
        self.to_symbol = transition[3];
        self.direction = Direction::transform(transition[4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let transition: Transition = Transition {
            from_state: 0,
            from_symbol: 0,
            to_state: 1,
            to_symbol: 1,
            direction: Direction::RIGHT,
        };
        let transition_encoding = transition.encode();

        assert_eq!(transition_encoding, "0,0,1,1,1");
    }

    #[test]
    fn encode_from_hashmap() {
        let transition: (&(u8, u8), &(u8, u8, Direction)) = (&(0, 0), &(1, 1, Direction::RIGHT));
        let transition_encoding_from_hashmap: String = Transition::encode_from_hashmap(transition);

        assert_eq!(transition_encoding_from_hashmap, "0,0,1,1,1");
    }

    #[test]
    fn decode() {
        let transition_encoded: String = "0,0,1,1,1".to_string();
        let mut transition: Transition = Transition::new();
        transition.decode(transition_encoded);

        assert_eq!(transition.from_state, 0);
        assert_eq!(transition.from_symbol, 0);
        assert_eq!(transition.to_state, 1);
        assert_eq!(transition.to_symbol, 1);
        assert_eq!(transition.direction, Direction::RIGHT);
    }
}

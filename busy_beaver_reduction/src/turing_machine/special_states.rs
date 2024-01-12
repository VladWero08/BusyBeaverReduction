pub enum SpecialStates {
    STATE_START,
    STATE_HALT,
    DEFAULT,
}

impl SpecialStates {
    pub fn value(&self) -> u8 {
        match *self {
            SpecialStates::STATE_START => 0,
            SpecialStates::STATE_HALT => 101,
            SpecialStates::DEFAULT => 1,
        }
    }

    pub fn transform(state: u8) -> Self {
        match state {
            0 => SpecialStates::STATE_START,
            101 => SpecialStates::STATE_HALT,
            _ => SpecialStates::DEFAULT,
        }
    }
}

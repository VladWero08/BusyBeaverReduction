pub enum SpecialStates {
    STATE_START,
    STATE_ACCEPT,
    STATE_REJECT,
    DEFAULT,
}

impl SpecialStates {
    pub fn value(&self) -> u8 {
        match *self {
            SpecialStates::STATE_START => 0,
            SpecialStates::STATE_ACCEPT => 8,
            SpecialStates::STATE_REJECT => 9,
            SpecialStates::DEFAULT => 1,
        }
    }

    pub fn transform(state: u8) -> Self {
        match state {
            0 => SpecialStates::STATE_START,
            8 => SpecialStates::STATE_ACCEPT,
            9 => SpecialStates::STATE_REJECT,
            _ => SpecialStates::DEFAULT,
        }
    }
}

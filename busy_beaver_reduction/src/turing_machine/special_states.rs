pub enum SpecialStates {
    STATE_START,
    STATE_HALT,
    DEFAULT,
}

impl SpecialStates {
    /// Gets the value (`u8`) associated to each special state:
    /// - `STATE_START` = 0
    /// - `STATE_HALT` = 101
    /// - `DEFAULT` = 1
    pub fn value(&self) -> u8 {
        match *self {
            SpecialStates::STATE_START => 0,
            SpecialStates::STATE_HALT => 101,
            SpecialStates::DEFAULT => 1,
        }
    }

    /// Transforms the value given (`u8`) to a SpecialStates:
    /// - `0` = STATE_START
    /// - `101` = STATE_HALT
    /// - `_` = DEFAULT
    pub fn transform(state: u8) -> Self {
        match state {
            0 => SpecialStates::STATE_START,
            101 => SpecialStates::STATE_HALT,
            _ => SpecialStates::DEFAULT,
        }
    }
}

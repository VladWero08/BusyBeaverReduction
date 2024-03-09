pub enum SpecialStates {
    StateStart,
    StateHalt,
    Default,
}

impl SpecialStates {
    /// Gets the value (`u8`) associated to each special state:
    /// - `StateStart` = 0
    /// - `StateHalt` = 101
    /// - `Default` = 1
    pub fn value(&self) -> u8 {
        match *self {
            SpecialStates::StateStart => 0,
            SpecialStates::StateHalt => 101,
            SpecialStates::Default => 1,
        }
    }

    /// Transforms the value given (`u8`) to a SpecialStates:
    /// - `0` = StateStart
    /// - `101` = StateHalt
    /// - `_` = Default
    pub fn transform(state: u8) -> Self {
        match state {
            0 => SpecialStates::StateStart,
            101 => SpecialStates::StateHalt,
            _ => SpecialStates::Default,
        }
    }
}

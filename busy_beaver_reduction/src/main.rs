mod delta;
mod turing_machine;

use delta::transition::Transition;
use turing_machine::turing_machine::TuringMachine;

fn main() {
    let tranzitie_1: Transition = Transition {
        from_state: 0,
        from_symbol: 0,
        to_state: 1,
        to_symbol: 1,
        direction: 1
    };

    let tranzitie_2: Transition = Transition {
        from_state: 1,
        from_symbol: 0,
        to_state: 2,
        to_symbol: 1,
        direction: 1
    };

    let tranzitie_3: Transition = Transition {
        from_state: 2,
        from_symbol: 0,
        to_state: 101,
        to_symbol: 0,
        direction: 0
    };
    
    let mut tm = TuringMachine::new();    
    tm.transition_function.add_transition(tranzitie_1);
    tm.transition_function.add_transition(tranzitie_2);
    tm.transition_function.add_transition(tranzitie_3);

    while tm.make_transition() && !tm.halted {

    }
}

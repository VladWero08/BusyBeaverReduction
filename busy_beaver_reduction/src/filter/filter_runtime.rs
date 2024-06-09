use crate::filter::filter_cyclers::FilterCyclers;
use crate::filter::filter_escapees::FilterEscapees;
use crate::filter::filter_translated_cyclers::FilterTranslatedCyclers;
use crate::turing_machine::turing_machine::TuringMachine;

/// Enum for the filter runtime type, to mark
/// each running Turing machine with the filter that
/// identified it as non-halting.
#[derive(Clone)]
pub enum FilterRuntimeType {
    ShortEscapee,
    LongEscapee,
    Cycler,
    TranslatedCycler,
    None,
}

/// Filter class that acts as a wrapper for all
/// the filters that are applied during the execution
/// of a Turing Machine:
/// - `FilterCyclers`
/// - `FilterTranslatedCyclers`
/// - `FilterEscapees`
///
/// The same Turing Machine will be passed to the other
/// classes in order to filter it.
///
/// The `FilterRuntime`
/// will be part of the execution of a Turing Machine,
/// afterwards the object will be deleted.
pub struct FilterRuntime {
    filter_cyclers: FilterCyclers,
    filter_translated_cyclers: FilterTranslatedCyclers,
    filter_escapees: FilterEscapees,
}

impl FilterRuntime {
    pub fn new() -> Self {
        return FilterRuntime {
            filter_cyclers: FilterCyclers::new(),
            filter_translated_cyclers: FilterTranslatedCyclers::new(),
            filter_escapees: FilterEscapees::new(),
        };
    }

    /// Applies all filters of the `FilterRuntime` struct to the provided
    /// `TuringMachine` and returns true if they were `all` passed.
    pub fn filter_all(&mut self, turing_machine: &TuringMachine) -> FilterRuntimeType {
        if self.filter_escapees.filter_short_escapees(turing_machine) == false {
            return FilterRuntimeType::ShortEscapee;
        }

        if self.filter_escapees.filter_long_escapees(turing_machine) == false {
            return FilterRuntimeType::LongEscapee;
        };

        if self.filter_cyclers.filter(turing_machine) == false {
            return FilterRuntimeType::Cycler;
        }

        if self.filter_translated_cyclers.filter(turing_machine) == false {
            return FilterRuntimeType::TranslatedCycler;
        }

        return FilterRuntimeType::None;
    }
}

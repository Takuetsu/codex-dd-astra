//! Deterministic task-complexity floors for codexdd 0.3.0.
//!
//! Complexity can authorize a stronger *starting floor* before implementation begins. It does not
//! authorize arbitrary ladder jumps during an active Worker attempt and therefore remains separate
//! from failure pressure and trusted capability reports.

use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_policy::AdaptiveRoute;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AdaptiveComplexitySignals {
    pub(crate) estimated_files: u16,
    pub(crate) cross_module: bool,
    pub(crate) public_api_or_data_model: bool,
    pub(crate) persistent_state_or_serialization: bool,
    pub(crate) concurrency_or_async: bool,
    pub(crate) build_release_or_toolchain: bool,
    pub(crate) uncertain_root_cause: bool,
    pub(crate) broad_test_surface: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveComplexityClass {
    Routine,
    Standard,
    Complex,
    Architectural,
}

impl AdaptiveComplexityClass {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Routine => "Routine",
            Self::Standard => "Standard",
            Self::Complex => "Complex",
            Self::Architectural => "Architectural",
        }
    }
}

pub(crate) fn classify_complexity(
    signals: AdaptiveComplexitySignals,
) -> AdaptiveComplexityClass {
    let mut score = 0u8;

    score = score.saturating_add(match signals.estimated_files {
        0..=2 => 0,
        3..=5 => 1,
        6..=10 => 2,
        _ => 3,
    });
    score = score.saturating_add(u8::from(signals.cross_module));
    score = score.saturating_add(u8::from(signals.public_api_or_data_model) * 2);
    score = score.saturating_add(u8::from(signals.persistent_state_or_serialization) * 2);
    score = score.saturating_add(u8::from(signals.concurrency_or_async) * 2);
    score = score.saturating_add(u8::from(signals.build_release_or_toolchain) * 2);
    score = score.saturating_add(u8::from(signals.uncertain_root_cause));
    score = score.saturating_add(u8::from(signals.broad_test_surface));

    match score {
        0..=1 => AdaptiveComplexityClass::Routine,
        2..=4 => AdaptiveComplexityClass::Standard,
        5..=7 => AdaptiveComplexityClass::Complex,
        _ => AdaptiveComplexityClass::Architectural,
    }
}

pub(crate) fn implementation_floor(class: AdaptiveComplexityClass) -> AdaptiveRoute {
    match class {
        AdaptiveComplexityClass::Routine => AdaptiveRoute {
            family: AdaptiveFamily::Luna,
            effort: AdaptiveEffort::Low,
        },
        AdaptiveComplexityClass::Standard => AdaptiveRoute {
            family: AdaptiveFamily::Luna,
            effort: AdaptiveEffort::High,
        },
        AdaptiveComplexityClass::Complex => AdaptiveRoute {
            family: AdaptiveFamily::Terra,
            effort: AdaptiveEffort::Low,
        },
        AdaptiveComplexityClass::Architectural => AdaptiveRoute {
            family: AdaptiveFamily::Terra,
            effort: AdaptiveEffort::Medium,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_local_change_remains_luna_low() {
        let signals = AdaptiveComplexitySignals {
            estimated_files: 1,
            ..Default::default()
        };
        assert_eq!(
            classify_complexity(signals),
            AdaptiveComplexityClass::Routine
        );
        assert_eq!(
            implementation_floor(classify_complexity(signals)),
            AdaptiveRoute {
                family: AdaptiveFamily::Luna,
                effort: AdaptiveEffort::Low,
            }
        );
    }

    #[test]
    fn normal_multi_file_work_uses_luna_high_floor() {
        let signals = AdaptiveComplexitySignals {
            estimated_files: 4,
            cross_module: true,
            ..Default::default()
        };
        assert_eq!(
            classify_complexity(signals),
            AdaptiveComplexityClass::Standard
        );
        assert_eq!(
            implementation_floor(classify_complexity(signals)),
            AdaptiveRoute {
                family: AdaptiveFamily::Luna,
                effort: AdaptiveEffort::High,
            }
        );
    }

    #[test]
    fn stateful_cross_module_work_can_start_on_terra() {
        let signals = AdaptiveComplexitySignals {
            estimated_files: 7,
            cross_module: true,
            persistent_state_or_serialization: true,
            uncertain_root_cause: true,
            ..Default::default()
        };
        assert_eq!(
            classify_complexity(signals),
            AdaptiveComplexityClass::Complex
        );
        assert_eq!(
            implementation_floor(classify_complexity(signals)),
            AdaptiveRoute {
                family: AdaptiveFamily::Terra,
                effort: AdaptiveEffort::Low,
            }
        );
    }

    #[test]
    fn architectural_risk_caps_initial_floor_at_terra_medium() {
        let signals = AdaptiveComplexitySignals {
            estimated_files: 12,
            cross_module: true,
            public_api_or_data_model: true,
            persistent_state_or_serialization: true,
            concurrency_or_async: true,
            build_release_or_toolchain: true,
            uncertain_root_cause: true,
            broad_test_surface: true,
        };
        assert_eq!(
            classify_complexity(signals),
            AdaptiveComplexityClass::Architectural
        );
        assert_eq!(
            implementation_floor(classify_complexity(signals)),
            AdaptiveRoute {
                family: AdaptiveFamily::Terra,
                effort: AdaptiveEffort::Medium,
            }
        );
    }

    #[test]
    fn complexity_floor_never_starts_directly_on_sol_or_astra() {
        for class in [
            AdaptiveComplexityClass::Routine,
            AdaptiveComplexityClass::Standard,
            AdaptiveComplexityClass::Complex,
            AdaptiveComplexityClass::Architectural,
        ] {
            assert!(matches!(
                implementation_floor(class).family,
                AdaptiveFamily::Luna | AdaptiveFamily::Terra
            ));
        }
    }
}

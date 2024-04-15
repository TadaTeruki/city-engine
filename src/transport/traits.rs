use crate::core::{
    geometry::{angle::Angle, site::Site},
    Stage,
};

use super::rules::TransportRules;

/// Provider of transport rules.
///
/// Rules for constructing a path are associated with sites.
pub trait TransportRulesProvider {
    fn get_rules(&self, site_end: &Site, stage: Stage, angle: Angle) -> Option<TransportRules>;
}

/// Provider of random f64 values.
///
/// The range of the value is the same as the range of `f64` (not constrained).
pub trait RandomF64Provider {
    fn gen_f64(&mut self) -> f64;
}

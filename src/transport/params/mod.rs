use metrics::PathMetrics;
use numeric::Stage;
use rules::TransportRules;

pub mod metrics;
pub mod numeric;
pub mod priority;
pub mod rules;

/// Parameters of path to be extended.
#[derive(Debug, Clone, PartialEq)]
pub struct PathParams {
    pub stage: Stage,
    pub rules_start: TransportRules,
    pub metrics: PathMetrics,
    pub priority: f64,
}
use crate::system::rule::TransportRule;

use super::metrics::TransportMetrics;

/// Factors of the path which are referenced when the path is created.
#[derive(Debug, Clone, PartialEq)]
pub struct PathConstructionFactors {
    /// Metrics which are referenced when the path is created.
    metrics: TransportMetrics,

    /// Rules which are referenced when the path is created.
    rule: TransportRule,
}
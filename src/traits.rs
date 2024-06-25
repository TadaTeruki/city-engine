use crate::{
    core::geometry::site::Site,
    system::{node::numeric::Stage, path::factor::metrics::TransportMetrics, rule::TransportRule},
};

/// Provider of transport rules.
pub trait TransportRuleProvider {
    fn get_rule(
        &self,
        site: &Site,
        stage: Stage,
        metrics: TransportMetrics,
    ) -> Option<TransportRule>;
}

/// Provider of transport rules that always returns the same rules.
///
/// This is used only for testing purposes.
pub(crate) struct MockSameRuleProvider {
    rule: TransportRule,
}

impl MockSameRuleProvider {
    pub fn new(rule: TransportRule) -> Self {
        Self { rule }
    }
}

impl TransportRuleProvider for MockSameRuleProvider {
    fn get_rule(
        &self,
        _site: &Site,
        _stage: Stage,
        _metrics: TransportMetrics,
    ) -> Option<TransportRule> {
        Some(self.rule.clone())
    }
}

/// Provider of terrain elevation.
pub trait TerrainProvider {
    fn get_elevation(&self, site: &Site) -> Option<f64>;
}

/// Terrain provider that provides a flat surface.
///
/// This is used only for testing purposes.
pub(crate) struct MockSurfaceTerrain {
    elevation: f64,
}

impl MockSurfaceTerrain {
    pub fn new(elevation: f64) -> Self {
        Self {
            elevation: elevation,
        }
    }
}

impl TerrainProvider for MockSurfaceTerrain {
    fn get_elevation(&self, _site: &Site) -> Option<f64> {
        Some(self.elevation)
    }
}

/// Terrain provider that provides an elevation based on the nearest spot which has a predefined elevation.
///
/// This is used only for testing purposes.
pub(crate) struct MockVoronoiTerrain {
    spots: Vec<(Site, f64)>,
}

impl MockVoronoiTerrain {
    pub fn new(spots: Vec<(Site, f64)>) -> Self {
        Self { spots }
    }
}

impl TerrainProvider for MockVoronoiTerrain {
    fn get_elevation(&self, site: &Site) -> Option<f64> {
        self.spots
            .iter()
            .map(|(spot, elevation)| (spot.distance(site), elevation))
            .min_by(|(distance1, _), (distance2, _)| distance1.total_cmp(distance2))
            .map(|(_, elevation)| *elevation)
    }
}

/// Provider of random f64 values.
///
/// The range of the value is the same as the range of `f64` (not constrained).
pub trait RandomF64Provider {
    fn gen_f64(&mut self) -> f64;
}

use crate::core::geometry::site::Site;

/// Properties of a site for constructing a new path.
#[derive(Debug, Clone, PartialEq)]
pub struct TransportProperty {
    /// Priority to construct a path to this site.
    pub path_priority: f64,

    /// Elevation.
    pub elevation: f64,
    /// Population density.
    pub population_density: f64,

    /// Length of the path.
    pub path_length: f64,

    /// Probability of branching. If 1.0, the path will always create branch.
    pub branch_probability: f64,

    /// Property of curves.
    /// If None, the path will be always extended to straight.
    pub curve: Option<CurveProperty>,
}

/// Properties of curves.
#[derive(Debug, Clone, PartialEq)]
pub struct CurveProperty {
    /// Maximum angle of curves.
    pub max_radian: f64,
    /// Number of candidates of the next site to create a path.
    /// This parameter should be an odd number to evaluate the straight path.
    pub comparison_step: usize,
}

impl Default for CurveProperty {
    fn default() -> Self {
        Self {
            max_radian: 0.0,
            comparison_step: 1,
        }
    }
}

pub trait TransportPropertyProvider {
    fn get_property(&self, site: &Site) -> Option<TransportProperty>;
}

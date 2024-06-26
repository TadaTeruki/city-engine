use branch::BranchRule;
use bridge::BridgeRule;
use direction::PathDirectionRule;

use crate::unit::{Elevation, Length};

pub mod branch;
pub mod bridge;
pub mod direction;

/// Rules to construct a transport network.
#[derive(Debug, Clone, PartialEq)]
pub struct TransportRule {
    /// Normal length of the path.
    pub path_normal_length: Length,

    /// Extra length of the path to search intersections.
    pub path_extra_length_for_intersection: Length,

    /// Maximum elevation difference of the path.
    ///
    /// To extend a path, the elevation difference (=slope) between the start and end of the path should be less than this value.
    pub path_slope_elevation_diff_limit: GrowthRuleElevationDiff,

    /// Required elevation difference to construct grade-separate paths.
    ///
    /// To construct grade-separate paths, the elevation difference between the paths should be more than this value.
    pub path_grade_separation_elevation_diff_required: GrowthRuleElevationDiff,

    /// Rules to create branches.
    pub branch_rule: BranchRule,

    /// Rules to determine the direction of the path.
    pub path_direction_rule: PathDirectionRule,

    /// Rules to create bridges.
    pub bridge_rule: BridgeRule,
}

impl Default for TransportRule {
    fn default() -> Self {
        Self {
            path_normal_length: Length::new(0.0),
            path_extra_length_for_intersection: Length::new(0.0),
            path_slope_elevation_diff_limit: GrowthRuleElevationDiff::AlwaysAllow,
            path_grade_separation_elevation_diff_required: GrowthRuleElevationDiff::AlwaysDeny,
            branch_rule: Default::default(),
            path_direction_rule: Default::default(),
            bridge_rule: Default::default(),
        }
    }
}

impl TransportRule {
    /// Set the normal length of the path.
    pub fn path_normal_length(mut self, path_normal_length: Length) -> Self {
        self.path_normal_length = path_normal_length;
        self
    }

    /// Set the extra length of the path to search intersections.
    pub fn path_extra_length_for_intersection(
        mut self,
        path_extra_length_for_intersection: Length,
    ) -> Self {
        self.path_extra_length_for_intersection = path_extra_length_for_intersection;
        self
    }

    /// Set the maximum elevation difference of the path.    
    pub fn path_slope_elevation_diff_limit(
        mut self,
        path_elevation_diff_limit: GrowthRuleElevationDiff,
    ) -> Self {
        self.path_slope_elevation_diff_limit = path_elevation_diff_limit;
        self
    }

    /// Set the required elevation difference to construct grade-separate paths.
    pub fn path_grade_separation_elevation_diff_required(
        mut self,
        path_elevation_diff_required: GrowthRuleElevationDiff,
    ) -> Self {
        self.path_grade_separation_elevation_diff_required = path_elevation_diff_required;
        self
    }

    /// Set the rules to create branches.
    pub fn branch_rule(mut self, branch_rule: BranchRule) -> Self {
        self.branch_rule = branch_rule;
        self
    }

    /// Set the rules to determine the direction of the path.
    pub fn path_direction_rule(mut self, path_direction_rule: PathDirectionRule) -> Self {
        self.path_direction_rule = path_direction_rule;
        self
    }

    /// Set the rules to create bridges.
    pub fn bridge_rule(mut self, bridge_rule: BridgeRule) -> Self {
        self.bridge_rule = bridge_rule;
        self
    }
}

/// Express the elevation difference of the path.
#[derive(Debug, Clone, PartialEq)]
pub enum GrowthRuleElevationDiff {
    AlwaysAllow,
    AlwaysDeny,
    Linear(Elevation),
    NonLinear(fn(Length) -> Elevation),
}

impl GrowthRuleElevationDiff {
    /// Get the elevation difference from the path length.
    pub fn value(&self, path_length: Length) -> Elevation {
        match self {
            GrowthRuleElevationDiff::AlwaysAllow => Elevation::new(f64::INFINITY),
            GrowthRuleElevationDiff::AlwaysDeny => Elevation::new(f64::NEG_INFINITY),
            GrowthRuleElevationDiff::Linear(elevation) => {
                Elevation::new(elevation.value() * path_length.value())
            }
            GrowthRuleElevationDiff::NonLinear(f) => f(path_length),
        }
    }
}

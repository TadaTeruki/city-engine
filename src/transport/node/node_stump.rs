use crate::{
    core::{
        container::path_network::NodeId,
        geometry::{angle::Angle, line_segment::LineSegment, site::Site},
    },
    transport::{
        params::{rules::check_elevation_diff, PathParams},
        path_network_repository::RelatedNode,
        traits::TerrainProvider,
    },
};

use super::{
    growth_type::{GrowthTypes, NextNodeType},
    transport_node::TransportNode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStump {
    node_id: NodeId,
    angle_expected: Angle,
    params: PathParams,
}

impl Eq for NodeStump {}

impl PartialOrd for NodeStump {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeStump {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.params.evaluation.total_cmp(&other.params.evaluation)
    }
}

impl NodeStump {
    /// Create a new node stump.
    pub fn new(node_id: NodeId, angle_expected: Angle, params: PathParams) -> Self {
        Self {
            node_id,
            angle_expected,
            params,
        }
    }

    /// Get node id
    pub fn get_node_id(&self) -> NodeId {
        self.node_id
    }

    /// Get the end site of the path.
    pub fn angle_expected(&self) -> Angle {
        self.angle_expected
    }

    pub fn get_path_params(&self) -> &PathParams {
        &self.params
    }

    /// Get the end site of the path with extra length.
    /// This is temporary used for searching intersections.
    fn get_expected_site_to_with_extra_length(
        &self,
        start_site: Site,
        site_expected_end: Site,
    ) -> Site {
        let path_length = site_expected_end.distance(&start_site);
        let scale = (path_length + self.params.rules_start.path_extra_length_for_intersection)
            / path_length;
        Site::new(
            start_site.x + (site_expected_end.x - start_site.x) * scale,
            start_site.y + (site_expected_end.y - start_site.y) * scale,
        )
    }

    /// Determine the next node type from related(close) nodes and paths.
    pub fn determine_growth<TP>(
        &self,
        node_start: &TransportNode,
        node_expected_end: &TransportNode,
        related_nodes: &[RelatedNode],
        related_paths: &[(RelatedNode, RelatedNode)],
        terrain_provider: &TP,
    ) -> GrowthTypes
    where
        TP: TerrainProvider,
    {
        let search_start = node_start.site;

        // Existing Node
        // For this situation, path crosses are needed to be checked again because the direction of the path can be changed from original.
        {
            let existing_node_id = related_nodes
                .iter()
                .filter(|existing| {
                    // distance check for decreasing the number of candidates
                    LineSegment::new(search_start, node_expected_end.site)
                        .get_distance(&existing.node.site)
                        < self.params.rules_start.path_extra_length_for_intersection
                })
                .filter(|existing| {
                    // no intersection check
                    let has_intersection = related_paths.iter().any(|(path_start, path_end)| {
                        if existing.node_id == path_start.node_id
                            || existing.node_id == path_end.node_id
                        {
                            // ignore
                            return false;
                        }
                        let path_line = LineSegment::new(path_start.node.site, path_end.node.site);
                        let search_line = LineSegment::new(search_start, existing.node.site);
                        path_line.get_intersection(&search_line).is_some()
                    });
                    !has_intersection
                })
                .filter_map(|existing| {
                    // slope check
                    // if the elevation difference is too large, the path cannot be connected.
                    let distance = existing.node.site.distance(&search_start);
                    check_elevation_diff(
                        terrain_provider.get_elevation(&search_start)?,
                        terrain_provider.get_elevation(&existing.node.site)?,
                        distance,
                        self.params.rules_start.path_elevation_diff_limit,
                    )
                    .then_some(existing)
                })
                .min_by(|a, b| {
                    let distance_a = a.node.site.distance_2(&search_start);
                    let distance_b = b.node.site.distance_2(&search_start);
                    distance_a.total_cmp(&distance_b)
                });

            if let Some(existing) = existing_node_id {
                return GrowthTypes {
                    next_node: NextNodeType::Existing(existing.node_id),
                };
            }
        }

        // Crossing Paths
        {
            let search_end = self
                .get_expected_site_to_with_extra_length(node_start.site, node_expected_end.site);
            let search_line = LineSegment::new(search_start, search_end);

            let crossing_path = related_paths
                .iter()
                .filter_map(|(path_start, path_end)| {
                    let path_line = LineSegment::new(path_start.node.site, path_end.node.site);

                    if let Some(intersect) = path_line.get_intersection(&search_line) {
                        return Some((
                            TransportNode::new(
                                intersect,
                                path_start.node.path_stage(path_end.node),
                                path_start.node.path_creates_bridge(path_end.node),
                            ),
                            (path_start, path_end),
                        ));
                    }
                    None
                })
                .filter_map(|(crossing_node, path)| {
                    // slope check
                    // if the elevation difference is too large, the path cannot be connected.
                    let distance = crossing_node.site.distance(&search_start);
                    check_elevation_diff(
                        terrain_provider.get_elevation(&search_start)?,
                        terrain_provider.get_elevation(&crossing_node.site)?,
                        distance,
                        self.params.rules_start.path_elevation_diff_limit,
                    )
                    .then_some((crossing_node, path))
                })
                .min_by(|a, b| {
                    let distance_a = a.0.site.distance_2(&search_start);
                    let distance_b = b.0.site.distance_2(&search_start);
                    distance_a.total_cmp(&distance_b)
                });

            if let Some((crossing_node, (path_start, path_end))) = crossing_path {
                // if it cross the bridge, the path cannot be connected.
                if path_start.node.path_creates_bridge(path_end.node) {
                    return GrowthTypes {
                        next_node: NextNodeType::None,
                    };
                }

                return GrowthTypes {
                    next_node: NextNodeType::Intersect(
                        crossing_node,
                        (path_start.node_id, path_end.node_id),
                    ),
                };
            }
        }

        // check slope
        let distance = search_start.distance(&node_expected_end.site);
        let slope_ok = {
            if let (Some(elevation_start), Some(elevation_end)) = (
                terrain_provider.get_elevation(&search_start),
                terrain_provider.get_elevation(&node_expected_end.site),
            ) {
                check_elevation_diff(
                    elevation_start,
                    elevation_end,
                    distance,
                    self.params.rules_start.path_elevation_diff_limit,
                )
            } else {
                false
            }
        };

        if !slope_ok {
            return GrowthTypes {
                next_node: NextNodeType::None,
            };
        }

        // New Node
        // Path crosses are already checked in the previous steps.
        GrowthTypes {
            next_node: NextNodeType::New(*node_expected_end),
        }
    }
}

use std::hash::{Hash, Hasher};

use rstar::{PointDistance, RTreeObject, AABB};

use super::angle::Angle;

/// Representation of a 2D site.
#[derive(Debug, Copy, Clone)]
pub struct Site {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Site {}

impl Hash for Site {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.x.to_bits());
        state.write_u64(self.y.to_bits());
    }
}

impl RTreeObject for Site {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y])
    }
}

impl PointDistance for Site {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        ((self.x - point[0]).powi(2) + (self.y - point[1]).powi(2)).sqrt()
    }
}

impl Site {
    /// Create a site from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate the euclidean distance to the other site.
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Calculate the site moved by the angle and distance.
    fn extend(&self, angle: Angle, distance: f64) -> Self {
        let x = self.x + angle.radian().cos() * distance;
        let y = self.y + angle.radian().sin() * distance;
        Self::new(x, y)
    }

    /// Calculate the angle to the other site.
    fn get_angle(&self, other: &Self) -> Option<Angle> {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx == 0.0 && dy == 0.0 {
            return None;
        }
        Some(Angle::new(dy.atan2(dx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let site1 = Site::new(0.0, 0.0);
        let site2 = Site::new(3.0, 4.0);
        assert_eq!(site1.distance(&site2), 5.0);
    }

    #[test]
    fn test_extend() {
        let site = Site::new(0.0, 0.0);
        let angle = Angle::new(std::f64::consts::PI / 4.0);
        let distance = 1.0;
        let extended = site.extend(angle, distance);
        let expected = Site::new(1.0, 1.0);
        assert!(extended.distance(&expected) - distance < 1e-9);
    }

    #[test]
    fn test_get_angle() {
        let site1 = Site::new(0.0, 0.0);
        let site2 = Site::new(1.0, 1.0);
        assert_eq!(
            site1.get_angle(&site2).unwrap().radian(),
            std::f64::consts::PI / 4.0
        );

        let site1 = Site::new(0.0, 0.0);
        let site2 = Site::new(-1.0, -1.0);
        assert_eq!(
            site1.get_angle(&site2).unwrap().radian(),
            -3.0 * std::f64::consts::PI / 4.0
        );

        let site1 = Site::new(0.0, 0.0);
        let site2 = Site::new(0.0, 0.0);
        assert_eq!(site1.get_angle(&site2), None);
    }
}
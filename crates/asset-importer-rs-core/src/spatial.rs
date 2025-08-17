use std::cmp::Ordering;

use asset_importer_rs_scene::{AiReal, AiVector3D};

pub const INITIAL_PLANE_NORMAL: AiVector3D = AiVector3D {
    x: 0.8523,
    y: 0.34321,
    z: 0.5736,
};

pub trait SpatialLookup {
    /// Find the positions within a radius of the given position.
    fn find_position(&self, position: AiVector3D, radius: AiReal) -> Vec<usize>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialEntry {
    pub original_index: usize,
    pub distance: AiReal,
    pub position: AiVector3D,
}

/// A Spatial is a data structure that allows for fast lookup of positions within a radius.
/// It sorts the positions by their distance from the centroid along the plane normal.
/// It then uses a range search along that plane to find position candidates.
/// Then those candidates are checked for actual 3D distance.
/// This helps reduce the number of distance calculations needed.
#[derive(Debug, Clone)]
pub struct Spatial {
    /// The normal of the sorting plane.
    pub plane_normal: AiVector3D,
    /// The centroid of the positions as a point on the plane.
    pub centroid: AiVector3D,
    /// The entries of the spatial.
    pub entries: Vec<SpatialEntry>,
}

impl Spatial {
    pub fn new(positions: &[AiVector3D]) -> Self {
        Self::new_with_normal(INITIAL_PLANE_NORMAL, positions)
    }

    pub fn new_with_normal(normal_plane: AiVector3D, positions: &[AiVector3D]) -> Self {
        let plane_normal = normal_plane.norm();
        let scale: AiReal = 1.0 / positions.len() as AiReal;
        let mut centroid = AiVector3D::zero();
        for position in positions {
            let position = position * scale;
            centroid += position;
        }
        let mut entries = Vec::with_capacity(positions.len());
        for (index, position) in positions.iter().enumerate() {
            let mut distance = (position - &centroid) * plane_normal;
            if distance.is_nan() {
                distance = AiReal::MAX;
            }
            entries.push(SpatialEntry {
                original_index: index,
                distance,
                position: *position,
            });
        }
        entries.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        Self {
            plane_normal,
            centroid,
            entries,
        }
    }
}

impl SpatialLookup for Spatial {
    fn find_position(&self, position: AiVector3D, radius: AiReal) -> Vec<usize> {
        let mut results = Vec::new();
        if self.entries.is_empty() {
            return results;
        }

        // Calculate distance from query position to centroid along plane normal
        let distance = (position - self.centroid) * self.plane_normal;
        let min_distance = distance - radius;

        // Early exit: query range is beyond the last entry
        if min_distance > self.entries[self.entries.len() - 1].distance {
            return results;
        }
        let max_distance = distance + radius;

        // Early exit: query range is before the first entry
        if max_distance < self.entries[0].distance {
            return results;
        }

        // Binary search to find approximate starting position
        let mut index = self.entries.len() / 2;
        let mut step_size = self.entries.len() / 4;
        while step_size > 0 {
            if self.entries[index].distance < min_distance {
                index += step_size; // Move right if too small
            } else if self.entries[index].distance > max_distance {
                index -= step_size; // Move left if too large
            }
            step_size /= 2; // Halve step size for next iteration
        }

        // Fine-tune index to find first entry >= min_distance
        while index > 0 && self.entries[index].distance > min_distance {
            index -= 1; // Move left until we're at or below min_distance
        }
        while index < self.entries.len() - 1 && self.entries[index].distance < min_distance {
            index += 1; // Move right if we're still below min_distance
        }

        // Now index points to first entry >= min_distance
        let radius_squared = radius * radius; // Use squared distance to avoid sqrt
        while let Some(entry) = self.entries.get(index) {
            if entry.distance > max_distance {
                break; // Exit when we're beyond the search range
            }
            // Check actual 3D distance (not just along plane normal)
            let distance_squared = (entry.position - position).square_length();
            if distance_squared < radius_squared {
                results.push(entry.original_index); // Add to results if within radius
            }
            index += 1; // Move to next entry
        }
        results
    }
}

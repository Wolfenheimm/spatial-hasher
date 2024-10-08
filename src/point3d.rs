//! The `point3d` module provides the `Point3D` struct, representing a point in 3D space.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Represents a point in 3D space.
///
/// # Fields
///
/// - `x`: The x-coordinate.
/// - `y`: The y-coordinate.
/// - `z`: The z-coordinate.
///
/// # Examples
///
/// ```
/// use spatial_hasher::Point3D;
///
/// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
/// ```
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point3D {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
    /// The z-coordinate.
    pub z: f64,
}

impl Hash for Point3D {
    /// Hashes the `Point3D` by writing the bit representations of its coordinates into the provided hasher.
    ///
    /// This method uses the `to_bits()` function to obtain the raw memory representation of the floating-point numbers, ensuring consistent hashing.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to a type implementing the `Hasher` trait.
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.x.to_bits());
        state.write_u64(self.y.to_bits());
        state.write_u64(self.z.to_bits());
    }
}

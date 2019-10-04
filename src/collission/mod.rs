mod box_collider;
mod line_collider;

mod point;
mod vector;

pub use box_collider::*;
pub use line_collider::*;

pub use point::*;
pub use vector::*;

pub type Coord = i32;

/// When a variable of type Coord has value METER, it corresponds to a meter
pub const METER: Coord = 100_000;
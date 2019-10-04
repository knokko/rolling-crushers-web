use super::Coord;

pub struct BoxCollider {

    pub(super) min_x: Coord,
    pub(super) min_y: Coord,
    pub(super) min_z: Coord,

    pub(super) max_x: Coord,
    pub(super) max_y: Coord,
    pub(super) max_z: Coord
}

impl BoxCollider {

    pub fn new(min_x: Coord, min_y: Coord, min_z: Coord, max_x: Coord, max_y: Coord, max_z: Coord) -> BoxCollider {
        BoxCollider {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z
        }
    }
}
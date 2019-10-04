use super::Coord;

#[derive(Clone,Copy)]
pub struct Vector {

    x: Coord,
    y: Coord,
    z: Coord
}

impl Vector {

    pub fn new(x: Coord, y: Coord, z: Coord) -> Vector {
        Vector {
            x,y,z
        }
    }

    pub fn get_x(&self) -> Coord {
        self.x
    }

    pub fn get_y(&self) -> Coord {
        self.y
    }

    pub fn get_z(&self) -> Coord {
        self.z
    }
}
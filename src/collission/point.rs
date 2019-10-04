use super::Coord;

#[derive(Clone,Copy)]
pub struct Point {

    x: Coord,
    y: Coord,
    z: Coord
}

impl Point {

    pub fn new(x: Coord, y: Coord, z:Coord) -> Point {
        Point {
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
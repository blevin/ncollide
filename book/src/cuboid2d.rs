extern crate nalgebra;
extern crate ncollide = "ncollide2df32";

use nalgebra::na::Vec2;
use ncollide::geom::Cuboid;

fn main() {
    let cuboid = Cuboid::new(Vec2::new(2.0, 1.0));
    assert!(cuboid.margin() == 0.04); // default margin
}

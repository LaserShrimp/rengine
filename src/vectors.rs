pub mod vectors {

use std::ops::Add;
use std::ops::Sub;

#[derive(Default, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, vec: Self) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z
        }
    } 
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, vec: Self) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z
        }
    } 
}

impl Vector3D {
    pub fn normalize(&self) {
        todo!()
    }

    pub fn rotate(&self, rotation_vector: Vector3D) {
        todo!()
    }
}

pub fn find_coordinates_on_screen_from_two_vectors(pov_vec: &Vector3D, point_to_project: &Vector3D, z_axis: f64) -> Vector3D {
    let z_axis = pov_vec.z + 100.0;

    let a = pov_vec;
    let b = point_to_project;

    let x_a = (b.x - a.x) / (b.z - a.z);
    let x_b = a.x - x_a*a.z;

    let y_a = (b.y - a.y)/(b.z - a.z);
    let y_b = a.y - y_a*a.z;

    Vector3D{
        x: x_a*z_axis + x_b,
        y: y_a*z_axis + y_b,
        z: 0.0
    }
}

}

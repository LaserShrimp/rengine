pub mod curves {

use crate::vectors::vectors::Vector3D;

pub fn butterfly(t: f64) -> Vector3D {
    Vector3D {
        x: t.sin()*(t.cos().exp() - 2.0*(4.0*t).cos() - (t/12.0).sin().powf(5.0)),
        y: t.cos()*(t.cos().exp() - 2.0*(4.0*t).cos() - (t/12.0).sin().powf(5.0)),
        z: t
    }
}

pub fn spiral(t: f64, phi: f64) -> Vector3D {
    Vector3D {
        x: (t + phi).cos()*10.0,
        y: t,
        z: (t + phi).sin()*10.0 + 20.0 
    }
}

}

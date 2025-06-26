pub(crate) mod color;
pub(crate) mod interval;
pub mod vec3;

// const INFINITY:f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub(crate) fn random_double() -> f64 {
    //Return a random real in [0,1)
    rand::random::<f64>()
}

fn ramdom_double(min: f64, max: f64) -> f64 {
    //Return a random real in [min,max)
    rand::random::<f64>() * (max - min) + min
}

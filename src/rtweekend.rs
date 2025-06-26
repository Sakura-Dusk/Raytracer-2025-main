pub(crate) mod color;
pub mod vec3;

// const INFINITY:f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

use iced::Vector;

pub fn rotate_vector_90(v: Vector) -> Vector {
    Vector::new(-v.y, v.x)
}

pub fn dot_product(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
}

/// Returns unit vector representing the direction to this point from (0, 0)
pub fn normalized_vector(x: f32, y: f32) -> Vector {
    let length = (x.powi(2) + y.powi(2)).sqrt();
    Vector::new(x / length, y / length)
}

/// This class has all internal methods, used by Polygon, Morph, etc.
pub fn distance(v: Vector) -> f32 {
    (v.x.powi(2) + v.y.powi(2)).sqrt()
}

// internal fun distanceSquared(x: Float, y: Float) = x * x + y * y
pub fn distance_squared(x: f32, y: f32) -> f32 {
    x.powi(2) + y.powi(2)
}

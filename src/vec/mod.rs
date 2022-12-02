mod vec3;

pub mod math {
    use super::vec3::*;

    pub fn cross(vec_a: Vec3, vec_b: Vec3) -> Vec3 {
        let i = (vec_a.y * vec_b.z) - (vec_b.y * vec_a.z);
        let j = -((vec_a.x * vec_b.z) - (vec_b.x * vec_a.z));
        let k = (vec_a.x * vec_b.y) - (vec_b.x * vec_a.y);
        Vec3::new(i, j, k)
    }

    pub fn dot(vec_a: Vec3, vec_b: Vec3) -> f32 {
        (vec_a.x * vec_b.x) + (vec_a.y * vec_b.y) + (vec_a.z * vec_b.z)
    }

    pub fn get_angle(vec_a: Vec3, vec_b: Vec3) -> f32 {
        let dot: f32 = dot(vec_a, vec_b);
        f32::acos(dot/(vec_a.magnitude() * vec_b.magnitude()))
    }
}

pub mod prelude {
    pub use super::vec3::*;
    pub use super::math;
}
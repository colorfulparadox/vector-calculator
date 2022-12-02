use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Div;

use crate::vec;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    #[inline(always)]
    pub fn from_str(str: &String) -> Vec3 {
        let mut xyz: Vec<f32> = Vec::with_capacity(3);
        
        xyz = str.lines().next().unwrap()
            .split(',').map(|s| s.trim())
            .filter(|s| !s.is_empty() && !s.eq(&"-") && is_num(s.to_string())) 
            .map(|s| s.parse().unwrap()) 
            .collect();
        
        if xyz.len() == 2 {
            xyz.extend([1.].iter());
        }

        if xyz.len() == 1 {
            xyz.extend([1., 1.].iter());
        }

        Vec3::new(xyz[0], xyz[1], xyz[2])
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[allow(dead_code)]
    pub fn fast_magnitude(&self) -> f32 {
        vec::math::dot(*self, *self)
    }

    pub fn normalize(&self) -> Vec3 {
        let mag: f32 = self.magnitude();
        if mag > 1. {
            return self.clone() / mag;
        }
        return *self;
    }
}

fn is_num(num_str: String) -> bool {
    let num = num_str.parse::<f32>();
    match num {
        Ok(_val) => true,
        Err(_why) => false
    }
}
impl Add<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}
use std::ops::{Mul, Sub, Add, Div};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ThreeVector {
    pub x: f64, 
    pub y: f64,
    pub z: f64,
}

impl ThreeVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ThreeVector {x, y, z}
    }

    pub fn zeros() -> ThreeVector {
        ThreeVector::new(0f64, 0f64, 0f64)
    }

    pub fn dot(&self, other: ThreeVector) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        x + y + z 
    }

    pub fn cross(&self, other: ThreeVector) -> ThreeVector {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.z;
        let z = self.x*other.y - self.y*other.x;
        
        ThreeVector::new(x, y , z)
    }

    pub fn make_unit_vector(self) -> ThreeVector {
        let norm = 1.0/ (self.x * self.x + self.y*self.y + self.z*self.z).sqrt();
        self * norm
    }
}

impl Div<f64> for ThreeVector {
    type Output = ThreeVector;

    fn div(self, rhs: f64) -> ThreeVector {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;

        ThreeVector::new(x, y, z)
    }
}

impl Mul<f64> for ThreeVector {
    type Output = ThreeVector;

    fn mul(self, rhs: f64)  -> ThreeVector {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;

        ThreeVector::new(x, y, z)
    }
}

impl Add<&ThreeVector> for ThreeVector {
    type Output = ThreeVector;

    fn add(self, other: &ThreeVector) -> ThreeVector {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        ThreeVector::new(x, y, z)
    }
}

impl Sub<ThreeVector> for ThreeVector {
    type Output = ThreeVector;

    fn sub(self, other: ThreeVector) -> ThreeVector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        ThreeVector::new(x, y, z)
    }
}

impl fmt::Display for ThreeVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x , self.y, self.z)
    }
}


pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r, g, b}
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(r: {} g: {} b: {})", self.r, self.g, self.b)
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        let r = self.r / rhs;
        let g = self.g / rhs;
        let b = self.b / rhs;

        Color::new(r, g, b)
    }
}


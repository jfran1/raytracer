use std::ops::{Mul, Sub, Add, Div};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn dot(lhs: &ThreeVector, rhs: &ThreeVector) -> f64 {
        let x = lhs.x * rhs.x;
        let y = lhs.y * rhs.y;
        let z = lhs.z * rhs.z;

        x + y + z 
    }

    pub fn cross(&self, other: ThreeVector) -> ThreeVector {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.z;
        let z = self.x*other.y - self.y*other.x;
        
        ThreeVector::new(x, y , z)
    }

    pub fn unit_vector(self) -> ThreeVector {
        let norm = 1.0/ (self.x * self.x + self.y*self.y + self.z*self.z).sqrt();
        self * norm
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn into_color(self) -> Color {
        let r = self.x;
        let g = self.y;
        let b = self.z;
        Color::new(r, g, b)
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

impl Div<f64> for &ThreeVector {
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

impl Mul<f64> for &ThreeVector {
    type Output = ThreeVector;

    fn mul(self, rhs: f64) -> ThreeVector {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;

        ThreeVector::new(x, y , z)
    }
}

impl Add<ThreeVector> for ThreeVector {
    type Output = ThreeVector;

    fn add(self, other: ThreeVector) -> ThreeVector {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        ThreeVector::new(x, y, z)
    }
}

impl Add<&ThreeVector> for &ThreeVector {
    type Output = ThreeVector;

    fn add(self, rhs: &ThreeVector) -> ThreeVector {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

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

impl Sub<&ThreeVector> for &ThreeVector {
    type Output = ThreeVector;

    fn sub(self, other: &ThreeVector) -> ThreeVector {
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
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }

    pub fn write_color(&self) {
        println!["{}", self]
    }

}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = 255.999;
        let a = (self.r*scale) as i64;
        let b = (self.g*scale) as i64;
        let c = (self.b*scale) as i64;
        write!(f, "{} {} {}", a, b, c)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        let r = self.r / rhs;
        let g = self.g / rhs;
        let b = self.b / rhs;

        Color::new(r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        let r = self.r * rhs;
        let g = self.g * rhs;
        let b = self.b * rhs;

        Color::new(r, g, b)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        let r = self.r + rhs.r;
        let g = self.g + rhs.g;
        let b = self.b + rhs.b;

        Color::new(r, g, b)
    }
}

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Neg;

use super::Trivector;

impl Add for Trivector {

    type Output = Trivector;

    fn add(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl Add<f64> for Trivector {
    type Output = Trivector;
    fn add(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl AddAssign for Trivector {
    fn add_assign(&mut self, rhs: Trivector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }    
}
impl AddAssign<f64> for Trivector {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }    
}
impl Sub for Trivector {
    type Output = Trivector;
    fn sub(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<f64> for Trivector {
    type Output = Trivector;
    fn sub(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
impl SubAssign for Trivector {
    fn sub_assign(&mut self, rhs: Trivector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }    
}
impl SubAssign<f64> for Trivector {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }    
}
impl Mul for Trivector {
    type Output = Trivector;
    fn mul(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Trivector {
    type Output = Trivector;
    fn mul(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Trivector {
    type Output = Trivector;
    fn div(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Trivector {
    type Output = Trivector;
    fn neg(self) -> Trivector {
        Trivector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
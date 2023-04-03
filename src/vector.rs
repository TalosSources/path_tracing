
use rand::{thread_rng, Rng};

pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vec3 {

    pub const ZERO : Vec3 = Vec3 {x : 0.0, y : 0.0, z : 0.0};
    pub const ONE : Vec3 = Vec3 {x: 1.0, y: 1.0, z: 1.0};

    pub fn add(&self, other : &Vec3) -> Vec3{
        Vec3 {x : self.x + other.x, y : self.y + other.y, z : self.z + other.z}
    }

    pub fn scale(&self, s : f64) -> Vec3{
        Vec3 {x : self.x * s, y : self.y * s, z : self.z * s}
    }

    pub fn minus(&self, other : &Vec3) -> Vec3 {
        self.add(&other.scale(-1.0))
    }

    pub fn mult(&self, other : &Vec3) -> Vec3 {
        Vec3 {x : self.x * other.x, y : self.y * other.y, z : self.z * other.z}
    }

    pub fn dot(&self, other:  &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        self.scale(1.0 / self.norm())
    }

    pub fn random_vector_in_unit_cube() -> Vec3 {
        Vec3 {x : thread_rng().gen(), 
                y : thread_rng().gen(), 
                    z : thread_rng().gen()}
            .affine(0.0, 1.0, -1.0, 1.0)
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut vec;
        loop {
            vec = Self::random_vector_in_unit_cube();
            if vec.norm() < 1.0 {
                break;
            }
        }

        vec.normalized()
        //Self::random_vector_in_unit_cube().normalized()
    }

    pub fn random_vector_in_hemisphere(normal : &Vec3) -> Vec3 {
        //algorithm : we take a random vector, and if its not in the hemisphere
        //(i.e. normal.dot(vector) <= 0) we take it's opposite.

        let vec = Self::random_unit_vector();
        vec.scale(normal.dot(&vec).signum())
    }

    pub fn affine(&self, x1: f64, x2: f64, y1: f64, y2: f64) -> Vec3 {
        self.apply_to_each(|x| {
            (y2 - y1) * (x - x1) / (x2 - x1) + y1
        })
    }

    pub fn apply_to_each<F>(&self, closure: F) -> Vec3 where F : Fn(f64) -> f64 {

        Vec3 {
            x: closure(self.x),
            y: closure(self.y),
            z: closure(self.z),
        }
    }
 
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3 {x: self.x, y: self.y, z: self.z}
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

pub struct Mat4 {
    mat : f64
}
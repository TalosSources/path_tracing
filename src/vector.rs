use std::f64::consts::PI;

use rand::{thread_rng, Rng};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn minus(&self, other: &Vec3) -> Vec3 {
        self.add(&other.scale(-1.0))
    }

    pub fn mult(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        self.scale(1.0 / self.norm())
    }

    pub fn random_vector_in_unit_cube() -> Vec3 {
        Vec3 {
            x: thread_rng().gen(),
            y: thread_rng().gen(),
            z: thread_rng().gen(),
        }
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
    }

    pub fn random_vector_in_hemisphere(normal: &Vec3) -> Vec3 {
        //algorithm : we take a random vector, and if its not in the hemisphere
        //(i.e. normal.dot(vector) <= 0) we take it's opposite.

        let vec = Self::random_unit_vector();
        vec.scale(normal.dot(&vec).signum())
    }

    pub fn random_vector_hemisphere_wrong(normal :&Self) -> Self {
        let phi : f64 = 2.0 * PI * thread_rng().gen::<f64>();
        let theta : f64 = 1.0 * PI * thread_rng().gen::<f64>();

        let vec = Vec3 {x : phi.cos() * theta.sin(), y : phi.sin() * theta.sin(), z : theta.cos()};
        vec.scale(normal.dot(&vec).signum())
    }

    pub fn cosine_weighted_hemisphere(normal : &Self) -> Self {
        let phi : f64 = 2.0 * PI * thread_rng().gen::<f64>();
        let theta : f64 = thread_rng().gen::<f64>().sqrt().acos();
        let vec = Vec3 {x : phi.cos() * theta.sin(), y : phi.sin() * theta.sin(), z : theta.cos()};
        vec.rotate_to_face(&normal)
    }

    pub fn rotate_to_face(&self, normal : &Vec3) -> Self {
        /*Everything is symetrical around the z axis.
            So as long as unit  z is aligned with normal, everything good */
        let forward = Vec3 {x: 0.0, y: 0.0, z: 1.0};
        //we align forward with normal
        let angle = forward.dot(normal).acos();
        if angle < 0.001 {
            return self.clone();
        } else if (angle - PI).abs() < 0.001 {
            //println!("here, normal={}, return={}", normal, self.scale(-1.0));
            return self.scale(-1.0)
        }
        let axis = forward.cross(normal).normalized();
        self.rotate_around(&axis, angle)
    }

    pub fn rotate_around(&self, axis : &Vec3, angle : f64) -> Self {
        let cos = angle.cos(); let sin = angle.sin();
        let omc = 1.0 - cos;
        Self {
            x : 
                (cos + axis.x.powf(2.0) * omc) * self.x +
                (axis.x*axis.y * omc - axis.z * sin) * self.y +
                (axis.x*axis.z * omc + axis.y * sin) * self.z,
            y :
                (axis.x*axis.y * omc + axis.z * sin) * self.x + 
                (cos + axis.y.powf(2.0) * omc) * self.y + 
                (axis.y*axis.z * omc - axis.x * sin) * self.z,
            z :
                (axis.x*axis.z*omc - axis.y*sin) * self.x + 
                (axis.z*axis.y*omc + axis.x*sin) * self.y + 
                (cos + axis.z.powf(2.0)*omc) * self.z,
        }
    }

    pub fn affine(&self, x1: f64, x2: f64, y1: f64, y2: f64) -> Vec3 {
        self.apply_to_each(|x| (y2 - y1) * (x - x1) / (x2 - x1) + y1)
    }

    pub fn apply_to_each<F>(&self, closure: F) -> Vec3
    where
        F: Fn(f64) -> f64,
    {
        Vec3 {
            x: closure(self.x),
            y: closure(self.y),
            z: closure(self.z),
        }
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4 {
    pub fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

pub struct Mat4 {
    pub elems: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn product(&self, other: &Mat4) -> Mat4 {
        let mut res = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.elems[i][k] * other.elems[k][j];
                }
                res[i][j] = sum;
            }
        }
        Mat4 { elems: res }
    }

    pub fn apply(&self, other: &Vec4) -> Vec4 {
        let other_ = [other.x, other.y, other.z, other.w];
        let mut res = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.elems[i][j] * other_[j];
            }
            res[i] = sum;
        }

        Vec4 {
            x: res[0],
            y: res[1],
            z: res[2],
            w: res[3],
        }
    }

    pub fn apply_pos3(&self, other: &Vec3) -> Vec4 {
        let v4 = Vec4 {
            x: other.x,
            y: other.y,
            z: other.z,
            w: 1.0,
        };
        self.apply(&v4)
    }

    pub fn apply_dir3(&self, other: &Vec3) -> Vec4 {
        let v4 = Vec4 {
            x: other.x,
            y: other.y,
            z: other.z,
            w: 0.0,
        };
        self.apply(&v4)
    }

    pub fn fromColumns(col_1: &Vec3, col_2: &Vec3, col_3: &Vec3) -> Mat4 {
        let elems = [
            [col_1.x, col_2.x, col_3.x, 0.0],
            [col_1.y, col_2.y, col_3.y, 0.0],
            [col_1.z, col_2.z, col_3.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Mat4 { elems }
    }

    pub fn look_at(dir: &Vec3) -> Mat4 {
        //(0,0,1) -> -dir
        //(1,0,0) needs to be on ground plane -> dir cross up
        //(0,1,0) will be cross of other two

        let col_3 = dir.scale(-1.0);
        let col_1 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
        .cross(&col_3)
        .normalized();
        let col_2 = col_3.cross(&col_1);

        Mat4::fromColumns(&col_1, &col_2, &col_3)
    }
}

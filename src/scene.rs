use crate::render::{Intersection, Primitive, Ray};
use crate::{Mat4, Material, Vec3};

pub struct Parallelogram {
    pub pos: Vec3, //A corner of the rectangle
    v1: Vec3,      //one side of the rectangle, adjacent to the position
    v2: Vec3,      //the other side (both are normalized)
    l1: f64,       //length of sides
    l2: f64,
    normal: Vec3,
    pub mat: &'static Material,
}

impl Parallelogram {
    pub fn new(pos: Vec3, v1: Vec3, v2: Vec3, mat: &'static Material) -> Self {
        Parallelogram {
            pos,
            v1: v1.normalized(),
            v2: v2.normalized(),
            l1: v1.norm(),
            l2: v2.norm(),
            normal: v1.cross(&v2).normalized(),
            mat,
        }
    }
}

impl Primitive for Parallelogram {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mu = -ray.origin.minus(&self.pos).dot(&self.normal) / ray.dir.dot(&self.normal);
        let pos = &ray.origin + &ray.dir.scale(mu);
        let hit_plane_pos = pos.minus(&self.pos);
        let proj_1 = hit_plane_pos.dot(&self.v1);
        let proj_2 = hit_plane_pos.dot(&self.v2);
        let hit =
            0.0 <= proj_1 && proj_1 <= self.l1 && 0.0 <= proj_2 && proj_2 <= self.l2 && mu > 0.0;

        if hit {
            let inside = self.normal.dot(&ray.dir) > 0.0;
            let inter = Intersection {
                hit,
                dist: mu,
                pos,
                normal: self.normal.scale(if inside { -1.0 } else { 1.0 }), //the normal must be opposing incoming ray
                mat: self.mat,
            };
            //println!("inter at plgm : {:?}", inter);
            //println!("normal at plgm : {}, ray.dir : {}", inter.normal, ray.dir);
            inter
        } else {
            Intersection::default()
        }
    }
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub mat: &'static Material,
}

pub struct Plane {
    pub normal: Vec3,
    pub pos: Vec3,
    pub mat: &'static Material,
}

fn cube(pos: Vec3, size: f64, mat: &'static Material) -> [Parallelogram; 6] {
    let X = Vec3 {
        x: size,
        y: 0.0,
        z: 0.0,
    };
    let Y = Vec3 {
        x: 0.0,
        y: size,
        z: 0.0,
    };
    let Z = Vec3 {
        x: 0.0,
        y: 0.0,
        z: size,
    };
    let mX = X.scale(-1.0);
    let mY = Y.scale(-1.0);
    let mZ = Z.scale(-1.0);

    let opp_pos = &pos
        + &Vec3 {
            x: size,
            y: size,
            z: size,
        };

    [
        Parallelogram::new(pos.clone(), Z.clone(), X.clone(), mat), //bottom
        Parallelogram::new(pos.clone(), X.clone(), Y.clone(), mat), //near
        Parallelogram::new(pos.clone(), Y.clone(), Z.clone(), mat), //left
        Parallelogram::new(opp_pos.clone(), mX.clone(), mZ.clone(), mat), //top
        Parallelogram::new(opp_pos.clone(), mY.clone(), mX.clone(), mat), //far
        Parallelogram::new(opp_pos.clone(), mZ.clone(), mY.clone(), mat), //right
    ]
}

pub struct Scene {
    pub objects: Vec<Box<dyn Primitive + Send + Sync>>,
}

impl Scene {
    pub fn _scene_1() -> Scene {
        const MAT_1: Material = Material {
            albedo: Vec3 {
                x: 1.0,
                y: 0.5,
                z: 0.5,
            },
            emissive: Vec3::ZERO,
            roughness: 0.001,
            specular: Vec3::ONE,
            specularity: 0.3,
            fresnel_0: 0.0,
            transparency: 1.0,
            n: 1.2,
        };
        const MAT_2: Material = Material {
            albedo: Vec3 {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            emissive: Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            roughness: 0.3,
            specular: Vec3::ZERO,
            specularity: 0.0,
            fresnel_0: 0.0,
            transparency: 1.0,
            n: 1.2,
        };
        const MAT_3: Material = Material {
            albedo: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            emissive: Vec3::ZERO,
            roughness: 1.0,
            specular: Vec3::ONE,
            specularity: 0.5,
            fresnel_0: 0.0,
            transparency: 1.0,
            n: 1.2,
        };
        const MAT_4: Material = Material {
            albedo: Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            emissive: Vec3::ZERO,
            roughness: 0.15,
            specular: Vec3::ONE,
            specularity: 0.2,
            fresnel_0: 0.0,
            transparency: 1.0,
            n: 1.2,
        };

        const GROUND_MAT: Material = Material {
            albedo: Vec3 {
                x: 0.73,
                y: 0.7,
                z: 0.7,
            },
            emissive: Vec3::ZERO,
            roughness: 0.45,
            specular: Vec3::ONE,
            specularity: 0.3,
            fresnel_0: 0.70,
            transparency: 0.0,
            n: 1.3,
        };
        const WALL_MAT_1: Material = Material {
            albedo: Vec3 {
                x: 0.9,
                y: 0.5,
                z: 0.9,
            },
            emissive: Vec3 {
                x: 0.4,
                y: 0.4,
                z: 0.4,
            },
            roughness: 0.4,
            specular: Vec3::ZERO,
            specularity: 0.0,
            fresnel_0: 0.70,
            transparency: 0.0,
            n: 1.3,
        };
        const WALL_MAT_2: Material = Material {
            albedo: Vec3 {
                x: 0.5,
                y: 0.9,
                z: 0.5,
            },
            emissive: Vec3 {
                x: 0.4,
                y: 0.4,
                z: 0.4,
            },
            roughness: 0.9,
            specular: Vec3::ZERO,
            specularity: 0.0,
            fresnel_0: 0.70,
            transparency: 0.0,
            n: 1.3,
        };

        let spheres = vec![
            Sphere {
                centre: Vec3 {
                    x: 0.2,
                    y: -0.4,
                    z: -2.0 - 1.0,
                },
                radius: 0.5,
                mat: &MAT_1,
            },
            Sphere {
                centre: Vec3 {
                    x: 0.6,
                    y: -0.74,
                    z: -1.5 - 1.0,
                },
                radius: 0.26,
                mat: &MAT_2,
            },
            Sphere {
                centre: Vec3 {
                    x: -0.6,
                    y: 0.3,
                    z: -2.4 - 1.0,
                },
                radius: 0.4,
                mat: &MAT_3,
            },
            Sphere {
                centre: Vec3 {
                    x: -0.4,
                    y: -0.6,
                    z: -1.3 - 1.0,
                },
                radius: 0.4,
                mat: &MAT_4,
            },
        ];

        let planes = vec![
            Plane {
                normal: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                pos: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                mat: &GROUND_MAT,
            },
            Plane {
                normal: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                pos: Vec3 {
                    x: -2.0,
                    y: 0.0,
                    z: 0.0,
                },
                mat: &WALL_MAT_2,
            },
            Plane {
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                pos: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                mat: &WALL_MAT_1,
            },
        ];

        let mut objects: Vec<Box<dyn Primitive + Send + Sync>> = vec![];
        for p in planes {
            objects.push(Box::new(p));
        }
        for s in spheres {
            objects.push(Box::new(s));
        }

        Scene { objects }
    }

    pub fn _scene_2() -> Scene {
        const SPHERE_MAT_1: Material = Material {
            albedo: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            specular: Vec3::ONE,
            specularity: 0.0,
            emissive: Vec3::ZERO,
            roughness: 0.0,
            fresnel_0: 0.3,
            transparency: 1.0,
            n: 1.1,
        };

        const SPHERE_MAT_2: Material = Material {
            albedo: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            specular: Vec3::ONE,
            specularity: 0.1,
            emissive: Vec3::ZERO,
            roughness: 1.0,
            fresnel_0: 0.8,
            transparency: 0.0,
            n: 1.1,
        };

        const PLANE_MAT_1: Material = Material {
            albedo: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            specular: Vec3::ONE,
            specularity: 0.0,
            emissive: Vec3::ZERO,
            roughness: 1.0,
            fresnel_0: 1.0,
            transparency: 0.0,
            n: 1.0,
        };

        const PLANE_MAT_2: Material = Material {
            albedo: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            ..PLANE_MAT_1
        };

        const PLANE_MAT_3: Material = Material {
            albedo: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            ..PLANE_MAT_1
        };

        const EMISSIVE_MAT: Material = Material {
            albedo: Vec3::ZERO,
            specular: Vec3::ZERO,
            specularity: 0.0,
            emissive: Vec3 {
                x: 1.2,
                y: 1.2,
                z: 1.2,
            },
            roughness: 0.0,
            fresnel_0: 0.0,
            transparency: 0.0,
            n: 0.0,
        };

        const EMISSIVE_SPHERE_MAT: Material = Material {
            emissive: Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
            ..EMISSIVE_MAT
        };

        let sphere1 = Sphere {
            centre: Vec3 {
                x: 0.3,
                y: 0.0,
                z: -3.0,
            },
            radius: 0.3,
            mat: &SPHERE_MAT_1,
        };
        let sphere2 = Sphere {
            centre: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            radius: 0.3,
            mat: &SPHERE_MAT_1,
        };
        let sphere3 = Sphere {
            centre: Vec3 {
                x: -0.2,
                y: 0.0,
                z: -1.5,
            },
            radius: 0.3,
            mat: &SPHERE_MAT_1,
        };
        let sphere4 = Sphere {
            centre: Vec3 {
                x: 0.7,
                y: 0.0,
                z: -2.3,
            },
            radius: 0.3,
            mat: &SPHERE_MAT_2,
        };
        let emissive_sphere = Sphere {
            centre: Vec3 {
                x: 0.0,
                y: 0.8,
                z: -0.5,
            },
            radius: 0.3,
            mat: &EMISSIVE_SPHERE_MAT,
        };

        let plane1 = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -8.0,
            },
            mat: &PLANE_MAT_2,
        };
        let plane2 = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            mat: &PLANE_MAT_1,
        };
        let plane3 = Plane {
            normal: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            mat: &PLANE_MAT_3,
        };
        let plane4 = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            mat: &PLANE_MAT_2,
        };
        let plane5 = Plane {
            normal: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            mat: &PLANE_MAT_3,
        };
        let emissive_plane = Plane {
            normal: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 10.0,
                z: 0.0,
            },
            mat: &EMISSIVE_MAT,
        };

        Scene {
            objects: vec![
                Box::new(sphere1),
                Box::new(sphere2),
                Box::new(sphere3),
                Box::new(sphere4),
                Box::new(emissive_sphere),
                Box::new(plane1),
                Box::new(plane2),
                Box::new(plane3),
                Box::new(emissive_plane),
            ],
        }
    }

    pub fn cornell_box() -> Scene {
        const RED_DIFFUSE: Material = Material {
            albedo: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            ..Material::DIFFUSE
        };

        const GREEN_DIFFUSE: Material = Material {
            albedo: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            ..Material::DIFFUSE
        };

        const BLUE_DIFFUSE: Material = Material {
            albedo: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            ..Material::DIFFUSE
        };

        let left = Plane {
            normal: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            mat: &RED_DIFFUSE,
        };
        let right = Plane {
            normal: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            mat: &GREEN_DIFFUSE,
        };
        let ground = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            mat: &Material::DIFFUSE,
        };
        let roof = Plane {
            normal: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            mat: &Material::DIFFUSE,
        }; // TODO : use a smaller light instead
        let far = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            mat: &Material::MIRROR,
        };
        let near = Plane {
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            mat: &Material::DIFFUSE,
        };

        let light_area = Parallelogram::new(
            Vec3 {
                x: -0.5,
                y: 0.99,
                z: -0.5,
            },
            Vec3 {
                x: 1.0,
                ..Vec3::ZERO
            },
            Vec3 {
                z: 1.0,
                ..Vec3::ZERO
            },
            &Material::WHITE_LIGHT,
        );

        let sphere1 = Sphere {
            centre: Vec3 {
                x: -0.4,
                y: -0.3,
                z: -0.7,
            },
            radius: 0.25,
            mat: &Material::DIFFUSE,
        };
        let sphere2 = Sphere {
            centre: Vec3 {
                x: -0.4,
                y: 0.3,
                z: -0.7,
            },
            radius: 0.25,
            mat: &Material::GLOSSY,
        };
        let sphere3 = Sphere {
            centre: Vec3 {
                x: 0.4,
                y: -0.3,
                z: -0.7,
            },
            radius: 0.25,
            mat: &Material::MIRROR,
        };
        let sphere4 = Sphere {
            centre: Vec3 {
                x: 0.0,
                y: -0.6,
                z: -0.3,
            },
            radius: 0.3,
            mat: &Material::FRESNEL_GLASS,
        };
        let sphere5 = Sphere {
            centre: Vec3 {
                x: 0.0,
                y: -0.65,
                z: -0.7,
            },
            radius: 0.25,
            mat: &Material::TOMATO,
        };

        let cube1 = cube(
            Vec3 {
                x: 0.0,
                y: -0.9,
                z: 0.3,
            },
            0.5,
            &Material::FRESNEL_GLASS,
        );

        let square1 = Parallelogram::new(
            Vec3::ZERO,
            Vec3 {
                x: 0.0,
                y: 0.2,
                z: 0.0,
            },
            Vec3 {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
            &BLUE_DIFFUSE,
        );

        let mut objects: Vec<Box<dyn Primitive + Send + Sync>> = vec![
            //Box::new(sphere1),
            //Box::new(sphere2),
            //Box::new(sphere3),
            Box::new(sphere4),
            //Box::new(sphere5),
            Box::new(left),
            Box::new(right),
            Box::new(ground),
            Box::new(roof),
            Box::new(far),
            Box::new(near),
            Box::new(light_area),
        ];

        for p in cube1 {
            objects.push(Box::new(p));
        }

        //objects.push(Box::new(square1));

        Scene { objects }
    }
}

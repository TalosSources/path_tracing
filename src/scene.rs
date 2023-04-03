use crate::Vec3;
use crate::Material;
use crate::render::Object;

pub struct Light {
    pub dir : Vec3,
    pub color : Vec3
}

pub struct Sphere {
    pub centre : Vec3,
    pub radius : f64,
    pub mat :  &'static Material
}

pub struct Plane {
    pub normal : Vec3,
    pub pos : Vec3,
    pub mat : &'static Material
}

pub struct Scene {
    pub spheres : Vec<Sphere>,
    pub lights : Vec<Light>,
    pub planes : Vec<Plane>
}

impl Scene {
    pub fn scene_1() -> Scene {
        const MAT_1 : Material = Material {albedo : Vec3{x:1.0, y:0.5, z:0.5}, emissive : Vec3::ZERO, roughness: 0.001, specular:Vec3::ONE,specularity:0.3, fresnel_0:0.0, transparency:1.0, n:1.2};
        const MAT_2: Material = Material {albedo : Vec3{x:0.5, y:1.0, z:0.5}, emissive : Vec3{x:5.0, y:5.0, z:5.0}, roughness: 0.3, specular:Vec3::ZERO,specularity:0.0, fresnel_0:0.0, transparency:1.0, n:1.2};
        const MAT_3: Material = Material {albedo : Vec3{x:0.5, y:0.5, z:1.0}, emissive : Vec3::ZERO, roughness: 1.0, specular:Vec3::ONE,specularity:0.5, fresnel_0:0.0, transparency:1.0, n:1.2};
        const MAT_4: Material = Material {albedo : Vec3{x: 0.8, y: 0.8, z: 0.8}, emissive : Vec3::ZERO, roughness: 0.15, specular:Vec3::ONE,specularity:0.2, fresnel_0:0.0, transparency:1.0, n:1.2};

        const GROUND_MAT: Material = Material {albedo : Vec3{x: 0.73, y: 0.7, z: 0.7}, emissive: Vec3::ZERO, roughness : 0.45, specular:Vec3::ONE, specularity:0.3, fresnel_0 : 0.70, transparency: 0.0, n:1.3};
        const WALL_MAT_1: Material = Material {albedo : Vec3{x: 0.9, y: 0.5, z: 0.9}, emissive: Vec3{x:0.4, y:0.4, z:0.4}, roughness : 0.4, specular:Vec3::ZERO, specularity:0.0, fresnel_0 : 0.70, transparency: 0.0, n:1.3};
        const WALL_MAT_2: Material = Material {albedo : Vec3{x: 0.5, y: 0.9, z: 0.5}, emissive: Vec3{x:0.4, y:0.4, z:0.4}, roughness : 0.9, specular:Vec3::ZERO, specularity:0.0, fresnel_0 : 0.70, transparency: 0.0, n:1.3};
    
        let spheres = vec![
            Sphere {centre : Vec3{x:0.2, y:-0.4, z:-2.0  -1.0}, radius : 0.5, mat: &MAT_1},
            Sphere {centre : Vec3{x:0.6, y:-0.74, z:-1.5  -1.0}, radius : 0.26, mat: &MAT_2},
            Sphere {centre : Vec3{x:-0.6, y:0.3, z:-2.4  -1.0}, radius : 0.4, mat: &MAT_3},
            Sphere {centre : Vec3{x: -0.4, y: -0.6, z:-1.3  -1.0}, radius : 0.4, mat: &MAT_4}
        ];

        let planes = vec![
            Plane {normal : Vec3{x:0.0, y:1.0,z:0.0}, pos : Vec3{x: 0.0, y: -1.0, z: 0.0}, mat: &GROUND_MAT},
            Plane {normal : Vec3{x:1.0, y:0.0,z:0.0}, pos : Vec3{x: -2.0, y: 0.0, z: 0.0}, mat: &WALL_MAT_2},
            Plane {normal : Vec3{x:0.0, y:0.0,z:1.0}, pos : Vec3{x: 0.0, y: 0.0, z: -5.0}, mat: &WALL_MAT_1},
        ];
    
        let light_1 = Light{
            dir : Vec3{x: 5.0, y: 1.0, z: 1.0}.normalized(),
            color : Vec3 {x:1.4, y:1.3, z: 1.2}.scale(21.0) 
        };
    
        let light_2 = Light{
            dir : Vec3{x: -5.0, y: 1.0, z: 1.0}.normalized(),
            color : Vec3 {x:1.2, y:1.3, z: 1.4}.scale(21.0)
        };
    
        let light_3 = Light{
            dir : Vec3{x: -0.4, y: 2.0, z: -2.0}.normalized(),
            color : Vec3 {x:1.3, y:1.3, z: 1.3}.scale(21.0)
        };
    
        Scene { spheres : spheres, lights : vec![light_1, light_2, light_3], planes: planes}
    } 

    pub fn scene_2() -> Scene {

        const SPHERE_MAT_1 : Material = Material {
            albedo : Vec3{x:1.0, y:1.0, z:1.0},
            specular : Vec3::ONE,
            specularity : 0.0,
            emissive : Vec3::ZERO,
            roughness : 0.0,
            fresnel_0 : 0.3,
            transparency : 1.0,
            n : 1.1
        };

        const SPHERE_MAT_2 : Material = Material {
            albedo : Vec3{x:0.0, y:0.0, z:1.0},
            specular : Vec3::ONE,
            specularity : 0.1,
            emissive : Vec3::ZERO,
            roughness : 1.0,
            fresnel_0 : 0.8,
            transparency : 0.0,
            n : 1.1
        };

        const PLANE_MAT_1 : Material = Material {
            albedo : Vec3{x:1.0, y:1.0, z:1.0},
            specular : Vec3::ONE,
            specularity : 0.0,
            emissive : Vec3::ZERO,
            roughness : 1.0,
            fresnel_0 : 1.0,
            transparency : 0.0,
            n : 1.0
        };

        const PLANE_MAT_2 : Material = Material {
            albedo : Vec3{x:1.0, y:0.0, z:0.0},
            ..PLANE_MAT_1
        };

        const PLANE_MAT_3 : Material = Material {
            albedo : Vec3{x:0.0,y:1.0,z:0.0},
            ..PLANE_MAT_1
        };

        const EMISSIVE_MAT : Material = Material {
            albedo : Vec3::ZERO,
            specular : Vec3::ZERO,
            specularity : 0.0,
            emissive : Vec3{x:1.2, y:1.2,z:1.2},
            roughness : 0.0,
            fresnel_0 : 0.0,
            transparency : 0.0,
            n : 0.0
        };

        const EMISSIVE_SPHERE_MAT : Material = Material {
            emissive : Vec3{x:3.0, y:3.0, z:3.0},
            ..EMISSIVE_MAT
        };

        let light1 = Light {dir : Vec3{x:3.0, y:1.0, z:1.0}, color : Vec3::ONE.scale(1.5)};
        let light2 = Light {dir : Vec3{x:-3.0, y:1.0, z:1.0}, color : Vec3::ONE.scale(1.3)};

        let sphere1 = Sphere {centre : Vec3{x: 0.3, y: 0.0, z: -3.0}, radius : 0.3, mat : &SPHERE_MAT_1};
        let sphere2 = Sphere {centre : Vec3{x: 0.0, y: 0.0, z: -2.0}, radius : 0.3, mat : &SPHERE_MAT_1};
        let sphere3 = Sphere {centre : Vec3{x: -0.2, y: 0.0, z: -1.5}, radius : 0.3, mat : &SPHERE_MAT_1};
        let sphere4 = Sphere {centre : Vec3{x: 0.7, y: 0.0, z: -2.3}, radius : 0.3, mat : &SPHERE_MAT_2};
        let emissive_sphere = Sphere {centre: Vec3{x:0.0, y:0.8, z:-0.5}, radius:0.3, mat: &EMISSIVE_SPHERE_MAT};

        let plane1 = Plane {normal : Vec3{x:0.0, y:0.0, z:1.0}, pos : Vec3{x:0.0, y:0.0, z:-8.0}, mat : &PLANE_MAT_2};
        let plane2 = Plane {normal : Vec3{x:0.0, y:1.0, z:0.0}, pos : Vec3{x:0.0, y:-1.0, z:0.0}, mat : &PLANE_MAT_1};
        let plane3 = Plane {normal : Vec3{x:1.0, y:0.0, z:0.0}, pos : Vec3{x:-1.0, y:0.0, z:0.0}, mat : &PLANE_MAT_3};
        let plane4 = Plane {normal : Vec3{x:0.0, y:0.0, z:-1.0}, pos : Vec3{x:0.0, y:0.0, z:1.0}, mat : &PLANE_MAT_2};
        let plane5 = Plane {normal : Vec3{x:-1.0, y:0.0, z:0.0}, pos : Vec3{x:2.0, y:0.0, z:0.0}, mat : &PLANE_MAT_3};
        let emissive_plane = Plane {normal : Vec3{x:0.0, y:-1.0, z:0.0}, pos: Vec3{x:0.0, y:10.0, z:0.0}, mat : &EMISSIVE_MAT};

        Scene{spheres : vec![sphere1, sphere2, sphere3, sphere4, emissive_sphere], lights : vec![light1/*, light2*/], planes: vec![plane1, plane2, plane3, /*plane4, plane5,*/ emissive_plane]}
    }

    pub fn cornell_box() -> Scene {

        const RED_DIFFUSE : Material = Material {
            albedo : Vec3 {x: 1.0, y: 0.0, z: 0.0},
            ..Material::DIFFUSE
        };

        const GREEN_DIFFUSE : Material = Material {
            albedo : Vec3 {x: 0.0, y: 1.0, z: 0.0},
            ..Material::DIFFUSE
        };

        const BLUE_DIFFUSE : Material = Material {
            albedo : Vec3 {x: 0.0, y: 0.0, z: 1.0},
            ..Material::DIFFUSE
        };

        let left = Plane {normal : Vec3{x:1.0, y:0.0, z:0.0}, pos : Vec3{x:-1.0, y:0.0, z:0.0}, mat : &RED_DIFFUSE};
        let right = Plane {normal : Vec3{x:-1.0, y:0.0, z:0.0}, pos : Vec3{x:1.0, y:0.0, z:0.0}, mat : &GREEN_DIFFUSE};
        let ground = Plane {normal : Vec3{x:0.0, y:1.0, z:0.0}, pos : Vec3{x:0.0, y:-1.0, z:0.0}, mat : &Material::DIFFUSE};
        let roof = Plane {normal : Vec3{x:0.0, y:-1.0, z:0.0}, pos : Vec3{x:0.0, y:1.0, z:0.0}, mat : &Material::WHITE_LIGHT}; // TODO : use a smaller light instead
        let far = Plane {normal : Vec3{x:0.0, y:0.0, z:1.0}, pos : Vec3{x:0.0, y:0.0, z:-1.0}, mat : &Material::MIRROR};
        let near = Plane {normal : Vec3{x:0.0, y:0.0, z:-1.0}, pos : Vec3{x:0.0, y:0.0, z:1.0}, mat : &Material::DIFFUSE};

        let sphere1 = Sphere {centre : Vec3{x: -0.4, y: -0.3, z: -0.7}, radius : 0.25, mat : &Material::DIFFUSE};
        let sphere2 = Sphere {centre : Vec3{x: -0.4, y: 0.3, z: -0.7}, radius : 0.25, mat : &Material::GLOSSY};
        let sphere3 = Sphere {centre : Vec3{x: 0.4, y: -0.3, z: -0.7}, radius : 0.25, mat : &Material::MIRROR};
        let sphere4 = Sphere {centre : Vec3{x: 0.4, y: 0.3, z: -0.7}, radius : 0.25, mat : &Material::FRESNEL_GLASS};
        let sphere5 = Sphere {centre : Vec3{x: 0.0, y: -0.65, z: -0.7}, radius : 0.25, mat : &Material::TOMATO};


        Scene{spheres : vec![sphere1, sphere2, sphere3, sphere4, sphere5], lights : vec![], planes: vec![left, right, ground, roof, far, near]}

    }
}
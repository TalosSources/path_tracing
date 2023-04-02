
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use image::ImageBuffer;
use image::Rgb;

mod vector;
use vector::Vec3;

use rand::{thread_rng, Rng};

struct Scene {
    spheres : Vec<Sphere>,
    lights : Vec<Light>,
    planes : Vec<Plane>
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
}

struct Light {
    dir : Vec3,
    color : Vec3
}

struct Sphere {
    centre : Vec3,
    radius : f64,
    mat :  &'static Material
}

struct Plane {
    normal : Vec3,
    pos : Vec3,
    mat : &'static Material
}

struct Material {
    albedo : Vec3,
    specular: Vec3,
    specularity: f64,
    emissive : Vec3,
    roughness : f64,
    fresnel_0 : f64,
    transparency : f64,
    n : f64
}

impl Material {

    const DEFAULT_MAT: Material = Material {albedo : Vec3::ZERO, emissive : Vec3::ZERO, roughness : 0.2, specular:Vec3::ZERO, specularity:0.0, fresnel_0 : 0.70, transparency: 0.0, n:1.3};

    const N_AIR : f64 = 1.0;

    fn default() -> &'static Material {
        &Material::DEFAULT_MAT
    }

}

struct Ray {
    origin : Vec3,
    dir : Vec3,
    color : Vec3,
    emitted : Vec3,
    n : f64
}

struct Intersection<'a> {
    hit : bool,
    dist : f64,
    pos : Vec3,
    normal : Vec3,
    mat : &'a Material
}

impl <'a> Default for Intersection<'a> {
    fn default() -> Intersection<'a> {
        Intersection {
            hit : false,
            dist : 0.0,
            pos : Vec3::ZERO,
            normal : Vec3::ZERO,
            mat : Material::default()
        }
    }
}

struct Context {
    scene : Scene,
    width : u32,
    height : u32,
    focal_length : f64
}

fn intersect<'a>(ray : &Ray, scene : &'a Scene) -> Intersection<'a> {

    //let mut inter = Intersection::default();

    let mut inter = Intersection::default();
    let mut hit = false;
    let mut dist = 0.0;

    for sphere in &scene.spheres {
        let s_inter = intersect_sphere(ray, &sphere);
        if s_inter.hit && (s_inter.dist < dist || !hit) {
            hit = true;
            dist = s_inter.dist;
            inter = s_inter;
        }
    }

    for plane in &scene.planes {
        let s_inter = intersect_plane(ray, &plane);
        if s_inter.hit && (s_inter.dist < dist || !hit) {
            hit = true;
            dist = s_inter.dist;
            inter = s_inter;
        }
    }

    inter
}

fn intersect_sphere<'a>(ray : &Ray, sphere : &'a Sphere) -> Intersection<'a> {

    let dp : Vec3 = ray.origin.minus(&sphere.centre);  
    let dir_dot_dp = ray.dir.dot(&dp);
    let discr = dir_dot_dp.powf(2.0) - dp.norm().powf(2.0) + sphere.radius.powf(2.0);

    if discr >= 0.0 {
        let mut dist = -dir_dot_dp - discr.sqrt();
        let mut inside = false;
        if dist < 0.0 {
            dist = -dir_dot_dp + discr.sqrt();
            inside = true;
        }
        let hit = dist > 0.0;
        if hit {
            let pos = ray.origin.add(&ray.dir.scale(dist));
            let normal = pos.minus(&sphere.centre).normalized();
            return Intersection {
                hit : hit,
                dist : dist,
                pos : pos,
                normal : normal.scale(if inside {-1.0} else {1.0}),
                mat: &sphere.mat
            };
        }
    }

    Intersection::default()
}

fn intersect_plane<'a>(ray : &Ray, plane : &Plane) -> Intersection<'a> {

    let mu = -ray.origin.minus(&plane.pos).dot(&plane.normal) / ray.dir.dot(&plane.normal);

    Intersection {
        hit : mu > 0.0,
        dist : mu,
        pos : ray.origin.add(&ray.dir.scale(mu)),
        normal : plane.normal.clone(),
        mat : plane.mat,
    }

}   

fn reflect(dir : &Vec3, normal : &Vec3, roughness: f64) -> Vec3 {
    let reflected_dir = dir.minus(&normal.scale(2.0 * normal.dot(&dir))).normalized();
    
    let random_dir = &Vec3::random_vector_in_hemisphere(normal);

    reflected_dir.scale(1.0 - roughness).add(&random_dir.scale(roughness)).normalized()
}

fn refract(dir : &Vec3, normal : &Vec3, n1 : f64, n2 : f64) -> Vec3 {
    let normal = normal.scale(-1.0);
    let nd = normal.dot(&dir);
    let sqr_root = (nd.powf(2.0) + (n2/n1).powf(2.0) - 1.0).sqrt();

    normal.scale(sqr_root - nd).add(dir).scale(n1/n2).normalized()
    //Vec3{x:dir.x, y:dir.y, z:dir.z}
}

fn pixel_shader(ctx : &Context, i : u32, j : u32, bounces : u8, samples_per_pixel: u32) -> Rgb<u8> {

    //for now, we use orthographic projection in the -1 to 1 window, looking at z
    let x = 2.0 * (i as f64) / (ctx.width as f64) - 1.0;
    let y = 2.0 * ((ctx.height - j - 1) as f64) / (ctx.height as f64) - 1.0;

    //let ray = Ray{origin : Vec3{x : x, y : y, z : 0.0}, dir : Vec3{x : 0.0, y : 0.0, z : -1.0}}; //ORTHOGRAPHIC PROJETION

    let mut acc_color = Vec3::ZERO;
    for _ in 0..samples_per_pixel {

        let mut ray = Ray{ 
            origin : Vec3::ZERO, 
            dir : Vec3 {x : x, y : y, z : -ctx.focal_length}.normalized(),
            color : Vec3::ONE,
            emitted : Vec3::ZERO,
            n : Material::N_AIR
        };

        let mut iter = 0;
        loop {
        
            let int = intersect(&ray, &ctx.scene);
            if int.hit {
                //let intensity = int.normal.dot(&ctx.scene.light_dir);
                //let col = (255.0 * intensity) as u8;
                //let col = int.mat.albedo.scale(intensity);

                let dotp = -int.normal.dot(&ray.dir);
                let k_fresnel = int.mat.fresnel_0 + (1.0 - int.mat.fresnel_0) * (1.0 - dotp).powf(5.0);
    
                ray.emitted = ray.emitted.add(&int.mat.emissive.mult(&ray.color));

                //dir stuff TODO refactor this mess
                let is_specular_bounce = thread_rng().gen::<f64>() < int.mat.specularity;
                if is_specular_bounce {
                    ray.dir = reflect(&ray.dir, &int.normal, 0.0);
                    ray.color = ray.color.mult(&int.mat.specular).scale(k_fresnel);
                } else {
                    let is_refraction = thread_rng().gen::<f64>() < (1.0-k_fresnel);
                    if is_refraction {
                        let next_n;
                        if ray.n == int.mat.n {
                            next_n = Material::N_AIR;
                        } else {
                            next_n = int.mat.n;
                        }
                        ray.dir = refract(&ray.dir, &int.normal, ray.n, next_n);
                        ray.n = next_n;
                        let scaler = int.mat.transparency;
                        let new_color = ray.color.mult(&int.mat.albedo).scale(scaler);
                        //println!("old_color = {}, new color = {}", ray.color, new_color);
                        ray.color = new_color;
                        ray.origin = int.pos.add(&int.normal.scale(-0.001));
                    } else {
                        ray.dir = reflect(&ray.dir, &int.normal, int.mat.roughness);
                        ray.color = ray.color.mult(&int.mat.albedo);
                        ray.origin = int.pos.add(&int.normal.scale(0.001));
                    }
                }

            } else {
                //let sky_color = Vec3 {x:0.8, y:0.8, z:1.0};
                //let mut light_component = Vec3::ZERO;
                //for light in &ctx.scene.lights {
                //    let scalar = ray.dir.dot(&light.dir);
                //    if scalar > 0.9 { //simulates a circular emmissive surface in the sky for the sun
                //        light_component = light_component.add(
                //            &light.color.scale(ray.dir.dot(&light.dir))
                //        );
                //    }
                //}
                
                //ray.emitted = ray.emitted.add(&light_component.mult(&ray.color));
                break;
            }

            iter += 1;
            if iter > bounces {
                break;
            }
        }

        acc_color = acc_color.add(&ray.emitted);
    }
    
    acc_color = acc_color.scale(1.0 / (samples_per_pixel as f64));

    let f_to_u8 = |f: f64| (255.0 * f.max(0.0).min(1.0)) as u8;
    return Rgb([f_to_u8(acc_color.x), f_to_u8(acc_color.y), f_to_u8(acc_color.z)]);

}

fn main() {
    
    let (width, height) : (u32, u32) = (1000, 1000);

    let ctx = Context {scene : Scene::scene_2(), width : width, height : height, focal_length : 2.0};
    let img : image::RgbImage = ImageBuffer::new(width, height);

    let img_ref = Arc::new(Mutex::new(img));
    let ctx_ref = Arc::new(ctx);

    let mut threads = vec![];
    let n_threads = 8;

    let loading_counter = Arc::new(Mutex::new(0)); //we increment for every column. percentage is this counter over the width. 

    for t in 0..n_threads {
        let img_clone = Arc::clone(&img_ref);
        let ctx_clone = Arc::clone(&ctx_ref);
        let counter_clone = Arc::clone(&loading_counter);
        //we go from t * width / n_threads to (t+1) * width / n_threads
        let low = t * width / n_threads;
        let up = (t+1) * width / n_threads;

        let new_thread = thread::spawn(
            move || {
                for i in low..up {
                    for j in 0..height {
                        let pixel = pixel_shader(&ctx_clone, i, j, 7, 500);
                        img_clone.lock().unwrap().put_pixel(i, j, pixel);
                    }
                    let mut ref_ = counter_clone.lock().unwrap();

                    *ref_ += 1;
                    println!("{}%", 100.0 * (*ref_ as f32) / (width as f32));
                }
            }
        );
        threads.push(new_thread);
    }
    for t in threads {
        t.join().unwrap();
    }

    img_ref.lock().unwrap().save("render5.png").expect("Couldn't save image");

}

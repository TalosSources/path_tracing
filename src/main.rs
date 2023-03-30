use std::thread;

use image::ImageBuffer;
use image::Rgb;

mod vector;
use vector::Vec3;

use rand::{thread_rng, Rng};

struct Scene<'a> {
    spheres : Vec<Sphere<'a>>,
    lights : Vec<Light>
}

struct Light {
    dir : Vec3,
    color : Vec3
}

struct Sphere<'a> {
    centre : Vec3,
    radius : f64,
    mat :  &'a Material
}

struct Material {
    albedo : Vec3,
    emissive : Vec3,
    roughness : f64
}

impl Material {

    const DEFAULT_MAT: Material = Material {albedo : Vec3::ZERO, emissive : Vec3::ZERO, roughness : 0.2};
    const GROUND_MAT: Material = Material {albedo : Vec3{x: 0.75, y: 0.35, z: 0.7}, emissive: Vec3::ZERO, roughness : 1.0};

    fn default() -> &'static Material {
        &Material::DEFAULT_MAT
    }

}

struct Ray {
    origin : Vec3,
    dir : Vec3,
    color : Vec3
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
        //let mat = Material{albedo:Vec3::zero()};
        Intersection {
            hit : false,
            dist : 0.0,
            pos : Vec3::ZERO,
            normal : Vec3::ZERO,
            mat : Material::default()
        }
    }
}

struct Context<'a> {
    scene : Scene<'a>,
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

    let s_inter = intersect_ground_plane(ray);
    if s_inter.hit && (s_inter.dist < dist || !hit) {
        inter = s_inter;
    }

    inter
}

fn intersect_sphere<'a>(ray : &Ray, sphere : &'a Sphere) -> Intersection<'a> {

    let dp : Vec3 = ray.origin.minus(&sphere.centre);  
    let dir_dot_dp = ray.dir.dot(&dp);
    let discr = dir_dot_dp.powf(2.0) - dp.norm().powf(2.0) + sphere.radius.powf(2.0);

    if discr >= 0.0 {
        let dist = -dir_dot_dp - discr.sqrt();
        let hit = dist > 0.0;
        if hit {
            let pos = ray.origin.add(&ray.dir.scale(dist));
            let normal = pos.minus(&sphere.centre).normalized();
            return Intersection {
                hit : hit,
                dist : dist,
                pos : pos,
                normal : normal,
                mat: &sphere.mat
            };
        }
    }

    Intersection::default()
}

fn intersect_ground_plane(ray : &Ray) -> Intersection<'static> {
    /*
        Intersection with ground plane : p = ray.origin + ray.dir * mu. we know p.z = -1 = ray.origin.z + ray.dir.z * mu
        => mu = -ray.origin.z / ray.dir.z
    */
    let mu = (-1.0 -ray.origin.y) / ray.dir.y;

    Intersection { hit: mu > 0.0, dist: mu, pos: ray.origin.add(&ray.dir.scale(mu)), normal: Vec3{x:0.0, y:1.0,z:0.0}, mat: &Material::GROUND_MAT }
}

fn reflect(dir : Vec3, normal : Vec3, roughness: f64) -> Vec3 {
    dir
        .minus(&normal.scale(2.0 * normal.dot(&dir)))
            .add(&Vec3::random_vector()
                .affine(0.0, 1.0, -1.0, 1.0)
                    .scale(roughness))
                        .normalized()
}

fn pixel_shader(ctx : &Context, i : u32, j : u32) -> Rgb<u8> {

    //for now, we use orthographic projection in the -1 to 1 window, looking at z
    let x = 2.0 * (i as f64) / (ctx.width as f64) - 1.0;
    let y = 2.0 * ((ctx.height - j - 1) as f64) / (ctx.height as f64) - 1.0;

    //let ray = Ray{origin : Vec3{x : x, y : y, z : 0.0}, dir : Vec3{x : 0.0, y : 0.0, z : -1.0}}; //ORTHOGRAPHIC PROJETION

    let mut acc_color = Vec3::ZERO;
    let iters = 20;
    for _ in 0..iters {

        let mut ray = Ray{ 
            origin : Vec3::ZERO, 
            dir : Vec3 {x : x, y : y, z : -ctx.focal_length}.normalized(),
            color : Vec3{x:1.0, y:1.0,z:1.0}
        };

        let mut iter = 0;
        loop {
        
            let int = intersect(&ray, &ctx.scene);
            if int.hit {
                //let intensity = int.normal.dot(&ctx.scene.light_dir);
                //let col = (255.0 * intensity) as u8;
                //let col = int.mat.albedo.scale(intensity);
    
                ray.origin = int.pos.add(&int.normal.scale(0.001));
                ray.dir = reflect(ray.dir, int.normal, int.mat.roughness);
                let p = int.mat.emissive.norm() / int.mat.albedo.norm(); //proba of having been emitted
                if p > thread_rng().gen() {
                    ray.color = ray.color.mult(&int.mat.emissive);
                    break;
                } else {
                    ray.color = ray.color.mult(&int.mat.albedo);
                }
            
            } else {
                let sky_color = Vec3 {x:0.8, y:0.8, z:1.0};
                let mut light_component = Vec3::ZERO;
                for light in &ctx.scene.lights {
                    light_component = light_component.add(
                        &light.color.scale(ray.dir.dot(&light.dir))
                    )
                }
                
                ray.color = ray.color
                    .mult(&light_component); // sky color
                break;
            }

            iter += 1;
            if iter > 5 {
                break;
            }
        }

        acc_color = acc_color.add(&ray.color);
    }
    
    acc_color = acc_color.scale(1.0 / (iters as f64));

    let f_to_u8 = |f: f64| (255.0 * f.max(0.0).min(1.0)) as u8;
    return Rgb([f_to_u8(acc_color.x), f_to_u8(acc_color.y), f_to_u8(acc_color.z)]);

}

fn main() {
    
    let (width, height) : (u32, u32) = (1000, 1000);

    let mat_1 = Material {albedo : Vec3{x:1.0, y:0.5, z:0.5}, emissive : Vec3::ZERO, roughness: 0.001};
    let mat_2 = Material {albedo : Vec3{x:0.5, y:1.0, z:0.5}, emissive : Vec3{x:0.5, y:0.5, z:0.5}, roughness: 0.3};
    let mat_3 = Material {albedo : Vec3{x:0.5, y:0.5, z:1.0}, emissive : Vec3::ZERO, roughness: 0.8};
    let mat_4 = Material {albedo : Vec3{x: 0.8, y: 0.8, z: 0.8}, emissive : Vec3::ZERO, roughness: 0.15};

    let spheres = vec![
        Sphere {centre : Vec3{x:0.4, y:-0.4, z:-2.0  -1.0}, radius : 0.5, mat: &mat_1},
        Sphere {centre : Vec3{x:0.0, y:0.2, z:-1.5  -1.0}, radius : 0.26, mat: &mat_2},
        Sphere {centre : Vec3{x:-0.5, y:0.3, z:-2.4  -1.0}, radius : 0.4, mat: &mat_3},
        Sphere {centre : Vec3{x: -0.3, y: -0.6, z:-1.3  -1.0}, radius : 0.4, mat: &mat_4}
    ];

    let light_1 = Light{
        dir : Vec3{x: 5.0, y: 1.0, z: 1.0}.normalized(),
        color : Vec3 {x:1.5, y:1.3, z: 1.0} 
    };

    let light_2 = Light{
        dir : Vec3{x: -5.0, y: 1.0, z: 1.0}.normalized(),
        color : Vec3 {x:1.0, y:1.3, z: 1.5} 
    };

    let light_3 = Light{
        dir : Vec3{x: -0.4, y: 2.0, z: -2.0}.normalized(),
        color : Vec3 {x:1.3, y:1.3, z: 1.3} 
    };

    let scene = Scene { spheres : spheres, lights : vec![light_1, light_2, light_3]};

    let ctx = Context {scene : scene, width : width, height : height, focal_length : 2.0};

    let mut img : image::RgbImage = ImageBuffer::new(width, height);

    /*let mut threads  = vec![];
    for i in 0..8 {
        let new_thread = thread::spawn( move || {
            println!("hey");
            }
        );
        threads.push(new_thread);
    }*/

    for i in 0..width {
        for j in 0..height {
            let pixel = pixel_shader(&ctx, i, j);
            img.put_pixel(i, j, pixel);
        }
    }

    img.save("render.png").expect("Couldn't save image");

}

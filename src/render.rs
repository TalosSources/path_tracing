use crate::scene::{Plane, Scene, Sphere};
use crate::vector::{Mat4, Vec3};
use crate::Material;

use image::Rgb;
use rand::{thread_rng, Rng};

pub struct Context {
    pub scene: Scene,
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
}

pub struct Camera {
    pub focal_length: f64,
    pub pos: Vec3,
    pub rot: Mat4,
}

impl Camera {
    pub fn new(pos: Vec3, target: Vec3, focal_length: f64) -> Camera {
        Camera {
            focal_length,
            rot: Mat4::look_at(&pos, &target),
            pos,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub color: Vec3,
    pub emitted: Vec3,
    pub n: f64,
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub hit: bool,
    pub dist: f64,
    pub pos: Vec3,
    pub normal: Vec3,
    pub mat: &'a Material,
}

impl<'a> Default for Intersection<'a> {
    fn default() -> Intersection<'a> {
        Intersection {
            hit: false,
            dist: 0.0,
            pos: Vec3::ZERO,
            normal: Vec3::ZERO,
            mat: Material::default(),
        }
    }
}

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Intersection;
}

fn intersect<'a>(ray: &Ray, scene: &'a Scene) -> Intersection<'a> {
    //let mut inter = Intersection::default();

    let mut inter = Intersection::default();
    let mut hit = false;
    let mut dist = 0.0;

    for object in &scene.objects {
        let s_inter = object.intersect(&ray);
        if s_inter.hit && (s_inter.dist < dist || !hit) {
            hit = true;
            dist = s_inter.dist;
            inter = s_inter;
        }
    }

    inter
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let dp: Vec3 = ray.origin.minus(&self.centre);
        let dir_dot_dp = ray.dir.dot(&dp);
        let discr = dir_dot_dp.powf(2.0) - dp.norm().powf(2.0) + self.radius.powf(2.0);

        if discr >= 0.0 {
            let mut dist = -dir_dot_dp - discr.sqrt();
            let mut inside = false;
            if dist < 0.0 {
                dist = -dir_dot_dp + discr.sqrt();
                inside = true;
            }
            let hit = dist > 0.0;
            if hit {
                //let pos = ray.origin.add(&ray.dir.scale(dist));
                let pos = &ray.origin + &(&ray.dir * dist);
                let normal = pos.minus(&self.centre).normalized();
                return Intersection {
                    hit,
                    dist,
                    pos,
                    normal: normal.scale(if inside { -1.0 } else { 1.0 }),
                    mat: &self.mat,
                };
            }
        }

        Intersection::default()
    }
}

impl Primitive for Plane {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mu = -ray.origin.minus(&self.pos).dot(&self.normal) / ray.dir.dot(&self.normal);

        Intersection {
            hit: mu > 0.0,
            dist: mu,
            pos: &ray.origin + &ray.dir.scale(mu),
            normal: self.normal.clone(),
            mat: self.mat,
        }
    }
}

fn reflect(dir: &Vec3, normal: &Vec3, roughness: f64) -> Vec3 {
    let reflected_dir = dir
        .minus(&normal.scale(2.0 * normal.dot(&dir)))
        .normalized();

    //let random_dir = &Vec3::random_vector_in_hemisphere(normal);
    let random_dir = &Vec3::cosine_weighted_hemisphere(normal);

    (&(&reflected_dir * (1.0 - roughness)) + &random_dir.scale(roughness)).normalized()
}

fn refract(dir: &Vec3, normal: &Vec3, n1: f64, n2: f64) -> Vec3 {
    let normal = normal.scale(-1.0);
    let nd = normal.dot(&dir);
    let sqr_root = (nd.powf(2.0) + (n2 / n1).powf(2.0) - 1.0).sqrt();

    (&(&(&normal * (sqr_root - nd)) + dir) * (n1 / n2)).normalized()
}

pub fn pixel_shader(ctx: &Context, i: u32, j: u32, bounces: u8, samples_per_pixel: u32) -> Rgb<u8> {
    //for now, we use orthographic projection in the -1 to 1 window, looking at z
    let x = 2.0 * (i as f64) / (ctx.width as f64) - 1.0;
    let y = 2.0 * ((ctx.height - j - 1) as f64) / (ctx.height as f64) - 1.0;

    //let ray = Ray{origin : Vec3{x : x, y : y, z : 0.0}, dir : Vec3{x : 0.0, y : 0.0, z : -1.0}}; //ORTHOGRAPHIC PROJETION

    let mut acc_color = Vec3::ZERO;
    let cam_dir = ctx
        .camera
        .rot
        .apply_dir3(
            &Vec3 {
                x,
                y,
                z: -ctx.camera.focal_length,
            }
            .normalized(),
        )
        .as_vec3();
    for _ in 0..samples_per_pixel {
        let mut ray = Ray {
            origin: ctx.camera.pos.clone(),
            dir: cam_dir.clone(),
            color: Vec3::ONE,
            emitted: Vec3::ZERO,
            n: Material::N_AIR,
        };

        let mut iter = 0;
        loop {
            let int = intersect(&ray, &ctx.scene);
            if int.hit {
                let dotp = -int.normal.dot(&ray.dir);
                let k_fresnel =
                    int.mat.fresnel_0 + (1.0 - int.mat.fresnel_0) * (1.0 - dotp).powf(5.0);

                ray.emitted = &ray.emitted + &(&int.mat.emissive.mult(&ray.color));

                //dir stuff TODO refactor this mess
                let is_specular_bounce = thread_rng().gen::<f64>() < int.mat.specularity;
                if is_specular_bounce {
                    ray.dir = reflect(&ray.dir, &int.normal, 0.0);
                    ray.color = ray.color.mult(&int.mat.specular).scale(k_fresnel);
                } else {
                    let is_refraction = thread_rng().gen::<f64>() < (1.0 - k_fresnel);
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
                        ray.color = new_color;
                        ray.origin = &int.pos + &int.normal.scale(-0.001);
                    } else {
                        ray.dir = reflect(&ray.dir, &int.normal, int.mat.roughness);
                        //let cos_theta = ray.dir.dot(&int.normal);
                        ray.color = ray.color.mult(&int.mat.albedo);
                        ray.origin = &int.pos + &int.normal.scale(0.001);
                    }
                }
            } else {
                break;
            }

            iter += 1;
            if iter > bounces {
                break;
            }
        }

        acc_color = &acc_color + &ray.emitted;
    }

    acc_color = acc_color.scale(1.0 / (samples_per_pixel as f64));

    let f_to_u8 = |f: f64| (255.0 * f.max(0.0).min(1.0)) as u8;

    Rgb([
        f_to_u8(acc_color.x),
        f_to_u8(acc_color.y),
        f_to_u8(acc_color.z),
    ])
}

mod material;
mod render;
mod scene;
mod vector;

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use image::ImageBuffer;

use material::Material;
use render::{pixel_shader, Camera, Context};
use scene::Scene;
use vector::{Mat4, Vec3};

fn tests() {
    let normal = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    }
    .normalized();
    let rotated = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }
    .rotate_to_face(&normal);
    let dot = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }
    .dot(&normal);
    let acos = dot.acos();
    println!("rotated : {}, dot: {}, acos: {}", rotated, dot, acos)
}

fn main() {
    //tests();
    //return;

    let (width, height): (u32, u32) = (1000, 1000);

    let cam_pos = Vec3 {
        x: -0.5,
        y: -0.5,
        z: 0.6,
    };

    let cam_target = Vec3 {
        x: 0.2,
        y: -0.8,
        z: 0.2,
    };

    let ctx = Context {
        scene: Scene::cornell_box(),
        width,
        height,
        camera: Camera::new(cam_pos, cam_target, 0.7),
    };

    let img: image::RgbImage = ImageBuffer::new(width, height);

    let img_ref = Arc::new(Mutex::new(img));
    //let ctx_ref = Arc::new(ctx);

    let n_threads = 8;

    let loading_counter = Arc::new(Mutex::new(0)); //we increment for every column. percentage is this counter over the width.

    thread::scope(|s| {
        let mut threads = vec![];

        for t in 0..n_threads {
            //let img_clone = Arc::clone(&img_ref);
            //let ctx_clone = Arc::clone(&ctx_ref);
            //let counter_clone = Arc::clone(&loading_counter);
            //we go from t * width / n_threads to (t+1) * width / n_threads
            let low = t * width / n_threads;
            let up = (t + 1) * width / n_threads;

            let ctx_ref = &ctx;
            let img_ref = &img_ref;
            let counter_ref = &loading_counter;

            let new_thread = s.spawn(move || {
                for i in low..up {
                    for j in 0..height {
                        let pixel = pixel_shader(ctx_ref, i, j, 7, 2000);
                        img_ref.lock().unwrap().put_pixel(i, j, pixel);
                    }
                    let mut ref_ = counter_ref.lock().unwrap();

                    *ref_ += 1;
                    println!("{}%", 100.0 * (*ref_ as f32) / (width as f32));
                }
            });
            threads.push(new_thread);
        }
        for t in threads {
            t.join().unwrap();
        }
    });

    let img = img_ref
        .lock()
        .expect("couldn't acquire image after render :(");
    match img.save(Path::new("./renders/render.png")) {
        Ok(_) => (),
        Err(_) => {
            println!("WARNING : Couldn't save render at desired path, trying to save it at ./buffer_render.png");
            img.save("./buffer_render.png")
                .expect("DISK ERROR! IMPORTANT! WHY!");
        }
    }
}

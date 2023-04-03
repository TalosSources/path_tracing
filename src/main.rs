mod vector;
mod material;
mod scene;
mod render;

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use image::ImageBuffer;

use vector::Vec3;
use material::Material;
use scene::Scene;
use render::{Context, pixel_shader};


fn main() {
    
    let (width, height) : (u32, u32) = (1000, 1000);

    let ctx = Context {scene : Scene::cornell_box(), width : width, height : height, focal_length : 0.7};
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
                        let pixel = pixel_shader(&ctx_clone, i, j, 7, 100);
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

    let img = img_ref.lock().expect("couldn't acquire image after render :(");
    match img.save(Path::new("./renders/render.png")) {
        Ok(_) => (),
        Err(_) => {
            println!("WARNING : Couldn't save render at desired path, trying to save it at ./buffer_render.png");
            img.save("./buffer_render.png").expect("DISK ERROR! IMPORTANT! WHY!");
        }
    }

}

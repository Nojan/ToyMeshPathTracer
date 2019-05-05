mod camera;
mod hit;
mod obj_loader;
mod ppm_writer;
mod random;
mod ray;
mod triangle;
mod vec3;

use camera::*;
use ray::*;
use triangle::*;
use vec3::*;

fn compute_scene_boundary(triangle_list: &Vec<Triangle>) -> (Vec3, Vec3) {
    let compute_triangle_boundary = |(min_b, max_b), triangle: &Triangle| {
        triangle
            .vertices
            .iter()
            .fold((min_b, max_b), |(min_tr, max_tr), vertice| {
                (min(&min_tr, &vertice), max(&max_tr, &vertice))
            })
    };
    triangle_list.iter().fold(
        (Vec3::max_value(), Vec3::min_value()),
        compute_triangle_boundary,
    )
}

fn trace(ray: &Ray, depth: usize, rng_state: &mut u32, triangle_list: &Vec<Triangle>) -> Vec3 {
    if 0 == depth {
        return Vec3::zero();
    }
    Vec3::zero()
}

fn gamma_correction(color: Vec3) -> Vec3 {
    let mut result = Vec3::zero();
    for idx in 0..3 {
        result.data[idx] = color.data[idx].sqrt();
    }
    return result;
}

fn main() {
    let filename = "data/triangle.obj";
    println!("Loading {}", filename);
    let mut triangle_list = obj_loader::load_scene(filename).expect("!?");
    println!("Loaded {} triangles", triangle_list.len());
    {
        // add two triangles that are right "under the scene" and covering a larger area than the scene
        // itself, to serve as a "floor"
        let (scene_min, scene_max) = compute_scene_boundary(&triangle_list);
        let floor_size = (scene_max - scene_min) * 0.7;
        let v0 = Vec3::new(
            scene_min.x() - floor_size.x(),
            scene_min.y(),
            scene_min.z() - floor_size.z(),
        );
        let v1 = Vec3::new(
            scene_min.x() - floor_size.x(),
            scene_min.y(),
            scene_max.z() + floor_size.z(),
        );
        let v2 = Vec3::new(
            scene_max.x() + floor_size.x(),
            scene_min.y(),
            scene_min.z() - floor_size.z(),
        );
        let v3 = Vec3::new(
            scene_max.x() + floor_size.x(),
            scene_min.y(),
            scene_max.z() + floor_size.z(),
        );
        let tr0 = Triangle::new(v0, v1, v2);
        let tr1 = Triangle::new(v1, v3, v2);
        triangle_list.push(tr0);
        triangle_list.push(tr1);
    }
    let triangle_list = triangle_list;
    for tr in triangle_list.iter() {
        for vtx in tr.vertices.iter() {
            println!("{:?}", vtx);
        }
    }
    let (scene_min, scene_max) = compute_scene_boundary(&triangle_list);
    println!("{:?} {:?}", scene_min, scene_max);

    // place the camera
    let scene_size = scene_max - scene_min;
    let scene_center = (scene_min + scene_max) * 0.5;
    let look_from = scene_center + scene_size * Vec3::new(0.3, 0.6, 1.2);
    let look_at = scene_center + scene_size * Vec3::new(0.0, -0.1, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.03;
    let aspect = (WIDTH as f32) / (HEIGHT as f32);
    let camera = Camera::look_at(
        &look_from,
        &look_at,
        &Vec3::new(0.0, 1.0, 0.0),
        60.0,
        aspect,
        aperture,
        dist_to_focus,
    );

    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    let mut data = [0u8; 3 * WIDTH * HEIGHT];
    let mut data_iter = data.iter_mut();

    // trace image
    {
        let inv_width = 1.0f32 / (WIDTH as f32);
        let inv_height = 1.0f32 / (HEIGHT as f32);

        for y in 0..HEIGHT {
            let mut rng_state: u32 = (y as u32) * 9781 + 1;
            for x in 0..WIDTH {
                let mut color = Vec3::zero();
                const SPP: usize = 1;
                for s in 0..SPP {
                    let u = (x as f32) * inv_width;
                    let v = (y as f32) * inv_height;
                    let ray = camera.get_ray(u, v, &mut rng_state);
                    color = color + trace(&ray, 10, &mut rng_state, &triangle_list);
                }
                color = color * SPP as f32;
                color = gamma_correction(color);

                // saturate
                let color_0 = Vec3::fill(0.0);
                let color_1 = Vec3::fill(1.0);
                color = max(&color_0, &min(&color_1, &color));

                *data_iter.next().unwrap() = (color.x() * 255.0) as u8;
                *data_iter.next().unwrap() = (color.y() * 255.0) as u8;
                *data_iter.next().unwrap() = (color.z() * 255.0) as u8;
            }
        }
    }

    ppm_writer::write("test.ppm", WIDTH, HEIGHT, &data);
}

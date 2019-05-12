mod aabb;
mod bvh;
mod camera;
mod hit;
mod obj_loader;
mod ppm_writer;
mod random;
mod ray;
mod scene;
mod triangle;
mod vec3;

use camera::*;
use hit::*;
use ray::*;
use std::time::Instant;
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

fn gamma_correction(color: Vec3) -> Vec3 {
    let mut result = Vec3::zero();
    for idx in 0..3 {
        result.data[idx] = color.data[idx].sqrt();
    }
    return result;
}

fn main() {
    let filename = "data/suzanne.obj";
    println!("Loading {}", filename);
    let mut triangles = obj_loader::load_scene(filename).expect("!?");
    println!("Loaded {} triangles", triangles.len());
    {
        // add two triangles that are right "under the scene" and covering a larger area than the scene
        // itself, to serve as a "floor"
        let (scene_min, scene_max) = compute_scene_boundary(&triangles);
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
        triangles.push(tr0);
        triangles.push(tr1);
    }
    let mut scene = scene::Scene {
        triangle_list: triangles,
        bvh: None,
    };
    scene.bvh = Some(bvh::Bvh::create(&mut scene.triangle_list[..]));
    let scene = scene;
    let (scene_min, scene_max) = compute_scene_boundary(&scene.triangle_list);

    // place the camera
    let scene_size = scene_max - scene_min;
    let scene_center = (scene_min + scene_max) * 0.5;
    let look_from = if filename.contains("sponza.obj") {
        Vec3::new(-5.6, 4.08, -1.22)
    } else {
        scene_center + scene_size * Vec3::new(0.3, 0.6, 1.2)
    };
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

    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    let mut data = [0u8; 3 * WIDTH * HEIGHT];
    let mut data_iter = data.iter_mut();

    // trace image
    let mut ray_total_count = 0usize;
    let trace_begin = Instant::now();
    {
        let inv_width = 1.0f32 / (WIDTH as f32);
        let inv_height = 1.0f32 / (HEIGHT as f32);
        const SPP: usize = 4;
        const SPP_INV: f32 = 1.0 / (SPP as f32);

        for y in 0..HEIGHT {
            let mut rng_state: u32 = (y as u32) * 9781 + 1;
            for x in 0..WIDTH {
                let mut color = Vec3::zero();
                for _s in 0..SPP {
                    let u = (x as f32) * inv_width;
                    let v = 1.0 - (y as f32) * inv_height;
                    let ray = camera.get_ray(u, v, &mut rng_state);
                    let (ray_color, ray_count) = scene::trace(&ray, 10, &mut rng_state, &scene);
                    color = color + ray_color;
                    ray_total_count += ray_count;
                }
                color = color * SPP_INV;
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
    let trace_end = Instant::now();
    let trace_duration = trace_end.duration_since(trace_begin);
    let durations_sec =
        (trace_duration.as_secs() as f32) + (trace_duration.subsec_millis() as f32) / 1000.0;
    println!("{} rays in {} s", ray_total_count, durations_sec);
    println!(
        "{} K rays per second",
        (ray_total_count as f32) / durations_sec / 1000.0
    );

    ppm_writer::write("test.ppm", WIDTH, HEIGHT, &data);
}

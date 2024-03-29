use crate::bvh::*;
use crate::hit::*;
use crate::ray::*;
use crate::triangle::*;
use crate::vec3::*;

pub struct Scene {
    pub triangle_list: Vec<Triangle>,
    pub bvh: Option<Bvh>,
}

const RAY_MIN: f32 = 0.01;
const RAY_MAX: f32 = 100.0;
const LIGHT_DIR: [f32; 3] = [-0.5301519, 0.758786, 0.378395];

fn hit_scene(ray: &Ray, min_t: f32, max_t: f32, hit_type: HitType, scene: &Scene) -> Option<Hit> {
    if scene.bvh.is_some() {
        return scene
            .bvh
            .as_ref()
            .unwrap()
            .intersect(ray, min_t, max_t, hit_type, &scene.triangle_list[..]);
    }
    let mut min_distance = max_t;
    let mut best_hit: Option<Hit> = None;
    for triangle in scene.triangle_list.iter() {
        let hit = triangle.intersect(ray, min_t, max_t);
        if hit.is_none() {
            continue;
        }
        let hit = hit.unwrap();
        if hit.t < min_distance {
            min_distance = hit.t;
            best_hit = Some(hit);
            if let HitType::Any = hit_type {
                return best_hit;
            }
        }
    }
    return best_hit;
}

fn scatter(ray: &Ray, hit: &Hit, rng_state: &mut u32, scene: &Scene) -> (Ray, Vec3) {
    let mut light_ray = Vec3::zero();
    let target = hit.normal + Vec3::rand_unit(rng_state);
    let scattered = Ray::new(&hit.pos, &target.normalize());

    if hit_scene(
        &Ray::new(&hit.pos, &Vec3::from(LIGHT_DIR)),
        RAY_MIN,
        RAY_MAX,
        HitType::Any,
        scene,
    )
    .is_none()
    {
        let nl = hit.normal * Vec3::dot(&hit.normal, &ray.dir()).signum() * -1.0;
        light_ray = Vec3::fill(0.7) * 0.0f32.max(Vec3::dot(&Vec3::from(LIGHT_DIR), &nl));
    }

    return (scattered, light_ray);
}

pub fn trace(ray: &Ray, depth: usize, rng_state: &mut u32, scene: &Scene) -> (Vec3, usize) {
    if 0 == depth {
        return (Vec3::zero(), 1);
    }
    let hit = hit_scene(ray, RAY_MIN, RAY_MAX, HitType::Closest, scene);
    if hit.is_some() {
        let (ray_scatter, light_ray) = scatter(ray, &hit.unwrap(), rng_state, scene);
        let (color, ray_count) = trace(&ray_scatter, depth - 1, rng_state, scene);
        return (light_ray + color * 0.7, ray_count + 2);
    } else {
        let t = 0.5 * (ray.dir().y() + 1.0);
        let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t * 0.5;
        return (color, 1);
    }
}

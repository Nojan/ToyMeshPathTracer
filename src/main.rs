mod camera;
mod hit;
mod obj_loader;
mod ppm_writer;
mod random;
mod ray;
mod triangle;
mod vec3;

fn compute_scene_boundary(triangle_list: &Vec<triangle::Triangle>) -> (vec3::Vec3, vec3::Vec3) {
    let compute_triangle_boundary = |(min_b, max_b), triangle: &triangle::Triangle| {
        triangle
            .vertices
            .iter()
            .fold((min_b, max_b), |(min_tr, max_tr), vertice| {
                (vec3::min(&min_tr, &vertice), vec3::max(&max_tr, &vertice))
            })
    };
    triangle_list.iter().fold(
        (vec3::Vec3::max_value(), vec3::Vec3::min_value()),
        compute_triangle_boundary,
    )
}

fn main() {
    let filename = "data/triangle.obj";
    println!("Loading {}", filename);
    let triangle_list = obj_loader::load_scene(filename).expect("!?");
    println!("Loaded {} triangles", triangle_list.len());

    let (scene_min, scene_max) = compute_scene_boundary(&triangle_list);
    for tr in triangle_list.iter() {
        for vtx in tr.vertices.iter() {
            println!("{:?}", vtx);
        }
    }
    println!("{:?} {:?}", scene_min, scene_max);

    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    let data = [0u8; 3 * WIDTH * HEIGHT];
    ppm_writer::write("test.ppm", WIDTH, HEIGHT, &data);
}

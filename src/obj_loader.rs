use crate::triangle::Triangle;
use crate::vec3::Vec3;
use std::io::{Error, ErrorKind};

pub fn load_scene(filename: &str) -> std::io::Result<Vec<Triangle>> {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();
    {
        let contents = std::fs::read_to_string(filename)?;
        for line in contents.lines() {
            if line.starts_with("v ") {
                let error = Error::new(ErrorKind::Other, "bad vertex format");
                let mut word_iter = line.split_whitespace();
                assert_eq!(Some("v"), word_iter.next());
                for w in word_iter {
                    let val = match w.parse::<f32>() {
                        Ok(v) => v,
                        _ => return Err(error),
                    };
                    vertices.push(val);
                }
            } else if line.starts_with("f ") {
                let error = Error::new(ErrorKind::Other, "bad face form");
                let mut word_iter = line.split_whitespace();
                assert_eq!(Some("f"), word_iter.next());
                for w in word_iter {
                    let index = match w.split('/').next() {
                        Some(index_str) => index_str.parse::<usize>().unwrap(),
                        _ => return Err(error),
                    };
                    indexes.push(index - 1usize);
                }
            }
        }
    }
    let mut triangles_list: Vec<Triangle> = Vec::new();
    if indexes.is_empty() {
        for tr_chunk in vertices.chunks(9) {
            let mut v1 = [0f32; 3];
            v1.copy_from_slice(&tr_chunk[0..3]);
            let mut v2 = [0f32; 3];
            v2.copy_from_slice(&tr_chunk[3..6]);
            let mut v3 = [0f32; 3];
            v3.copy_from_slice(&tr_chunk[6..9]);
            triangles_list.push(Triangle {
                vertices: [Vec3 { data: v1 }, Vec3 { data: v2 }, Vec3 { data: v3 }],
            });
        }
    } else {
        for tr_chunk in indexes.chunks(3) {
            let mut v1 = [0f32; 3];
            let v1_idx = tr_chunk[0] * 3;
            v1.copy_from_slice(&vertices[v1_idx..v1_idx + 3]);
            let mut v2 = [0f32; 3];
            let v2_idx = tr_chunk[1] * 3;
            v2.copy_from_slice(&vertices[v2_idx..v2_idx + 3]);
            let mut v3 = [0f32; 3];
            let v3_idx = tr_chunk[2] * 3;
            v3.copy_from_slice(&vertices[v3_idx..v3_idx + 3]);
            triangles_list.push(Triangle {
                vertices: [Vec3 { data: v1 }, Vec3 { data: v2 }, Vec3 { data: v3 }],
            });
        }
    }

    return Ok(triangles_list);
}

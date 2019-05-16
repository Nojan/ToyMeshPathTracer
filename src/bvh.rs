use crate::aabb::*;
use crate::hit::*;
use crate::random::*;
use crate::ray::*;
use crate::triangle::*;

use std::cmp;

pub struct Bvh {
    nodes: Vec<BvhNode>,
}

struct BvhNode {
    v: Aabb,
    d1: usize,
    d2: usize,
    is_leaf: bool,
}

impl Bvh {
    fn order_triangle(a: &Triangle, b: &Triangle, idx: usize) -> cmp::Ordering {
        let a_min = a.vertices.iter().fold(std::f32::INFINITY, |min, vertice| {
            min.min(vertice.get(idx))
        });
        let b_min = b.vertices.iter().fold(std::f32::INFINITY, |min, vertice| {
            min.min(vertice.get(idx))
        });
        a_min.partial_cmp(&b_min).unwrap()
    }

    fn triangle_aabb(triangle: &Triangle) -> Aabb {
        triangle
            .vertices
            .iter()
            .fold(Aabb::empty(), |aabb, v| aabb.extend(v))
    }

    fn create_impl(
        tri_start: usize,
        tri_count: usize,
        triangle_list: &mut [Triangle],
        bvh: &mut Vec<BvhNode>,
        rng: &mut u32,
    ) -> usize {
        *rng = xor_shift_32(*rng);
        let idx = (*rng % 3) as usize;

        triangle_list.sort_unstable_by(|a, b| Bvh::order_triangle(a, b, idx));

        let node_idx = bvh.len();
        {
            let dummy = BvhNode {
                v: Aabb::empty(),
                d1: usize::max_value(),
                d2: usize::max_value(),
                is_leaf: false,
            };
            bvh.push(dummy);
        }
        let mut node = BvhNode {
            v: Aabb::empty(),
            d1: usize::max_value(),
            d2: usize::max_value(),
            is_leaf: false,
        };
        if tri_count <= 4 {
            node.d1 = tri_start;
            node.d2 = tri_count;
            node.is_leaf = true;
            node.v = triangle_list.iter().fold(Aabb::empty(), |aabb, tr| {
                Aabb::union_aabb(&aabb, &Bvh::triangle_aabb(tr))
            });
        } else {
            //let split_idx = triangle_list.len() / 2;
            let split_idx = {
                let mut split_cost = Vec::with_capacity(triangle_list.len());
                for idx in 0..triangle_list.len() {
                    let left_aabb = triangle_list
                        .iter()
                        .take(idx)
                        .fold(Aabb::empty(), |aabb, tr| {
                            Aabb::union_aabb(&aabb, &Bvh::triangle_aabb(tr))
                        });
                    let right_aabb = triangle_list
                        .iter()
                        .skip(idx)
                        .fold(Aabb::empty(), |aabb, tr| {
                            Aabb::union_aabb(&aabb, &Bvh::triangle_aabb(tr))
                        });
                    let total_aabb = Aabb::union_aabb(&left_aabb, &right_aabb);
                    let cost: f32 = 0.125f32
                        + (idx as f32 * left_aabb.surface_area()
                            + (triangle_list.len() - idx) as f32 * right_aabb.surface_area())
                            / total_aabb.surface_area();
                    split_cost.push(cost);
                }
                let split_min_cost = split_cost
                    .iter()
                    .fold(std::f32::INFINITY, |min_val, &val| min_val.min(val));
                split_cost
                    .iter()
                    .position(|&val| val == split_min_cost)
                    .unwrap()
            };
            let (left, right) = triangle_list.split_at_mut(split_idx);
            node.d1 = Bvh::create_impl(tri_start, split_idx, left, bvh, rng);
            node.d2 = Bvh::create_impl(
                tri_start + split_idx,
                tri_count - split_idx,
                right,
                bvh,
                rng,
            );
            node.is_leaf = false;
            node.v = Aabb::union_aabb(&bvh[node.d1].v, &bvh[node.d2].v);
        }

        bvh[node_idx] = node;
        return node_idx;
    }

    pub fn create(triangle_list: &mut [Triangle]) -> Bvh {
        let mut bvh: Vec<BvhNode> = Vec::new();
        let mut rng = 0xF215C12Eu32;
        Bvh::create_impl(0, triangle_list.len(), triangle_list, &mut bvh, &mut rng);
        return Bvh { nodes: bvh };
    }

    fn intersect_impl(
        &self,
        idx: usize,
        ray: &Ray,
        tmin: f32,
        tmax: &mut f32,
        triangles: &[Triangle],
    ) -> Option<Hit> {
        let node = &self.nodes[idx];
        if !node.v.test_intersection(ray, tmin, *tmax) {
            return None;
        }

        let mut hit = None;
        if node.is_leaf {
            for tri_idx in node.d1..(node.d1 + node.d2) {
                let tri_hit = triangles[tri_idx].intersect(ray, tmin, *tmax);
                if tri_hit.is_some() {
                    let tri_hit = tri_hit.unwrap();
                    *tmax = tri_hit.t;
                    hit = Some(tri_hit);
                }
            }
        } else {
            let left_hit = self.intersect_impl(node.d1, ray, tmin, tmax, triangles);
            let right_hit = self.intersect_impl(node.d2, ray, tmin, tmax, triangles);
            if right_hit.is_some() {
                hit = right_hit;
            } else {
                hit = left_hit;
            }
        }

        return hit;
    }

    pub fn intersect(
        &self,
        ray: &Ray,
        tmin: f32,
        tmax: f32,
        triangles: &[Triangle],
    ) -> Option<Hit> {
        let mut local_tmax = tmax;
        self.intersect_impl(0, ray, tmin, &mut local_tmax, triangles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {}
}

use glam::Vec2;
use glam::Vec3;

pub fn average<'a, T>(list: impl Iterator<Item = T>) -> T
where
    T: std::ops::Add<Output = T> + std::default::Default + std::ops::Div<f32, Output = T>,
{
    let (sum, count) = list.fold((T::default(), 0.), |(sum, count), elem| {
        (sum + elem, count + 1.)
    });
    sum / (count as f32)
}

// (p * s) + t = p'
pub fn transform_coordinates(translation: Vec3, scale: f32, position: Vec3) -> Vec3 {
    (position * scale) + translation
}

// (p' - t) / s = p'
pub fn inv_transform_coordinates(translation: Vec3, scale: f32, position: Vec3) -> Vec3 {
    (position - translation) / scale
}

pub fn intersection_in_sequence(elem_a: usize, elem_b: usize, sequence: &Vec<usize>) -> bool {
    let mut sequence_copy = sequence.clone();
    sequence_copy.retain(|&elem| elem == elem_a || elem == elem_b);
    debug_assert!(sequence_copy.len() == 4, "{:?}", sequence_copy);
    sequence_copy.dedup();
    sequence_copy.len() >= 4
}

pub fn set_intersection<T: std::cmp::PartialEq + Clone>(
    collection_a: &Vec<T>,
    collection_b: &Vec<T>,
) -> Vec<T> {
    let mut intesection = collection_b.clone();
    intesection.retain(|edge_id| collection_a.contains(edge_id));
    intesection
}

pub fn inside_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> bool {
    let v0 = c - a;
    let v1 = b - a;
    let v2 = p - a;

    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);

    let inv_denom = 1. / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    u >= 0. && v >= 0. && u + v < 1.
}

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
pub fn intersection_exact_in_2d(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> Option<Vec2> {
    let t = ((p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x))
        / ((p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x));

    let u = ((p1.x - p3.x) * (p1.y - p2.y) - (p1.y - p3.y) * (p1.x - p2.x))
        / ((p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x));

    if t >= 0. && t <= 1. && u >= 0. && u <= 1. {
        let intersection_x = p1.x + t * (p2.x - p1.x);
        let intersection_y = p1.y + t * (p2.y - p1.y);

        return Some(Vec2::new(intersection_x, intersection_y));
    }

    None
}

pub fn convert_3d_to_2d(point: Vec3, reference: Vec3) -> Vec2 {
    let alpha = point.angle_between(reference);
    Vec2::new(point.length() * alpha.cos(), point.length() * alpha.sin())
}

// Draw objects by returning a list of lines to render
pub fn draw_vertex(p: Vec3, n: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p, p + n)]
}

pub fn draw_line(p1: Vec3, p2: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2)]
}

pub fn draw_triangle(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2), (p2, p3), (p3, p1)]
}

pub fn draw_quad(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2), (p2, p3), (p3, p4), (p4, p1)]
}

pub fn draw_polygon(vertices: Vec<Vec3>) -> Vec<(Vec3, Vec3)> {
    let mut lines = Vec::new();
    for i in 0..vertices.len() {
        lines.push((vertices[i], vertices[(i + 1) % vertices.len()]));
    }
    lines
}

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

pub fn transform_coordinates(translation: Vec3, scale: f32, position: Vec3) -> Vec3 {
    (position * scale) + translation
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

pub fn intersection_exact_in_2d(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> Option<Vec2> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
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

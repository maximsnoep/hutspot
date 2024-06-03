type Vec2 = glam::f64::DVec2;
type Vec3 = glam::f64::DVec3;
const EPS: f64 = f64::EPSILON;

#[must_use]
pub fn average<T>(list: impl Iterator<Item = T>) -> T
where
    T: std::ops::Add<Output = T> + std::default::Default + std::ops::Div<f32, Output = T>,
{
    let (sum, count) = list.fold((T::default(), 0.), |(sum, count), elem| {
        (sum + elem, count + 1.)
    });
    sum / count
}

// // (p * s) + t = p'
// #[must_use]
// pub fn transform_coordinates(translation: Vec3, scale: f32, position: Vec3) -> Vec3 {
//     (position * scale) + translation
// }

// // (p' - t) / s = p'
// #[must_use]
// pub fn inv_transform_coordinates(translation: Vec3, scale: f32, position: Vec3) -> Vec3 {
//     (position - translation) / scale
// }

#[must_use]
pub fn intersection_in_sequence(elem_a: usize, elem_b: usize, sequence: &[usize]) -> bool {
    let mut sequence_copy = sequence.to_owned();
    sequence_copy.retain(|&elem| elem == elem_a || elem == elem_b);
    debug_assert!(sequence_copy.len() == 4, "{sequence_copy:?}");
    sequence_copy.dedup();
    sequence_copy.len() >= 4
}

#[must_use]
pub fn set_intersection<T: std::cmp::PartialEq + Clone>(
    collection_a: &[T],
    collection_b: &[T],
) -> Vec<T> {
    let mut intesection = collection_b.to_owned();
    intesection.retain(|edge_id| collection_a.contains(edge_id));
    intesection
}

#[must_use]
pub fn project_point_on_plane(point: Vec3, plane: (Vec3, Vec3), reference: Vec3) -> Vec2 {
    Vec2::new(
        (point - reference).dot(plane.0),
        (point - reference).dot(plane.1),
    )
}

// https://en.wikipedia.org/wiki/Area_of_a_triangle
#[must_use]
pub fn triangle_area(p1: Vec3, p2: Vec3, p3: Vec3) -> f64 {
    let u = p2 - p1;
    let v = p3 - p1;
    u.cross(v).length() * 0.5
}

// https://en.wikipedia.org/wiki/Barycentric_coordinate_system
#[must_use]
pub fn inside_triangle(point: Vec3, triangle: (Vec3, Vec3, Vec3)) -> bool {
    let s1 = triangle_area(triangle.0, triangle.1, point);
    let s2 = triangle_area(triangle.1, triangle.2, point);
    let s3 = triangle_area(triangle.2, triangle.0, point);
    let st = triangle_area(triangle.0, triangle.1, triangle.2);
    s1 + s2 + s3 == st
        && (0.0..=st).contains(&s1)
        && (0.0..=st).contains(&s2)
        && (0.0..=st).contains(&s3)
}

#[test]
fn test_inside_triangle() {
    let triangle = (
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 1., 0.),
    );

    let point = Vec3::new(0.5, 0.5, 0.0);
    let epsilon_x = Vec3::new(EPS, 0.0, 0.0);
    let epsilon_y = Vec3::new(0.0, EPS, 0.0);
    let epsilon_z = Vec3::new(0.0, 0.0, EPS);

    // On the boundary
    assert!(inside_triangle(point, triangle));
    // Inside the triangle (by epsilon)
    assert!(inside_triangle(point - epsilon_x, triangle));
    assert!(inside_triangle(point - epsilon_y, triangle));

    // Outside the triangle (by epsilon)
    assert!(!inside_triangle(point + epsilon_x, triangle));
    assert!(!inside_triangle(point + epsilon_y, triangle));

    // Outside the triangle (by epsilon in z axis)
    assert!(!inside_triangle(point + epsilon_z, triangle));
    assert!(!inside_triangle(point - epsilon_z, triangle));
}

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
#[must_use]
pub fn intersection_in_2d(p_u: Vec2, p_v: Vec2, q_u: Vec2, q_v: Vec2) -> Option<Vec2> {
    let (x1, x2, x3, x4, y1, y2, y3, y4) = (
        f64::from(p_u.x),
        f64::from(p_v.x),
        f64::from(q_u.x),
        f64::from(q_v.x),
        f64::from(p_u.y),
        f64::from(p_v.y),
        f64::from(q_u.y),
        f64::from(q_v.y),
    );

    let t_numerator = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let u_numerator = -(x1 - x2) * (y1 - y3) + (y1 - y2) * (x1 - x3);
    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    return if ((0.0..=denominator).contains(&t_numerator)
        && (0.0..=denominator).contains(&u_numerator))
        || ((0.0..=-denominator).contains(&-t_numerator)
            && (0.0..=-denominator).contains(&-u_numerator))
    {
        let t = t_numerator / denominator;
        let sx_t = x1 + t * (x2 - x1);
        let sy_t = y1 + t * (y2 - y1);
        let s_t = Vec2::new(sx_t, sy_t);
        let u = u_numerator / denominator;
        let sx_u = x3 + u * (x4 - x3);
        let sy_u = y3 + u * (y4 - y3);
        let s_u = Vec2::new(sx_u, sy_u);
        assert!(s_t.abs_diff_eq(s_u, EPS));
        Some(s_t)
    } else {
        None
    };
}

#[test]
fn test_intersection_in_2d() {
    let p1 = Vec2::new(0., 0.);
    let p2 = Vec2::new(1., 1.);
    let p3 = Vec2::new(1., 0.);
    let p4 = Vec2::new(0., 1.);
    assert!(intersection_in_2d(p1, p2, p3, p4) == Some(Vec2::new(0.5, 0.5)));
    assert!(intersection_in_2d(p2, p1, p4, p3) == Some(Vec2::new(0.5, 0.5)));
    assert!(intersection_in_2d(p1, p3, p2, p4) == None);
    assert!(intersection_in_2d(p3, p1, p4, p2) == None);

    let epsilon_x = Vec2::new(EPS, 0.0);
    let epsilon_y = Vec2::new(0.0, EPS);
    assert!(intersection_in_2d(p1, p3, p2, p3) == Some(p3));

    assert!(intersection_in_2d(p1, p3, p2, p3 + epsilon_y) == None);
    assert!(intersection_in_2d(p1, p3, p2, p3 - epsilon_y) == Some(p3));
    assert!(intersection_in_2d(p1, p3, p2, p3 + epsilon_x) == None);
    assert!(intersection_in_2d(p1, p3, p2, p3 - epsilon_x) == Some(p3 - epsilon_x));

    assert!(intersection_in_2d(p1, p3 + epsilon_y, p2, p3) == Some(p3 + epsilon_y));
    assert!(intersection_in_2d(p1, p3 - epsilon_y, p2, p3) == None);
    assert!(intersection_in_2d(p1, p3 + epsilon_x, p2, p3) == Some(p3));
    assert!(intersection_in_2d(p1, p3 - epsilon_x, p2, p3) == None);
}

#[must_use]
pub fn are_coplanar(points: &[Vec3]) -> bool {
    let a = points[0];
    let b = points[1];
    let c = points[2];

    let ab = b - a;
    let bc = c - b;

    let r = a + lambda * ab + mu * bc;
}

#[must_use]
pub fn intersection_in_3d(p_u: Vec3, p_v: Vec3, q_u: Vec3, q_v: Vec3) -> Option<Vec3> {
    let p = p_v - p_u;
    let q = q_v - q_u;
    let normal_vector = p.cross(q).normalize();
    let reference_point = p_u;
    let plane = (p.normalize(), p.cross(normal_vector).normalize());

    let p_u_2d = project_point_on_plane(p_u, plane, reference_point);
    let p_v_2d = project_point_on_plane(p_v, plane, reference_point);
    let q_u_2d = project_point_on_plane(q_u, plane, reference_point);
    let q_v_2d = project_point_on_plane(q_v, plane, reference_point);

    return if let Some(intersection_2d) = intersection_in_2d(p_u_2d, p_v_2d, q_u_2d, q_v_2d) {
        let intersection_3d =
            reference_point + (plane.0 * intersection_2d.x) + (plane.1 * intersection_2d.y);
        Some(intersection_3d)
    } else {
        None
    };
}

#[test]
pub fn test_intersection_in_3d() {
    let p1 = Vec3::new(0.0, 0.0, 0.);
    let p2 = Vec3::new(1., 1., 0.);
    let p3 = Vec3::new(1., 0., 0.);
    let p4 = Vec3::new(0., 1., 0.);

    assert!(
        intersection_in_3d(p1, p2, p3, p4)
            .is_some_and(|x| x.abs_diff_eq(Vec3::new(0.5, 0.5, 0.), EPS)),
        "{:?}",
        intersection_in_3d(p1, p2, p3, p4)
    );
    assert!(intersection_in_3d(p2, p1, p4, p3)
        .is_some_and(|x| x.abs_diff_eq(Vec3::new(0.5, 0.5, 0.), EPS)));
    assert!(intersection_in_3d(p1, p3, p2, p4) == None);

    let pert = Vec3::new(0., 0., 1.0);

    assert!(
        intersection_in_3d(p1 + pert, p2 + pert, p3 + pert, p4 + pert)
            .is_some_and(|x| x.abs_diff_eq(Vec3::new(0.5, 0.5, 0.) + pert, EPS))
    );

    assert!(intersection_in_3d(
        p1 + 10000. * pert,
        p2 + 10000. * pert,
        p3 + 10000. * pert,
        p4 + 10000. * pert
    )
    .is_some_and(|x| x.abs_diff_eq(Vec3::new(0.5, 0.5, 0.) + 10000. * pert, EPS)));

    let epsilon_x = Vec3::new(EPS, 0.0, 0.0);
    let epsilon_y = Vec3::new(0.0, EPS, 0.0);
    let epsilon_z = Vec3::new(0.0, 0.0, EPS);

    let intersection = intersection_in_3d(p1, p3, p2, p4);
    assert!(intersection == None, "{intersection:?} == None");

    let intersection = intersection_in_3d(p1, p3, p2, p3 - epsilon_y);
    assert!(
        intersection.is_some_and(|x| x.abs_diff_eq(p3, EPS)),
        "{intersection:?} == {p3:?}"
    );

    let intersection = intersection_in_3d(p1, p3, p2, p3 + epsilon_x);
    assert!(intersection == None, "{intersection:?} == None");

    let intersection = intersection_in_3d(p1, p3, p2, p3 - epsilon_x);
    assert!(
        intersection.is_some_and(|x| x.abs_diff_eq(p3 - epsilon_x, EPS)),
        "{intersection:?} == {:?}",
        p3 - epsilon_x
    );

    let intersection = intersection_in_3d(p1, p3 + epsilon_y, p2, p3);
    assert!(
        intersection.is_some_and(|x| x.abs_diff_eq(p3 + epsilon_y, EPS)),
        "{intersection:?} == {:?}",
        p3 + epsilon_y
    );

    let intersection = intersection_in_3d(p1, p3 - epsilon_y, p2, p3);
    assert!(intersection == None, "{intersection:?} == None");

    let intersection = intersection_in_3d(p1, p3 + epsilon_x, p2, p3);
    assert!(
        intersection.is_some_and(|x| x.abs_diff_eq(p3, EPS)),
        "{intersection:?} == {p3:?}"
    );

    let intersection = intersection_in_3d(p1, p3 - epsilon_x, p2, p3);
    assert!(intersection == None, "{intersection:?} == None");
}

#[must_use]
pub fn convert_3d_to_2d(point: Vec3, reference: Vec3) -> Vec2 {
    let alpha = point.angle_between(reference);
    Vec2::new(point.length() * alpha.cos(), point.length() * alpha.sin())
}

// Draw objects by returning a list of lines to render
#[must_use]
pub fn draw_vertex(p: Vec3, n: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p, p + n)]
}

#[must_use]
pub fn draw_line(p1: Vec3, p2: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2)]
}

#[must_use]
pub fn draw_triangle(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2), (p2, p3), (p3, p1)]
}

#[must_use]
pub fn draw_quad(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3) -> Vec<(Vec3, Vec3)> {
    vec![(p1, p2), (p2, p3), (p3, p4), (p4, p1)]
}

#[must_use]
pub fn draw_polygon(vertices: &[Vec3]) -> Vec<(Vec3, Vec3)> {
    let mut lines = Vec::new();
    for i in 0..vertices.len() {
        lines.push((vertices[i], vertices[(i + 1) % vertices.len()]));
    }
    lines
}

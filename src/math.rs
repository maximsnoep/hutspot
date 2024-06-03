use approx::AbsDiffEq;

pub type Vector2D = nalgebra::SVector<f64, 2>;
pub type Vector3D = nalgebra::SVector<f64, 3>;
pub const EPS: f64 = f64::EPSILON;

// https://en.wikipedia.org/wiki/Area_of_a_triangle
#[must_use]
pub fn triangle_area(p1: Vector3D, p2: Vector3D, p3: Vector3D) -> f64 {
    let u = p2 - p1;
    let v = p3 - p1;
    u.cross(&v).magnitude() * 0.5
}

// https://en.wikipedia.org/wiki/Barycentric_coordinate_system
#[must_use]
pub fn inside_triangle(point: Vector3D, triangle: (Vector3D, Vector3D, Vector3D)) -> bool {
    let s1 = triangle_area(triangle.0, triangle.1, point);
    let s2 = triangle_area(triangle.1, triangle.2, point);
    let s3 = triangle_area(triangle.2, triangle.0, point);
    let st = triangle_area(triangle.0, triangle.1, triangle.2);
    (s1 + s2 + s3 - st).abs() < EPS
        && (0.0 - EPS..=st + EPS).contains(&s1)
        && (0.0 - EPS..=st + EPS).contains(&s2)
        && (0.0 - EPS..=st + EPS).contains(&s3)
}

type Intersection2D = (Vector2D, IntersectionType);
type Intersection3D = (Vector3D, IntersectionType);

#[derive(Debug, Clone, Copy)]
pub enum IntersectionType {
    Proper,
    Endpoint,
}

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
#[must_use]
pub fn intersection_in_2d(
    p_u: Vector2D,
    p_v: Vector2D,
    q_u: Vector2D,
    q_v: Vector2D,
) -> Option<Intersection2D> {
    let (x1, x2, x3, x4, y1, y2, y3, y4) = (p_u.x, p_v.x, q_u.x, q_v.x, p_u.y, p_v.y, q_u.y, q_v.y);

    let t_numerator = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let u_numerator = -(x1 - x2) * (y1 - y3) + (y1 - y2) * (x1 - x3);
    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if ((0.0..=denominator).contains(&t_numerator) && (0.0..=denominator).contains(&u_numerator))
        || ((0.0..=-denominator).contains(&-t_numerator)
            && (0.0..=-denominator).contains(&-u_numerator))
    {
        if t_numerator == 0.0 {
            return Some((p_u, IntersectionType::Endpoint));
        }
        if t_numerator == denominator {
            return Some((p_v, IntersectionType::Endpoint));
        }
        if u_numerator == 0.0 {
            return Some((q_u, IntersectionType::Endpoint));
        }
        if u_numerator == denominator {
            return Some((q_v, IntersectionType::Endpoint));
        }

        let t = t_numerator / denominator;
        let sx_t = t.mul_add(x2 - x1, x1);
        let sy_t = t.mul_add(y2 - y1, y1);
        let s_t = Vector2D::new(sx_t, sy_t);
        let u = u_numerator / denominator;
        let sx_u = u.mul_add(x4 - x3, x3);
        let sy_u = u.mul_add(y4 - y3, y3);
        let s_u = Vector2D::new(sx_u, sy_u);
        assert!(s_t.abs_diff_eq(&s_u, EPS));
        Some((s_t, IntersectionType::Proper))
    } else {
        None
    }
}

#[must_use]
pub fn project_point_on_plane(
    point: Vector3D,
    plane: (Vector3D, Vector3D),
    reference: Vector3D,
) -> Vector2D {
    Vector2D::new(
        (point - reference).dot(&plane.0),
        (point - reference).dot(&plane.1),
    )
}

// Are `a`, `b`, `c` and `d` coplanar?
#[must_use]
pub fn coplanar(a: Vector3D, b: Vector3D, c: Vector3D, d: Vector3D) -> bool {
    (b - a).cross(&(c - a)).dot(&(d - a)) == 0.
}

#[must_use]
pub fn intersection_in_3d(
    p_u: Vector3D,
    p_v: Vector3D,
    q_u: Vector3D,
    q_v: Vector3D,
) -> Option<Intersection3D> {
    if !coplanar(p_u, p_v, q_u, q_v) {
        return None;
    }

    let p = p_v - p_u;
    let q = q_v - q_u;
    let normal_vector = p.cross(&q).normalize();
    let reference_point = p_u;
    let plane = (p.normalize(), p.cross(&normal_vector).normalize());

    intersection_in_2d(
        project_point_on_plane(p_u, plane, reference_point),
        project_point_on_plane(p_v, plane, reference_point),
        project_point_on_plane(q_u, plane, reference_point),
        project_point_on_plane(q_v, plane, reference_point),
    )
    .map(|(point_in_2d, intersection_type)| {
        let point_in_3d = reference_point + (plane.0 * point_in_2d.x) + (plane.1 * point_in_2d.y);
        (point_in_3d, intersection_type)
    })
}

// Returns the average of summable elements `list`
#[must_use]
pub fn average<T, D>(list: impl Iterator<Item = T>) -> T
where
    T: Default + std::ops::Add<Output = T> + std::ops::Div<D, Output = T>,
    D: Default + std::ops::Add<Output = D> + From<u8>,
{
    let (sum, count): (T, D) = list.fold((T::default(), D::default()), |(sum, count), elem| {
        (sum + elem, count + D::from(1))
    });
    sum / count
}

// #[must_use]
// pub fn convert_3d_to_2d(point: Vector3D, reference: Vector3D) -> Vector2D {
//     let alpha = point.angle_between(reference);
//     Vector2D::new(point.length() * alpha.cos(), point.length() * alpha.sin())
// }

// Draw objects by returning a list of lines to render
#[must_use]
pub fn draw_vertex(p: Vector3D, n: Vector3D) -> Vec<(Vector3D, Vector3D)> {
    vec![(p, p + n)]
}

#[must_use]
pub fn draw_line(p1: Vector3D, p2: Vector3D) -> Vec<(Vector3D, Vector3D)> {
    vec![(p1, p2)]
}

#[must_use]
pub fn draw_triangle(p1: Vector3D, p2: Vector3D, p3: Vector3D) -> Vec<(Vector3D, Vector3D)> {
    vec![(p1, p2), (p2, p3), (p3, p1)]
}

#[must_use]
pub fn draw_quad(
    p1: Vector3D,
    p2: Vector3D,
    p3: Vector3D,
    p4: Vector3D,
) -> Vec<(Vector3D, Vector3D)> {
    vec![(p1, p2), (p2, p3), (p3, p4), (p4, p1)]
}

#[must_use]
pub fn draw_polygon(vertices: &[Vector3D]) -> Vec<(Vector3D, Vector3D)> {
    let mut lines = Vec::new();
    for i in 0..vertices.len() {
        lines.push((vertices[i], vertices[(i + 1) % vertices.len()]));
    }
    lines
}

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

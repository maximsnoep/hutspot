use crate::consts::EPS;
use approx::AbsDiffEq;

pub type Vector2D = nalgebra::SVector<f64, 2>;
pub type Vector3D = nalgebra::SVector<f64, 3>;

/// Calculates the area of a triangle given its three vertices in 3D space.
/// # Arguments
/// * `t` - A tuple of three vertices representing the triangle.
/// # Returns
/// * `f64` - The area of the triangle.
///
/// # Source
/// This method uses the cross product of vectors to find the area of a triangle in 3D space. For more details, see [Wikipedia](https://en.wikipedia.org/wiki/Area_of_a_triangle#Using_vector_cross_products).
#[must_use]
#[inline]
pub fn calculate_triangle_area(t: (Vector3D, Vector3D, Vector3D)) -> f64 {
    (t.1 - t.0).cross(&(t.2 - t.0)).magnitude() * 0.5
}

/// Checks if four points are coplanar.
/// # Arguments
/// * `a` - First point.
/// * `b` - Second point.
/// * `c` - Third point.
/// * `d` - Fourth point.
/// # Returns
/// * `bool` - `true` if the points are coplanar, `false` otherwise.
///
/// # Source
/// This method is based on the concept of coplanarity in vector mathematics, which can be determined using the scalar triple product. For more details, see [Wikipedia](https://en.wikipedia.org/wiki/Coplanarity).
#[must_use]
#[inline]
pub fn are_points_coplanar(a: Vector3D, b: Vector3D, c: Vector3D, d: Vector3D) -> bool {
    (b - a).cross(&(c - a)).dot(&(d - a)) == 0.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    C,   // Collinear
    CW,  // Clockwise
    CCW, // Counterclockwise
}

/// Calculates the orientation of three points in 3D space.
/// # Arguments
/// * `a` - First point.
/// * `b` - Second point.
/// * `c` - Third point.
/// * `n` - Normal vector of the plane containing the points.
/// # Returns
/// * `Orientation` - The orientation of the points.
///
/// # Source
/// This method is based on the right-hand rule for the cross product and the dot product to determine the relative orientation of the points in a plane.
/// For more details, see [Wikipedia](https://en.wikipedia.org/wiki/Orientation_(vector_space)).
#[must_use]
#[inline]
pub fn calculate_orientation(a: Vector3D, b: Vector3D, c: Vector3D, n: Vector3D) -> Orientation {
    let orientation = (b - a).cross(&(c - a)).dot(&n);
    if orientation > 0. {
        Orientation::CW
    } else if orientation < 0. {
        Orientation::CCW
    } else {
        Orientation::C
    }
}

/// Projects a point onto a plane.
/// # Arguments
/// * `point` - The point to project.
/// * `plane` - A tuple representing the plane (two vectors).
/// * `reference` - A reference point on the plane.
/// # Returns
/// * `Vector2D` - The projected point in 2D space.
#[must_use]
#[inline]
pub fn project_point_onto_plane(
    point: Vector3D,
    plane: (Vector3D, Vector3D),
    reference: Vector3D,
) -> Vector2D {
    Vector2D::new(
        (point - reference).dot(&plane.0),
        (point - reference).dot(&plane.1),
    )
}

/// Checks if a point is inside a triangle using barycentric coordinates.
/// # Arguments
/// * `p` - The point to check.
/// * `t` - A tuple of three vertices representing the triangle.
/// # Returns
/// * `bool` - `true` if the point is inside the triangle, `false` otherwise.
#[must_use]
#[inline]
pub fn is_point_inside_triangle(p: Vector3D, t: (Vector3D, Vector3D, Vector3D)) -> bool {
    let s1 = calculate_triangle_area((t.0, t.1, p));
    let s2 = calculate_triangle_area((t.1, t.2, p));
    let s3 = calculate_triangle_area((t.2, t.0, p));
    let st = calculate_triangle_area(t);
    (s1 + s2 + s3 - st).abs() < EPS
        && (0.0 - EPS..=st + EPS).contains(&s1)
        && (0.0 - EPS..=st + EPS).contains(&s2)
        && (0.0 - EPS..=st + EPS).contains(&s3)
}

/// Checks whether the element `a` lies within the range `(b..=c)` or `(c..=b)`.
/// # Arguments
/// * `a` - The element to check.
/// * `b` - The first boundary value.
/// * `c` - The second boundary value.
/// # Returns
/// * `bool` - `true` if `a` lies within the specified ranges, `false` otherwise.
#[must_use]
#[inline]
pub fn is_within_inclusive_range(a: f64, b: f64, c: f64) -> bool {
    if b < c {
        (b..=c).contains(&a)
    } else {
        (c..=b).contains(&a)
    }
}

/// Represents the type of intersection between lines.
#[derive(Debug, Clone, Copy)]
pub enum IntersectionType {
    Proper,
    Endpoint,
}

/// Calculates the intersection of two line segments in 2D space.
/// # Arguments
/// * `p_u` - First point of the first line segment.
/// * `p_v` - Second point of the first line segment.
/// * `q_u` - First point of the second line segment.
/// * `q_v` - Second point of the second line segment.
/// # Returns
/// * `Option<Intersection2D>` - The intersection point and type, or `None` if no intersection.
type Intersection2D = (Vector2D, IntersectionType);
#[must_use]
pub fn calculate_2d_lineseg_intersection(
    p_u: Vector2D,
    p_v: Vector2D,
    q_u: Vector2D,
    q_v: Vector2D,
) -> Option<Intersection2D> {
    let (x1, x2, x3, x4, y1, y2, y3, y4) = (p_u.x, p_v.x, q_u.x, q_v.x, p_u.y, p_v.y, q_u.y, q_v.y);

    let t_numerator = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let u_numerator = -(x1 - x2) * (y1 - y3) + (y1 - y2) * (x1 - x3);
    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if is_within_inclusive_range(t_numerator, 0.0, denominator)
        && is_within_inclusive_range(u_numerator, 0.0, denominator)
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

/// Calculates the intersection of two line segments in 3D space.
/// # Arguments
/// * `p_u` - First point of the first line segment.
/// * `p_v` - Second point of the first line segment.
/// * `q_u` - First point of the second line segment.
/// * `q_v` - Second point of the second line segment.
/// # Returns
/// * `Option<Intersection3D>` - The intersection point and type, or `None` if no intersection.
type Intersection3D = (Vector3D, IntersectionType);
#[must_use]
pub fn calculate_3d_lineseg_intersection(
    p_u: Vector3D,
    p_v: Vector3D,
    q_u: Vector3D,
    q_v: Vector3D,
) -> Option<Intersection3D> {
    if !are_points_coplanar(p_u, p_v, q_u, q_v) {
        return None;
    }

    let p = p_v - p_u;
    let q = q_v - q_u;
    let normal_vector = p.cross(&q).normalize();
    let reference_point = p_u;
    let plane = (p.normalize(), p.cross(&normal_vector).normalize());

    calculate_2d_lineseg_intersection(
        project_point_onto_plane(p_u, plane, reference_point),
        project_point_onto_plane(p_v, plane, reference_point),
        project_point_onto_plane(q_u, plane, reference_point),
        project_point_onto_plane(q_v, plane, reference_point),
    )
    .map(|(point_in_2d, intersection_type)| {
        let point_in_3d = reference_point + (plane.0 * point_in_2d.x) + (plane.1 * point_in_2d.y);
        (point_in_3d, intersection_type)
    })
}

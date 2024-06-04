#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub mod color;
pub mod consts;
pub mod geom;
pub mod graph;
pub mod math;
pub mod timer;

#[cfg(test)]
mod tests {

    use crate::consts::EPS;
    use crate::geom::calculate_2d_lineseg_intersection;
    use crate::geom::calculate_3d_lineseg_intersection;
    use crate::geom::is_point_inside_triangle;
    use crate::geom::IntersectionType::Endpoint;
    use crate::geom::IntersectionType::Proper;
    use crate::geom::Vector2D;
    use crate::geom::Vector3D;
    use approx::AbsDiffEq;

    #[test]
    fn test_inside_triangle() {
        let triangle = (
            Vector3D::new(0., 0., 0.),
            Vector3D::new(1., 0., 0.),
            Vector3D::new(0., 1., 0.),
        );
        let point = Vector3D::new(0.5, 0.5, 0.0);
        let epsilon_x = Vector3D::new(EPS, 0.0, 0.0);
        let epsilon_y = Vector3D::new(0.0, EPS, 0.0);
        let epsilon_z = Vector3D::new(0.0, 0.0, EPS);
        let tests = vec![
            (point, triangle, true),              // On the boundary
            (point + epsilon_x, triangle, false), // Outside the triangle (by epsilon)
            (point - epsilon_x, triangle, true),  // Inside the triangle (by epsilon)
            (point + epsilon_y, triangle, false), // Outside the triangle (by epsilon)
            (point - epsilon_y, triangle, true),  // Inside the triangle (by epsilon)
            (point + epsilon_z, triangle, false), // Outside the triangle (by epsilon in z axis)
            (point - epsilon_z, triangle, false), // Outside the triangle (by epsilon in z axis)
        ];

        for (point, triangle, expected) in tests {
            let inside = is_point_inside_triangle(point, triangle);
            assert_eq!(
                inside,
                expected,
                "inside_triangle({point:?}, {triangle:?}) = {inside:?}, but should be: {expected:?}"
            );
        }
    }

    #[test]
    fn test_intersection_in_2d() {
        let p1 = Vector2D::new(0., 0.);
        let p2 = Vector2D::new(1., 1.);
        let mid = Vector2D::new(0.5, 0.5);
        let p3 = Vector2D::new(1., 0.);
        let p4 = Vector2D::new(0., 1.);
        let epsilon_x = Vector2D::new(EPS, 0.0);
        let epsilon_y = Vector2D::new(0.0, EPS);
        let tests = vec![
            (p1, p2, p3, p4, Some((mid, Proper))),
            (p2, p1, p4, p3, Some((mid, Proper))),
            (p1, p3, p2, p4, None),
            (p3, p1, p4, p2, None),
            (p1, p3, p2, p3, Some((p3, Endpoint))),
            (p1, p3, p2, p3 + epsilon_y, None),
            (p1, p3, p2, p3 - epsilon_y, Some((p3, Endpoint))),
            (p1, p3, p2, p3 + epsilon_x, None),
            (p1, p3, p2, p3 - epsilon_x, Some((p3 - epsilon_x, Proper))),
            (p1, p3 + epsilon_y, p2, p3, Some((p3 + epsilon_y, Proper))),
            (p1, p3 - epsilon_y, p2, p3, None),
            (p1, p3 + epsilon_x, p2, p3, Some((p3, Endpoint))),
            (p1, p3 - epsilon_x, p2, p3, None),
        ];

        for (a, b, c, d, expected) in tests {
            let intersection = calculate_2d_lineseg_intersection(a, b, c, d);
            assert!(
                match (intersection, expected) {
                    (Some((a, _)), Some((b, _))) => a.abs_diff_eq(&b, EPS),
                    (None, None) => true,
                    _ => false,
                },
                "intersection({a:?}, {b:?}, {c:?}, {d:?}) = {intersection:?}, but should be: {expected:?}"
            );
        }
    }

    #[test]
    pub fn test_intersection_in_3d() {
        let p1 = Vector3D::new(0.0, 0.0, 0.);
        let p2 = Vector3D::new(1., 1., 0.);
        let mid = Vector3D::new(0.5, 0.5, 0.);
        let p3 = Vector3D::new(1., 0., 0.);
        let p4 = Vector3D::new(0., 1., 0.);
        let d1 = Vector3D::new(0., 0., 1.0);
        let d2 = Vector3D::new(0., 0., -1.0);
        let d3 = Vector3D::new(424242., 424242., 424242.0);
        let d4 = Vector3D::new(-424242., -424242., -424242.0);
        let epsilon_x = Vector3D::new(EPS, 0.0, 0.0);
        let epsilon_y = Vector3D::new(0.0, EPS, 0.0);
        let epsilon_z = Vector3D::new(0.0, 0.0, EPS);
        let tests = vec![
            (p1, p2, p3, p4, Some((mid, Proper))),
            (p2, p1, p4, p3, Some((mid, Proper))),
            (p1 + d1, p2 + d1, p3 + d1, p4 + d1, Some((mid + d1, Proper))),
            (p1 + d2, p2 + d2, p3 + d2, p4 + d2, Some((mid + d2, Proper))),
            (p1 + d3, p2 + d3, p3 + d3, p4 + d3, Some((mid + d3, Proper))),
            (p1 + d4, p2 + d4, p3 + d4, p4 + d4, Some((mid + d4, Proper))),
            (p1, p3, p2, p4, None),
            (p1, p3, p2, p3 - epsilon_y, Some((p3, Endpoint))),
            (p1, p3, p2, p3 + epsilon_x, None),
            (p1, p3, p2, p3 - epsilon_x, Some((p3 - epsilon_x, Proper))),
            (p1, p3 + epsilon_y, p2, p3, Some((p3 + epsilon_y, Proper))),
            (p1, p3 - epsilon_y, p2, p3, None),
            (p1, p3 + epsilon_x, p2, p3, Some((p3, Endpoint))),
            (p1, p3 - epsilon_x, p2, p3, None),
            (p1, p3 + epsilon_z, p2, p3, None),
            (p1, p3 - epsilon_z, p2, p3, None),
            (p1, p3, p2, p3 + epsilon_z, None),
            (p1, p3, p2, p3 - epsilon_z, None),
        ];

        for (a, b, c, d, expected) in tests {
            let intersection = calculate_3d_lineseg_intersection(a, b, c, d);
            assert!(
            match (intersection, expected) {
                (Some((a, _)), Some((b, _))) => a.abs_diff_eq(&b, EPS),
                (None, None) => true,
                _ => false,
            },
            "intersection({a:?}, {b:?}, {c:?}, {d:?}) = {intersection:?}, but should be: {expected:?}"
        );
        }
    }
}

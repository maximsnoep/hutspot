use glam::Vec3;

// (p * s) + t = p'
#[must_use]
pub fn transform_coordinates<T>(translation: T, scale: f32, position: T) -> Vec3
where
    Vec3: From<T>,
{
    (Vec3::from(position) * scale) + Vec3::from(translation)
}

pub struct DrawableLine {
    pub u: Vec3,
    pub v: Vec3,
}

pub fn draw_line(p: Vec3, q: Vec3, translation: Vec3, scale: f32) -> DrawableLine {
    DrawableLine {
        u: transform_coordinates(translation, scale, p),
        v: transform_coordinates(translation, scale, q),
    }
}

pub fn draw_vertex(
    point: Vec3,
    normal: Vec3,
    translation: Vec3,
    scale: f32,
    length: f32,
) -> DrawableLine {
    let u = transform_coordinates(translation, scale, point);
    let v = transform_coordinates(translation, scale, point + normal);

    draw_line(u, v, translation, scale)
}

pub fn draw_arrow(u: Vec3, v: Vec3, n: Vec3, translation: Vec3, scale: f32) -> [DrawableLine; 3] {
    let forward = v - u;
    let backward = u - v;

    let cross = forward.cross(n).normalize() * backward.length();

    // height of wing
    const W1: f32 = 0.3;
    // width of wing
    const W2: f32 = 0.1;
    let wing1 = W1 * backward + W2 * cross;
    let wing2 = W1 * backward - W2 * cross;

    [
        draw_line(u, v, translation, scale),
        draw_line(v, v + wing1, translation, scale),
        draw_line(v, v + wing2, translation, scale),
    ]
}

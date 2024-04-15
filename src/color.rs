use bevy_render::color::Color;

// CB dark red: rgb(226, 26, 27) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_X: Color = Color::rgb(0.8862745098, 0.10196078431, 0.10588235294);
// CB dark blue: rgb(30, 119, 179) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_Y: Color = Color::rgb(0.11764705882, 0.46666666666, 0.70196078431);
// CB yellow: rgb(255, 215, 0) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_Z: Color = Color::rgb(1., 1., 0.6);

// CB light red: rgb(250, 153, 153) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_X_LIGHT: Color = Color::rgb(0.98039215686, 0.6, 0.6);
// CB light blue: rgb(166, 205, 226) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_Y_LIGHT: Color = Color::rgb(0.65098039215, 0.80392156862, 0.8862745098);
// white: rgb(255, 255, 255) ranged from 0.0 to 1.0
pub const COLOR_PRIMARY_Z_LIGHT: Color = Color::rgb(1.0, 1.0, 0.8);

// others
pub const COLOR_SECONDARY_X: Color = Color::hsl((61. / 239.) * 360., 0.8, 0.7);
pub const COLOR_SECONDARY_Y: Color = Color::hsl((23. / 239.) * 360., 0.9, 0.7);
pub const COLOR_SECONDARY_Z: Color = Color::hsl((187. / 239.) * 360., 0.6, 0.7);

// others
pub const COLOR_SECONDARY_X_LIGHT: Color = Color::hsl((61. / 239.) * 360., 0.8, 0.9);
pub const COLOR_SECONDARY_Y_LIGHT: Color = Color::hsl((23. / 239.) * 360., 0.9, 0.9);
pub const COLOR_SECONDARY_Z_LIGHT: Color = Color::hsl((187. / 239.) * 360., 0.6, 0.9);

// Parula colormap
// 53, 42, 134
pub const PARULA_1: Color = Color::rgb(35. / 255., 42. / 255., 134. / 255.);
// 53, 60, 172
pub const PARULA_2: Color = Color::rgb(53. / 255., 60. / 255., 172. / 255.);
// 31, 82, 211
pub const PARULA_3: Color = Color::rgb(31. / 255., 82. / 255., 211. / 255.);
// 4, 108, 224
pub const PARULA_4: Color = Color::rgb(4. / 255., 108. / 255., 224. / 255.);
// 16, 120, 218
pub const PARULA_5: Color = Color::rgb(16. / 255., 120. / 255., 218. / 255.);
// 20, 132, 211
pub const PARULA_6: Color = Color::rgb(20. / 255., 132. / 255., 211. / 255.);
// 8, 152, 209
pub const PARULA_7: Color = Color::rgb(8. / 255., 152. / 255., 209. / 255.);
// 37, 180, 169
pub const PARULA_8: Color = Color::rgb(37. / 255., 180. / 255., 169. / 255.);
// 9, 171, 189
pub const PARULA_9: Color = Color::rgb(9. / 255., 171. / 255., 189. / 255.);
// 37, 180, 169
pub const PARULA_10: Color = Color::rgb(37. / 255., 180. / 255., 169. / 255.);
// 65, 186, 151
pub const PARULA_11: Color = Color::rgb(65. / 255., 186. / 255., 151. / 255.);
// 112, 190, 128
pub const PARULA_12: Color = Color::rgb(112. / 255., 190. / 255., 128. / 255.);
// 145, 190, 114
pub const PARULA_13: Color = Color::rgb(145. / 255., 190. / 255., 114. / 255.);
// 174, 189, 103
pub const PARULA_14: Color = Color::rgb(174. / 255., 189. / 255., 103. / 255.);
// 208, 186, 89
pub const PARULA_15: Color = Color::rgb(208. / 255., 186. / 255., 89. / 255.);
// 233, 185, 78
pub const PARULA_16: Color = Color::rgb(233. / 255., 185. / 255., 78. / 255.);
// 253, 190, 61
pub const PARULA_17: Color = Color::rgb(253. / 255., 190. / 255., 61. / 255.);
// 249, 210, 41
pub const PARULA_18: Color = Color::rgb(249. / 255., 210. / 255., 41. / 255.);
// 244, 228, 28
pub const PARULA_19: Color = Color::rgb(244. / 255., 228. / 255., 28. / 255.);

pub const PARULA: [Color; 19] = [
    PARULA_1, PARULA_2, PARULA_3, PARULA_4, PARULA_5, PARULA_6, PARULA_7, PARULA_8, PARULA_9,
    PARULA_10, PARULA_11, PARULA_12, PARULA_13, PARULA_14, PARULA_15, PARULA_16, PARULA_17,
    PARULA_18, PARULA_19,
];

pub fn color_map(value: f32, colors: Vec<Color>) -> Color {
    let index = (value * (colors.len() - 1) as f32).round() as usize;
    colors[index]
}

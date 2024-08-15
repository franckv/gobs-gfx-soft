use glam::Vec3;

pub fn to_screen_coord(pos: Vec3, width: u32, height: u32) -> Vec3 {
    // x: -1 1 -> 0 .. width
    // y: -1 1 -> height .. 0

    let x = (0.5 + 0.5 * pos.x) * (width - 1) as f32;
    let y = (0.5 - 0.5 * pos.y) * (height - 1) as f32;
    let z = 0.;

    Vec3::new(x, y, z)
}

use glam::{IVec2, Vec3};

pub fn to_screen_coord(pos: Vec3, width: u32, height: u32) -> IVec2 {
    // x: -1 1 -> 0 .. width
    // y: -1 1 -> height .. 0

    let x = (0.5 + 0.5 * pos.x) * (width - 1) as f32;
    let y = (0.5 - 0.5 * pos.y) * (height - 1) as f32;

    IVec2::new(x as i32, y as i32)
}

pub fn edge(v0: IVec2, v1: IVec2, v2: IVec2) -> i32 {
    (v1.x as i32 - v0.x as i32) * (v2.y as i32 - v0.y as i32)
        - (v1.y as i32 - v0.y as i32) * (v2.x as i32 - v0.x as i32)
}

/// Triangle winding: true if CW, false if CCW
pub fn winding(v0: IVec2, v1: IVec2, v2: IVec2) -> bool {
    edge(v0, v1, v2) > 0
}

/// true if P is inside (v0, v1, v2)
/// (v0, v1, v2) must be CW
pub fn is_inside(v0: IVec2, v1: IVec2, v2: IVec2, p: IVec2) -> bool {
    let w = barycentric_coords(v0, v1, v2, p);

    w.x > 0. && w.y > 0. && w.z > 0.
}

pub fn barycentric_coords(v0: IVec2, v1: IVec2, v2: IVec2, p: IVec2) -> Vec3 {
    let w = edge(v0, v1, v2) as f32;
    let w0 = edge(v1, v2, p) as f32 / w;
    let w1 = edge(v2, v0, p) as f32 / w;
    let w2 = edge(v0, v1, p) as f32 / w;

    Vec3::new(w0, w1, w2)
}

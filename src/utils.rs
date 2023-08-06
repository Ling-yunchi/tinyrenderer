use glam::{IVec2, Vec2};

#[inline]
pub fn f2i_2(v: Vec2) -> IVec2 {
    (v.x as i32, v.y as i32).into()
}

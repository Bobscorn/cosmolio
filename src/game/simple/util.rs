use bevy::{prelude::*, window::PrimaryWindow};


pub fn get_screenspace_cursor_pos(
    window: &Window,
    camera: &Camera,
    camera_trans: &GlobalTransform
) -> Option<Vec2>
{
    let Some(window_pos) = window.cursor_position() else { return None; };

    let Some(camera_ray) = camera.viewport_to_world(camera_trans, window_pos) else { return None; };

    Some(camera_ray.origin.truncate())
}

pub fn get_screenspace_cursor_pos_from_queries(
    window_q: &Query<&Window, With<PrimaryWindow>>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let Ok(window) = window_q.get_single() else { return None; };
    let Ok((camera, camera_trans)) = camera_q.get_single() else { return None; };

    return get_screenspace_cursor_pos(window, camera, camera_trans);
}

pub fn get_direction_to_cursor(
    window_q: &Query<&Window, With<PrimaryWindow>>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
    position: Vec2
) -> Option<Vec2> {
    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(window_q, camera_q) else { return None; };

    return Some(cursor_pos - position);
}

pub trait ReflectVecExt
{
    /// Reflect self along an axis vector as if the axis vector was a line of symmetry
    fn reflect_along(&self, axis: Vec2) -> Vec2;
    /// Reflect the part of self that aligns with dir (using dot product)
    fn reflect_against(&self, dir: Vec2) -> Vec2;
}

impl ReflectVecExt for Vec2
{
    fn reflect_along(&self, axis: Vec2) -> Vec2 {
        let reflect_vector = axis.extend(0.0).cross(Vec3::Z).truncate().normalize_or_zero();

        *self - 2.0 * reflect_vector * (reflect_vector.dot(*self))
    }

    fn reflect_against(&self, dir: Vec2) -> Vec2 {
        *self - 2.0 * dir * (dir.dot(*self))
    }
}


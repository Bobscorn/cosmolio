use bevy::prelude::*;


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


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


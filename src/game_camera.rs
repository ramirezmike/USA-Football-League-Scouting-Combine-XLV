use crate::{player};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::{PerspectiveProjection, ScalingMode};

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

pub fn follow_player(
    mut cameras: Query<&mut Transform, (With<PanOrbitCamera>, Without<player::Player>)>,
    players: Query<&Transform, With<player::Player>>,
) {
    for mut camera_transform in cameras.iter_mut() {
        for player_transform in players.iter() {
            camera_transform.translation.x = player_transform.translation.x - 5.0;
            camera_transform.translation.z = player_transform.translation.z + 5.0;
        }
    }
}

pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let orbit_key = KeyCode::LShift;
    let pan_button = MouseButton::Middle;
    let pan_key = KeyCode::LAlt;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) || keyboard_input.pressed(orbit_key) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) || keyboard_input.pressed(pan_key) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button)
        || input_mouse.just_pressed(orbit_button)
        || keyboard_input.just_released(orbit_key)
        || keyboard_input.just_pressed(orbit_key)
    {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            //          let window = get_primary_window_size(&windows);
            //          pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            //          // translate by local axes
            //          let right = transform.rotation * Vec3::X * -pan.x;
            //          let up = transform.rotation * Vec3::Y * pan.y;
            //          // make panning proportional to distance away from focus point
            //          let translation = (right + up) * pan_orbit.radius;
            //          pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}

pub fn spawn_camera<T: Component + Clone>(commands: &mut Commands, cleanup_marker: T) {
    let translation = Vec3::new(-5.0, 5.0, 0.0);

    let radius = translation.length();
    println!("Spawning camera");
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-18.0, 16.0, -0.2).looking_at(Vec3::ZERO, Vec3::Y),
//      projection: OrthographicProjection {
//          scale: 10.0,
//          scaling_mode: ScalingMode::FixedVertical(1.0),
//          near: -100.0,
//          ..default()
//      }.into(),
        ..default()
    })
    .insert(cleanup_marker.clone())
    .insert(PanOrbitCamera {
        radius,
        ..Default::default()
    });

    const HALF_SIZE: f32 = 100.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            illuminance: 10000.0,
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(cleanup_marker);
}


use crate::{player, LEFT_GOAL, assets::GameAssets};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use std::f32::consts::{TAU, PI};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::camera::{PerspectiveProjection, ScalingMode, RenderTarget};
use bevy::render::{
    view::RenderLayers,
};

#[derive(Component)]
pub struct HostCamera;

pub const INGAME_CAMERA_X: f32 = -32.2; 
pub const INGAME_CAMERA_Y: f32 = 14.0; 
pub const INGAME_CAMERA_ROTATION_AXIS: Vec3 = Vec3::new(-0.2211861, -0.9493068, -0.22336805);
pub const INGAME_CAMERA_ROTATION_ANGLE: f32 = 1.6325973;

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
    mut cameras: Query<&mut Transform, With<PanOrbitCamera>>,
    players: Query<&Transform, (With<player::Player>, Without<PanOrbitCamera>)>,
    time: Res<Time>,
) {
    let camera_speed = 1.2;
    for mut camera_transform in cameras.iter_mut() {
        for player_transform in players.iter() {
            camera_transform.translation.z += 
                (player_transform.translation.z - camera_transform.translation.z)
                * camera_speed
                * time.delta_seconds();

        }
    }
}

pub fn handle_will_camera( 
    mut will_camera: Query<(&mut Transform, &HostCamera)>,
    time: Res<Time>,
) {
//  for (mut transform, _) in &mut will_camera {
//      transform.rotate_y(time.delta_seconds());
//  }
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
//        println!("C: {:?}", transform.rotation.to_axis_angle());
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

pub fn spawn_camera<T: Component + Clone>(
    commands: &mut Commands, cleanup_marker: T,
    game_assets: &Res<GameAssets>,
    translation: Vec3,
    rotation: Quat,
) {
    let radius = translation.length();

//  let first_pass_layer = RenderLayers::layer(1);
//  // Will Camera
//  commands.spawn_bundle(Camera3dBundle {
//      transform: {
//          let mut t = Transform::from_xyz(21.5, 2.0, 1.0);
//          t.rotation = Quat::from_rotation_y(TAU * 0.75);
//          t
//      },
//      camera: Camera {
//          priority: -1,
//          target: RenderTarget::Image(game_assets.will_camera.clone()),
//          ..default()
//      },
//      camera_3d: Camera3d {
//          clear_color: ClearColorConfig::Default,
//          ..default()
//      },
//      ..default()
//  })
//  .insert(UiCameraConfig {
//      show_ui: false,
//  })
//  .insert(HostCamera)
//  .insert(cleanup_marker.clone());

    println!("Spawning camera");
    commands.spawn_bundle(Camera3dBundle {
        transform: {
            let mut t = Transform::from_translation(translation);
            t.rotation = rotation;

            t
        },
        camera: Camera {
            priority: 0,
            ..default()
        },
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

    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            priority: 1,
            ..default()
        },
        ..default()
    })
    .insert(cleanup_marker.clone());
//    .insert(first_pass_layer);

    const HALF_SIZE: f32 = 100.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
//            illuminance: 10000.0,
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
            rotation: Quat::from_rotation_x(0.80 * TAU),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(cleanup_marker);
}


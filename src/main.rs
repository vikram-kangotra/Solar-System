mod solar_system;

use bevy::{input::{keyboard, mouse::MouseMotion}, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use solar_system::SolarSystemPlugin;

#[derive(Component)]
struct MyCamera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((DefaultPlugins, SolarSystemPlugin))
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, navigation)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                ..Default::default()
            }.into(),
            transform: Transform::from_translation(Vec3::new(10.0, 30.0, 50.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MyCamera,
    ));
}

fn cursor_grab(mut windows: Query<&mut Window, With<PrimaryWindow>>) {

    let mut primary_window = windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn navigation(
    mut keyboard_evr: EventReader<keyboard::KeyboardInput>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<MyCamera>>,
) {

    let mut camera_transform = camera.single_mut();

    for motion in motion_evr.read() {
        camera_transform.rotate(Quat::from_rotation_x(-motion.delta.y * 5e-4));
        camera_transform.rotate(Quat::from_rotation_y(-motion.delta.x * 5e-4));
    }

    for event in keyboard_evr.read() {
        let mut translation = Vec3::ZERO;
        if event.state.is_pressed() {
            match event.key_code {
                Some(keyboard::KeyCode::W) => translation.z -= 1.0,
                Some(keyboard::KeyCode::A) => translation.x -= 1.0,
                Some(keyboard::KeyCode::S) => translation.z += 1.0,
                Some(keyboard::KeyCode::D) => translation.x += 1.0,
                Some(keyboard::KeyCode::Space) => translation.y += 1.0,
                _ => {}
            }
        }
        let translation = camera_transform.rotation * translation;
        camera_transform.translation += translation;
    }
}

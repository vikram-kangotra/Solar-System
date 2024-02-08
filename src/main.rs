mod solar_system;

use bevy::prelude::*;
use solar_system::SolarSystemPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SolarSystemPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let background_image: Handle<Image> = asset_server.load("textures/stars_milky_way.jpg");

    commands.spawn(Camera2dBundle::default())
        .insert(SpriteBundle {
            texture: background_image,
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            ..Default::default()
        });
}

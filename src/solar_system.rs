use bevy::prelude::*;

#[derive(Component)]
struct Planet;

#[derive(Component)]
enum PlanetType {
    Sun,
    Mercury,
    Venus,
    Earth,
    Mars,
}

#[derive(Component)]
struct Velocity(Vec3);

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_sun, add_planet, add_star))
            .add_systems(Update, update_position);
    }
}

fn add_star(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let star_handle: Handle<Image> = server.load("textures/stars_milky_way.jpg");

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1000.0,
            sectors: 50,
            stacks: 50,
        })),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(star_handle),
            unlit: true,
            cull_mode: None,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn add_sun(
    mut commands: Commands, 
    server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sun_handle: Handle<Image> = server.load("textures/sun.jpg");

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere {
                radius: 5.0,
                sectors: 50,
                stacks: 50,
            }.into()),
            material: materials.add(
                StandardMaterial {
                    base_color_texture: Some(sun_handle),
                    emissive: Color::WHITE,
                    unlit: true,
                    ..Default::default()
                }
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Planet,
        PlanetType::Sun,
    ));

    commands.spawn(
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            point_light: PointLight {
                intensity: 10000.0,
                range: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        }
    );
}

fn add_planet(
    mut commands: Commands, 
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let mercury_handle: Handle<Image> = server.load("textures/mercury.jpg");
    let venus_handle: Handle<Image> = server.load("textures/venus.jpg");
    let earth_handle: Handle<Image> = server.load("textures/earth.jpg");
    let mars_handle: Handle<Image> = server.load("textures/mars.jpg");

    let planets = vec![
        (PlanetType::Mercury, mercury_handle, 1.0, Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 10.)),
        (PlanetType::Venus, venus_handle, 2.5, Vec3::new(20.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 7.)),
        (PlanetType::Earth, earth_handle, 3.0, Vec3::new(30.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 6.)),
        (PlanetType::Mars, mars_handle, 2.0, Vec3::new(40.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 5.)),
    ];

    for (planet_type, texture, size, position, velocity) in planets {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius: size,
                    sectors: 50,
                    stacks: 50,
                }.into()),
                material: materials.add(
                    StandardMaterial {
                        base_color_texture: Some(texture),
                        ..Default::default()
                    }
                ),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Planet,
            planet_type,
            Velocity(velocity),
        ));
    }
}

fn update_position(
    mut query: Query<(&mut Transform, &PlanetType, &mut Velocity), With<Planet>>,
    time: Res<Time>
) {

    let sun_position = Vec3::ZERO;
    let sun_mass = 1e6;
    let gravitational_constant = 1e-5;

    for (mut transform, planet_type, mut velocity) in query.iter_mut() {
        let rotation_speed = match planet_type {
            PlanetType::Mercury => 0.01,
            PlanetType::Venus => 0.005,
            PlanetType::Earth => 0.003,
            PlanetType::Mars => 0.002,
            _ => 0.0,
        };

        transform.rotate(Quat::from_rotation_y(rotation_speed));

        let distance = sun_position - transform.translation;
        let direction = distance.normalize();

        let acceleration = gravitational_constant * sun_mass / distance.length_squared() * direction;

        velocity.0 += acceleration;

        transform.translation += velocity.0 * time.delta_seconds();
    }
}

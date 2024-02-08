use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
        app.add_systems(Startup, add_planet)
            .add_systems(Update, update_position);
    }
}

fn add_planet(
    mut commands: Commands, 
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let sun_handle: Handle<Image> = server.load("textures/sun.jpg");
    let mercury_handle: Handle<Image> = server.load("textures/mercury.jpg");
    let venus_handle: Handle<Image> = server.load("textures/venus.jpg");
    let earth_handle: Handle<Image> = server.load("textures/earth.jpg");
    let mars_handle: Handle<Image> = server.load("textures/mars.jpg");

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(sun_handle.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    })
    .insert(Planet)
    .insert(PlanetType::Sun);

    let planets = vec![
        (10., PlanetType::Mercury, 100.0, Vec3::new(0., 3., 0.), mercury_handle),
        (14., PlanetType::Venus, 200.0, Vec3::new(0., 2., 0.), venus_handle),
        (20., PlanetType::Earth, 300.0, Vec3::new(0., 1.5, 0.), earth_handle),
        (16., PlanetType::Mars, 400.0, Vec3::new(0., 1.5, 0.), mars_handle),
    ];

    for (size, planet_type, distance, velocity, texture) in planets {
        commands.spawn((
            Planet, 
            planet_type, 
            Velocity(velocity), 
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size).into()).into(),
                material: materials.add(texture.into()),
                transform: Transform::from_translation(Vec3::new(distance, 0.0, 0.0)),
                ..Default::default()
            })
        );
    }
}

fn update_position(mut query: Query<(&mut Transform, &PlanetType, &mut Velocity), With<Planet>>) {

    let sun_position = Vec3::ZERO;
    let sun_mass = 1e6;
    let gravitational_constant = 1e-3;

    for (mut transform, _, mut velocity) in query.iter_mut() {
        let distance = sun_position - transform.translation;
        let direction = distance.normalize();
        let acceleration = gravitational_constant * sun_mass * direction / distance.length_squared();
        velocity.0 += acceleration;
        transform.translation += velocity.0;
    }
}

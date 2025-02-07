//The world chages automatic by the modifications of comoponents 
//Comands are instructions that changue the Word direcly like spawn a new Entities (Bundle is the template of an Entity)
//System are statless functions, 
//they are triggered by schedules some are Setup and Update
//A schedule is a collection od systems, with metadata of how they should run
//and the logic of executor algorithm to run the systems
use avian3d::prelude::*;
use bevy::prelude::*;
//Define a structure for an Entity (Bundle)
#[derive(Component)] 
struct Player;
#[derive(Component)] 
struct PlayerCamera;

fn main() {
    App::new()
        // Enable physics
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_player)
        .add_systems(Update, move_player)
        //.add_systems(Update, follow_player_camera)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(10.0, 0.1),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(10.0, 0.1)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn((PlayerCamera, 
        Camera3dBundle {
            //transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
            transform: Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        }
    ));
}

//Player System
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((Player,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        //AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(0, 0, 255)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));
}

//Input system
const MOVE_SPEED: f32 = 2.0;
fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>, //Obten todos los componentes tipo transform dentro de las Entidades tipo Player
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keys.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize_or_zero() * time.delta_seconds();
        }
    }
}

// fn follow_player_camera(
//     mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
//     //mut player_query: Query<&mut Transform, With<Player>>,
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let mut camera_transform = match camera_query.get_single_mut() {
//         Ok(transform) => transform,
//         Err(_) => return, // Salir si no se encuentra la cámara
//     };

//     // let mut player_transform = match player_query.get_single_mut() {
//     //     Ok(transform) => transform,
//     //     Err(_) => return, // Salir si no se encuentra el jugador
//     // };

//     // Movimiento hacia adelante/atrás del jugador
//     let mut forward_back_input = 0.0;
//     if keys.pressed(KeyCode::KeyW) {
//         forward_back_input += 1.0; // Mover hacia adelante
//     }
//     if keys.pressed(KeyCode::KeyS) {
//         forward_back_input -= 1.0; // Mover hacia atrás
//     }

//     if forward_back_input != 0.0 {
//         let forward = (camera_transform.translation - player_transform.translation).normalize();
//         player_transform.translation += forward * forward_back_input * MOVE_SPEED * time.delta_seconds();
//     }

//     // Actualizar la orientación de la cámara para mirar al jugador
//     camera_transform.look_at(player_transform.translation, Vec3::Y);
// }

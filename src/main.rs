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
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
     // Static physics object with a collision shape
     commands.spawn((
        RigidBody::Static,
        Collider::cylinder(15.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(15.0, 0.1))),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba
             { red: (0.0), green: (155.0), blue: (0.0), alpha: (255.0) }))),
    ));

     // Light
     commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    //     PlayerCamera,
    // ));
}

//Player System
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // Player blue cube Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        //Collider::cuboid(1.0, 1.0, 1.0),
        Collider::capsule(0.5, 1.5),
        //Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(224, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
        Player
    ))
    .with_children(|parent| {
        // child cube
        // Camera
        parent.spawn((
            Camera3d::default(),
            //Transform::from_xyz(-1.5, 4.0, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
            Transform::from_xyz(0.0, 4.0, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
            PlayerCamera,
        ));
        // // child cube
        // parent.spawn((
        //     Mesh3d(cube_handle),
        //     MeshMaterial3d(cube_material_handle),
        //     Transform::from_xyz(0.0, 0.0, 3.0),
        // ));
    });
}

//Input system
const MOVE_SPEED: f32 = 3.0;
fn move_player(
    mut player_transforms: Query<&mut Transform, With<Player>>, //Obten todos los componentes tipo transform dentro de las Entidades tipo Player
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in player_transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keys.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize_or_zero() * time.delta_secs();
        }
    }
}

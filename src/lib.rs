use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use heron::rapier_plugin::{
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::{ColliderBuilder, ColliderSet},
    },
    BodyHandle,
};
use heron::*;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: EventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub sprint_mult: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00006,
            speed: 12.,
            sprint_mult: 1.5,
        }
    }
}

/// Used in queries when you want FPS controllers
pub struct FPSController;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
) {
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .restrict_rotations(false, false, false)
        .build();
    let collider = ColliderBuilder::capsule_y(1.0, 1.0).build();
    let rigid_body = rigid_bodies.insert(rigid_body);
    let collider = colliders.insert(collider, rigid_body, &mut rigid_bodies);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                ..Default::default()
            });
        })
        .with(BodyHandle::new(rigid_body, collider))
        .with(Velocity::from(Vec3::zero()))
        .with(FPSController);
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<(&mut Transform, &mut Velocity), With<FPSController>>,
) {
    let window = windows.get_primary().unwrap();
    for (transform, mut vel) in query.iter_mut() {
        let mut velocity = Vec3::zero();
        let forward = -Vec3::new(transform.forward().x, 0., transform.forward().z);
        let right = Vec3::new(transform.forward().z, 0., -transform.forward().x);
        let mut sprint = false;
        for key in keys.get_pressed() {
            if window.cursor_locked() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::LShift => sprint = true,
                    KeyCode::Space => velocity += Vec3::unit_y(),
                    _ => (),
                }
            }
        }

        velocity = velocity.normalize();
        if sprint {
            velocity = velocity * settings.sprint_mult;
        }
        if !velocity.is_nan() {
            velocity *= time.delta_seconds() * settings.speed;
            let velocity: Velocity = velocity.into();
            vel.linear += velocity.linear;
        }
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&mut Transform, &Children), With<FPSController>>,
    mut camera_query: Query<&mut Transform, Without<FPSController>>,
) {
    let window = windows.get_primary().unwrap();
    for (mut transform, children) in query.iter_mut() {
        let mut camera_transform = camera_query.get_mut(children[0]).unwrap();
        for ev in state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                state.pitch -= (settings.sensitivity * ev.delta.y * window.height()).to_radians();
                state.yaw -= (settings.sensitivity * ev.delta.x * window.width()).to_radians();
            }
            state.pitch = state.pitch.clamp(-1.54, 1.54);
            camera_transform.rotation = Quat::from_axis_angle(Vec3::unit_x(), state.pitch);
            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), state.yaw);
        }
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}

/// Contains everything needed to add first-person-shooter player to your game
pub struct FPSControllerPlugin;
impl Plugin for FPSControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player.system())
            .add_startup_system(initial_grab_cursor.system())
            .add_system(player_look.system())
            .add_system(player_move.system())
            .add_system(cursor_grab.system());
    }
}

/// Same as `FPSControllerPlugin` but does not spawn a player
pub struct NoControllerPlugin;
impl Plugin for NoControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(initial_grab_cursor.system())
            .add_system(player_move.system())
            .add_system(player_look.system())
            .add_system(cursor_grab.system());
    }
}

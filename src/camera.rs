use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraEvent>()
            .insert_resource(CameraLerpFactor(0.8))
            .insert_resource(CursorSpeed(0.1))
            .add_systems(Startup, (spawn_cursor, spawn_camera))
            .add_systems(PreUpdate, (process_camera_events,))
            .add_systems(Update, (update_cursor_position, move_towards_target));
    }
}

/// The camera interpolation factor.
#[derive(Debug, Default, Resource)]
struct CameraLerpFactor(f32);

/// An enum of camera events.
#[derive(Debug, Event)]
pub enum CameraEvent {
    Target(Entity),
}

fn process_camera_events(
    mut commands: Commands,
    mut events: EventReader<CameraEvent>,
    target: Query<Entity, With<CameraTarget>>,
) {
    for event in events.read() {
        match event {
            CameraEvent::Target(entity) => {
                // remove the CameraTarget component from all entities
                commands.entity(target.single()).remove::<CameraTarget>();
                commands.entity(*entity).insert(CameraTarget);
            }
        }
    }
}

/// Mark this entity as a camera target.
#[derive(Debug, Component)]
pub struct CameraTarget;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Move the camera towards the target.
fn move_towards_target(
    factor: Res<CameraLerpFactor>,
    target: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = camera.single_mut();
    let target = target.single();

    let dist = (target.translation - camera.translation).length();
    if dist < 0.1 {
        return;
    }

    camera.translation = camera.translation.lerp(target.translation, factor.0);
}

/// The cursor's speed.
#[derive(Debug, Default, Resource)]
struct CursorSpeed(f32);

/// The cursor's virtual position.
#[derive(Debug, Component)]
struct Cursor;

#[derive(Debug, Bundle)]
struct CursorBundle {
    pub cursor: Cursor,
    pub transform: Transform,
}

fn spawn_cursor(mut commands: Commands) {
    commands
        .spawn(CursorBundle {
            cursor: Cursor,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
        })
        .insert(CameraTarget);
}

fn update_cursor_position(
    speed: Res<CursorSpeed>,
    mut mouse_ev: EventReader<MouseMotion>,
    mut cursor: Query<&mut Transform, With<Cursor>>,
) {
    let mut cursor = cursor.single_mut();
    for event in mouse_ev.read() {
        cursor.translation.x += event.delta.x * speed.0;
        cursor.translation.y -= event.delta.y * speed.0;
    }
}

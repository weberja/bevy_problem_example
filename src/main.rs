use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_flycam::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin::default()),
            LogDiagnosticsPlugin::default(),
        ))
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, draw_gizmo))
        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    let pin = commands
        .spawn(SceneBundle {
            scene: ass.load("pin.glb#Scene0"),
            ..Default::default()
        })
        .id();

    let note = commands
        .spawn(SceneBundle {
            scene: ass.load("note.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..Default::default()
        })
        .id();

    commands.entity(pin).add_child(note);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
    ));
}

fn rotate(
    camera: Query<&GlobalTransform, With<Camera>>,
    mut notes: Query<(&mut Transform, &GlobalTransform), Without<Camera>>,
) {
    let target = camera.single();
    for (mut start, start_glob) in notes.iter_mut() {
        let up = start.up();
        start.look_at(target.translation(), up);
    }
}

fn draw_gizmo(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Transform), Without<Camera>>) {
    for (glob_trans, transform) in query.iter() {
        gizmos.ray(glob_trans.translation(), transform.forward(), Color::GREEN);
    }
}

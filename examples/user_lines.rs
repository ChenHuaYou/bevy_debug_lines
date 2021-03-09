use bevy::prelude::*;

use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines, Line };

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut lines: ResMut<DebugLines>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        });


    // User lines are not cleared every frame, so we only need to push them once.  We are
    // also responsible for removing them, however.
    let line1 = Line::new(Vec3::new(-2.0, -1.0, 0.0), Vec3::new(2.0, -1.0, 0.0), 0.01, Color::PINK, Color::GOLD);
    let line2 = Line::new(Vec3::new(-2.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0), 0.01, Color::YELLOW, Color::GREEN);
    let line3 = Line::new(Vec3::new(-2.0, 1.0, 0.0), Vec3::new(2.0, 1.0, 0.0), 0.01, Color::RED, Color::TEAL);
    lines.user_lines.push(line1);
    lines.user_lines.push(line2);
    lines.user_lines.push(line3);
}

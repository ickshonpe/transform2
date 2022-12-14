use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use transform2::Transform2;
use transform2::Transform2Plugin;

#[derive(Component)]
struct OuterMarker;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let size = Vec2::splat(16.0);
    let inner_translation = size * 0.5;
    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
    for (i, color) in colors.into_iter().enumerate() {
        let rotation = i as f32 * PI;
        let inner = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(0.25 * size),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Transform2::from_translation(inner_translation).with_depth(1.0))
            .id();
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    anchor: Anchor::BottomRight,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Transform2::from_rotation(rotation))
            .insert(OuterMarker)
            .add_child(inner);
    }
}

fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut transform2s: Query<&mut Transform2, With<OuterMarker>>,
) {
    let a = PI * time.delta_seconds();
    transform2s.for_each_mut(|mut tf2| {
        if keys.pressed(KeyCode::Z) {
            tf2.rotation -= a;
        }
        if keys.pressed(KeyCode::X) {
            tf2.rotation += a;
        }

        if keys.pressed(KeyCode::S) {
            tf2.scale += 3.0 * time.delta_seconds();
        }

        if keys.pressed(KeyCode::A) {
            tf2.scale -= 3.0 * time.delta_seconds();
        }
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(Transform2Plugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

use std::f32::consts::PI;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use transform_2::GlobalTransform2;
use transform_2::Transform2;
use transform_2::Transform2Plugin;

#[derive(Component)]
struct OuterMarker;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    
    let size = Vec2::splat(16.0);
    let inner_translation = size * 0.5;
    let colors = [
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::YELLOW,
    ];
    for _ in 0..10_000 {
        let mut rotation = 0.0;
        for color in colors.into_iter() {
            let inner =
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(0.25 * size),
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Transform2::from_translation(inner_translation).with_depth(1.0))
               // .insert(GlobalTransform2::default())
                .id();
            let outer = commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(size),
                        anchor: Anchor::BottomRight,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Transform2::from_rotation(rotation))
             //   .insert(GlobalTransform2::default())
                .insert(OuterMarker)
                .add_child(inner);
            rotation += 0.5 * PI;
        }
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
    .insert_resource(Msaa { samples: 1 })
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowDescriptor {
        present_mode: bevy::window::PresentMode::Immediate,
        ..Default::default()
    })
    
    .add_plugins(DefaultPlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(Transform2Plugin)
    .add_startup_system(
        setup 
    )
    .add_system(
        update 
    )
    .run();
}
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,setup);
    app.run();
    println!("Hello, world!");
}

fn setup(mut commands: Commands) {
    // カメラ
    commands.spawn(Camera2d);

    // 床
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(200.0, 10.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.2),
            custom_size: Some(Vec2::new(400.0,20.0)),
            ..default()
        }
    ));

    // 落ちる箱
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        Transform::from_xyz(0.0, 100.0, 0.0),
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.9),
            custom_size: Some(Vec2::new(20.0,20.0)),
            ..default()
        }
    ));
}

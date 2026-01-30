use bevy::{
    prelude::*,
    camera::ScalingMode,
    color::palettes::tailwind::*
};

fn main() -> AppExit{
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, ball_movement)
        .run()
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                CANVAS_SIZE.x + 4.,
                CANVAS_SIZE.y +4.,
            )),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0.,0.,-3.),
    ));

    commands.spawn(
        (
            Sprite {
                custom_size: Some(CANVAS_SIZE),
                color: Color::from(SKY_800),
                ..default()
            },
            Transform::from_xyz(0., 0., -2.),
        )
    );

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BRICK_SIZE.x,
                min_height: CANVAS_SIZE.y + BRICK_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(-200., -400.)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(
            materials.add(Color::from(SLATE_950))
        ),
        Transform::from_xyz(0.,0.,0.),
        children![(
            Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.,0.,1.),
        )]
    ));
}

fn ball_movement(
    mut balls: Query<
        (&mut Transform, &Velocity),
        With<Ball>
    >,
    time: Res<Time>
) {
    for (mut transform, velocity) in &mut balls {
        let ball_movement_this_frame = 
            velocity.0 * time.delta_secs();

        transform.translation += 
            ball_movement_this_frame.extend(0.);
    }
}

const BALL_SIZE: f32 = 10.;

#[derive(Component)]
struct Ball;

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);

const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);

#[derive(Debug, Component)]
struct Velocity(Vec2);

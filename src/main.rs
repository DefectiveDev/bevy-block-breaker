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

    commands.spawn(
        (
            Sprite {
                custom_size: Some(DEFAULT_PADDLE_SIZE),
                color: SKY_50.into(),
                ..default()
            },
            Transform::from_xyz(
                0.,
                -CANVAS_SIZE.y * (3. / 8.),
                0.
            ),
            Paddle,
            HalfSize(DEFAULT_PADDLE_SIZE / 2.),
        )
    );

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


    commands.spawn(
        (
            Wall(Plane2d::new(Vec2::X)),
            Transform::from_xyz(-CANVAS_SIZE.x / 2., 0., 0.)
        )
    );
    commands.spawn(
        (
            Wall(Plane2d::new(Vec2::NEG_X)),
            Transform::from_xyz(CANVAS_SIZE.x / 2., 0., 0.)
        )
    );
    commands.spawn(
        (
            Wall(Plane2d::new(Vec2::Y)),
            Transform::from_xyz(0., -CANVAS_SIZE.y / 2., 0.)
        )
    );
    commands.spawn(
        (
            Wall(Plane2d::new(Vec2::NEG_Y)),
            Transform::from_xyz(0., CANVAS_SIZE.y / 2., 0.)
        )
    );
}

fn ball_movement(
    mut balls: Query<
        (&mut Transform, &mut Velocity),
        With<Ball>
    >,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut balls {
        // a ray that casts infinitely in the direction
        // the ball is moving
        let ball_ray =  Ray2d::new(
            //location of ball
            transform.translation.xy(),
            // the direction the ball is moving
            Dir2::new(velocity.0).unwrap(),
        );

        // how far the ball is going to go this frame
        // reprsented as vec2
        let mut ball_movement_this_frame = 
            velocity.0 * time.delta_secs();
        let ball_move_distance =
            ball_movement_this_frame.length();

        // for each wall, check if we're going to hit it this frame
        for (wall, origin) in walls {
            if let Some(hit_distance) = ball_ray
                .intersect_plane(
                    origin.translation.xy(),
                    wall.0
                )
                && hit_distance <= ball_move_distance
            {
                // velocity is just the reflection of the hit
                // this is basically inverting the X or Y direction
                // to move in the opposite direction
                velocity.0 = velocity
                    .0
                    .reflect(wall.0.normal.as_vec2());
                ball_movement_this_frame = 
                    velocity.0 * time.delta_secs();
                break;
            }
        }

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

#[derive(Debug, Component)]
struct Wall(Plane2d);

const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200.,20.);
const PADDLE_SPEED: f32 = 400.;

#[derive(Component)]
struct Paddle;

#[derive(Debug, Component)]
struct HalfSize(Vec2);

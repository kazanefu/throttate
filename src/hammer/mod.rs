use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod definition;
use definition::*;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>().add_message::<ChangeHandleDirection>().add_systems(
            Update,
            (handle_hammer_input, update_hammer, change_handle_direction),
        );
    }
}

#[derive(Clone, Copy)]
enum HammerState {
    Spinning,
    Flying,
}

#[derive(Message)]
struct ChangeHandleDirection(HandleDirection);

#[derive(Clone, Copy)]
enum HandleDirection {
    LeftLeft,
    RightRight,
    LeftRight,
    RightLeft,
}
impl HandleDirection {
    fn offset(&self) -> Vec2 {
        match self {
            Self::LeftLeft => HAMMER_HANDLE_OFFSET,
            Self::RightRight => HAMMER_HANDLE_OFFSET * -1.0,
            Self::LeftRight => HAMMER_HANDLE_OFFSET,
            Self::RightLeft => HAMMER_HANDLE_OFFSET * -1.0,
        }
    }
    fn spin(&self) -> (f32, f32) {
        match self {
            Self::LeftLeft => HAMMER_SPIN,
            Self::RightRight => (-HAMMER_SPIN.0, HAMMER_SPIN.1),
            Self::LeftRight => (-HAMMER_SPIN.0, HAMMER_SPIN.1),
            Self::RightLeft => HAMMER_SPIN,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Hammer {
    pivot_entity: Entity,
    state: HammerState,
    handle_direction: HandleDirection,
}

#[derive(Component)]
pub struct Pivot;

#[derive(Message)]
struct HammerActionMessage;

pub fn hammer_bundle(pivot_entity: Entity, translate: Vec2) -> impl Bundle {
    (
        Hammer {
            pivot_entity,
            state: HammerState::Spinning,
            handle_direction: HandleDirection::LeftLeft,
        },
        RigidBody::Dynamic,
        Transform::from_xyz(translate.x, translate.y, 0.0),
        Collider::ball(10.0),
        ImpulseJoint::new(
            pivot_entity,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(HAMMER_HANDLE_OFFSET)
                .motor_velocity(HAMMER_SPIN.0, HAMMER_SPIN.1),
        ),
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.9),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    )
}

#[allow(unused)]
pub fn spawn_hammer<'a>(commands: &'a mut Commands, translate: Vec2) -> EntityCommands<'a> {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 0.0),
            Pivot,
        ))
        .id();
    commands.spawn(hammer_bundle(pivot, translate))
}

fn update_hammer(
    mut commands: Commands,
    mut hammer_query: Query<(Entity, &mut Hammer)>,
    mut transform_query: Query<&mut Transform>,
    mut hammer_action_reader: MessageReader<HammerActionMessage>,
) {
    for _ in hammer_action_reader.read() {
        for (hammer_entity, mut hammer) in hammer_query.iter_mut() {
            let hammer_transform = {
                let hammer_transform = transform_query
                    .get(hammer_entity)
                    .expect("hammer has no transform");
                (hammer_transform.translation, hammer_transform.rotation)
            };
            match hammer.state {
                HammerState::Spinning => {
                    commands.entity(hammer_entity).remove::<ImpulseJoint>();
                    hammer.state = HammerState::Flying;
                }
                HammerState::Flying => {
                    let mut pivot_transform = transform_query
                        .get_mut(hammer.pivot_entity)
                        .expect("This hammer has no pivot");
                    pivot_transform.translation = hammer_transform.0
                        + (hammer_transform.1 * hammer.handle_direction.offset().extend(0.0));
                    commands.entity(hammer_entity).insert(ImpulseJoint::new(
                        hammer.pivot_entity,
                        RevoluteJointBuilder::new()
                            .local_anchor1(Vec2::ZERO)
                            .local_anchor2(hammer.handle_direction.offset())
                            .motor_velocity(hammer.handle_direction.spin().0, HAMMER_SPIN.1),
                    ));
                    hammer.state = HammerState::Spinning;
                }
            }
        }
    }
}

fn handle_hammer_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hammer_action_writer: MessageWriter<HammerActionMessage>,
    mut handle_direction_writer: MessageWriter<ChangeHandleDirection>,
) {
    if keys.just_pressed(HAMMER_ACTION_KEY_CODE) {
        hammer_action_writer.write(HammerActionMessage);
    }
    if keys.just_pressed(CHANGE_DIRECTION_KEY_CODE_LL) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftLeft));
    }
    if keys.just_pressed(CHANGE_DIRECTION_KEY_CODE_RR) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightRight));
    }
    if keys.just_pressed(CHANGE_DIRECTION_KEY_CODE_LR) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftRight));
    }
    if keys.just_pressed(CHANGE_DIRECTION_KEY_CODE_RL) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightLeft));
    }
}
fn change_handle_direction(
    mut hammer_query: Query<&mut Hammer>,
    mut change_detection_message: MessageReader<ChangeHandleDirection>,
) {
    for message in change_detection_message.read() {
        for mut hammer in &mut hammer_query {
            hammer.handle_direction = message.0;
        }
    }
}

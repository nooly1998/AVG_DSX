use bevy::prelude::*;

pub struct ProgressBarPlugin;

#[derive(Component)]
pub struct ProgressBar {
    pub value: f32,
    pub max_value: f32,
    pub size: Vec2,
    pub bar_color: Color,
    pub background_color: Color,
}

#[derive(Component)]
struct ProgressBarBackground;

#[derive(Component)]
struct ProgressBarFill;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,entities_spawn)
            .add_systems(Update, (update_progress_bars,animate_progress_bars));
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            max_value: 100.0,
            size: Vec2::new(200.0, 20.0),
            bar_color: Color::srgba(0.0, 1.0, 0.0, 0.5),
            background_color: Color::BLACK,
        }
    }
}

fn update_progress_bars(
    mut commands: Commands,
    mut query: Query<(Entity, &ProgressBar, Option<&Children>)>,
    mut bar_query: Query<(&mut Style, &mut BackgroundColor, Option<&ProgressBarFill>)>,
) {
    for (entity, progress_bar, children) in query.iter() {
        let progress = (progress_bar.value / progress_bar.max_value).clamp(0.0, 1.0);

        match children {
            Some(children) => {
                // 更新现有进度条
                for &child in children.iter() {
                    if let Ok((mut style, mut background_color, is_fill)) = bar_query.get_mut(child) {
                        if is_fill.is_some() {
                            style.width = Val::Percent(progress * 100.0);
                            background_color.0 = progress_bar.bar_color;
                        } else {
                            background_color.0 = progress_bar.background_color;
                        }
                    }
                }
            }
            None => {
                // 创建背景
                let background = commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(progress_bar.size.x),
                                height: Val::Px(progress_bar.size.y),
                                ..default()
                            },
                            background_color: progress_bar.background_color.into(),
                            ..default()
                        },
                        ProgressBarBackground,
                    ))
                    .id();

                // 创建填充条
                let fill = commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(progress * 100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: progress_bar.bar_color.into(),
                            ..default()
                        },
                        ProgressBarFill,
                    ))
                    .id();

                commands.entity(background).push_children(&[fill]);
                commands.entity(entity).push_children(&[background]);
            }
        }
    }
}

// 使用示例
fn entities_spawn(mut commands: Commands) {

    // Root UI node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // 进度条容器
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(30.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(100.0),
                        top: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ProgressBar {
                    value: 75.0,
                    max_value: 100.0,
                    size: Vec2::new(300.0, 30.0),
                    bar_color: Color::srgb(0.25, 0.75, 0.25),
                    background_color: Color::srgb(0.15, 0.15, 0.15),
                });
        });
}

// 添加动画效果示例
#[derive(Component)]
struct AnimatedProgressBar {
    target: f32,
    speed: f32,
}

fn animate_progress_bars(
    time: Res<Time>,
    mut query: Query<(&mut ProgressBar, &AnimatedProgressBar)>,
) {
    for (mut progress_bar, animation) in query.iter_mut() {
        let delta = animation.speed * time.delta_seconds();
        let diff = animation.target - progress_bar.value;
        if diff.abs() > delta {
            progress_bar.value += delta * diff.signum();
        } else {
            progress_bar.value = animation.target;
        }
    }
}

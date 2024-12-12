use bevy::prelude::*;

pub struct CheckboxPlugin;

#[derive(Component)]
pub struct Checkbox {
    pub checked: bool,
    pub size: f32,
    pub box_color: Color,
    pub check_color: Color,
    pub hovered_color: Color,
    pub disabled: bool,
}

#[derive(Component)]
struct CheckboxBackground;

#[derive(Component)]
struct CheckboxCheck;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entities_spawn)
            .add_systems(Update, checkbox_changed)
            .add_systems(Update, (update_checkboxes, checkbox_interaction));
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            checked: false,
            size: 20.0,
            box_color: Color::srgb(0.2, 0.2, 0.2),
            check_color: Color::srgb(0.0, 0.8, 0.0),
            hovered_color: Color::srgb(0.25, 0.25, 0.25),
            disabled: false,
        }
    }
}

fn update_checkboxes(
    mut commands: Commands,
    query: Query<(Entity, &Checkbox, Option<&Children>)>,
    check_query: Query<(&Children, &Parent), With<CheckboxBackground>>,
    mut check_sign_query: Query<(&mut Style, &mut BackgroundColor), With<CheckboxCheck>>,
) {
    for (entity, checkbox, children) in query.iter() {
        match children {
            Some(children) => {
                // 更新现有复选框
                for &child in children.iter() {
                    // assert_eq!(children.type_id(),child.type_id());
                    if let Ok((children1, parent)) = check_query.get(child)
                    {
                        println!("checked!");

                        if parent.get() == entity {
                            for &child1 in children1 {
                                if let Ok((mut ch_style, mut ch_bg_color)) =
                                    check_sign_query.get_mut(child1)
                                {
                                    ch_style.display = if checkbox.checked {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    };
                                    ch_bg_color.0 = checkbox.check_color;
                                } else {
                                    return;
                                }
                            }
                        } else {
                            return;
                        }
                    }
                }
            }
            None => {
                // 创建新的复选框
                let background = commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(checkbox.size),
                                height: Val::Px(checkbox.size),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: Color::WHITE.into(),
                            background_color: checkbox.box_color.into(),
                            ..default()
                        },
                        Interaction::default(),
                        CheckboxBackground,
                    ))
                    .with_children(|parent| {
                        // 勾选标记
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(checkbox.size * 0.6),
                                    height: Val::Px(checkbox.size * 0.6),
                                    display: if checkbox.checked {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    ..default()
                                },
                                background_color: checkbox.check_color.into(),
                                ..default()
                            },
                            CheckboxCheck,
                        ));
                    })
                    .id();

                commands.entity(entity).push_children(&[background]);
            }
        }
    }
}

fn checkbox_interaction(
    mut checkbox_query: Query<(&mut Checkbox, &Children)>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Parent),
        (Changed<Interaction>, With<CheckboxBackground>),
    >,
) {
    for (interaction, mut background_color, parent) in interaction_query.iter_mut() {
        if let Ok((mut checkbox, _)) = checkbox_query.get_mut(parent.get()) {
            if checkbox.disabled {
                continue;
            }

            match *interaction {
                Interaction::Pressed => {
                    checkbox.checked = !checkbox.checked;
                }
                Interaction::Hovered => {
                    background_color.0 = checkbox.hovered_color;
                }
                Interaction::None => {
                    background_color.0 = checkbox.box_color;
                }
            }
        }
    }
}

// 使用示例
fn entities_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(100.0),
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // 复选框容器
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // 复选框
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                ..default()
                            },
                            ..default()
                        },
                        Checkbox {
                            checked: true,
                            size: 24.0,
                            box_color: Color::srgb(0.2, 0.2, 0.2),
                            check_color: Color::srgb(0.0, 0.8, 0.0),
                            hovered_color: Color::srgb(0.25, 0.25, 0.25),
                            disabled: false,
                        },
                    ));
                    let font = asset_server.load("fonts/zfft.ttf");
                    // 标签文本
                    parent.spawn(TextBundle::from_section(
                        "checkbox",
                        TextStyle {
                            font,
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

// 监听复选框状态变化
fn checkbox_changed(query: Query<&Checkbox, Changed<Checkbox>>) {
    for checkbox in query.iter() {
        println!("Checkbox state changed: {}", checkbox.checked);
    }
}

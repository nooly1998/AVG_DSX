use bevy::prelude::*;
use bevy::ui::Val::Px;

pub struct DropDownPlugin;

#[derive(Component)]
struct ListVisibility(bool); // 用于控制列表可见性的组件

#[derive(Component)]
struct ItemVisibility(bool); // 用于控制列表可见性的组件

#[derive(Resource, Default)]
struct DropDownOptions {
    options: Vec<String>,
    selected_index: usize,
    selected_option: String,
}

impl Plugin for DropDownPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DropDownOptions {
            options: vec!["720p".to_string(), "1080p".to_string(), "1440p".to_string()],
            ..default()
        })
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, (button_system, toggle_list_visibility));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.827, 0.827, 0.827)),
            Interaction::None => BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        };
    }
}

fn toggle_list_visibility(
    mut interaction_query: Query<
        (&Interaction, &mut ListVisibility),
        (Changed<Interaction>, With<Button>),
    >,
    mut visibility_query: Query<(&mut ItemVisibility, &mut Visibility)>,
) {
    for (interaction, mut list_visibility) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            list_visibility.0 = !list_visibility.0; // 切换可见性状态

            for (mut item, mut visibility) in visibility_query.iter_mut() {
                item.0 = list_visibility.0; // 更新子节点的可见性
                if item.0 {
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn spawn_entities(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    options_res: Res<DropDownOptions>,
) {
    let options = &options_res.options;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Px(150.0),
                display: Display::Grid,
                height: Px(30.0),
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor::from(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let button_image = asset_server.load("component/dropdownctl.png");
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Px(150.0),
                        height: Px(30.0),
                        display: Display::Block,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|drop_down| {
                    drop_down
                        .spawn(ButtonBundle {
                            style: {
                                Style {
                                    left: Px(150.0 - 30.0),
                                    width: Px(30.0),
                                    height: Px(30.0),
                                    border: UiRect::all(Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                }
                            },
                            image: UiImage::new(button_image),
                            interaction: Interaction::None,
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(ListVisibility(true));
                });

            for option in options {
                parent
                    .spawn(ButtonBundle {
                        style: {
                            Style {
                                width: Px(150.0),
                                height: Px(30.0),
                                // border: UiRect::all(Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            }
                        },
                        interaction: Interaction::None,
                        border_radius: BorderRadius::all(Px(4.0)),
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    })
                    .insert(ItemVisibility(true))
                    .with_children(|drop_down| {
                        drop_down.spawn(TextBundle {
                            text: Text::from_section(
                                option.as_str(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}

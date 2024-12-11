use bevy::prelude::*;
use bevy::reflect::List;
use bevy::ui::Val::{Percent, Px};

#[derive(Default)]
pub struct DropDownPlugin{
    pub(crate) options:DropDownOptions,
    pub(crate) style: Style
}

#[derive(Component)]
struct ListVisibility(bool); // 用于控制列表可见性的组件

#[derive(Component)]
struct ListShowText; // 用于控制列表可见性的组件

#[derive(Component)]
struct ItemVisibility(bool); // 用于控制列表可见性的组件

#[derive(Resource, Reflect, Clone, Default)]
pub struct DropDownOptions {
    pub(crate) options: Vec<String>,
    pub(crate) selected_index: usize,
    pub(crate) selected_option: String,
}

impl DropDownOptions {
    fn from_option(options:Vec<String>,option:String) -> Self{
        let mut index = 0;
        let mut sel_option = String::new();
        for (n,item) in options.clone().iter().enumerate(){
            let Some(val) = item.downcast_ref::<String>()else { continue };
            if *val == option{
                index = n;
                sel_option = option.clone();
            }
        }
        Self{
            options,
            selected_index: index,
            selected_option: sel_option,
        }
    }
}

impl Plugin for DropDownPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.options.clone())
        .add_systems(Startup, spawn_entities)
        .add_systems(
            Update,
            (
                button_system,
                toggle_list_visibility,
                clicked_list_item,
                list_item_hidden_update,
            ),
        );
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

fn clicked_list_item(
    mut option: ResMut<DropDownOptions>,
    mut button_query: Query<(&Interaction, &mut Children, &mut ItemVisibility)>,
    mut list_query: Query<&mut ListVisibility>,
    text_query: Query<&mut Text>,
) {
    for (interaction, button_child, _) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            for entity in &button_child {
                if let Ok(child_text) = text_query.get(*entity) {
                    let text = child_text.sections[0].value.clone();
                    let options = option.options.clone();
                    option.selected_option = text.clone();
                    for (n, str) in options.iter().enumerate() {
                        if let Some(ref_val) = str.downcast_ref::<String>() {
                            if *ref_val == text {
                                option.selected_index = n;
                            }
                        }
                    }
                    for mut list_visibility in list_query.iter_mut() {
                        list_visibility.0 = false;
                    }
                }
            }
        }
    }
}

fn list_item_hidden_update(
    option: Res<DropDownOptions>,
    mut visibility_query: Query<&mut Visibility, With<ItemVisibility>>,
    list_query: Query<&ListVisibility>,
    mut text_query: Query<&mut Text, With<ListShowText>>,
) {
    for mut visibility in visibility_query.iter_mut() {
        for list in list_query.iter() {
            //update list visibility
            if !list.0 {
                *visibility = Visibility::Hidden;
                //update list top text
                for mut list_text in text_query.iter_mut() {
                    list_text.sections[0].value = option.selected_option.clone();
                }
            } else {
                *visibility = Visibility::Inherited;
            }
        }
    }
}

fn toggle_list_visibility(
    mut interaction_query: Query<
        (&Interaction, &mut ListVisibility),
        (Changed<Interaction>, With<Button>),
    >,
    mut visibility_query: Query<&mut ItemVisibility>,
) {
    for (interaction, mut list_visibility) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            list_visibility.0 = !list_visibility.0; // 切换可见性状态

            for mut item in visibility_query.iter_mut() {
                item.0 = list_visibility.0; // 更新子节点的可见性
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
                                    width: Percent(100.0),
                                    height: Percent(100.0),
                                    // border: UiRect::all(Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                }
                            },
                            // image: UiImage::new(button_image),
                            interaction: Interaction::None,
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(ListVisibility(true))
                        .with_children(|ctl| {
                            ctl.spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    width: Percent(100.0),
                                    height: Percent(100.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|btn| {
                                btn.spawn(TextBundle {
                                    text: Text::from_section(
                                        String::new(),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 24.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    style: Style {
                                        width: Percent(80.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(ListShowText);

                                btn.spawn(ImageBundle {
                                    style: Style {
                                        width: Percent(20.0),
                                        ..default()
                                    },
                                    image: UiImage::new(button_image),
                                    ..default()
                                });
                            });
                        });
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

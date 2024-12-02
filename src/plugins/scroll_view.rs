use crate::global_def::global_define::RESOLUTION_720P;
use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::core::Name;
use bevy::input::mouse::MouseWheel;
use bevy::math::Vec2;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub struct ScrollViewPlugin;

#[derive(Resource)]
struct ScrollViewResource {
    value: i32,
}

#[derive(Component, Clone, Default)]
pub struct ScrollView {
    pub current_top: f32,
    pub current_len: f32,
    pub parent_top: f32,
    pub parent_len: f32,
    pub is_dragging: bool, // 追踪是否正在拖动
    pub drag_offset: f32,  // 追踪拖动开始时的偏移
    pub bar: bool,
    pub view_top: f32,
    pub view_len: f32,
    pub content_len: f32,
}

#[derive(Component)]
pub struct TextFiledHidden;

#[derive(Component)]
pub struct TextFiledHiddenButton;

impl Plugin for ScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScrollViewResource { value: 0 })
            .add_systems(Startup, spawn_entities)
            .add_systems(
                Update,
                (
                    scroll_view_system,
                    scroll_bar_drag_system,
                    scroll_view_drag_system,
                    text_filed_hidden,
                ),
            );
    }
}

fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    let box_size = Vec2::new(RESOLUTION_720P.0, RESOLUTION_720P.1 * 0.3);
    let box_text_position = Vec2::new(-RESOLUTION_720P.0 / 3f32, 0.0);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(box_size.x),
                    height: Val::Percent(30.0),
                    padding: UiRect::all(Px(RESOLUTION_720P.0 * 0.01)),
                    top: Px(RESOLUTION_720P.1 * 0.7 - 30.0 - 2.0 * RESOLUTION_720P.0 * 0.01),
                    ..default()
                },
                transform: Transform::from_translation(box_text_position.extend(1.0)),
                ..default()
            },
            Name::new("ButtonList"),
        ))
        .with_children(|button_list| {
            button_list
                .spawn((
                    ButtonBundle {
                        style: {
                            Style {
                                width: Px(150.0),
                                height: Px(30.0),
                                border: UiRect::all(Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            }
                        },
                        interaction: Interaction::None,
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Name::new("TextFiledHidden"),
                    TextFiledHiddenButton,
                ))
                .with_children(|button_bundle| {
                    button_bundle.spawn(
                        (TextBundle::from_section(
                            "FiledHidden",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 18.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        )),
                    );
                });
        });

    let view_size = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1 * 0.2);
    let view_position = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1 * 0.4);
    let cover_size = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Px(view_size.x),
                    height: Px(view_size.y),
                    left: Px(view_position.x),
                    top: Px(view_position.y),
                    overflow: Overflow::clip(),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::srgb(0.25, 0.25, 0.75)),
                transform: Transform::from_translation(view_position.extend(2.0)),
                ..default()
            },
            Name::new("scroll_view"),
            TextFiledHidden,
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Px(cover_size.x - 16.0),
                            height: Px(cover_size.y),
                            left: Px(16.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("content_view"),
                ))
                .insert(ScrollView {
                    content_len: cover_size.y + view_size.y,
                    view_top: view_position.y,
                    view_len: view_size.y,
                    bar: false,
                    ..default()
                })
                .with_children(|content| {
                    for i in 0..10 {
                        content.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Item {}", i + 1),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });
                    }
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Px(16.0),
                    height: Px(view_size.y),
                    left: Px(view_position.x - 16.0),
                    top: Px(view_position.y),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::srgb(0.9, 0.9, 0.9)),
                transform: Transform::from_translation(view_position.extend(5.0)),
                ..default()
            },
            Name::new("scroll_bar"),
            TextFiledHidden,
        ))
        .with_children(|status| {
            status
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Px(16.0),
                            height: Px(20.0),
                            ..default()
                        },
                        transform: Transform::from_translation(view_position.extend(2.0)),
                        background_color: BackgroundColor::from(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    Name::new("scroll_status"),
                ))
                .insert(ScrollView {
                    parent_top: view_position.y,
                    parent_len: view_size.y,
                    current_top: view_position.y,
                    current_len: 20.0,
                    is_dragging: false,
                    drag_offset: 0.0,
                    bar: true,
                    ..default()
                });
        });
}
fn scroll_bar_drag_system(
    mut scroll_query: Query<(&mut Style, &mut ScrollView)>,
    mut current_window: Query<&mut Window>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
) {
    // println!("windows size:{:?}",current_window.iter().len());
    for (mut style, mut scroll_bar) in scroll_query.iter_mut() {
        // 判断是否有鼠标按钮被按下
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // println!("scroll_bar_pressed");
            // 获取鼠标点击位置
            for window in current_window.iter() {
                let Some(cursor_position) = window.cursor_position() else {
                    continue;
                };
                // println!("cursor_position: {:?}", cursor_position);
                if cursor_position.y >= scroll_bar.current_top
                    && cursor_position.y <= scroll_bar.current_top + scroll_bar.current_len
                {
                    scroll_bar.is_dragging = true;
                    scroll_bar.drag_offset = cursor_position.y - scroll_bar.current_top;
                }
            }
        }

        // 判断是否正在拖动
        if scroll_bar.is_dragging {
            // 获取鼠标当前位置
            for window in current_window.iter() {
                let Some(cursor_position) = window.cursor_position() else {
                    continue;
                };
                let new_top = cursor_position.y - scroll_bar.drag_offset;

                // 确保新的位置在父容器的范围内
                if new_top >= scroll_bar.parent_top
                    && new_top + scroll_bar.current_len
                        <= scroll_bar.parent_top + scroll_bar.parent_len
                {
                    style.top = Val::Px(new_top - scroll_bar.parent_top);
                    scroll_bar.current_top = new_top;
                } else {
                    if new_top < scroll_bar.parent_top {
                        style.top = Val::Px(0.0);
                        scroll_bar.current_top = scroll_bar.parent_top;
                    }
                    if new_top + scroll_bar.current_len
                        > scroll_bar.parent_top + scroll_bar.parent_len
                    {
                        style.top = Val::Px(scroll_bar.parent_len - scroll_bar.current_len);
                        scroll_bar.current_top = scroll_bar.parent_len - scroll_bar.current_len;
                    }
                }
            }

            // 当鼠标按钮释放时，停止拖动
            if mouse_button_input.just_released(MouseButton::Left) {
                scroll_bar.is_dragging = false;
            }
        }
    }
}

pub fn scroll_view_drag_system(mut scroll_query: Query<(&mut Style, &mut ScrollView)>) {
    let mut offset = 0.0;
    for (_, mut scroll_bar) in scroll_query.iter_mut().filter(|p| p.1.bar) {
        offset = (scroll_bar.parent_top - scroll_bar.current_top) / scroll_bar.parent_len
    }
    for (mut view_style, mut scroll_view) in scroll_query.iter_mut().filter(|p| !p.1.bar) {
        //if offset = 0, view_top = parent_top, if offset = 1, view_top = parent_top - ( view_len - parent_len )
        scroll_view.view_top =
            scroll_view.parent_top - offset * (scroll_view.view_len - scroll_view.parent_len);
        view_style.top = Val::Px(scroll_view.parent_top - scroll_view.view_top);
    }
}

fn text_filed_hidden(
    mut filed_query: Query<&mut Visibility, With<TextFiledHidden>>,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<TextFiledHiddenButton>)>,
) {
    // println!("button pressed!");
    for button in button_query.iter() {
        match *button {
            Interaction::Pressed => {
                for mut node in filed_query.iter_mut() {
                    let vcp = (*node).clone();
                    // println!("vcp:{:?}",vcp);
                    match vcp {
                        Visibility::Inherited => {
                            *node = Visibility::Hidden;
                        }
                        Visibility::Hidden => {
                            *node = Visibility::Inherited;
                        }
                        _ => {
                            return;
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
}

fn scroll_view_system(
    mut scroll_query: Query<(&mut Style, &mut ScrollView)>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut scroll_delta = 0.0;

    // 读取鼠标滚轮事件
    for event in mouse_wheel_events.par_read() {
        scroll_delta += event.0.y; // 每次滚动的像素值
    }

    // 更新子容器的位置
    if scroll_delta != 0.0 {
        for (mut style, mut content) in scroll_query.iter_mut() {
            let cst = content.current_top.clone();
            let len = content.current_len;
            // print!(
            //     "len:{:?},pl:{:?},cst{:?},pt{:?}\n",
            //     len, content.parent_len, cst, content.parent_top
            // );
            if cst + scroll_delta <= (content.parent_top + content.parent_len - len)
                && cst + scroll_delta >= content.parent_top
            {
                // println!("compare! {:?}\n", scroll_delta);
                // println!("has current");
                style.top = Val::Px(cst + scroll_delta - content.parent_top);
                content.current_top += scroll_delta;
            }
        }
    }
}

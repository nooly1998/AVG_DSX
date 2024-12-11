use crate::global_def::global_define::RESOLUTION_720P;
use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::core::Name;
use bevy::input::mouse::MouseWheel;
use bevy::math::Vec2;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

/// A Bevy Plugin for creating a scroll view functionality.
///
/// This plugin adds systems for managing a user interface scrollable
/// view, handling user inputs for scrolling and dragging interactions.
///
/// When added to an app, it initializes entities
/// and components for the scroll view.
///
/// Usage:
///
/// ```rust
/// use crate::ScrollViewPlugin;
/// use bevy::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(ScrollViewPlugin)
///         .run();
/// }
/// ```
pub struct ScrollViewPlugin;

/// A resource struct for managing scroll view updates.
///
/// This resource keeps track of the current value of the scroll view
/// to handle updates and modifications.
#[derive(Resource)]
struct ScrollViewResource {
    value: i32,
}

/// A struct representing a scroll view component.
///
/// This struct is used to track the state and configuration
/// of a scroll view UI element, including its dimensions,
/// position, and interaction state.
#[derive(Component, Clone, Default)]
pub struct ScrollView {
    /// The current top position of the scroll view in pixels.
    pub current_top: f32,

    /// The current length (height) of the scroll view in pixels.
    pub current_len: f32,

    /// The top position of the parent container in pixels.
    pub parent_top: f32,

    /// The length (height) of the parent container in pixels.
    pub parent_len: f32,

    /// Flag indicating whether the scroll view is currently being dragged.
    ///
    /// Used to track the dragging state during user interaction.
    pub is_dragging: bool,

    /// The offset value when dragging starts, in pixels.
    ///
    /// Used to calculate the new position based on initial click position.
    pub drag_offset: f32,

    /// Flag indicating whether this instance represents a scroll bar.
    pub bar: bool,

    /// The top position for the view window relative to its parent.
    pub view_top: f32,

    /// The length (height) of the view window in pixels.
    pub view_len: f32,

    /// The total length (height) of the scrollable content in pixels.
    pub content_len: f32,
}

/// Component representing a hidden text field.
///
/// This component can be used to associate entities that should be
/// treated as hidden text fields within the user interface.
#[derive(Component)]
pub struct TextFiledHidden;

/// Component representing a button for hiding a text field.
///
/// This component can be used to identify the button entity
/// associated with the action to hide or show text fields
/// within the user interface.
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

/// Spawns UI entities with scrolling capabilities for a Bevy application.
///
/// This system sets up a scrolling view with a hidden text field button,
/// a scrollable content area, and a scroll bar.
///
/// It organizes UI components vertically, providing a paginated view
/// for the content items.
///
/// # Parameters
/// - `commands`: Command queue for spawning entities.
/// - `asset_server`: Access to the asset loader for loading fonts.
///
/// # Panics
/// This function might panic if any of the UI component initialization fails.
///
///
/// Ensure the appropriate resources (e.g., font files) exist in your asset folder.
fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    current_window: Query<&mut Window>,
    mouse_button_input: ResMut<ButtonInput<MouseButton>>,
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
    for (_, scroll_bar) in scroll_query.iter_mut().filter(|p| p.1.bar) {
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
    button_query: Query<&Interaction, (Changed<Interaction>, With<TextFiledHiddenButton>)>,
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

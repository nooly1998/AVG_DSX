pub mod string_utils {
    use bevy::input::mouse::{MouseWheel};
    use bevy::input::ButtonInput;
    use bevy::prelude::*;
    use bevy::time::Timer;

    pub fn string_auto_split(value: impl Into<String>, len_px: f32, font_size: usize) -> String {
        let len = (len_px * 1000.0) as usize / font_size / 1000;
        let val = value.into();
        let vals = val.split(",").collect::<Vec<&str>>();
        let mut result = String::new();

        for item in vals.iter() {
            let mut current_length = 0;
            for c in item.chars() {
                if current_length == len {
                    result.push('\n');
                    current_length = 0;
                }
                result.push(c);
                current_length += 1;
            }
        }

        result
    }

    #[derive(Component, Clone,Default)]
    pub struct ScrollView {
        pub current_top: f32,
        pub current_len: f32,
        pub parent_top: f32,
        pub parent_len: f32,
        pub is_dragging: bool, // 追踪是否正在拖动
        pub drag_offset: f32,  // 追踪拖动开始时的偏移
        pub bar:bool,
        pub view_top: f32,
        pub view_len: f32,
        pub content_len: f32,
    }

    #[derive(Component, Clone)]
    pub struct TypingText {
        pub(crate) full_text: String,
        pub(crate) displayed_text: String,
        pub(crate) current_index: usize,
        pub(crate) timer: Timer,
    }

    #[derive(Component)]
    pub struct TextFiledHidden;

    pub fn scroll_view_system(
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
                print!(
                    "len:{:?},pl:{:?},cst{:?},pt{:?}\n",
                    len, content.parent_len, cst, content.parent_top
                );
                if cst + scroll_delta <= (content.parent_top + content.parent_len - len)
                    && cst + scroll_delta >= content.parent_top
                {
                    println!("compare! {:?}\n", scroll_delta);
                    println!("has current");
                    style.top = Val::Px(cst + scroll_delta - content.parent_top);
                    content.current_top += scroll_delta;
                }
            }
        }
    }

    pub fn update_typing_text(
        time: Res<Time>,
        input: Res<ButtonInput<MouseButton>>,
        mut query: Query<(&mut TypingText, &mut Text)>,
    ) {
        for (mut typing_text, mut text) in query.iter_mut() {
            if input.just_pressed(MouseButton::Left)
                && typing_text.current_index < typing_text.full_text.len()
            {
                typing_text.displayed_text = typing_text.full_text.clone();
                typing_text.current_index = typing_text.full_text.len();

                text.sections[0].value = typing_text.displayed_text.clone();
                return;
            }

            typing_text.timer.tick(time.delta());
            if typing_text.timer.finished()
                && typing_text.current_index < typing_text.full_text.len()
            {
                let clone_text = typing_text.clone();
                let Some(update_text) = clone_text.full_text.chars().nth(clone_text.current_index)
                else {
                    break;
                };
                typing_text.displayed_text.push(update_text);
                typing_text.current_index += 1;
                text.sections[0].value = typing_text.displayed_text.clone();
            }
        }
    }

    pub fn scroll_bar_drag_system(
        mut scroll_query: Query<(&mut Style, &mut ScrollView)>,
        mut current_window:Query<&mut Window>,
        mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    ) {
        println!("windows size:{:?}",current_window.iter().len());
        for (mut style, mut scroll_bar) in scroll_query.iter_mut() {
            // 判断是否有鼠标按钮被按下
            if mouse_button_input.just_pressed(MouseButton::Left) {
                println!("scroll_bar_pressed");
                // 获取鼠标点击位置
                for window in current_window.iter() {
                    let Some(cursor_position) = window.cursor_position() else { continue };
                    println!("cursor_position: {:?}", cursor_position);
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
                    let Some(cursor_position) = window.cursor_position()else { continue };
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

    pub fn scroll_view_drag_system(
        mut scroll_query: Query<(&mut Style, &mut ScrollView)>,
    ) {
        let mut offset = 0.0;
        for (_, mut scroll_bar) in scroll_query.iter_mut().filter(|p| {p.1.bar}) {
            offset = (scroll_bar.parent_top - scroll_bar.current_top) / scroll_bar.parent_len
        }
        for (mut view_style, mut scroll_view) in scroll_query.iter_mut().filter(|p| {!p.1.bar}){
            //if offset = 0, view_top = parent_top, if offset = 1, view_top = parent_top - ( view_len - parent_len )
            scroll_view.view_top = scroll_view.parent_top - offset * (scroll_view.view_len - scroll_view.parent_len);
            view_style.top = Val::Px(scroll_view.parent_top - scroll_view.view_top);
        }
    }

    // pub fn text_filed_hidden(filed_query:Query<Style, With<TextFiledHidden>>){
    //     for style in filed_query.iter(){
    //         style.visibility = Visibility::Hidden;
    //     }
    // }
}

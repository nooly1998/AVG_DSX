pub mod string_utils {
    use bevy::input::mouse::MouseWheel;
    use bevy::input::ButtonInput;
    use bevy::prelude::*;
    use bevy::time::Timer;

    pub fn string_auto_split(value: impl Into<String>, len_px: f32, font_size: usize) -> String {
        let len = (len_px * 1000.0) as usize / font_size / 1000;
        let val = value.into();
        let vals = val.split(",")
            .collect::<Vec<&str>>();
        let mut result = String::new();

        for item in vals.iter()
        {
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

    #[derive(Component)]
    #[derive(Clone)]
    pub struct ScrollBar {
        pub current_top: f32,
        pub current_len: f32,
        pub parent_top: f32,
        pub parent_len: f32,
    }

    #[derive(Component)]
    #[derive(Clone)]
    pub struct TypingText {
        pub(crate) full_text: String,
        pub(crate) displayed_text: String,
        pub(crate) current_index: usize,
        pub(crate) timer: Timer,
    }

    pub fn scroll_view_system(
        mut scroll_query: Query<(&mut Style, &mut ScrollBar)>,
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
                print!("len:{:?},pl:{:?},cst{:?},pt{:?}\n", len, content.parent_len, cst, content.parent_top);
                if cst + scroll_delta <= (content.parent_top + content.parent_len - len) && cst + scroll_delta >= content.parent_top
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
            if input.just_pressed(MouseButton::Left) && typing_text.current_index < typing_text.full_text.len() {
                typing_text.displayed_text = typing_text.full_text.clone();
                typing_text.current_index = typing_text.full_text.len();

                text.sections[0].value = typing_text.displayed_text.clone();
                return;
            }

            typing_text.timer.tick(time.delta());
            if typing_text.timer.finished() && typing_text.current_index < typing_text.full_text.len() {
                let clone_text = typing_text.clone();
                let Some(update_text) = clone_text.full_text.chars().nth(clone_text.current_index)else { break };
                typing_text.displayed_text.push(update_text);
                typing_text.current_index += 1;
                text.sections[0].value = typing_text.displayed_text.clone();
            }
        }
    }
}
pub mod string_utils{
    use bevy::input::ButtonInput;
    use bevy::prelude::{Component, MouseButton, Query, Res, Text, Time};
    use bevy::time::Timer;

    pub fn string_auto_split(value: impl Into<String>, len: usize) -> String {
        let val = value.into();
        let vals = val.split(",").collect::<Vec<&str>>();
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
    pub struct TypingText {
        pub(crate) full_text: String,
        pub(crate) displayed_text: String,
        pub(crate) current_index: usize,
        pub(crate) timer: Timer,
    }

    pub fn update_typing_text(
        time: Res<Time>,
        input:Res<ButtonInput<MouseButton>>,
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
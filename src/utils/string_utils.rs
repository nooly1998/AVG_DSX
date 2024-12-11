/// Splits a string into lines based on pixel width and font size.
///
/// # Parameters
/// - `value`: The string to split. Can be any type that converts into a `String`.
/// - `len_px`: The maximum line length in pixels.
/// - `font_size`: The size of the font used to measure character width.
///
/// # Returns
/// A `String` with newline characters embedded to split the input into lines of specified pixel length.
///
/// The function attempts to place newline characters such that no line exceeds
/// the length in pixels defined by `len_px`, assuming each character is roughly
/// `font_size` wide.
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

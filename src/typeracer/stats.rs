#[derive(Debug)]
pub struct Stats<'a> {
    text:           &'a str,
    keyboard_input: &'a str,
    index:          usize,
    wrong_index:    usize,
    text_len:       usize // total_lines:    Option<usize>
}

impl<'a> Stats<'a> {
    pub fn new(
        text: &'a str,
        keyboard_input: &'a str,
        index: usize,
        wrong_index: usize,
        text_len: usize
    ) -> Self {
        Self {
            text,
            keyboard_input,
            index,
            wrong_index,
            text_len // total_lines: None
        }
    }
}

// impl<'a> Stats<'a> {
//     pub fn total_lines(&self) -> usize {
//     }
// }

impl<'a> std::fmt::Display for Stats<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        let keyboard_input = self.keyboard_input;
        let index = self.index;
        let wrong_index = self.wrong_index;
        let text_length = self.text_len;
        let text_length_minus_one = self.text_len - 1;
        let index_plus_wrong_index = self.index + self.wrong_index;

        let next_char = if index != text_length {
            let next_char =
                self.text.chars().nth(index).ok_or(std::fmt::Error)?;
            let next_char = if next_char == '\n' {
                "\\n".to_string()
            } else {
                next_char.to_string()
            };
            next_char
        } else {
            "no next char".into()
        };

        let stats_to_string = format!(
            r#"Keyboard input: '{keyboard_input}'
Index: {index}
Wrong: {wrong_index}
Index + Wrong: {index_plus_wrong_index}
text.len(): {text_length}
text.len() - 1: {text_length_minus_one}
Next char: '{next_char}'"#
        );

        // Previous char: '{}'
        // Current char: '{}'
        // Next char: '{}'
        write!(f, "{stats_to_string}")
    }
}

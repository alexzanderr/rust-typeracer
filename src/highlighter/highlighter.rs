use syntect::easy::{HighlightLines};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{ThemeSet, Style};
use syntect::highlighting::Theme;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub const CODE: &str = r#"use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet,Style};
use syntect::util::{as_latex_escaped,LinesWithEndings};

fn main() {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}\n";

    let mut h = HighlightLines::new(syntax, &ts.themes["InspiredGitHub"]);
    for line in LinesWithEndings::from(s) { // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_latex_escaped(&ranges[..]);
        println!("\n{:?}", line);
        println!("\n{}", escaped);
    }
}
"#;

#[derive(Debug)]
pub struct TyperacerHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl TyperacerHighlighter {
    pub fn new() -> Self {
        // Load these once at the start of your program
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme = ThemeSet::get_theme(std::path::Path::new("static/one-dark")).unwrap();

        let mut _self = Self {
            syntax_set,
            theme,
        };

        _self
    }

    pub fn highlight_line(&mut self, line: &str) -> String {
        let rust_syntax = self.syntax_set.find_syntax_by_extension("rs").unwrap();
        let mut highlighter = HighlightLines::new(rust_syntax, &self.theme);
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);

        escaped.replace('\n', "")
    }


    pub fn highlight_code_block_to_string(&mut self, code_block: &str) -> String {
        let rust_syntax = self.syntax_set.find_syntax_by_extension("rs").unwrap();
        let mut highlighter = HighlightLines::new(rust_syntax, &self.theme);

        let total_lines = code_block.matches("\n").count();
        let lines_with_endings = LinesWithEndings::from(code_block).into_iter();

        let mut highlighted_lines = Vec::<String>::with_capacity(total_lines);

        for line in lines_with_endings {
            let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            // println!("\n{:?}", line);
            // let esc = escaped.replace('\n', "");
            // let esc = format!("{esc}\n");
            highlighted_lines.push(escaped);
        }

        // they already have \n at the end
        highlighted_lines.join("")
    }


    #[deprecated(since = "5.0.0", note = "Renamed to `highlight_line` to make it clear it should be passed a single line at a time")]
    pub fn highlight_code_block(&mut self, code_block: &str) -> Vec<String> {
        let rust_syntax = self.syntax_set.find_syntax_by_extension("rs").unwrap();
        let mut highlighter = HighlightLines::new(rust_syntax, &self.theme);

        let total_lines = code_block.matches("\n").count();
        let lines_with_endings = LinesWithEndings::from(code_block).into_iter();

        let mut highlighted_lines = Vec::<String>::with_capacity(total_lines);

        for line in lines_with_endings {
            let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            // println!("\n{:?}", line);
            highlighted_lines.push(escaped);
        }

        highlighted_lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_block() {
        let instant = std::time::Instant::now();
        let mut th = TyperacerHighlighter::new();
        let lines = th.highlight_code_block(CODE);
        for line in lines {
            print!("{}", line);
        }
        let elapsed = instant.elapsed();
        println!("\nduration: {}", elapsed.as_secs_f32());
    }

    #[test]
    fn single_line() {
        let instant = std::time::Instant::now();

        let mut th = TyperacerHighlighter::new();
        let line = th.highlight_line(r#"let syntax = ps.find_syntax_by_extension("rs").unwrap();"#);
        print!("{}", line);

        let elapsed = instant.elapsed();
        println!("\nduration: {}", elapsed.as_secs_f32());
    }

    #[test]
    fn half_line() {
        let instant = std::time::Instant::now();

        let mut th = TyperacerHighlighter::new();
        let line = th.highlight_line(r#"let syntax = ps.find_syntax"#);
        print!("{}", line);

        let elapsed = instant.elapsed();
        println!("\nduration: {}", elapsed.as_secs_f32());
    }
}

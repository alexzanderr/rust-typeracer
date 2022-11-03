use ansi_parser::{
    AnsiParser,
    Output
};
use ansi_parser::AnsiSequence;

fn main() {
    // Parse the first two blocks in the list
    // By parsing it this way, it allows you to iterate over the
    // elements returned.
    //
    // The parser only every holds a reference to the data,
    // so there is no allocation.
    let parsed: Vec<Output> =
        "This is \u{1b}[3Asome text!".ansi_parse().collect();

    let parsed: Vec<Output> = "This is some text!".ansi_parse().collect();

    for par in parsed {
        match par {
            Output::Escape(ansi) => {
                let s = ansi.to_string();
                println!("{s:?} - len: {}", s.len());
            },
            _ => {}
        }
    }

    // let original_text_without_ansi = parsed.join("");

    // dbg!(&original_text_without_ansi);

    // assert_eq!(
    //     vec![
    //         Output::TextBlock("This is "),
    //         Output::Escape(AnsiSequence::CursorUp(3))
    //     ],
    //     parsed
    // );

    // for block in parsed.into_iter() {
    //     match block {
    //         Output::TextBlock(text) => println!("{}", text),
    //         Output::Escape(seq) => println!("{}", seq)
    //     }
    // }
}

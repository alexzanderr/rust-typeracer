#![feature(type_alias_impl_trait)]
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros
)]

/// this traits returns iterator of item from
/// &str, String,
/// &[&str], &[String]
/// &[&str; N], &[String; N]
/// Vec<&str>, Vec<String>
pub trait TermLines<'a>: core::fmt::Debug {
    type IteratorItem: core::fmt::Debug + core::fmt::Display;
    type OutputIterator: IntoIterator<Item = Self::IteratorItem>
        + core::fmt::Debug;

    fn term_lines(&'a self) -> Self::OutputIterator;
}

/// &str
impl<'a> TermLines<'a> for &'a str {
    type IteratorItem = &'a str;
    type OutputIterator = std::str::Split<'a, char>;

    fn term_lines(&'a self) -> Self::OutputIterator {
        self.split('\n')
    }
}

/// String
impl<'a> TermLines<'a> for String {
    type IteratorItem = &'a str;
    type OutputIterator = std::str::Split<'a, char>;

    fn term_lines(&'a self) -> Self::OutputIterator {
        self.split('\n')
    }
}

/// &[&str]
impl<'a> TermLines<'a> for &'a [&'a str] {
    type IteratorItem = &'a str;
    // type OutputIterator = core::slice::Iter<'a, &'a str>;
    type OutputIterator = core::iter::Map<
        std::slice::Iter<'a, &'a str>,
        impl FnMut(&'a &'a str) -> &'a str
    >;

    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = *self;
        _self.into_iter().map(|s| *s)
    }
}

/// &[String]
impl<'a> TermLines<'a> for &[String] {
    type IteratorItem = &'a str;
    type OutputIterator = core::iter::Map<
        std::slice::Iter<'a, String>,
        impl FnMut(&'a String) -> &'a str
    >;

    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = *self;
        _self.into_iter().map(|s: &'a String| s.as_str())
    }
}

/// &[&str; N]
impl<'a, const N: usize> TermLines<'a> for [&'a str; N] {
    type IteratorItem = &'a str;
    // type OutputIterator = std::slice::Iter<'a, &'a str>;
    type OutputIterator = std::array::IntoIter<&'a str, N>;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = *self;
        _self.into_iter()
    }
}

/// &[String; N]
impl<'a, const N: usize> TermLines<'a> for [String; N] {
    type IteratorItem = &'a str;
    // type OutputIterator = std::slice::Iter<'a, &'a str>;
    type OutputIterator = core::iter::Map<
        std::slice::Iter<'a, String>,
        impl FnMut(&'a String) -> &'a str
    >;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = self;
        _self.into_iter().map(|s: &'a String| s.as_str())
    }
}

/// Vec<&str>
impl<'a> TermLines<'a> for Vec<&str> {
    type IteratorItem = &'a str;
    // type OutputIterator = std::slice::Iter<'a, &'a str>;
    type OutputIterator = core::iter::Map<
        std::slice::Iter<'a, &'a str>,
        impl FnMut(&'a &'a str) -> &'a str
    >;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = self;
        _self.into_iter().map(|s| *s)
    }
}

/// Vec<String>
impl<'a> TermLines<'a> for Vec<String> {
    type IteratorItem = &'a str;
    type OutputIterator = std::iter::Map<
        std::slice::Iter<'a, String>,
        impl FnMut(&'a String) -> &'a str
    >;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        self.into_iter().map(|s: &String| s.as_str())
    }
}

/// Vec<String>
// impl<'a> TermLines<'a> for Vec<String> {
//     type IteratorItem = &'a str;
//     type OutputIterator = std::iter::Map<
//         std::slice::Iter<'a, String>,
//         impl FnMut(&'a String) -> &'a str
//     >;

//     /// "hello\nworld\nfrom\ncommand-line"
//     fn term_lines(&'a self) -> Self::OutputIterator {
//         self.into_iter().map(|s: &String| s.as_str())
//     }
// }

pub fn draw_rectangle<'a, L: TermLines<'a> + 'a>(lines: &'a L) {
    let mut index = 0usize;
    for line in lines.term_lines() {
        println!("{index:?}: {line:#?}");
        index += 1;
    }
    println!()
}

pub fn get_rectangle<'a, L: TermLines<'a, IteratorItem = &'a str> + 'a>(
    lines: &'a L
) -> Vec<String> {
    let mut index = 0usize;
    let lines = lines.term_lines();
    let mut items = Vec::new();
    for line in lines {
        // check_str(&line);
        println!("{index:?}: {line:#?}");
        let line = format!("{line}");
        items.push(line);
        index += 1;
    }
    items
}

pub fn get_rectangle_vec_str<
    'a,
    L: TermLines<'a, IteratorItem = &'a str> + 'a
>(
    // if you are not putting &'a L
    // error[E0597]: `lines` does not live long enough
    // cuz its moved and then deallocated at the end of the function
    // and im returning data from this function which was part of lines
    lines: &'a L
) -> Vec<&'a str> {
    let mut index = 0usize;
    let lines = lines.term_lines();
    let mut items = Vec::<&'a str>::new();
    for line in lines {
        // check_str(&line);
        println!("{index:?}: {line:#?}");
        items.push(line);
        index += 1;
    }
    items
}

fn main() {
    let lines = vec![
        "typeracer".to_string(),
        "its a very good".to_string(),
        "game".to_string(),
    ];
    let lines = get_rectangle_vec_str(&lines);
    dbg!(&lines);
    // let ml = "hello\nworld\nfrom\ncommand-line\n".term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = String::from("hello2\nworld2\nfrom2\ncommand-line2\n");
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = vec!["hello3", "world3"];
    // let ml = ml.as_slice();
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = ["hello4", "world4"];
    // let ml = ml.as_slice();
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = vec!["hello5".to_string(), "world5".to_string()];
    // let ml = ml.as_slice();
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = vec!["hello6", "world6"];
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }

    // let ml = vec!["hello7".to_string(), "world7".to_string()];
    // let ml = ml.term_lines();
    // for m1 in ml {
    //     println!("{m1}");
    // }
}

// pub fn test_boilerplate<
//     'a,
//     L: TermLines<'a, IteratorItem = &'a str> + 'a
// >(
//     lines: &'a L
// ) {
//     use assert2::assert;
//     let items = get_rectangle(lines);
//     assert!(items == vec!["typeracer", "its a very good", "game"]);
// }

#[cfg(test)]
mod tests {
    use assert2::assert;

    use super::*;

    #[test]
    fn test_str() {
        let lines = "typeracer\nits a very good\ngame";
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_string() {
        let lines = "typeracer\nits a very good\ngame".to_string();
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_slice_of_str() {
        let lines = vec!["typeracer", "its a very good", "game"];
        let lines = lines.as_slice();
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_slice_of_string() {
        let lines = vec!["typeracer", "its a very good", "game"];
        let lines: Vec<String> =
            lines.iter().map(|s| s.to_string()).collect();
        let lines = lines.as_slice();
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_array_of_str() {
        let lines = ["typeracer", "its a very good", "game"];
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_array_of_string() {
        let lines = [
            "typeracer".to_string(),
            "its a very good".to_string(),
            "game".to_string()
        ];
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_vec_of_str() {
        let lines = vec!["typeracer", "its a very good", "game"];
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }

    #[test]
    fn test_vec_of_string() {
        let lines = vec![
            "typeracer".to_string(),
            "its a very good".to_string(),
            "game".to_string(),
        ];
        let lines = get_rectangle_vec_str(&lines);
        assert!(lines == vec!["typeracer", "its a very good", "game"]);
    }
}

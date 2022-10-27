/// this traits returns iterator of item from
/// &str, String,
/// &[&str], &[String]
/// &[&str; N], &[String; N]
/// Vec<&str>, Vec<String>
pub trait TermLines<'a>: core::fmt::Debug + core::clone::Clone {
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
    // type OutputIterator = core::iter::Map<
    //     std::slice::Iter<'a, &'a str>,
    //     impl FnMut(&'a &'a str) -> &'a str
    // >;
    type OutputIterator = std::iter::Copied<std::slice::Iter<'a, &'a str>>;

    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = *self;
        _self.iter().copied()
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
        _self.iter().map(|s: &'a String| s.as_str())
    }
}

/// &[&str; N]
impl<'a, const N: usize> TermLines<'a> for [&'a str; N] {
    type IteratorItem = &'a str;
    // type OutputIterator = std::slice::Iter<'a, &'a str>;
    type OutputIterator = std::iter::Copied<std::slice::Iter<'a, &'a str>>;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        // let _self = *self;
        self.iter().copied()
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
        _self.iter().map(|s: &'a String| s.as_str())
    }
}

/// Vec<&str>
impl<'a> TermLines<'a> for Vec<&str> {
    type IteratorItem = &'a str;
    // type OutputIterator = std::slice::Iter<'a, &'a str>;
    // type OutputIterator = core::iter::Map<
    //     std::slice::Iter<'a, &'a str>,
    //     impl FnMut(&'a &'a str) -> &'a str
    // >;
    type OutputIterator = std::iter::Copied<std::slice::Iter<'a, &'a str>>;

    /// "hello\nworld\nfrom\ncommand-line"
    fn term_lines(&'a self) -> Self::OutputIterator {
        let _self = self;
        _self.iter().copied()
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
        self.iter().map(|s: &String| s.as_str())
    }
}

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
unused_macros,
)]

use dbg_pls::{pretty, DebugPls, color};

#[derive(DebugPls, Copy, Clone, Debug)]
pub struct Demo {
    foo: i32,
    bar: &'static str,
}

use std::sync::*;

fn main() {
    let mut val = [Demo { foo: 5, bar: "hello" }; 10];
    val[6].bar = "Hello, world! I am a very long string";


    for index in 0..1 {
        // 100x times slower
        let output = format!("{}", color(&val));
        // let output = format!("{:#?}", &val);
        println!("{}", output);
        let output = format!("{}", color(&String::from("asdasdasd")));
        println!("{}", output);

        color!(&val);
        color!(&String::from("asdasdasd"));
        color!(&Arc::new(Mutex::new(String::from("its not working"))));
    }
}

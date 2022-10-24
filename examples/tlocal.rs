#![allow(unused, non_upper_case_globals)]

// using this i dont need lazy_static! anymore
// only if i have 1 thread
thread_local! {
    pub static global_string: String = {
        String::from("im a global string")
    };
    pub static global_vector: Vec<&'static str> = {
        vec![
        "im a global string vector",
        "im a global string vector",
        "im a global string vector",
        "im a global string vector",
        "im a global string vector",
        ]
    };
}

fn main() {
    global_vector.with(|item| {
        println!("{:#?}", item);
    });
    global_string.with(|item| {
        println!("{:#?}", item);
    });

    let handle = std::thread::spawn(move || {
        global_vector.with(|vector| {
            for item in vector.iter() {
                println!("{}", item);
            }
        });
    });
    handle.join().unwrap();
    //  what????????????
    global_vector.with(|item| {
        println!("{:#?}", item);
    });
}

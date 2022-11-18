use quote::quote;
use syn::*;
use colored::*;
use chrono::prelude::*;
use colors_transform::{
    Color,
    Rgb
};

use crate::TokenStream;

/// Instruments the function with a `timer!`, which logs a message at the end of function
/// execution stating the elapsed time.
///
/// The attribute accepts two string literals as arguments. The first is the log level,
/// valid values of which are "error", "warn", "info", "debug", "trace" or "never".
/// The default value is "debug". "never" can be used to temporarily disable instrumentation
/// of the function without deleting the attribute.
///
/// The second argument is the function name pattern. The pattern is helpful to
/// disambiguate functions when you have many functions in the same module with the same
/// name: `new` might occur many times on different structs, for example. In the pattern,
/// "{}" will be replaced with the name of the function.
///
/// Examples:
///     #[time]                                 // Use default log level of Debug
///     #[time("info")]                         // Set custom log level
///     #[time("info", "FirstStruct::{}")]      // Logs "FirstStruct::new()" at Info
///     #[time("info", "SecondStruct::{}")]     // Logs "SecondStruct::new()" at Info
///     #[time("ThirdStruct::{}")]              // Logs "ThirdStruct::new()" at Debug
///     #[time("never")]                        // Turn off instrumentation at compile time
pub fn stopwatch_internal(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream
) -> TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);

    let attrs = input_fn.attrs;
    let visibility = input_fn.vis;
    let function_name = input_fn.sig.ident;
    let inputs = input_fn.sig.inputs;
    let output = input_fn.sig.output;
    let generics = &input_fn.sig.generics;
    let where_clause = &input_fn.sig.generics.where_clause;
    let block = input_fn.block;
    let pretty_fn_name = format!("{}()", function_name);
    // doesnt work from being here
    let time_profiler_name =
        "TIME-PROFILER".truecolor(144, 194, 157).to_string();

    pub fn hex_colored(
        _string: &str,
        hex_string: &str
    ) -> String {
        let rgb_color = Rgb::from_hex_str(hex_string).unwrap();
        let (r, g, b) = (
            rgb_color.get_red() as u8,
            rgb_color.get_green() as u8,
            rgb_color.get_blue() as u8
        );
        _string.truecolor(r, g, b).to_string()
    }

    let code = quote! {
        #(#attrs)* #visibility fn #function_name #generics (#inputs) #output #where_clause {
            let __stopwatch = ::std::time::Instant::now();
            use ::colored::Colorize;


            let function_result = #block;

            let elapsed = __stopwatch.elapsed();
            let elapsed_pretty = format!("{:?}", elapsed);

            let dt = ::chrono::Local::now();
            let current_datetime = dt.format("%d-%m-%Y %H:%M:%S");

            println!("[{}] {}\n\t[{}:{}::{}]\n\t\t execution time: {}\n",
                "TIME-PROFILER".truecolor(144, 194, 157).to_string(),
                current_datetime.to_string().truecolor(97, 146, 208).to_string(),
                file!(),
                line!(),
                (#pretty_fn_name).to_string().truecolor(211, 131, 38).to_string(),
                elapsed_pretty.yellow());

            function_result
        }
    };

    TokenStream::from(code)
}

pub fn stopwatch_to_file_internal(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream
) -> TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);

    let attrs = input_fn.attrs;
    let visibility = input_fn.vis;
    let function_name = input_fn.sig.ident;
    let inputs = input_fn.sig.inputs;
    let output = input_fn.sig.output;
    let generics = &input_fn.sig.generics;
    let where_clause = &input_fn.sig.generics.where_clause;
    let block = input_fn.block;
    let pretty_fn_name = format!("{}()", function_name);
    // doesnt work from being here
    let time_profiler_name =
        "TIME-PROFILER".truecolor(144, 194, 157).to_string();

    pub fn hex_colored(
        _string: &str,
        hex_string: &str
    ) -> String {
        let rgb_color = Rgb::from_hex_str(hex_string).unwrap();
        let (r, g, b) = (
            rgb_color.get_red() as u8,
            rgb_color.get_green() as u8,
            rgb_color.get_blue() as u8
        );
        _string.truecolor(r, g, b).to_string()
    }

    let code = quote! {
        #(#attrs)* #visibility fn #function_name #generics (#inputs) #output #where_clause {
            let __stopwatch = ::std::time::Instant::now();
            use ::colored::Colorize;
            use ::std::io::Write;


            let function_result = #block;

            let elapsed = __stopwatch.elapsed();
            let elapsed_pretty = format!("{:?}", elapsed);
            let elapsed_pretty = format!("{} ms ", elapsed.as_millis());

            let dt = ::chrono::Local::now();
            let current_datetime = dt.format("%d-%m-%Y %H:%M:%S");

            let mut file_handler = ::std::fs::File::options()
                .append(true)
                .open("time-profiler.log").unwrap();

            let out = format!("[{}] {}\n\t[{}:{}::{}]\n\t\t execution time: {}\n",
                "TIME-PROFILER".truecolor(144, 194, 157).to_string(),
                current_datetime.to_string().truecolor(97, 146, 208).to_string(),
                file!(),
                line!(),
                (#pretty_fn_name).to_string().truecolor(211, 131, 38).to_string(),
                elapsed_pretty.yellow());

            let out = strip_ansi_escapes::strip(&out).unwrap();
            let out = String::from_utf8(out).unwrap();
            let _ = writeln!(file_handler, "{}", out);

            function_result
        }
    };

    TokenStream::from(code)
}

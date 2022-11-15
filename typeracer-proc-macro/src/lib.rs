#[macro_use]
extern crate proc_macro;

use proc_macro::TokenStream;

mod internal;

#[doc = "
# BuilderMethod proc-macro

This macro implements `builder() -> <StructName>Builder` function for a struct that has `#[derive(Builder)]` from `derive_builder` crate

The source code looks like this:
```rust
use derive_builder::Builder;

// This is our beautiful proc-macro
use derive_builder_method::BuilderMethod;

#[derive(Builder, Debug, Default, BuilderMethod)]
struct SomeStruct {
    field: String
}

// The source code you get from BuilderMethod proc-macro is
impl SomeStruct {
    pub fn builder() -> SomeStructBuilder {
        SomeStructBuilder::default()
    }
}
// Thats it
```
Note: SomeStruct must derive Builder, otherwise its not going to work
"]
#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    internal::colorize_internal(input)
}

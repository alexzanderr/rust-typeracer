

# dev-ideas as in Development Ideas
this is a place where i write code as an experimental example to help me think how would i integrate it in the main codebase

# examples folder
`examples` is a place for new experimental ideas that i could implement or that i tried to think and implement an app design


# to run an example
```shell
❱ cargo run -p dev-ideas --example $example --all-features
```
you need `--all-features` because this entire workspace member called `dev-ideas` has its app dependencies under a feature called `dev-ideas`.

why?

because i dont want the dependencies which i use in experimental tests to be compiled in the main codebase

and also there is only one feature `dev-ideas`

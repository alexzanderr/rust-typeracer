

# git hooks
why there is a `githooks` here instead of being in the folder `.git/hooks/**` ?

because this hooks are useful and maybe other developers just want to use this hooks

## to install these hooks into your .git/hooks

run
```shell
git config core.hooksPath githooks
```
this will add the `githooks` directory as main git hooks dir

and you are ready to go

when you run commit or push these hooks will be triggered

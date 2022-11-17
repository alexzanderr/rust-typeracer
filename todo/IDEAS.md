# TODOs Ideas

- [x] implement config logic into the project

- [ ] when doing CI, build for Linux, MacOS and windows (3 targets) and also try all toolchains (stable, beta, nightly);
  so 3 targets with 3 toolchains == 3 * 3 == 9 tests with
  - cargo fmt --all -- --check
  - cargo clippy
    - cargo test --workspace --all-features -- --show-output
  - cargo test --lib -- --show-output
  - cargo audit check

    a total of 9 * 4 == 36 integration checks

  also: after merging from `dev` or `some-feature-branch` into `main` the book should be recreated if changes are made.
  huh? this doesnt make sense

- [ ] what do to in case i want to show the README.md from another branch
  all the included links are on blob/main, or show the readme for that branch only with data that is looks the same
  between branches, we need another host for that; `common static items` i should call it

- [ ] check this
  nice `mdbook github action` [`https://github.com/peaceiris/actions-mdbook/blob/main/README.md`](https://github.com/peaceiris/actions-mdbook/blob/main/README.md)

- [ ] when using CI; there is a dependency cacher for github actions to not install all dependecies at every CI action



- [ ] use another linker and more stuff from that article to improve compile time speed

- [ ] improve performance of the code

- [ ] add option to mute (`Soloud::set_volume(0.0)?;`) the song in the middle of the game
  for this we need a music menu to control what will happen with the music

- [ ] add auto detection of window lostfocus and then the game should pause automatically

- [ ] fix typeracer game logic inside the match block, some things are redundant;
  inside `Typeracer::handle_key_event(&mut self)` method

- [ ] implement control + backspace for the typeracer_text


- [ ] download the default music from yt using yt-dlp api in rust (later) or from my mirror; or from github, lol, why
  from yt?? thats slow

  or the user can put a url of a song to download at runtime

- [ ] add ansi code to print url as text inside TUI to click on it and to go on docs for example

- [ ] dont forget that there are `TODO`s inside the source code as well

- [ ] HARD: make a TUI for debugging and logging data from main app to look at while developing; connect them with tpc
  sockets; they will be separate processes

- [ ] so the secret to `no cursor flickering` is to put a higher sleep for the ui, something like 500 ms or 1 second

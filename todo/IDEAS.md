# TODOs Ideas

- [ ] to get 60 FPS (frames per second) you need 16.66 ms sleep in the game loop
  explanation: if you sleep 1 second (1000ms), you need to 1000/60 to get the sleep in milliseconds to actually draw 60
  frames per second. it will be 1 frame at every 16.66 ms. 16.66 ms * 60 == 999.6 ms  ~= 1000 ms (60 frames in a single
  second on the clock)

  to get the `total_ms_sleep`, divide `1000/fps`


- [ ] rename the entire project to `tty-racer`

- [ ] implement config logic into the project

- [ ] when doing CI, build for Linux, MacOS and windows (3 targets) and also try all toolchains (stable, beta, nightly);
  so 3 targets with 3 toolchains == 3 * 3 == 9 tests with
  - cargo fmt --all -- --check
  - cargo clippy
  - cargo test --workspace -- --show-output
  - cargo test --lib -- --show-output
  - audit check

    a total of 9 * 4 == 36 integration checks

  also: after merging from `dev` or `some-feature-branch` into `main` the book should be recreated if changes are made.
  huh? this doesnt make sense

- [ ] what do to in case i want to show the README.md from another branch
  all the included links are on blob/main, or show the readme for that branch only with data that is looks the same
  between branches, we need another host for that; `common static items` i should call it

- [ ] check this
  nice `mdbook github action` [`https://github.com/peaceiris/actions-mdbook/blob/main/README.md`](https://github.com/peaceiris/actions-mdbook/blob/main/README.md)

- [ ] when using CI; there is a dependency cacher for github actions to not install all dependecies at every CI action

- [ ] create a typeracer-proc-macro crate inside this workspace

- [ ] add progress bar for showing how much text is left to type

- [ ] use another linker and more stuff from that article to improve compile time speed

- [ ] improve performance of the code

- [ ] add option to mute (`Soloud::set_volume(0.0)?;`) the song in the middle of the game
  for this we need a music menu to control what will happen with the music

- [ ] add auto detection of window lostfocus and then the game should pause automatically

- [ ] fix typeracer game logic inside the match block, some things are redundant


- [ ] implement control + backspace for the typeracer_text

- [ ] calculate WPM and print it on the screen

- [ ] cand wpm depaseste 50 sau valoare definita de user atunci ruleaza in alt thread unstoppable, sau "blazingly fast"
  - primeagen + plus sunetul ala de piuuu

- [ ] download the default music from yt using yt-dlp api in rust (later)

- [ ] add ansi code to print url as text inside TUI to click on it and to go on docs for example

- [ ] dont forget that there are `TODO`s inside the source code as well

- [ ] you can make a separate thread to receive signals if the main thread blocks; then you can `std::process::exit(1)`
  from there

- [ ] add custom made builder for `TerminalScreen` which is not something like `#[derive(Builder)]`

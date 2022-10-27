

# TODO

- [x] implement ctrl-backspace to previous delete an entire word

- [x] add alternate screen on which the game will be run without changing the state of the root terminal

- [x] make this project a library + binary (so that i can use the lib inside examples or other project crates)

- [x] add music

- [ ] improve music player with methods like
    - `stop_playing`
    - `pause_playing`
    - `is_playing`

- [ ] improve performance of the code

- [ ] fix unicode errors like this one: byte index 201 is not a char boundary; it is inside '’' (bytes 200..203)

    maybe with:
        - unicode segmentation
        - unicode crates stuff

- [ ] add option from cli and config to show invisibles like `tab` or `\n`

- [ ] add option for the border type
```rs
pub enum BorderType {
    Round,
    Square
}
```
- [x] add multi-line typeracing (hardest one)
- [ ] implement control + backspace for the typeracer_text
- [ ] calculate WPM and print it on the screen
- [ ] migrate to some TUI framework
- [ ] check this https://github.com/kraanzu/termtyper
- [ ] check this crates that are using crossterm:
    - https://github.com/nushell/reedline
    - https://github.com/rhysd/tui-textarea
    - https://github.com/d-e-s-o/notnow/tree/3fa49e41550926d865afc56446bd950a39a139fc/
- [ ] cand wpm depaseste 50 sau valoare definita de user atunci ruleaza in alt thread unstoppable, sau "blazingly fast" - primeagen + plus sunetul ala de piuuu
- [ ] download the default music from yt using yt-dlp api in rust (later)


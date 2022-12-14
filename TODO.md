

# TODO

- [x] implement ctrl-backspace to previous delete an entire word

- [x] add alternate screen on which the game will be run without changing the state of the root terminal

- [x] make this project a library + binary (so that i can use the lib inside examples or other project crates)

- [x] add music

- [ ] add progress bar for showing how much text is left to type

- [ ] use another linker and more stuff from that article to improve compile time speed

- [ ] you can bench the MusicPlayer load time, if its so slow

- [ ] handle screen resize, just update the term height and width inside the TyperacerUI

- [ ] if you want to have mutliple threads you can use `channels/crossbeam-channels/arc<mutexAppState>>` to send data and to tell the music thread to stop the music completely or to pause the music
    resources:
        - https://www.reddit.com/r/rust/comments/7um395/dynamic_load_at_compile_time/

    why do i want a thread for music?
    1. well, the load time for the music is very slow and we need to do that on a separate thread while printing the UI instantly and start playing
    2. also you cant send the Music player between thread cuz Soloud (`*mut *mut c_void`) cannot ne sent between threads safely, so it doesnt implement `Send` `Sync` traits

    idea: add music state which is represented by an enum
    ```rs
    pub enum MusicState {
        Stopped,
        Paused,
        Playing
    }
    ```
    another cool idea: add this music state as field to AppState

- [ ] remove the arc.clone() every time you pass app state to Typeracer methods

- [x] improve music player with methods like
    - `stop_playing`
    - `pause_playing`
    - `is_playing`

- [ ] improve project quality by adding a better readme, import from other rust projects, by adding CI, by adding tests for individual methods or functions

- [ ] load the music files using rayon crate for parallelism

- [ ] improve performance of the code

- [ ] add configuration for the UI layout to please the user; maybe some users want to see the WPM at the bottom; maybe some users dont want to see WPM at all

- [x] add option to pause the song in the middle of the game
- [ ] add option to mute (`Soloud::set_volume(0.0)?;`) the song in the middle of the game

- [ ] add auto detection of window lostfocus and then the game should pause automatically

- [ ] add a stopwatch that is on a separate thread which can be paused for when i pause the game

- [ ] to solve this error:

    error[E0277]: `RefCell<usize>` cannot be shared between threads safely
    EDIT (29.10.2022-17:39):
        - this error originates from the usage of `Arc<Mutex<&AppState>>`; notice: the &reference;
        - the problem is that even im using a reference to app state, and that reference cant be sync;
        - removing the `&` solves the problem


    2 options:
        1. use Mutex for every field on AppState (EDIT: dont need to, the ref was the problem)
        2. send MusicState enum as a message between threads to communicate
            with the music player and to stuff accordingly


- [ ] right now cant use unicode inside the ??????????
    fix unicode errors like this one: byte index 201 is not a char boundary; it is inside '???' (bytes 200..203)

    maybe with:
        - unicode segmentation
        - unicode crates stuff

- [ ] add option from cli and config to show invisibles like `tab` or `\n`

- [ ] fix typeracer game logic inside the match block, some things are redundant

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


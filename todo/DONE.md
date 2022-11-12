# TODOs Done

- [x] implement ctrl-backspace to previous delete an entire word

- [x] add alternate screen on which the game will be run without changing the state of the root terminal

- [x] make this project a library + binary (so that i can use the lib inside examples or other project crates)

- [x] add music

- [x] start the stopwatch thread only after the first keyboard strike

- [x] you can bench the MusicPlayer load time, if its so slow

- [x] handle screen resize, just update the term height and width inside the TyperacerUI

- [x] if you want to have mutliple threads you can use `channels/crossbeam-channels/arc<mutex<AppState>>` to send data
  and to tell the music thread to stop the music completely or to pause the music
  resources:
  - https://www.reddit.com/r/rust/comments/7um395/dynamic_load_at_compile_time/

  why do i want a thread for music?
  1. well, the load time for the music is very slow and we need to do that on a separate thread while printing the UI
     instantly and start playing
  2. also you cant send the Music player between thread cuz Soloud (`*mut *mut c_void`) cannot ne sent between threads
     safely, so it doesnt implement `Send` `Sync` traits

  idea: add music state which is represented by an enum
    ```rs
    pub enum MusicState {
        Stopped,
        Paused,
        Playing
    }
    ```
  another cool idea: add this music state as field to AppState

- [x] remove the arc.clone() every time you pass app state to Typeracer methods

- [x] improve music player with methods like
  - `stop_playing`
  - `pause_playing`
  - `is_playing`
  -
- [x] add option to pause the song in the middle of the game

- [x] load the music files using rayon crate for parallelism or using `std::thread::spawn`
  im not sure if Soloud can be modified on multiple threads, it is!

  `Wav` cannot be sent between threads safely, even with unsafe! WHY??

  because the compiler doesnt care about unsafe when Send and Sync are dont impl for that type


- [x] add a stopwatch that is on a separate thread which can be paused for when i pause the game

- [x] `RefCell<usize>` cannot be shared between threads safely

  error[E0277]: `RefCell<usize>` cannot be shared between threads safely
  EDIT (29.10.2022-17:39):
  - this error originates from the usage of `Arc<Mutex<&AppState>>`; notice: the &reference;
  - the problem is that even im using a reference to app state, and that reference cant be sync;
  - removing the `&` solves the problem

  2 options:
  1. use Mutex for every field on AppState (EDIT: dont need to, the ref to app state (`&AppState`) was the problem)
  2. send MusicState enum as a message between threads to communicate
     with the music player and to stuff accordingly (NEAH)

- [x] added unicode support in #58aa37b
  right now cant use unicode inside the `текст`(text in russian)
  fix unicode errors like this one: byte index 201 is not a char boundary; it is inside '’' (bytes 200..203)

  maybe with:
  - unicode
    segmentation (https://docs.rs/unicode-segmentation/1.10.0/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes)
  - unicode crates stuff

- [x] add multi-line typeracing (hardest one)

- [x] add pre-commit hooks

- [x] add private tests for the function `color_formatted_text`

- [x] put music on separate feature, for example; features are for conditional compilation on libraries, this has
  nothing to do with triggering ON/OFF music inside a binary game; features in this project are for the `typeracer-lib`

- [x] split this `TODO.md` file on multiple files within a folder, because it will grow very large in the future

- [x] also add in config toml
    ```toml
        [ui]
        fps = 60
    ```
  and also limit the frame frate between: `[1; 100]` if not; runtime-error: ... msg

- [x] add configuration for the UI layout to please the user; maybe some users want to see the WPM at the bottom; maybe
  some users dont want to see WPM at all
    ```toml
        [ui]    
        # maybe some people will get intimidated while
        # watching the WPM and playing at the same time
        # some people just dont want to see `WPM dropping`
        show_wpm = "true"
        show_invisibles = "true"
    ```


- [x] add option for the border type in config
    ```rust
    pub enum BorderType {
        Round,
        Square
    }
    ```


- [x] add option from cli and config to show invisibles like `tab`(`⭾`) or `\n`(`↵`)

    ```toml
        [ui]
        show_invisibles = "true"
    ```
  this would be at `~/.config/typeracer/config.toml`
  we need to come up with a different name for the folder because there is already a typeracer game called `typeracer`(
  binary name)

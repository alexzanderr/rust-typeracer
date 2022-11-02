
# Mutex lock problem

## problem description:
I had two loops which locked the mutex;
the main loop was locking but no release/drop of the mutex-lock cuz it wasnt inside a secondary block of code.

## here's the entire code that you can test
```rust
use std::time::{
    Duration,
    Instant
};
use std::thread::{
    sleep,
    spawn,
    Builder as ThreadBuilder
};
use std::sync::{
    Arc,
    Mutex
};
use std::io::{
    stdout,
    Stdout,
    Write
};

fn main() {
    let elapsed_arc = Arc::new(Mutex::new(0usize));
    let elapsed_arc_clone = elapsed_arc.clone();

    let stopwatch_thread = ThreadBuilder::new()
        .name("stopwatch-thread".to_string())
        .spawn(move || {
            let elapsed_arc = elapsed_arc_clone;

            loop {
                println!("time thread start loop");

                // is waiting for the other lock to be released
                if let Ok(mut elapsed_mutex) = elapsed_arc.lock() {
                    *elapsed_mutex += 1;
                    println!("{}", *elapsed_mutex)
                } else {
                    eprintln!("wrong")
                }

                sleep(Duration::from_millis(1000));
            }
        })
        .unwrap();

    let mut stdout = stdout();
    loop {
        // get current elapsed time from the secondary thread
        let elapsed_mutex = elapsed_arc.lock().unwrap();
        println!("elapsed from main: {}", *elapsed_mutex);
        // if let Ok(elapsed_mutex) = elapsed_arc.lock() {
        //     println!("elapsed from main: {}", *elapsed_mutex);
        // } else {
        //     eprintln!("wrong main");
        // }
        // write!(stdout, "\r{}", *elapsed);
        // stdout.flush().unwrap();

        sleep(Duration::from_millis(500));
    }
}
```
this code would print
```sh
elapsed from main: 0
time thread start loop
elapsed from main: 0
elapsed from main: 0
elapsed from main: 0
elapsed from main: 0
elapsed from main: 0
^C
```
notice that the `stopwatch-thread`'s loop prints `time thread start loop` only one time then it stops and the counter is not incremented.

meaning that the loop stopped at this line
`if let Ok(mut elapsed_mutex) = elapsed_arc.lock() {`
which called `lock()` and never returned a response

sooo ... from what we know about locks and mutexes, lock is `blocking` and it waits for the lock to be released (by other threads) in order for it to acquire.

but why it never acquires the lock? because is waiting for `'mainloop: loop { ... }` to release the lock which was acquired on the main thread

the solution is the manually force drop the mutex
```rs
loop {
    {
        let elapsed_mutex = elapsed_arc.lock().unwrap();
        println!("elapsed from main: {}", *elapsed_mutex);
        // here mutex guard will be dropped
    }

    sleep(Duration::from_millis(500));
}
```
now it prints the expected output: seeing the elapsed time being incremented
```shell
elapsed from main: 0
elapsed from main: 1
elapsed from main: 2
elapsed from main: 2
elapsed from main: 3
elapsed from main: 3
elapsed from main: 4
elapsed from main: 4
elapsed from main: 5
elapsed from main: 5
# ...
# ...
```

but wouldnt that happen at the end of the loop? yes!

but the loop is fast, first of all, and second of all (which is most important), the lock is dropped only at the end of loop's block. but immediatelly after dropping the lock, its immediately acquired again, while the other mutex is waiting from the `stopwatcth-thread`

### Conclusion
to make sure everything is working while working with mutexes: always force drop your mutexes using a secondary block `{ .. }`
```rust
/// ...
{
    let mutex_guard = arc_pointer.lock().unwrap();
    // do something with `mutex_guard`

    // here the lock is released on calling `drop`
}
/// ...
```

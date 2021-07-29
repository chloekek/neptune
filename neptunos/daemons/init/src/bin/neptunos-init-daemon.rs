use std::thread;
use std::time::Duration;

fn main()
{
    println!("Hello, world!");
    loop {
        let dur = Duration::from_secs(60);
        thread::sleep(dur);
    }
}

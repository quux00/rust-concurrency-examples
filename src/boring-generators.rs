use std::io::timer::sleep;
use std::rand::Rng;
use std::os;

//
// Rust v0.9
//
// Channels-driven concurrency using Go examples from:
// Rob Pike's talk on Google I/O 2012:
// http://www.youtube.com/watch?v=f6kdp27TYZs&feature=youtu.be
//
// Gist I made of Pike's code: https://gist.github.com/quux00/4428907
//

fn main() {
    if (os::args().len() != 2) {
        println("Usage: boring-generators [single|multiple]");
    } else if os::args()[1] == ~"single" {
        single_generator();
    } else {
        multiple_generators();
    }
}

// (1) Generator: function that returns the port (receiving side of the chan)
fn boring(msg: ~str) -> Port<~str> {
    let (port, chan): (Port<~str>, Chan<~str>) = Chan::new();
    do spawn {
        let mut i = 0;
        loop {
            let success = chan.try_send(format!("{} {}", msg, i));
            if !success { break; }
            let r = std::rand::rng().gen_range(1u64, 1000);
            sleep(r);
            i += 1;
        }
    }
    port
}

fn single_generator() {
    let port = boring(~"boring!");
    for _ in range(0, 5) {
        println!("You say: {}", port.recv());
    }
    println("You're boring: I'm leaving");
}


fn multiple_generators() {
    let joe = boring(~"Joe");
    let ann = boring(~"Ann");

    for _ in range(0, 10) {
        println(joe.recv());
        println(ann.recv());
    }
    println("You're boring: I'm leaving");
}

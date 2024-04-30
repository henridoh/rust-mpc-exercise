use std::io;
use std::thread;
use crate::circuit::Circuit;
use crate::party::new_party_pair;

pub mod circuit;
pub mod party;
pub mod mul_triple;

/// For argument parsing, my favorite crate is clap https://docs.rs/clap/latest/clap/
/// Especially its derive feature makes declarative argument parsing really easy.
/// You can add clap as a dependency with the derive feature and annotate this struct
/// and add the necessary fields.
/*struct Args {
    arg: PathBuf
}*/


fn main() {
    // The main function should first parse the passed arguments (I recommend to use a crate like
    // clap), and then evaluate the passed circuit. Note that you will likely need to run each
    // Party in its own thread (see https://doc.rust-lang.org/std/thread/index.html).
    println!("Hello, world!");

    let _stdin = io::stdin();

    let c = Circuit::parse_lines(&mut _stdin.lines().map(|x| x.unwrap()));

    let (mut p1, mut p2) = new_party_pair(c);

    let x: u64 = 123;
    let y: u64 = 456;

    let mut ser_x = [false; 64];
    let mut ser_y = [false; 64];

    for i in 0..64 {
        ser_x[i] = (x >> i) & 1 == 1;
        ser_y[i] = (y >> i) & 1 == 1;
    }

    let h1 = thread::spawn(move || {
        p1.execute(&ser_x).unwrap()
    });

    let h2 = thread::spawn(move || {
        p2.execute(&ser_y).unwrap()
    });

    let res1 = h1.join().unwrap();
    let res2 = h2.join().unwrap();

    assert_eq!(res1, res2);

    let mut res: i64 = 0;
    for i in 0..64 {
        res += if res1[i] {1} else {0} << i;
    }

    println!("{res}")
}

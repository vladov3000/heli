use std::{thread, time};

const FRAME0: &str = "

LOL  ROFL:ROFL:LOL
  \\\\        ____I____
    ========    | |[ \\
            \\___O==___)
            ___I_I__/
";

const FRAME1: &str = "
 L
 O             LOL:ROFL:ROFL
 L\\\\        ____I____
    ========    | |[ \\
            \\___O==___)
            ___I_I__/
";

fn main() {
    let frames = [FRAME0, FRAME1];
    let mut frame_num = 0;

    let frame_clear = "\x1b[1A\x1b[2K".repeat(FRAME0.chars().filter(|c| *c == '\n').count());

    loop {
        print!("{}", frames[frame_num]);
        thread::sleep(time::Duration::from_millis(100));
        print!("{}", frame_clear);
        frame_num = 1 - frame_num;
    }
}

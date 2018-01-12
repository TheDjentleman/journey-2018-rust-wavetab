use std::error::Error;  // for error description
use std::io::{Write};
use std::process::{Command, Stdio};

pub fn plot_wave<T: ToString>(wave: &Vec<T>) {
        let wave_val_str: Vec<String> = wave.iter().map(|i| i.to_string()).collect();

        let mut x_arr = String::from("[");
        let mut y_arr = String::from("[");
        for (i, st) in wave_val_str.iter().enumerate() {
            x_arr.push_str(&i.to_string()[..]);
            y_arr.push_str(&st[..]);
            if i < wave_val_str.len() - 1 {
                x_arr.push_str(",");
                y_arr.push_str(",");
            } else {
                x_arr.push_str("]");
                y_arr.push_str("]");
            }
        }

        // https://rustbyexample.com/std_misc/process/pipe.html
        // https://doc.rust-lang.org/std/process/struct.Stdio.html#method.piped
        // https://rustbyexample.com/std_misc/process/wait.html
        // set a stdin pipe to pipe our commands to the process running python (pipe to python process stdin)
        let exec_str = format!("import matplotlib.pyplot as plt\nplt.plot({}, {})\nplt.show()", &x_arr[..], &y_arr[..]);
        let mut process = match Command::new("python").stdin(Stdio::piped()).spawn() {
            Err(why) => panic!("Couldn't spawn python process: {}", why.description()),
            Ok(process) => process
        };
        {
            let ref mut stdin = process.stdin.as_mut().unwrap();
            stdin.write_all(exec_str.as_bytes()).expect("Failed to write python command");
            stdin.write_all(b"\n").expect("Failed to write python command");
        }
        process.wait().unwrap();
}

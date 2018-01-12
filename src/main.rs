extern crate wavetab;
extern crate hound;

use std::env;

use wavetab::Wavetab; // bring the struct into scope
use wavetab::plotting; // plotting utils

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file_path = &args[1];

    let wt = Wavetab::from_file(file_path);
    plotting::plot_wave(wt.wave());
    wt.convert_wave(wavetab::AnalysisMethod::Dsp);
}

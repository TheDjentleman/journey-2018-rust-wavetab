use utils::ArrExt;
use plotting;

pub fn analyse_dsp(wave: &Vec<i16>, sample_rate: u32) {
    println!("Analysing wave of length: {} sec.", wave.len() as u32 / sample_rate);
    println!("Counted {} played notes", count_notes(wave, sample_rate));
}

pub fn analyse_genetic(wave: &Vec<i16>, sample_rate: u32) {

}

pub fn analyse_mixed(wave: &Vec<i16>, sample_rate: u32) {
    
}

// we will try to detect "hard" jumps in the signal
fn count_notes(wave: &Vec<i16>, sample_rate: u32) -> u32 {
    // I assume we don't pick notes faster than every 0.02 sec (at least for now), which is playing 16'th notes at 200 bpm
    // window: 0.01sec (oversample by using half the duration) -> sample_rate * window
    let window_size = (sample_rate as f32 * 0.01) as usize;
    println!("window: {} -> number of buckets: {}", window_size, wave.len() / window_size);

    // we don't care if the signal is positive or negative, thus we use absolute values only
    let abs_wave: Vec<i16> = wave.abs();  
    plotting::plot_wave(&abs_wave);
    let mut pos: usize = 0;
    while pos + window_size < abs_wave.len() {
        let max_val = abs_wave[pos..pos+window_size].max_val();

        println!("Max val: {}", max_val);
        pos += window_size;
    }
    1
}

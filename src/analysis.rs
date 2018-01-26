use utils::ArrExt;
use plotting;

pub fn analyse_dsp(wave: &Vec<i16>, sample_rate: u32) {
    println!("Analysing wave of length: {} sec.", wave.len() as u32 / sample_rate);
    println!("Counted {} played notes", count_notes(wave, sample_rate));
}

pub fn analyse_genetic(wave: &Vec<i16>, sample_rate: u32) {
    println!("{:?}, {}", wave, sample_rate);
}

pub fn analyse_mixed(wave: &Vec<i16>, sample_rate: u32) {
    println!("{:?}, {}", wave, sample_rate);
}

// we will try to detect "hard" jumps in the signal
fn count_notes(wave: &Vec<i16>, sample_rate: u32) -> u32 {
    // I assume we don't pick notes faster than every 0.02 sec (at least for now), which is playing 16'th notes at 200 bpm
    // window: 0.01sec (oversample by using half the duration) -> sample_rate * window
    let window_size = (sample_rate as f32 * 0.01) as usize;
    println!("window: {} -> number of buckets: {}", window_size, wave.len() / window_size);

    // we don't care if the signal is positive or negative, thus we use absolute values only
    //let abs_wave: Vec<i16> = wave.abs();  
    //plotting::plot_wave(&abs_wave);
    let clamped_wave: Vec<i16> = wave.clamp_zero();
    plotting::plot_wave(&clamped_wave);

    // calculate local maxima for each oscillation (is always located between zeros)
    let mut peak_end = 0;
    let mut peak_vals: Vec<i16> = Vec::new();
    while peak_end < wave.len() {
        let mut peak_start = peak_end;

        // find start
        while peak_start < wave.len() && wave[peak_start] <= 0 { peak_start += 1; }
        if peak_start == wave.len() { break; }
        peak_start -= 1; // set peak start back to last position that was 0, to prevent the first peak_end to not being out of range (which might happen when the signal is cut off at the end of the clip)

        // peak end
        peak_end = peak_start + 1;
        while peak_end < wave.len() && wave[peak_end] > 0 { peak_end += 1; }
        if peak_end == wave.len() { break; }

        peak_vals.push(wave[peak_start..peak_end].max_val().clone());
    }
    println!("{:?}", peak_vals);

    /*
    let mut pos: usize = 0;
    while pos + window_size < abs_wave.len() {
        let max_val = abs_wave[pos..pos+window_size].max_val();

        println!("Max val: {}", max_val);
        pos += window_size;
    }
    */
    1
}

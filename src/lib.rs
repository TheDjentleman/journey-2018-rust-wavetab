// build library with cargo build
extern crate hound;
extern crate num;

pub mod plotting;
mod analysis;
mod tab_generation;
mod utils;

pub enum AnalysisMethod {
    Dsp,
    Genetic,
    Mixed
}

pub struct Wavetab {
    wave: Vec<i16>,
    sample_rate: u32
}

impl Wavetab {
    pub fn new(wave: Vec<i16>, sample_rate: u32) -> Wavetab {
        Wavetab { wave, sample_rate }
    }

    pub fn from_file(file_path: &str) -> Wavetab {
        let mut reader = match hound::WavReader::open(file_path) {
            Ok(r) => r,
            Err(_) => panic!("There was an error reading the file")
        };
        let wav_spec = reader.spec();
        let samples: Vec<i16> = reader.samples::<i16>().map(|sample| {
            match sample {
                Ok(s) => s as i16,
                Err(_) => panic!("Broken sample")
            }
        }).collect();

        Wavetab {
            wave: samples,
            sample_rate: wav_spec.sample_rate
        }
    }

    pub fn wave(&self) -> &Vec<i16> {
        &self.wave
    }

    pub fn convert_wave(&self, method: AnalysisMethod) -> String {
        match method {
            AnalysisMethod::Dsp => analysis::analyse_dsp(&self.wave, self.sample_rate),
            AnalysisMethod::Genetic => analysis::analyse_genetic(&self.wave, self.sample_rate),
            AnalysisMethod::Mixed => analysis::analyse_mixed(&self.wave, self.sample_rate)
        }; // should return some results wrapped in a struct        
        tab_generation::generate_tab();
        String::from("")
    }
}

// // run with cargo test
// #[cfg(test)]
// mod tests {
//     // access modules from parent module (wavetab lib)
//     // super::wave

//     #[test]
//     fn it_works() {
//         // wave::...
//         assert_eq!(2 + 2, 4);
//     }
// }

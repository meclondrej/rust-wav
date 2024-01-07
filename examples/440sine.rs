use rust_wav;

fn map(x: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    return (x - from_min) / (from_max - from_min) * (to_max - to_min) + to_min;
}

const SAMPLE_RATE: u32 = 44100u32;
const BYTES_PER_CHANNEL_SAMPLE: u16 = 2u16;
const DURATION: u32 = 10;

const SAMPLES: u32 = SAMPLE_RATE * DURATION;

const SIZE: usize = SAMPLES as usize;
const FREQUENCY: f32 = 440f32;
const FILENAME: &str = "output.wav";

fn main() {
    let mut data: Vec<i16> = Vec::with_capacity(SIZE);
    for i in 0usize..SIZE {
        let sample_time: f32 = (i as f32) / (SAMPLE_RATE as f32); // t = f^-1 => t = i/f
        let value: f32 = (sample_time * 2f32 * std::f32::consts::PI * FREQUENCY).sin(); // f(t) = sin(t2PIq)
        data.push(map(value, -1f32, 1f32, -32768f32, 32767f32) as i16); // maps to an i16 and pushes
    }
    let header: rust_wav::WavH = rust_wav::WavH {
        audio_size_bytes: (SIZE as u32) * (BYTES_PER_CHANNEL_SAMPLE as u32),
        audio_channels: 1,
        sample_rate: SAMPLE_RATE,
        bytes_per_channel_sample: BYTES_PER_CHANNEL_SAMPLE,
    };
    let wav: Vec<u8> = rust_wav::make_wav_bytes(&header, &data);
    let mut file: std::fs::File = std::fs::File::create(FILENAME)
        .expect("could not create file");
    std::io::Write::write_all(&mut file, &wav)
        .expect("could not write into file");
}

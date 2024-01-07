use rust_wav;

/*

Example 440sine

The goal of this example is to
create a wav file with a 440Hz
sine wave for the duration of
10 seconds.

*/

// This function can map values from one range to another.
// It will help us converting values between -1 to 1,
// which will be the output of the sine function,
// to an integer like i16.
fn map(x: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    return (x - from_min) / (from_max - from_min) * (to_max - to_min) + to_min;
}

// Here we declare our constants

// The sample rate, also know as sampling frequency,
// determines how many samples will be stored in
// the file for a single second.
// (hence the Hz unit)
const SAMPLE_RATE: u32 = 44100u32;

// Bytes per channel per sample determines the
// size of a single sample for one channel.
// Here, we will be storing the samples
// in 16bit signed integers, which
// have the size of 2 bytes.
const BYTES_PER_CHANNEL_SAMPLE: u16 = 2u16;

// This constant will determine the
// duration of our sound in seconds.
const DURATION: u32 = 10;

// Here, we calculate how many samples we
// will actually need. We can achieve this by
// simply multiplying the sample rate (how
// many samples per one second) and the
// duration (how many seconds).
const SAMPLES: u32 = SAMPLE_RATE * DURATION;

// This constant will determine the size
// of our sample vector, which is simply
// the number of samples represented
// as usize.
const SIZE: usize = SAMPLES as usize;

// Here we set the frequency of our
// sine wave.
const FREQUENCY: f32 = 440f32;

// And finally, here we set what file
// should we write to.
const FILENAME: &str = "output.wav";

fn main() {
    // We start with initializing our vector.
    let mut data: Vec<i16> = Vec::with_capacity(SIZE);

    // Then, we proceed onto our main loop,
    // in which we will calculate the sample
    // values in the vector
    for i in 0usize..SIZE {
        // Here, we calculate the sample's timestamp.
        // We can use the following equations:
        // t = f^-1
        // t = 1/f
        // t = 1/f * i
        // t = i/f
        let sample_time: f32 = (i as f32) / (SAMPLE_RATE as f32);
        
        // Assuming the known time (t), and a known
        // frequency (q), we can calculate the sample
        // value:
        // f(t) = sin(t2PIq)
        let value: f32 = (sample_time * 2f32 * std::f32::consts::PI * FREQUENCY).sin();

        // However, the output of the sine function
        // is a float ranging from -1 to 1. So,
        // we will map it to a previously
        // mentioned i16 by using it's
        // lowest and highest value.
        data.push(map(value, -1f32, 1f32, -32768f32, 32767f32) as i16);
    }

    // We continue by entering the data
    // necessary to assemble the header.
    let header: rust_wav::WavH = rust_wav::WavH {
        sample_count: SAMPLES,
        audio_channels: 1,
        sample_rate: SAMPLE_RATE,
        bytes_per_channel_sample: BYTES_PER_CHANNEL_SAMPLE,
    };

    // And here we finally assemble
    // and write the wav onto a file.
    let wav: Vec<u8> = rust_wav::make_wav_bytes(&header, &data);
    let mut file: std::fs::File = std::fs::File::create(FILENAME)
        .expect("could not create file");
    std::io::Write::write_all(&mut file, &wav)
        .expect("could not write into file");
}

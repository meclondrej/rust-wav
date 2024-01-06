fn array_to_bytes<T>(arr: &[T]) -> Vec<u8> {
    let element_length = std::mem::size_of::<T>();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            arr.as_ptr() as *const u8,
            arr.len() * element_length
        )
    };
    return bytes.to_vec();
}

pub struct WavH {
    audio_size_bytes: u32,
    audio_channels: u16,
    sample_rate: u32,
    bytes_per_channel_sample: u16,
}

impl WavH {
    pub fn to_bytestring(&self) -> [u8; 44] {
        let mut header_bytes: [u8; 44] = [0u8; 44];

        let bytes_per_second: u32 = self.sample_rate
            * (self.bytes_per_channel_sample as u32)
            * (self.audio_channels as u32);
        let bytes_per_sample: u16 = self.bytes_per_channel_sample * self.audio_channels;
        let bits_per_channel_sample: u16 = self.bytes_per_channel_sample * 8u16;
        let file_size_bytes_after_entry_2: u32 = 36u32 + self.audio_size_bytes;

        // RIFF chunk, totals 12 bytes
        header_bytes[0..=3].copy_from_slice(b"RIFF"); // 4 bytes "RIFF" ascii
        header_bytes[4..=7].copy_from_slice(&(file_size_bytes_after_entry_2.to_le_bytes())); // 4 bytes file size u32
        header_bytes[8..=11].copy_from_slice(b"WAVE"); // 4 bytes "WAVE" ascii

        // format chunk, totals 24 bytes
        header_bytes[12..=15].copy_from_slice(b"fmt "); // 4 bytes "fmt " ascii
        header_bytes[16..=19].copy_from_slice(&(16u32.to_le_bytes())); // 4 bytes chunk remaining size u32
        header_bytes[20..=21].copy_from_slice(&(1u16.to_le_bytes())); // 2 bytes audio format type u16
        header_bytes[22..=23].copy_from_slice(&(self.audio_channels.to_le_bytes())); // 2 bytes audio channels u16
        header_bytes[24..=27].copy_from_slice(&(self.sample_rate.to_le_bytes())); // 4 bytes sample rate u32
        header_bytes[28..=31].copy_from_slice(&(bytes_per_second.to_le_bytes())); // 4 bytes bytes per second u32
        header_bytes[32..=33].copy_from_slice(&(bytes_per_sample.to_le_bytes())); // 2 bytes bytes per sample u16
        header_bytes[34..=35].copy_from_slice(&(bits_per_channel_sample.to_le_bytes())); // 2 bytes bits per channel sample u16

        // data chunk, totals 8 bytes excluding following data
        header_bytes[36..=39].copy_from_slice(b"data"); // 4 bytes "data" ascii
        header_bytes[40..=43].copy_from_slice(&(self.audio_size_bytes.to_le_bytes())); // 4 bytes audio bytes u32

        return header_bytes;
    }
}

pub fn make_wav_bytes<T>(header: &WavH, audio: Vec<T>) -> Vec<u8> {
    let mut wav: Vec<u8> = vec![];
    wav.extend(&(header.to_bytestring()));
    wav.extend(array_to_bytes::<T>(&audio));
    return wav;
}
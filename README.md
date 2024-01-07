# rust-wav

An oversimplified library for creating wav file headers in rust.

## Usage

### `WavH`

There is a structure called WavH, containing the necessary fields for the assembly of the header.

You can use it like so:

```rust
let header: rust_wav::WavH = rust_wav::WavH {
    sample_count: /* the number of samples you have */,
    audio_channels: /* the number of channels you use */,
    sample_rate: /* your sample rate */,
    bytes_per_channel_sample: /* the size of a single sample for one channel*/,
};
```

### `WavH::to_bytestring(&self) -> [u8; 44]`

You can directly convert the header into a bytestring:

```rust
let header_bytestring: [u8, 44] = header.to_bytestring();
```

However, if you want to assemble the whole file with the header, `make_wav_bytes` might be more suitable (more information below).

### `make_wav_bytes<T>(header: &WavH, audio: &Vec<T>) -> Vec<u8>`

You can use this function to assemble the whole wav file as follows:

```rust
// We will use u16 mono sampling, therefore the bytes_per_channel_sample filed in the header should be 2
let data: Vec<u16> = Vec::with_capacity(SAMPLES_COUNT as usize);

// <- here should be your code to calculate the samples

let wav: Vec<u8> = rust_wav::make_wav_bytes(&header, &data);
```

The `wav` is a u8 vector, which you can write directly into a file.

## Examples

For the usage examples, look in the `examples/` folder.
You can execute the examples with `cargo run --example EXAMPLENAME`.
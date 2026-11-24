# SDR File Reader Library

`sdr-iq-file-reader` is a Rust library for reading and parsing Software Defined Radio (SDR) files. It supports a wide range of sample formats, including unsigned and signed 8-bit and 16-bit integers, as well as 32-bit and 64-bit floating-point numbers. The library provides an easy interface for converting raw SDR data into complex samples that can be used for further processing.

## Features
- Supports multiple sample types: `u8`, `i8`, `i16`, `u16`, `f32`, and `f64`.
- Converts raw SDR data into complex samples (`Complex<f32>` or `Complex<f64>`).
- Efficient reading of SDR files in configurable chunks.

## Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
sdr-iq-file-reader = "1"
```

## Usage
Here is an example of how to use the `sdr-iq-file-reader` library to read SDR data from a file:

```rust
use sdr_iq_file_reader::{SampleType, SdrFileReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "gqrx_20240929_015218_580206500_2400000_fc.raw";
    let mut reader = SdrFileReader::builder()
        .file_path(file_path)
        .samples_per_chunk(1024)
        .sample_type(SampleType::F32)
        .build()?;

    while let Some(samples) = reader.read_next_chunk_complexf32()? {
        // Process the samples here
        println!("Read {} samples", samples.len());
    }

    Ok(())
}
```

## Sample Types
The `SampleType` enum represents the different formats of samples that can be used in SDR files:
- `U8`: Unsigned 8-bit integer
- `I8`: Signed 8-bit integer
- `I16`: Signed 16-bit integer
- `U16`: Unsigned 16-bit integer
- `F32`: 32-bit floating point
- `F64`: 64-bit floating point

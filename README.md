# SDR File Reader Library

`sdr_file_reader` is a Rust library for reading and parsing Software Defined Radio (SDR) files. It supports a wide range of sample formats, including unsigned and signed 8-bit and 16-bit integers, as well as 32-bit and 64-bit floating-point numbers. The library provides an easy interface for converting raw SDR data into complex samples that can be used for further processing.

## Features
- Supports multiple sample types: `u8`, `i8`, `i16`, `u16`, `f32`, and `f64`.
- Converts raw SDR data into complex samples (`Complex<f32>` or `Complex<f64>`).
- Efficient reading of SDR files in configurable chunks.

## Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
sdr_file_reader = "0.1.0"
```

## Usage
Here is an example of how to use the `sdr_file_reader` library to read SDR data from a file:

```rust
use sdr_file_reader::{SdrFileReader, SampleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "gqrx_20240929_015218_580206500_2400000_fc.raw";
    let mut reader = SdrFileReader::new(file_path, 1024, SampleType::F32)?;

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

## Methods
### `SdrFileReader::new`
Creates a new `SdrFileReader` instance.

**Arguments**:
- `file_path`: The path to the SDR file to be read.
- `samples_per_chunk`: The number of samples to read per chunk.
- `sample_type`: The type of samples in the SDR file.

### `read_next_chunk_complexf32`
Reads the next chunk of samples as `Complex<f32>`.

### `read_next_chunk_complexf64`
Reads the next chunk of samples as `Complex<f64>`.

## Testing
The library includes tests that verify the functionality of reading both `Complex<f32>` and `Complex<f64>` data from SDR files. Additionally, it has a test to compare the output of both reading functions to ensure consistency across different data types.

To run the tests, use:

```sh
cargo test
```

## License
This project is licensed under the MIT License.


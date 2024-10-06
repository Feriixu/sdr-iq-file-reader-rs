//! # SDR IQ File Reader
//!
//! This library provides a simple way to read samples from an SDR IQ file through the [`SdrFileReader`] struct.
//! See the [`SdrFileReader`] documentation for more information on how to use it.

use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::Path;
use bon::{bon};
use num_complex::{Complex};

/// Create a new SdrFileReader using the builder pattern.
/// Then call `read_next_chunk_complexf32` or `read_next_chunk_complexf64` to read the samples.
///
/// # Example
/// ```
/// use sdr_iq_file_reader::{SdrFileReader, SampleType};
/// let mut reader = SdrFileReader::builder()
///     .file_path("gqrx_20240929_015218_580206500_2400000_fc.raw")
///     .samples_per_chunk(1024)
///     .sample_type(SampleType::F32)
///     .build()
///     .expect("Failed to create SdrFileReader");
/// let samples = reader.read_next_chunk_complexf32().unwrap();
/// ```
pub struct SdrFileReader {
    reader: BufReader<File>,
    samples_per_chunk: usize,
    sample_type: SampleType,
}

/// The type of samples in the SDR file
/// You will have to look up what your SDR/software uses.
pub enum SampleType {
    /// Samples stored as unsigned 8-bit integers
    U8,
    /// Samples stored as signed 8-bit integers
    I8,
    /// Samples stored as unsigned 16-bit integers
    U16,
    /// Samples stored as signed 16-bit integers
    I16,
    /// Samples stored as 32-bit floating point numbers
    F32,
    /// Samples stored as 64-bit floating point numbers
    F64,
}

impl SampleType {
    /// The number of bytes per sample.
    /// One sample has 2 values: I and Q.
    /// Therefore the total number of bytes per sample twice the length of the datatype.
    pub fn sample_len(&self) -> usize {
        match self {
            SampleType::U8 => 2,
            SampleType::I8 => 2,
            SampleType::I16 => 4,
            SampleType::U16 => 4,
            SampleType::F32 => 8,
            SampleType::F64 => 16,
        }
    }
}


#[bon]
impl SdrFileReader {
    #[allow(missing_docs)]
    #[builder]
    pub fn new(file_path: impl AsRef<Path>, samples_per_chunk: usize, sample_type: SampleType) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        Ok(SdrFileReader {
            reader,
            samples_per_chunk,
            sample_type,
        })
    }

    /// Read the next chunk of samples as Complex<f32> from the file.
    ///
    /// # Returns
    /// - `Ok(Some(samples))` if there are samples in the chunk
    /// - `Ok(None)` if the end of the file is reached
    /// - `Err(why)` if there was an error reading the file other than reaching the end
    pub fn read_next_chunk_complexf32(&mut self) -> Result<Option<Vec<Complex<f32>>>, std::io::Error> {
        let mut buffer = vec![0u8; self.samples_per_chunk * self.sample_type.sample_len()]; // 2 for I and Q
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                let mut samples = Vec::with_capacity(self.samples_per_chunk);
                match self.sample_type {
                    SampleType::U8 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(s[0] as f32, s[1] as f32))),
                    SampleType::I8 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(i8::from_ne_bytes([s[0]]) as f32, i8::from_ne_bytes([s[1]]) as f32))),
                    SampleType::U16 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(u16::from_ne_bytes([s[0], s[1]]) as f32, u16::from_ne_bytes([s[2], s[3]]) as f32))),
                    SampleType::I16 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(i16::from_ne_bytes([s[0], s[1]]) as f32, i16::from_ne_bytes([s[2], s[3]]) as f32))),
                    SampleType::F32 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(f32::from_ne_bytes([s[0], s[1], s[2], s[3]]), f32::from_ne_bytes([s[4], s[5], s[6], s[7]])))),
                    SampleType::F64 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(f64::from_ne_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]) as f32, f64::from_ne_bytes([s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]]) as f32))),
                }
                Ok(Some(samples))
            }
            Err(why) => match why.kind() {
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(why),
            },
        }
    }

    /// Read the next chunk of samples as Complex<f64> from the file.
    ///
    /// # Returns
    /// - `Ok(Some(samples))` if there are samples in the chunk
    /// - `Ok(None)` if the end of the file is reached
    /// - `Err(why)` if there was an error reading the file other than reaching the end
    pub fn read_next_chunk_complexf64(&mut self) -> Result<Option<Vec<Complex<f64>>>, std::io::Error> {
        let mut buffer = vec![0u8; self.samples_per_chunk * self.sample_type.sample_len()]; // 2 for I and Q
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                let mut samples = Vec::with_capacity(self.samples_per_chunk);
                match self.sample_type {
                    SampleType::U8 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(s[0] as f64, s[1] as f64))),
                    SampleType::I8 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(i8::from_ne_bytes([s[0]]) as f64, i8::from_ne_bytes([s[1]]) as f64))),
                    SampleType::U16 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(u16::from_ne_bytes([s[0], s[1]]) as f64, u16::from_ne_bytes([s[2], s[3]]) as f64))),
                    SampleType::I16 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(i16::from_ne_bytes([s[0], s[1]]) as f64, i16::from_ne_bytes([s[2], s[3]]) as f64))),
                    SampleType::F32 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(f32::from_ne_bytes([s[0], s[1], s[2], s[3]]) as f64, f32::from_ne_bytes([s[4], s[5], s[6], s[7]]) as f64))),
                    SampleType::F64 => buffer.chunks_exact(self.sample_type.sample_len())
                        .for_each(|s| samples.push(Complex::new(f64::from_ne_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]), f64::from_ne_bytes([s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]])))),
                }
                Ok(Some(samples))
            }
            Err(why) => match why.kind() {
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(why),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdr_file_reader_f32() {
        let file_path = "gqrx_20240929_015218_580206500_2400000_fc.raw";
        let mut reader = SdrFileReader::builder()
            .file_path(file_path)
            .samples_per_chunk(1024)
            .sample_type(SampleType::F32)
            .build()
            .expect("Failed to create SdrFileReader");
        while let Some(samples) = reader.read_next_chunk_complexf32().unwrap() {
            assert_eq!(samples.len(), 1024);
            // Further assertions can be added based on expected values
        }
    }

    #[test]
    fn test_sdr_file_reader_f64() {
        let file_path = "gqrx_20240929_015218_580206500_2400000_fc.raw";
        let mut reader = SdrFileReader::builder()
            .file_path(file_path)
            .samples_per_chunk(1024)
            .sample_type(SampleType::F32)
            .build()
            .expect("Failed to create SdrFileReader");
        while let Some(samples) = reader.read_next_chunk_complexf64().unwrap() {
            assert_eq!(samples.len(), 1024);
            // Further assertions can be added based on expected values
        }
    }

    #[test]
    fn test_compare_f32_f64_readings() {
        let file_path = "gqrx_20240929_015218_580206500_2400000_fc.raw";
        let mut reader_f32 = SdrFileReader::builder()
            .file_path(file_path)
            .samples_per_chunk(1024)
            .sample_type(SampleType::F32)
            .build()
            .expect("Failed to create SdrFileReader for f32");

        let mut reader_f64 = SdrFileReader::builder()
            .file_path(file_path)
            .samples_per_chunk(1024)
            .sample_type(SampleType::F32)
            .build()
            .expect("Failed to create SdrFileReader for f64");

        while let (Some(samples_f32), Some(samples_f64)) = (
            reader_f32.read_next_chunk_complexf32().unwrap(),
            reader_f64.read_next_chunk_complexf64().unwrap(),
        ) {
            assert_eq!(samples_f32.len(), samples_f64.len());
            for (s32, s64) in samples_f32.iter().zip(samples_f64.iter()) {
                // Compare with epsilon
                assert!((s32.re - s64.re as f32).abs() < f32::EPSILON);
                assert!((s32.im - s64.im as f32).abs() < f32::EPSILON);

            }
        }
    }
}
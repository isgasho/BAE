//! # Sample Format
//! 
//! Module containing different output formats like stereo, 2.1, 5.1, 7.1, etc.

use super::*;

pub trait Panner<SF,G>
	where SF: Panner<SF,G>,
	      G: Sized
{
	type Output;
	fn to_sample_format(s: SampleT, g: G) -> Self::Output;
}

/// Converts a raw bytes to a Sample
/// It is assumed the bytes are 8-bit unsigned audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_u8(v:[u8;1]) -> SampleT {
	(v[0] as SampleT - 128.0) / 128.0
}

/// Converts raw bytes to a Sample
/// It is assumed that the bytes are 16-bit signed audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_i16(v:[u8;2]) -> SampleT {
	(i16::from_le_bytes(v) as SampleT) / (32768_f32)
}

/// Converts raw bytes to a Sample
/// It is assumed that the bytes are 24-bit signed audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_i24(v:[u8;3]) -> SampleT {
	(i32::from_le_bytes([v[0],v[1],v[2],0]) as SampleT) / (32768_f32)
}

/// Converts a decibel value to a linear gain value.
/// This assumes that 0dB is unity gain, and ~-6bB is 0.5 gain
pub fn db_linear(db:MathT) -> MathT {
	10.0_f64.powf(db/20.0)
}

/// Converts a linear gain value to a decibel value.
/// This assumes that 0dB is unity gain, and ~-6bB is 0.5 gain
pub fn linear_db(g:MathT) -> MathT {
	20.0 * g.log10()
}

pub mod stereodata;

pub use stereodata::*;

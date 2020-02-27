//! # Stereo Data
//! 
//! Module containing type for handling stereo audio data.

use super::*;

/// Struct representing a stereophonic audio sample.
#[derive(Copy,Clone,Default)]
pub struct StereoData{
	/// Left sample value.
	pub left:SampleT,
	/// Right sample value.
	pub right:SampleT,
}

impl StereoData {
	/// Returns a new StereoData object created from individual left and right
	/// audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `l` - the left audio sample.
	/// * `r` - the right audio sapmle.
	pub fn from_stereo(l:SampleT, r:SampleT) -> StereoData {
		StereoData{
			left:l,
			right:r
		}
	}

	/// Returns a new StereoData object where both left and right channels are
	/// copied from the passed sample value.
	pub fn single_stereo(x:SampleT) -> StereoData {
		StereoData{
			left:x,
			right:x
		}
	}

	/// Creates a new StereoData object from a single monophonic sample. This
	/// function reduces the power of the given sample by half to reflect human
	/// hearing.
	/// 
	/// # Parameters
	/// 
	/// * `x` - the input sample.
	pub fn from_mono(x:SampleT) -> StereoData {
		StereoData{
			left: SampleT::sqrt(0.5)*x,
			right: SampleT::sqrt(0.5)*x
		}
	}

	/// Converts the given stereophonic sample to a monophonic sample by summing
	/// the left and right samples and dividing by half power to get the full
	/// power monophonic sample.
	pub fn as_mono(self) -> SampleT {
		(self.left + self.right)/SampleT::sqrt(0.5)
	}
}

impl Panner<StereoData, f32> for StereoData {
	fn to_sample_format(s: SampleT, g: f32) -> StereoData {
		let l_lerp = lerp(g, -1.0, 1.0, 0.0, -120.0);
		let r_lerp = lerp(g, -1.0, 1.0, -120.0, 0.0);
		
		StereoData {
			left: db_linear(l_lerp as MathT) as SampleT * s,
			right: db_linear(r_lerp as MathT) as SampleT * s
		}
	}
}

impl std::ops::Neg for StereoData {
	type Output = Self;

	fn neg(self) -> Self::Output {
		StereoData {
			left: -self.left,
			right: -self.right,
		}
	}
}

impl std::ops::Add<StereoData> for StereoData {
	type Output = Self;

	fn add(self, rhs: StereoData) -> Self::Output {
		StereoData {
			left: self.left + rhs.left,
			right: self.right + rhs.right,
		}
	}
}

impl std::ops::AddAssign<StereoData> for StereoData {
	fn add_assign(&mut self, rhs: StereoData) {
		self.left += rhs.left;
		self.right += rhs.right;
	}
}

impl std::ops::Sub<StereoData> for StereoData {
	type Output = Self;

	fn sub(self, rhs: StereoData) -> Self {
		StereoData {
			left: self.left - rhs.left,
			right: self.right - rhs.right,
		}
	}
}

impl std::ops::SubAssign<StereoData> for StereoData {
	fn sub_assign(&mut self, rhs: StereoData) {
		self.left -= rhs.left;
		self.right -= rhs.right;
	}
}

impl std::ops::Mul<StereoData> for StereoData {
	type Output = StereoData;

	fn mul(self, rhs: StereoData) -> Self::Output {
		StereoData {
			left: self.left * rhs.left,
			right: self.right * rhs.right
		}
	}
}

impl std::ops::MulAssign<StereoData> for StereoData {
	fn mul_assign(&mut self, rhs: StereoData) {
		self.left *= rhs.left;
		self.right *= rhs.right;
	}
}

impl std::ops::Mul<SampleT> for StereoData {
	type Output = StereoData;

	fn mul(self, rhs: SampleT) -> Self::Output {
		StereoData {
			left: self.left * rhs,
			right: self.right * rhs,
		}
	}
}
impl std::ops::Mul<StereoData> for SampleT {
	type Output = StereoData;

	fn mul(self, rhs: StereoData) -> Self::Output {
		StereoData {
			left: self * rhs.left,
			right: self * rhs.right,
		}
	}
}

impl std::ops::MulAssign<SampleT> for StereoData {
	fn mul_assign(&mut self, rhs: SampleT) {
		self.left *= rhs;
		self.right *= rhs;
	}
}

impl std::ops::Mul<MathT> for StereoData {
	/// Output type of the multiplication
	type Output = StereoData;

	/// Multiplies a sample by a value. E.g. scaling the sample by a gain amount.
	fn mul(self, rhs: MathT) -> Self::Output {
		StereoData {
			left:(self.left as MathT * rhs) as SampleT,
			right:(self.right as MathT * rhs) as SampleT,
		}
	}
}
impl std::ops::Mul<StereoData> for MathT {
	type Output = StereoData;

	fn mul(self, rhs: StereoData) -> Self::Output {
		StereoData {
			left: self as SampleT * rhs.left,
			right: self as SampleT * rhs.right
		}
	}
}

impl std::ops::MulAssign<MathT> for StereoData {
	fn mul_assign(&mut self, rhs: MathT) {
		self.left *= rhs as SampleT;
		self.right *= rhs as SampleT;
	}
}

impl Into<Vec<u8>> for StereoData {
	/// Converts the StereoData into a vector of bytes.
	fn into(self) -> Vec<u8> {
		let mut v = Vec::new();

			// Converts the left sample from SampleT (f32) to i16, then to bytes
		let n = ((self.left * 32768_f32) as i16).to_le_bytes();
		v.push(n[0]);
		v.push(n[1]);

			// Converts the right sample from SampleT (f32) to i16, then to bytes
		let n = ((self.right * 32768_f32) as i16).to_le_bytes();
		v.push(n[0]);
		v.push(n[1]);

		v
	}
}

impl From<SampleT> for StereoData {
	/// Copies the given sample to the left and right channels. If you want to
	/// use the half-power conversion, use [`StereoData::from_mono`].
	/// 
	/// [`StereoData::from_mono`]: struct.StereoData.html#method.from_mono
	fn from(s: SampleT) -> Self {
		StereoData{
			left: s,
			right: s,
		}
	}
}

impl From<[u8;2]> for StereoData {
	/// Converts the array of 2 bytes into a StereoData object.
	/// It is assumed that the bytes are 8-bit unsigned audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;2]) -> Self {
		StereoData {
			left: sample_from_u8([v[0]]),
			right: sample_from_u8([v[1]])
		}
	}
}

impl From<[u8;4]> for StereoData {
	/// Converts the array of 4 bytes into a StereoData object.
	/// It is assumed that the bytes are 16-bit signed audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;4]) -> Self {
		StereoData {
			left: sample_from_i16([v[0],v[1]]),
			right: sample_from_i16([v[2],v[3]])
		}
	}
}

impl From<[u8;6]> for StereoData {
	/// Converts the array of 6 bytes into a StereoData object.
	/// It is assumed that the bytes are 24-bit signed audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;6]) -> Self {
		StereoData {
			left:  sample_from_i24([v[0],v[1],v[2]]),
			right: sample_from_i24([v[3],v[4],v[5]])
		}
	}
}

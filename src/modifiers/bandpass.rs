//! # Band Pass

use super::*;

pub struct BandPass {
	central_f: MathT,
	quality: MathT,
	a0: MathT,
	b1: MathT,
	b2: MathT,
	x1: StereoData,
	x2: StereoData,
	y1: StereoData,
	y2: StereoData,
}

impl BandPass {
	pub fn new(f: MathT, q: MathT) -> Self {
		let mut bp = BandPass {
			central_f: f,
			quality: q,
			a0: 0.0,
			b1: 0.0,
			b2: 0.0,
			x1: StereoData::default(),
			x2: StereoData::default(),
			y1: StereoData::default(),
			y2: StereoData::default(),
		};

		bp.reset();

		bp
	}

	pub fn from_corners(f: (MathT,MathT)) -> Self {
		let mut bp = BandPass {
			central_f: (f.0*f.1).abs().sqrt(),
			quality: (f.0*f.1).abs().sqrt()/(f.1-f.0).abs(),
			a0: 0.0,
			b1: 0.0,
			b2: 0.0,
			x1: StereoData::default(),
			x2: StereoData::default(),
			y1: StereoData::default(),
			y2: StereoData::default(),
		};

		bp.reset();

		bp
	}

	pub fn get_central_frequency(&self) -> MathT {
		self.central_f
	}

	pub fn set_central_frequency(&mut self, f: MathT) {
		self.central_f = f;

		self.reset();
	}

	pub fn get_quality(&self) -> MathT {
		self.quality
	}

	pub fn set_quality(&mut self, q: MathT) {
		self.quality = q;

		self.reset();
	}

	pub fn get_corner_frequencies(&self) -> (MathT,MathT) {
		let a = 1.0;
		let b = -self.central_f/self.quality;
		let c = -self.central_f*self.central_f;

		let (p,n) = quadratic(a,b,c);
		let fl = if p > 0.0 {
			p
		} else {
			n
		};
		let fh = fl + b;

		(fh,fl)
	}

	pub fn set_corner_frequencies(&mut self, f: (MathT,MathT)) {
		self.central_f = (f.0 * f.1).sqrt();
		self.quality = self.central_f/(f.0-f.1).abs();

		self.reset();
	}

	fn reset(&mut self) {
		let a = 1.0;
		let b = -self.central_f/self.quality;
		let c = -self.central_f*self.central_f;

		let (p,n) = quadratic(a,b,c);
		let fl = if p > 0.0 {
			p
		} else {
			n
		};
		let fh = fl + b;

		let theta_l = (std::f64::consts::PI * fl * INV_SAMPLE_RATE).tan();
		let theta_h = (std::f64::consts::PI * fh * INV_SAMPLE_RATE).tan();

		let al = 1.0 / (1.0+theta_l);
		let ah = 1.0 / (1.0+theta_h);

		let bl = (1.0-theta_l) / (1.0+theta_l);
		let bh = (1.0-theta_h) / (1.0+theta_h);

		self.a0 = (1.0-al) * ah;
		self.b1 = bl + bh;
		self.b2 = bl * bh;
	}
}

impl Modifier for BandPass {
	fn process(&mut self, x: StereoData) -> StereoData {
		let y = StereoData::from_stereo(
			(self.a0 * (x.left() - self.x2.left()) as MathT +
			self.b1 * self.y1.left() as MathT - 
			self.b2 * self.y2.left() as MathT) as SampleT,
			(self.a0 * (x.right() - self.x2.right()) as MathT +
			self.b1 * self.y1.right() as MathT - 
			self.b2 * self.y2.right() as MathT) as SampleT
		);

		self.y2 = self.y1;
		self.y1 = y;
		self.x2 = self.x1;
		self.x1 = x;

		y
	}
}

fn quadratic(a: MathT, b: MathT, c: MathT) -> (MathT,MathT) {
	(
		(-b + (b*b - 4.0*a*c).sqrt())/(2.0*a),
		(-b - (b*b - 4.0*a*c).sqrt())/(2.0*a)
	)
}

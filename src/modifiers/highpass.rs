//! # HighPass
//! 
//! 18dB/octave
//! Derived from 3rd Order Butterworth Low Pass Filter.

use super::*;

/// High pass filter adapted from the 3rd Order Butterworth Low Pass Filter with
/// resonance.
pub struct HighPass {
    a0: SampleT,
    a1: SampleT,
    a2: SampleT,
    a3: SampleT,
    b1: SampleT,
    b2: SampleT,
    b3: SampleT,

    x1: SampleT,
    x2: SampleT,
    x3: SampleT,
    y1: SampleT,
    y2: SampleT,
    y3: SampleT,

    fc: MathT,
    r: MathT
}

impl HighPass {
    /// Creates a new high pass from the given cutoff frequency and resonance
    /// values.
    /// 
    /// # Parameters
    /// 
    /// * `fc` - The cutoff frequency.
    /// * `r` - The resonance of the filter. Value should be in the range [0,1].
    /// If the value falls out of that range it is clamped to the closer value.
    pub fn new(fc: MathT, r: MathT) -> HighPass {
        let fc = fc.min(SAMPLE_RATE as MathT / 2.0);
        let r = r.min(1.0).max(0.0);
        let mut hp = HighPass {
            a0: SampleT::default(),
            a1: SampleT::default(),
            a2: SampleT::default(),
            a3: SampleT::default(),
            b1: SampleT::default(),
            b2: SampleT::default(),
            b3: SampleT::default(),
            x1: SampleT::default(),
            x2: SampleT::default(),
            x3: SampleT::default(),
            y1: SampleT::default(),
            y2: SampleT::default(),
            y3: SampleT::default(),
            fc,
            r,
        };

        hp.reset();

        hp
    }

    /// Returns the central frequency of the filter.
    pub fn get_central_frequency(&self) -> MathT {
        self.fc
    }

    /// Sets the central frequency of the filter.
    pub fn set_central_frequency(&mut self, fc: MathT) {
        let fc = fc.min(SAMPLE_RATE as MathT / 2.0);

        self.fc = fc;
        self.reset();
    }

    /// Returns the resonance of the filter.
    pub fn get_resonance(&self) -> MathT {
        self.r
    }

    /// Sets the resonance of the filter.
    pub fn set_resonance(&mut self, r: MathT) {
        let r = r.min(1.0).max(0.0);

        self.r = r;
        self.reset();
    }

    fn reset(&mut self) {
        let theta = std::f64::consts::PI * (4.0 - self.r) / 6.0;
        let k = 1.0 - 2.0 * theta.cos();
        let w = 2.0 * std::f64::consts::PI * self.fc;
        let t = w * INV_SAMPLE_RATE;
        let g = t.powf(3.0) + k*t.powf(2.0) + k*t + 1.0;

        self.a0 = ( 1.0/g) as SampleT;
        self.a1 = (-3.0/g) as SampleT;
        self.a2 = ( 3.0/g) as SampleT;
        self.a3 = (-1.0/g) as SampleT;

        self.b1 = ((k*t.powf(2.0) + 2.0*k*t + 3.0)/g) as SampleT;
        self.b2 = ((-k*t - 3.0)/g) as SampleT;
        self.b3 = (1.0/g) as SampleT;
    }
}

impl Modifier for HighPass {
    fn process(&mut self, x: SampleT) -> SampleT {
        let y = self.a0*x + self.a1*self.x1 + self.a2*self.x2 + self.a3*self.x3 +
            self.b1*self.y1 + self.b2*self.y2 + self.b3*self.y3;

        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y3 = self.y2;
        self.y2 = self.y1;
        self.y1 = y;

        y
    }
}

impl Clone for HighPass {
    fn clone(&self) -> Self {
        HighPass {
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            b1: self.b1,
            b2: self.b2,
            b3: self.b3,
            x1: SampleT::default(),
            x2: SampleT::default(),
            x3: SampleT::default(),
            y1: SampleT::default(),
            y2: SampleT::default(),
            y3: SampleT::default(),
            fc: self.fc,
            r: self.r,
        }
    }
}

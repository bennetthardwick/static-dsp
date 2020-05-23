use super::Node;
use core::f32::consts::PI;
use libm;

// Stolen from here:
// http://www.earlevel.com/main/2012/11/26/biquad-c-source-code/

#[derive(Copy, Clone)]
pub enum BiquadType {
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
    Peak,
    Lowshelf,
    Highshelf,
}

#[derive(Copy, Clone)]
pub struct BiquadParameters {
    pub cutoff: f32,
    pub q: f32,
    pub peak_gain: f32,
}

#[derive(Default)]
pub struct BiquadCoefficients {
    a0: f32,
    a1: f32,
    a2: f32,

    b1: f32,
    b2: f32,
}

impl BiquadCoefficients {
    fn update(
        &mut self,
        biquad_type: BiquadType,
        BiquadParameters {
            cutoff,
            q,
            peak_gain,
        }: BiquadParameters,
    ) {
        use BiquadType::*;

        let norm: f32;
        let v = libm::powf(10., libm::fabsf(peak_gain) / 20.0);
        let k = libm::tanf(PI * cutoff);

        match biquad_type {
            Lowpass => {
                norm = 1. / (1. + k / q + k * k);
                self.a0 = k * k * norm;
                self.a1 = 2. * self.a0;
                self.a2 = self.a0;
                self.b1 = 2. * (k * k - 1.) * norm;
                self.b2 = 1. - k / q + k * k;
            }
            Highpass => {
                norm = 1. / (1. + k / q + k * k);
                self.a0 = 1. * norm;
                self.a1 = -2. * self.a0;
                self.a2 = self.a0;
                self.b1 = 2. * (k * k - 1.) * norm;
                self.b2 = (1. - k / q + k * k) * norm;
            }
            Bandpass => {
                norm = 1. / (1. + k / q + k * k);
                self.a0 = k / q * norm;
                self.a1 = 0.;
                self.a2 = -self.a0;
                self.b1 = 2. * (k * k - 1.) * norm;
                self.b2 = (1. - k / q + k * k) * norm;
            }
            Notch => {
                norm = 1. / (1. + k / q + k * k);
                self.a0 = (1. + k * k) * norm;
                self.a1 = 2. * (k * k - 1.) * norm;
                self.a2 = self.a0;
                self.b1 = self.a1;
                self.b2 = (1. - k / q + k * k) * norm;
            }
            Peak => {
                if peak_gain >= 0. {
                    norm = 1. / (1. + 1. / q * k + k * k);
                    self.a1 = 2. * (k * k - 1.) * norm;
                    self.a0 = (1. + v / q * k + k * k) * norm;
                    self.a2 = (1. - v / q * k + k * k) * norm;
                    self.b1 = self.a1;
                    self.b2 = (1. - 1. / q * k + k * k) * norm;
                } else {
                    norm = 1. / (1. + v / q * k + k * k);
                    self.a1 = 2. * (k * k - 1.) * norm;
                    self.a0 = (1. + 1. / q * k + k * k) * norm;
                    self.a2 = (1. - 1. / q * k + k * k) * norm;
                    self.b1 = self.a1;
                    self.b2 = (1. - v / q * k + k * k) * norm;
                }
            }
            Lowshelf => {
                if peak_gain >= 0. {
                    norm = 1. / (1. + libm::sqrtf(2.) * k + k * k);
                    self.a0 = (1. + libm::sqrtf(2. * v) * k + v * k * k) * norm;
                    self.a1 = 2. * (v * k * k - 1.) * norm;
                    self.a2 = (1. - libm::sqrtf(2. * v) * k + v * k * k) * norm;
                    self.b1 = 2. * (k * k - 1.) * norm;
                    self.b2 = (1. - libm::sqrtf(2.) * k + k * k) * norm;
                } else {
                    norm = 1. / (1. + libm::sqrtf(2. * v) * k + v * k * k);
                    self.a0 = (1. + libm::sqrtf(2.) * k + k * k) * norm;
                    self.a1 = 2. * (k * k - 1.) * norm;
                    self.a2 = (1. - libm::sqrtf(2.) * k + k * k) * norm;
                    self.b1 = 2. * (v * k * k - 1.) * norm;
                    self.b2 = (1. - libm::sqrtf(2. * v) * k + v * k * k) * norm;
                }
            }
            Highshelf => {
                if peak_gain >= 0. {
                    norm = 1. / (1. + libm::sqrtf(2.) * k + k * k);
                    self.a0 = (v + libm::sqrtf(2. * v) * k + k * k) * norm;
                    self.a1 = 2. * (k * k - v) * norm;
                    self.a2 = (v - libm::sqrtf(2. * v) * k + k * k) * norm;
                    self.b1 = 2. * (k * k - 1.) * norm;
                    self.b2 = (1. - libm::sqrtf(2.) * k + k * k) * norm;
                } else {
                    norm = 1. / (v + libm::sqrtf(2. * v) * k + k * k);
                    self.a0 = (1. + libm::sqrtf(2.) * k + k * k) * norm;
                    self.a1 = 2. * (k * k - 1.) * norm;
                    self.a2 = (1. - libm::sqrtf(2.) * k + k * k) * norm;
                    self.b1 = 2. * (k * k - v) * norm;
                    self.b2 = (v - libm::sqrtf(2. * v) * k + k * k) * norm;
                }
            }
        }
    }
}

pub struct Biquad {
    parameters: BiquadParameters,
    biquad_type: BiquadType,
    coefficients: BiquadCoefficients,
    z1: f32,
    z2: f32,
}

impl Biquad {
    pub fn new(biquad_type: BiquadType, parameters: BiquadParameters) -> Self {
        let mut coefficients = BiquadCoefficients::default();
        coefficients.update(biquad_type, parameters);
        Self {
            parameters,
            biquad_type,
            coefficients,
            z1: 0.,
            z2: 0.,
        }
    }

    pub fn set_type(&mut self, biquad_type: BiquadType) {
        self.biquad_type = biquad_type;
        self.coefficients.update(self.biquad_type, self.parameters);
    }

    pub fn set_params(&mut self, params: BiquadParameters) {
        self.parameters = params;
        self.coefficients.update(self.biquad_type, self.parameters);
    }

    pub fn set_biquad(&mut self, params: BiquadParameters, biquad_type: BiquadType) {
        self.parameters = params;
        self.set_type(biquad_type);
    }
}

impl Default for Biquad {
    fn default() -> Self {
        let biquad_type = BiquadType::Lowpass;
        let parameters = BiquadParameters {
            cutoff: 0.5,
            q: 0.707,
            peak_gain: 0.,
        };
        Self::new(biquad_type, parameters)
    }
}

impl Node<f32, f32> for Biquad {
    fn process(&mut self, input: f32) -> f32 {
        let BiquadCoefficients { a0, a1, a2, b1, b2 } = self.coefficients;
        let out = input * a0 + self.z1;
        self.z1 = input * a1 + self.z2 - b1 * out;
        self.z2 = input * a2 - b2 * out;
        out
    }
}

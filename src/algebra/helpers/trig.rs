use crate::algebra::field::Field;
use crate::algebra::helpers::exp::Exponentiation;

pub trait TrigOps: Field + Exponentiation + Sized {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self {
        self.try_tan().unwrap()
    }
    fn sin_cos(self) -> (Self, Self);

    fn try_tan(self) -> Option<Self>;

    fn asin(self) -> Self {
        self.try_asin().unwrap()
    }
    fn acos(self) -> Self {
        self.try_acos().unwrap()
    }
    fn atan(self) -> Self {
        self.try_atan().unwrap()
    }
    fn atan2(self, rhs: Self) -> Self {
        self.try_atan2(rhs).unwrap()
    }

    fn try_asin(self) -> Option<Self>;
    fn try_acos(self) -> Option<Self>;
    fn try_atan(self) -> Option<Self>;
    fn try_atan2(self, rhs: Self) -> Option<Self>;

    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    fn coth(self) -> Self {
        self.try_coth().unwrap()
    }
    fn csch(self) -> Self {
        self.try_csch().unwrap()
    }

    fn try_coth(self) -> Option<Self>;
    fn try_csch(self) -> Option<Self>;

    fn asinh(self) -> Self;
    fn acosh(self) -> Self {
        self.try_acosh().unwrap()
    }
    fn atanh(self) -> Self {
        self.try_atanh().unwrap()
    }

    fn try_acosh(self) -> Option<Self>;
    fn try_atanh(self) -> Option<Self>;

    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
}

macro_rules! float_to_option {
    ($expr:expr) => {{
        let result = $expr;
        if result.is_infinite() || result.is_nan() {
            None
        } else {
            Some(result)
        }
    }};
}

macro_rules! impl_trig_float {
    ($($t:ty)*) => {
        $(
            impl TrigOps for $t {
                #[inline]
                fn try_tan(self) -> Option<Self> {
                    float_to_option!(self.tan())
                }
                #[inline]
                fn try_asin(self) -> Option<Self> {
                    float_to_option!(self.asin())
                }
                #[inline]
                fn try_acos(self) -> Option<Self> {
                    float_to_option!(self.acos())
                }
                #[inline]
                fn try_atan(self) -> Option<Self> {
                    float_to_option!(self.atan())
                }
                #[inline]
                fn try_atan2(self, rhs: Self) -> Option<Self> {
                    float_to_option!(self.atan2(rhs))
                }
                #[inline]
                fn try_acosh(self) -> Option<Self> {
                    float_to_option!(self.acosh())
                }
                #[inline]
                fn try_atanh(self) -> Option<Self> {
                    float_to_option!(self.atanh())
                }
                #[inline]
                fn try_coth(self) -> Option<Self> {
                    unimplemented!()
                }
                #[inline]
                fn try_csch(self) -> Option<Self> {
                    unimplemented!()
                }
                forward! {
                    fn sin(self) -> Self;
                    fn cos(self) -> Self;
                    fn tan(self) -> Self;
                    fn sin_cos(self) -> (Self, Self);

                    fn asin(self) -> Self;
                    fn acos(self) -> Self;
                    fn atan(self) -> Self;
                    fn atan2(self, rhs: Self) -> Self;

                    fn sinh(self) -> Self;
                    fn cosh(self) -> Self;
                    fn tanh(self) -> Self;

                    fn asinh(self) -> Self;
                    fn acosh(self) -> Self;
                    fn atanh(self) -> Self;

                    fn to_degrees(self) -> Self;
                    fn to_radians(self) -> Self;
                }
            }
        )*
    }
}

impl_trig_float!(f32 f64);

// Basic operations overrided for the Galois Field 256 (2**8)
// Constant-time multiply/exponentiation. Division uses Fermat's Little Theorem ($b^{-1} = b^{254}$) with exactly one branch on the divisor.

use core::iter::{Product, Sum};
use core::ops::{Add, Mul, Sub};

#[cfg(feature = "fuzzing")]
use arbitrary::Arbitrary;

#[cfg(feature = "zeroize_memory")]
use zeroize::Zeroize;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "fuzzing", derive(Arbitrary))]
#[cfg_attr(feature = "zeroize_memory", derive(Zeroize))]
#[cfg_attr(feature = "zeroize_memory", zeroize(drop))]
pub struct GF256(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FieldError {
    DivisionByZero,
}

impl GF256 {
    pub(crate) fn div(self, other: Self) -> Result<Self, FieldError> {
        if other.0 == 0 {
            return Err(FieldError::DivisionByZero);
        }

        let b = other.0;
        let b2 = mul_u8(b, b);
        let b4 = mul_u8(b2, b2);
        let b8 = mul_u8(b4, b4);
        let b16 = mul_u8(b8, b8);
        let b32 = mul_u8(b16, b16);
        let b64 = mul_u8(b32, b32);
        let b128 = mul_u8(b64, b64);

        let inv = mul_u8(
            mul_u8(mul_u8(b2, b4), mul_u8(b8, b16)),
            mul_u8(mul_u8(b32, b64), b128),
        );

        Ok(Self(mul_u8(self.0, inv)))
    }
}

#[allow(
    clippy::suspicious_arithmetic_impl,
    reason = "in characteristic two, addition and subtraction are both bitwise XOR, so XOR is the correct body"
)]
impl Add for GF256 {
    type Output = GF256;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

#[allow(
    clippy::suspicious_arithmetic_impl,
    reason = "in characteristic two, addition and subtraction are both bitwise XOR, so XOR is the correct body"
)]
impl Sub for GF256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

/// Poly: x^8 + x^4 + x^3 + x^2 + 1 (0x11D -> 0x1D)
#[inline]
fn mul_u8(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0;
    for _ in 0..8 {
        let mask = (b & 1).wrapping_neg();
        p ^= a & mask;
        b >>= 1;
        let carry = a >> 7;
        a <<= 1;
        a ^= carry.wrapping_neg() & 0x1d;
    }
    p
}

impl Mul for GF256 {
    type Output = Self;

    /// Constant-time peasant multiplication
    fn mul(self, other: Self) -> Self::Output {
        Self(mul_u8(self.0, other.0))
    }
}

impl Sum for GF256 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(0), |acc, x| acc + x)
    }
}

impl Product for GF256 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(1), |acc, x| acc * x)
    }
}

#[cfg(test)]
mod tests {
    use super::GF256;
    use alloc::vec;

    fn reference_mul(a: u8, b: u8) -> u8 {
        let mut product = 0u16;
        for bit in 0..8 {
            if (b >> bit) & 1 == 1 {
                product ^= u16::from(a) << bit;
            }
        }
        for bit in (8..=14).rev() {
            if product & (1 << bit) != 0 {
                product ^= 0x11d << (bit - 8);
            }
        }
        product as u8
    }

    fn reference_inv(b: u8) -> Option<u8> {
        if b == 0 {
            return None;
        }
        (1..=u8::MAX).find(|&c| reference_mul(b, c) == 1)
    }

    #[test]
    fn known_answer_products() {
        let anchors = [
            (0x02, 0x80, 0x1d),
            (0x02, 0x1d, 0x3a),
            (0x03, 0x03, 0x05),
            (0x57, 0x83, 0x31),
        ];
        for (a, b, expected) in anchors {
            assert_eq!((GF256(a) * GF256(b)).0, expected, "0x{a:02x} * 0x{b:02x}");
            assert_eq!(reference_mul(a, b), expected, "0x{a:02x} * 0x{b:02x}");
        }
    }

    #[test]
    fn known_answer_inverses() {
        let anchors = [(0x02, 0x8e), (0x03, 0xf4), (0x80, 0x1b)];
        for (element, expected) in anchors {
            assert_eq!(
                GF256(1).div(GF256(element)).unwrap().0,
                expected,
                "1 / 0x{element:02x}"
            );
            assert_eq!(
                reference_inv(element),
                Some(expected),
                "inverse of 0x{element:02x}"
            );
        }
    }

    #[test]
    fn add_works() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                assert_eq!(GF256(a) + GF256(b), GF256(a ^ b), "0x{a:02x} + 0x{b:02x}");
            }
        }
    }

    #[test]
    fn sub_works() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                assert_eq!(GF256(a) - GF256(b), GF256(a ^ b), "0x{a:02x} - 0x{b:02x}");
            }
        }
    }

    #[test]
    fn mul_works() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                assert_eq!(
                    GF256(a) * GF256(b),
                    GF256(reference_mul(a, b)),
                    "0x{a:02x} * 0x{b:02x}"
                );
            }
        }
    }

    #[test]
    fn div_works() {
        for divisor in 1u8..=255 {
            let inverse = reference_inv(divisor)
                .unwrap_or_else(|| panic!("inverse must exist for 0x{divisor:02x}"));
            for numerator in 0u8..=255 {
                assert_eq!(
                    GF256(numerator).div(GF256(divisor)).unwrap(),
                    GF256(reference_mul(numerator, inverse)),
                    "0x{numerator:02x} / 0x{divisor:02x}"
                );
            }
        }
    }

    #[test]
    fn div_zero_divisor_errors() {
        assert!(matches!(
            GF256(5).div(GF256(0)),
            Err(super::FieldError::DivisionByZero)
        ));
        assert!(matches!(
            GF256(0).div(GF256(0)),
            Err(super::FieldError::DivisionByZero)
        ));
    }

    #[test]
    fn div_zero_numerator_ok() {
        assert_eq!(GF256(0).div(GF256(1)).unwrap().0, 0);
        assert_eq!(GF256(0).div(GF256(42)).unwrap().0, 0);
    }

    #[test]
    fn div_roundtrip_mul_inv() {
        use core::ops::Mul;
        for b in 1u8..=255 {
            let gb = GF256(b);
            let inv_gb = gb.clone().div(gb.clone()).unwrap();
            assert_eq!(inv_gb.0, 1);
            for a in 0u8..=255 {
                let ga = GF256(a);
                let res = ga.clone().mul(gb.clone()).div(gb.clone()).unwrap();
                assert_eq!(res.0, ga.0);
            }
        }
    }

    #[test]
    fn sum_works() {
        let values = vec![GF256(0x53), GF256(0xCA), GF256(0)];
        assert_eq!(values.into_iter().sum::<GF256>().0, 0x99);
    }

    #[test]
    fn product_works() {
        let values = vec![GF256(1), GF256(1), GF256(4)];
        assert_eq!(values.into_iter().product::<GF256>().0, 4);
    }
}

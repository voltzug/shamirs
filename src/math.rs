// Polynomial generation, share evaluation, and Lagrange interpolation for Shamir secret sharing.

use alloc::vec::Vec;

use rand::distr::{Distribution, Uniform};

use super::field::GF256;
use super::share::Share;

// Finds the [root of the Lagrange polynomial](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach).
// The `shares` argument has the format produced by `get_evaluator`: each share's X coordinate is a key and its value vector holds one field value per secret byte.
// The inner loop pairs only shares whose X coordinates differ, so every denominator `x_j - x_i` is non-zero and the division cannot fail.
// A zero X coordinate is legal and contributes a zero numerator to the other basis terms.
pub fn interpolate(shares: &[Share]) -> Vec<u8> {
    (0..shares[0].y.len())
        .map(|s| {
            shares
                .iter()
                .map(|s_i| {
                    let weight = shares
                        .iter()
                        .filter(|s_j| s_j.x != s_i.x)
                        .try_fold(GF256(1), |acc, s_j| {
                            let numerator = s_j.x.clone();
                            let denominator = s_j.x.clone() - s_i.x.clone();
                            numerator
                                .div(denominator)
                                .map(|v| acc * v)
                        })
                        .expect("interpolate: non-zero denominator invariant violated (filtered X coordinates must differ)");
                    weight * s_i.y[s].clone()
                })
                .sum::<GF256>()
                .0
        })
        .collect()
}

// Builds `k` coefficients for `k >= 1`, with `s` as the constant term and the others drawn from `[0, 255]`.
// Coefficient degrees go from higher to lower in the returned vector order.
// With `k = 0` the result still has one coefficient, the secret.
pub fn random_polynomial<R: rand::Rng>(s: GF256, k: u8, rng: &mut R) -> Vec<GF256> {
    let k = k as usize;
    let mut poly = Vec::with_capacity(k);
    let between = Uniform::new_inclusive(0, 255).unwrap();

    for _ in 1..k {
        poly.push(GF256(between.sample(rng)));
    }
    poly.push(s);

    poly
}

// Returns an iterator over the evaluations of `polys` at consecutive X coordinates.
// Each item is a share with X coordinate `x` and one value `f_i(x)` per polynomial.
// Each polynomial corresponds to one byte chunk of the original secret.
// The iterator starts at `x = 1` and ends at `x = 255`, so it yields at most 255 shares.
pub fn get_evaluator(polys: Vec<Vec<GF256>>) -> impl Iterator<Item = Share> {
    (1..=u8::MAX).map(GF256).map(move |x| Share {
        x: x.clone(),
        y: polys
            .iter()
            .map(|p| {
                p.iter()
                    .fold(GF256(0), |acc, c| acc * x.clone() + c.clone())
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::{get_evaluator, interpolate, random_polynomial, Share, GF256};
    use alloc::{vec, vec::Vec};
    use rand_chacha::rand_core::SeedableRng;

    #[test]
    fn random_polynomial_works() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
        let poly = random_polynomial(GF256(1), 3, &mut rng);
        assert_eq!(poly.len(), 3);
        assert_eq!(poly[2], GF256(1));
    }

    #[test]
    fn evaluator_works() {
        let iter = get_evaluator(vec![vec![GF256(3), GF256(2), GF256(5)]]);
        let values: Vec<_> = iter.take(2).map(|s| (s.x.clone(), s.y.clone())).collect();
        assert_eq!(
            values,
            vec![(GF256(1), vec![GF256(4)]), (GF256(2), vec![GF256(13)])]
        );
    }

    #[test]
    fn interpolate_works() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
        let poly = random_polynomial(GF256(185), 10, &mut rng);
        let iter = get_evaluator(vec![poly]);
        let shares: Vec<Share> = iter.take(10).collect();
        let root = interpolate(&shares);
        assert_eq!(root, vec![185]);
    }

    #[test]
    fn interpolate_with_x_zero_share() {
        let shares = vec![
            Share {
                x: GF256(0),
                y: vec![GF256(7)],
            },
            Share {
                x: GF256(1),
                y: vec![GF256(5)],
            },
        ];
        let root = interpolate(&shares);
        assert_eq!(root, vec![7]);
    }
}

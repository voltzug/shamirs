//! Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate.
//!
//! Secrets are byte slices of any length, including the empty slice.
//! Each share holds one GF(2^8) field value per secret byte.
//! The threshold is the minimum number of distinct shares needed to recover a secret, and the dealer builds a polynomial of degree one less than the threshold for each byte.
//!
//! Field arithmetic is GF(2^8) with reduction polynomial `0x11D`.
//! Multiplication is branch-free shift-and-add and division is multiplication by the Fermat inverse, neither using lookup tables.
//! Division branches once on a zero divisor; the remaining arithmetic is branch-free.
//!
//! The dealer assigns X coordinates from 1 through 255 and therefore emits at most 255 shares.
//! Recovery accepts a share with X coordinate zero even though the dealer never emits one.
//! Recovery rejects share sets with differing value-vector lengths, repeated X coordinates, or fewer distinct shares than the threshold.
//!
//! Usage example (std):
//! ```
//! use shamirs::{ Sharks, Share };
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! # #[cfg(feature = "std")]
//! # {
//! let dealer = sharks.dealer(&[1, 2, 3, 4]);
//! // Get 10 shares
//! let shares: Vec<Share> = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(shares.as_slice()).unwrap();
//! assert_eq!(secret, vec![1, 2, 3, 4].into());
//! # }
//! ```
//!
//! Usage example (no std):
//! ```
//! use shamirs::{ Sharks, Share };
//! use rand_chacha::rand_core::SeedableRng;
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
//! let dealer = sharks.dealer_rng(&[1, 2, 3, 4], &mut rng);
//! // Get 10 shares
//! let shares: Vec<Share> = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(shares.as_slice()).unwrap();
//! assert_eq!(secret.as_slice(), &[1u8, 2, 3, 4]);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

mod field;
mod math;
mod share;

extern crate alloc;

use alloc::vec::Vec;
use hashbrown::HashSet;

use field::GF256;
pub use share::Share;

/// A Shamir threshold over GF(2^8).
///
/// The `u8` parameter is the threshold: the minimum number of distinct shares needed to recover a secret.
/// A threshold of `k` builds a polynomial of degree `k - 1` for each secret byte.
/// The dealer emits at most 255 shares, one per X coordinate from 1 through 255.
///
/// Usage example:
/// ```
/// # use shamirs::{ Sharks, Share };
/// // Set a minimum threshold of 10 shares
/// let sharks = Sharks(10);
/// // Obtain an iterator over the shares for secret [1, 2, 3, 4]
/// # #[cfg(feature = "std")]
/// # {
/// let dealer = sharks.dealer(&[1, 2, 3, 4]);
/// // Get 10 shares
/// let shares: Vec<Share> = dealer.take(10).collect();
/// // Recover the original secret!
/// let secret = sharks.recover(shares.as_slice()).unwrap();
/// assert_eq!(secret, vec![1, 2, 3, 4].into());
/// # }
/// ```
pub struct Sharks(pub u8);

impl Sharks {
    /// Deals `secret` using the supplied random number generator.
    ///
    /// This method is available without `std`; with `std`, see the `dealer` method for the thread-local generator.
    /// The iterator yields one share per X coordinate from 1 through 255, so it ends after 255 shares.
    /// Each share carries one value per byte of `secret`; an empty `secret` yields shares with empty value vectors.
    ///
    /// Example:
    /// ```
    /// # use shamirs::{ Sharks, Share };
    /// # use rand_chacha::rand_core::SeedableRng;
    /// # let sharks = Sharks(3);
    /// // Obtain an iterator over the shares for secret [1, 2]
    /// let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
    /// let dealer = sharks.dealer_rng(&[1, 2], &mut rng);
    /// // Get 3 shares
    /// let shares: Vec<Share> = dealer.take(3).collect();
    pub fn dealer_rng<R: rand::Rng>(
        &self,
        secret: &[u8],
        rng: &mut R,
    ) -> impl Iterator<Item = Share> {
        let mut polys = Vec::with_capacity(secret.len());

        for chunk in secret {
            polys.push(math::random_polynomial(GF256(*chunk), self.0, rng))
        }

        math::get_evaluator(polys)
    }

    /// Deals `secret` using the thread-local random number generator.
    ///
    /// The iterator yields one share per X coordinate from 1 through 255, so it ends after 255 shares.
    /// Each share carries one value per byte of `secret`; an empty `secret` yields shares with empty value vectors.
    ///
    /// Example:
    /// ```
    /// # use shamirs::{ Sharks, Share };
    /// # let sharks = Sharks(3);
    /// // Obtain an iterator over the shares for secret [1, 2]
    /// let dealer = sharks.dealer(&[1, 2]);
    /// // Get 3 shares
    /// let shares: Vec<Share> = dealer.take(3).collect();
    #[cfg(feature = "std")]
    pub fn dealer(&self, secret: &[u8]) -> impl Iterator<Item = Share> {
        let mut rng = rand::rngs::ThreadRng::default();
        self.dealer_rng(secret, &mut rng)
    }

    /// Recovers a secret from an iterable collection of shares.
    ///
    /// Recovery rejects shares whose value vectors differ in length, repeated X coordinates, and sets with fewer distinct shares than the threshold.
    /// On success it returns the recovered bytes, wrapped so memory is wiped on drop when `zeroize_memory` is enabled.
    /// An empty secret recovers as an empty slice.
    ///
    /// Example:
    /// ```
    /// # use shamirs::{ Sharks, Share };
    /// # use rand_chacha::rand_core::SeedableRng;
    /// # let sharks = Sharks(3);
    /// # let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
    /// # let mut shares: Vec<Share> = sharks.dealer_rng(&[1], &mut rng).take(3).collect();
    /// // Recover original secret from shares
    /// let mut secret = sharks.recover(&shares);
    /// // Secret correctly recovered
    /// assert!(secret.is_ok());
    /// // Remove shares for demonstration purposes
    /// shares.clear();
    /// secret = sharks.recover(&shares);
    /// // Not enough shares to recover secret
    /// assert!(secret.is_err());
    #[cfg(not(feature = "zeroize_memory"))]
    pub fn recover<'a, T>(&self, shares: T) -> Result<Vec<u8>, &str>
    where
        T: IntoIterator<Item = &'a Share>,
        T::IntoIter: Iterator<Item = &'a Share>,
    {
        let mut share_length: Option<usize> = None;
        let mut keys: HashSet<u8> = HashSet::new();
        let mut values: Vec<Share> = Vec::new();

        for share in shares.into_iter() {
            if share_length.is_none() {
                share_length = Some(share.y.len());
            }

            if Some(share.y.len()) != share_length {
                return Err("All shares must have the same length");
            }
            if !keys.insert(share.x.0) {
                return Err("Duplicate share X coordinate found");
            }
            values.push(share.clone());
        }

        if keys.is_empty() || (keys.len() < self.0 as usize) {
            Err("Not enough shares to recover original secret")
        } else {
            Ok(math::interpolate(values.as_slice()))
        }
    }

    /// Recovers a secret from an iterable collection of shares.
    ///
    /// Recovery rejects shares whose value vectors differ in length, repeated X coordinates, and sets with fewer distinct shares than the threshold.
    /// On success it returns the recovered bytes, wrapped so memory is wiped on drop when `zeroize_memory` is enabled.
    /// An empty secret recovers as an empty slice.
    ///
    /// Example:
    /// ```
    /// # use shamirs::{ Sharks, Share };
    /// # use rand_chacha::rand_core::SeedableRng;
    /// # let sharks = Sharks(3);
    /// # let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
    /// # let mut shares: Vec<Share> = sharks.dealer_rng(&[1], &mut rng).take(3).collect();
    /// // Recover original secret from shares
    /// let mut secret = sharks.recover(&shares);
    /// // Secret correctly recovered
    /// assert!(secret.is_ok());
    /// // Remove shares for demonstration purposes
    /// shares.clear();
    /// secret = sharks.recover(&shares);
    /// // Not enough shares to recover secret
    /// assert!(secret.is_err());
    #[cfg(feature = "zeroize_memory")]
    pub fn recover<'a, T>(&self, shares: T) -> Result<zeroize::Zeroizing<Vec<u8>>, &str>
    where
        T: IntoIterator<Item = &'a Share>,
        T::IntoIter: Iterator<Item = &'a Share>,
    {
        let mut share_length: Option<usize> = None;
        let mut keys: HashSet<u8> = HashSet::new();
        let mut values: Vec<Share> = Vec::new();

        for share in shares.into_iter() {
            if share_length.is_none() {
                share_length = Some(share.y.len());
            }

            if Some(share.y.len()) != share_length {
                return Err("All shares must have the same length");
            }
            if !keys.insert(share.x.0) {
                return Err("Duplicate share X coordinate found");
            }
            values.push(share.clone());
        }

        if keys.is_empty() || (keys.len() < self.0 as usize) {
            Err("Not enough shares to recover original secret")
        } else {
            Ok(zeroize::Zeroizing::new(math::interpolate(
                values.as_slice(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Share, Sharks};
    use alloc::vec::Vec;
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

    const THRESHOLDS: [u8; 5] = [1, 2, 3, 5, 255];
    const SECRET_LENGTHS: [usize; 5] = [0, 1, 3, 16, 32];

    impl Sharks {
        fn make_shares(&self, secret: &[u8]) -> impl Iterator<Item = Share> {
            let mut rng = ChaCha8Rng::from_seed([0x90; 32]);
            self.dealer_rng(secret, &mut rng)
        }
    }

    fn assert_roundtrip(threshold: u8, secret: &[u8], recovered: usize) {
        let sharks = Sharks(threshold);
        let shares: Vec<Share> = sharks.make_shares(secret).take(recovered).collect();
        assert_eq!(
            shares.len(),
            recovered,
            "threshold {threshold}, secret length {}",
            secret.len()
        );
        assert!(
            shares.iter().all(|share| share.y.len() == secret.len()),
            "threshold {threshold}, secret length {}",
            secret.len()
        );
        assert_eq!(
            sharks.recover(&shares).unwrap().as_slice(),
            secret,
            "threshold {threshold}, secret length {}",
            secret.len()
        );
    }

    #[test]
    fn roundtrip_matrix() {
        for threshold in THRESHOLDS {
            for length in SECRET_LENGTHS {
                if threshold == 255 && length > 1 {
                    continue;
                }
                let secret: Vec<u8> = (0..length).map(|index| index as u8).collect();
                assert_roundtrip(threshold, &secret, threshold as usize);
                if threshold != 255 {
                    assert_roundtrip(threshold, &secret, threshold as usize + 1);
                }
            }
        }
    }

    #[test]
    fn no_shares_at_threshold_one_errors() {
        let sharks = Sharks(1);
        let shares: Vec<Share> = Vec::new();
        assert!(sharks.recover(&shares).is_err());
    }

    #[test]
    fn duplicate_x_in_sufficient_set_errors() {
        let sharks = Sharks(2);
        let mut shares: Vec<Share> = sharks.make_shares(&[1, 2, 3]).take(3).collect();
        shares.push(Share {
            x: shares[0].x.clone(),
            y: shares[0].y.clone(),
        });
        assert!(sharks.recover(&shares).is_err());
    }

    #[test]
    fn mismatched_value_lengths_errors() {
        let sharks = Sharks(2);
        let first: Share = sharks.make_shares(&[1, 2, 3]).next().unwrap();
        let second: Share = sharks.make_shares(&[1]).nth(1).unwrap();
        assert_ne!(first.x.0, second.x.0);
        assert_ne!(first.y.len(), second.y.len());
        assert!(sharks.recover(&[first, second]).is_err());
    }

    #[test]
    fn dealer_yields_255_distinct_coordinates_on_over_request() {
        let sharks = Sharks(1);
        let shares: Vec<Share> = sharks.make_shares(&[1]).take(300).collect();
        assert_eq!(shares.len(), 255);
        let mut coordinates: Vec<u8> = shares.iter().map(|share| share.x.0).collect();
        coordinates.sort_unstable();
        coordinates.dedup();
        assert_eq!(coordinates.len(), 255);
    }

    #[cfg(feature = "std")]
    #[test]
    fn thread_local_dealer_roundtrip() {
        let sharks = Sharks(3);
        let shares: Vec<Share> = sharks.dealer(&[1, 2, 3]).take(3).collect();
        assert_eq!(shares.len(), 3);
        assert_eq!(sharks.recover(&shares).unwrap().as_slice(), &[1u8, 2, 3]);
    }

    #[test]
    fn test_insufficient_shares_err() {
        let sharks = Sharks(255);
        let shares: Vec<Share> = sharks.make_shares(&[1]).take(254).collect();
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_duplicate_shares_err() {
        let sharks = Sharks(255);
        let mut shares: Vec<Share> = sharks.make_shares(&[1]).take(255).collect();
        shares[1] = Share {
            x: shares[0].x.clone(),
            y: shares[0].y.clone(),
        };
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }
    #[test]
    fn test_duplicate_shares_push_err() {
        let sharks = Sharks(2);
        let mut shares: Vec<Share> = sharks.make_shares(&[1, 2, 3]).take(2).collect();
        shares.push(Share {
            x: shares[0].x.clone(),
            y: shares[0].y.clone(),
        });
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_integration_works() {
        let sharks = Sharks(255);
        let shares: Vec<Share> = sharks.make_shares(&[1, 2, 3, 4]).take(255).collect();
        let secret = sharks.recover(&shares).unwrap();
        assert_eq!(secret.as_slice(), &[1u8, 2, 3, 4]);
    }
}

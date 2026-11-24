use crate::contracts::support::{collect_shares, seeded_rng};
use shamirs::{Share, Sharks};

fn accepts_any_rng<R: rand::Rng>(sharks: &Sharks, rng: &mut R) -> Vec<Share> {
    collect_shares(sharks.dealer_rng(&[1, 2], rng).take(5))
}

#[test]
fn dealer_rng_accepts_rng() {
    let sharks = Sharks(3);
    let shares = accepts_any_rng(&sharks, &mut seeded_rng());
    assert_eq!(shares.len(), 5);
}

#[cfg(feature = "std")]
#[test]
fn dealer_produces_shares_via_iterator() {
    let sharks = Sharks(3);
    let shares = collect_shares(sharks.dealer(&[7, 8]).take(4));
    assert_eq!(shares.len(), 4);
}

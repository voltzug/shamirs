use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use shamirs::Share;

pub(crate) fn collect_shares<I>(iter: I) -> Vec<Share>
where
    I: IntoIterator<Item = Share>,
{
    iter.into_iter().collect()
}

pub(crate) fn seeded_rng() -> ChaCha8Rng {
    ChaCha8Rng::from_seed([0x90; 32])
}

pub(crate) fn sample_share() -> Share {
    Share::try_from(&[10u8, 1, 2, 3][..]).unwrap()
}

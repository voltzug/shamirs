use crate::contracts::support::{collect_shares, seeded_rng};
use shamirs::Sharks;

#[test]
fn recover_accepts_share_refs_vec() {
    let sharks = Sharks(3);
    let shares = collect_shares(sharks.dealer_rng(&[9], &mut seeded_rng()).take(3));
    let secret = sharks.recover(&shares);
    #[cfg(not(feature = "zeroize_memory"))]
    {
        let _secret: Vec<u8> = secret.unwrap();
    }
    #[cfg(feature = "zeroize_memory")]
    {
        let _secret: zeroize::Zeroizing<Vec<u8>> = secret.unwrap();
    }
}

#[test]
fn recover_accepts_iterator_of_share_refs() {
    let sharks = Sharks(2);
    let shares = collect_shares(sharks.dealer_rng(&[1, 2], &mut seeded_rng()).take(2));
    let secret = sharks.recover(shares.iter());
    #[cfg(not(feature = "zeroize_memory"))]
    {
        let _secret: Vec<u8> = secret.unwrap();
    }
    #[cfg(feature = "zeroize_memory")]
    {
        let _secret: zeroize::Zeroizing<Vec<u8>> = secret.unwrap();
    }
}

#[test]
fn recover_accepts_slice_of_shares() {
    let sharks = Sharks(2);
    let shares = collect_shares(sharks.dealer_rng(&[3], &mut seeded_rng()).take(2));
    let secret = sharks.recover(shares.as_slice());
    #[cfg(not(feature = "zeroize_memory"))]
    {
        let _secret: Vec<u8> = secret.unwrap();
    }
    #[cfg(feature = "zeroize_memory")]
    {
        let _secret: zeroize::Zeroizing<Vec<u8>> = secret.unwrap();
    }
}

#[test]
fn recover_error_is_str() {
    let sharks = Sharks(5);
    let shares = collect_shares(sharks.dealer_rng(&[1], &mut seeded_rng()).take(2));
    let secret = sharks.recover(&shares);
    match secret {
        Err(error) => {
            let _error: &str = error;
        }
        Ok(_) => panic!("expected err"),
    }
}

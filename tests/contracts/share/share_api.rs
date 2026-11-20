use crate::contracts::support::sample_share;
use shamirs::Share;

#[test]
fn share_public_fields_and_clone() {
    let share = sample_share();
    let cloned = share.clone();
    let _share = Share {
        x: cloned.x.clone(),
        y: cloned.y.clone(),
    };
}

#[test]
fn share_x_is_accessible_via_tuple_field() {
    let share = sample_share();
    let x: u8 = share.x.0;
    let _ = x;
}

#[test]
fn share_y_is_vec_of_field_values() {
    let share = sample_share();
    let _: &Vec<_> = &share.y;
    let y0: u8 = share.y[0].0;
    let _ = y0;
}

#[test]
fn share_field_values_support_partial_eq() {
    let share1 = sample_share();
    let share2 = sample_share();
    let eq: bool = share1.x == share2.x;
    let _ = eq;
}

#[cfg(feature = "zeroize_memory")]
#[test]
fn share_supports_zeroize_when_feature_enabled() {
    use zeroize::Zeroize;
    let mut share = sample_share();
    share.zeroize();
    assert_eq!(share.x.0, 0);
    for v in share.y.iter() {
        assert_eq!(v.0, 0);
    }
}

use shamirs::Share;

#[test]
fn share_roundtrips_via_byte_conversions() {
    let bytes_in = [10u8, 1, 2, 3];
    let share = Share::try_from(&bytes_in[..]).unwrap();
    let bytes_out: Vec<u8> = Vec::from(&share);
    assert_eq!(bytes_out, bytes_in);
}

#[test]
fn share_try_from_too_short_returns_static_str_error() {
    let bytes_in = [1u8];
    match Share::try_from(&bytes_in[..]) {
        Err(error) => {
            let _error: &'static str = error;
        }
        Ok(_) => panic!("expected error"),
    }
}

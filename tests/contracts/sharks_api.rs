use shamirs::Sharks;

#[test]
fn sharks_tuple_construction_and_threshold() {
    let sharks = Sharks(10);
    assert_eq!(sharks.0, 10u8);
}

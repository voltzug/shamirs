use crate::contracts::support::sample_share;

#[test]
fn share_field_values_support_add_sub_mul() {
    let share1 = sample_share();
    let share2 = sample_share();
    let x1 = share1.x.clone();
    let x2 = share2.x.clone();
    let add_res: u8 = (x1.clone() + x2.clone()).0;
    let _ = add_res;
    let sub_res: u8 = (x1.clone() - x2.clone()).0;
    let _ = sub_res;
    let mul_res: u8 = (x1.clone() * x2.clone()).0;
    let _ = mul_res;
}

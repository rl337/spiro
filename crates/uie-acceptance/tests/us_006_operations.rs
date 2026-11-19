//! US-006
use uie_core::ops::OpKind;

#[test]
fn us_006_fill_op_is_constructible() {
    let _ = OpKind::Fill { value: 3.14 };
}
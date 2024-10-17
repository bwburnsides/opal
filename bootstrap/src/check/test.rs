use crate::check::infer_type::*;
use crate::model::*;

#[test]
fn infer_0_u8() {
    assert_eq!(infer_integer_literal_type(0), Type::U8)
}

#[test]
fn infer_128_u8() {
    assert_eq!(infer_integer_literal_type(128), Type::U8)
}

#[test]
fn infer_255_u8() {
    assert_eq!(infer_integer_literal_type(255), Type::U8)
}

#[test]
fn infer_256_u16() {
    assert_eq!(infer_integer_literal_type(256), Type::U16)
}

#[test]
fn infer_32k_u16() {
    assert_eq!(infer_integer_literal_type(32 * 1024), Type::U16)
}

#[test]
fn infer_u16_max() {
    assert_eq!(infer_integer_literal_type(u16::MAX as u32), Type::U16)
}

#[test]
fn infer_64k_u32() {
    assert_eq!(infer_integer_literal_type((u16::MAX as u32) + 1), Type::U32)
}

#[test]
fn infer_u32_max() {
    assert_eq!(infer_integer_literal_type(u32::MAX), Type::U32)
}

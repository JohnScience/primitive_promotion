#![no_std]
#![doc = include_str!("../README.md")]

/// Extension trait offering PrimitivePromotion type.
/// 
/// * `<u8 as PrimitivePromotionExt>::PrimitivePromotion == u16`;
/// * `<i16 as PrimitivePromotionExt>::PrimitivePromotion == i32`;
/// * ...
pub trait PrimitivePromotionExt {
    type PrimitivePromotion;
}

impl PrimitivePromotionExt for u8 {
    type PrimitivePromotion = u16;
}

impl PrimitivePromotionExt for u16 {
    type PrimitivePromotion = u32;
}

impl PrimitivePromotionExt for u32 {
    type PrimitivePromotion = u64;
}

impl PrimitivePromotionExt for u64 {
    type PrimitivePromotion = u128;
}

impl PrimitivePromotionExt for i8 {
    type PrimitivePromotion = i16;
}

impl PrimitivePromotionExt for i16 {
    type PrimitivePromotion = i32;
}

impl PrimitivePromotionExt for i32 {
    type PrimitivePromotion = i64;
}

impl PrimitivePromotionExt for i64 {
    type PrimitivePromotion = i128;
}

impl PrimitivePromotionExt for f32 {
    type PrimitivePromotion = f64;
}

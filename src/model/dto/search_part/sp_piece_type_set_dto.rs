//!
//! 駒種類集合
//!

use super::super::super::super::controller::common_part::cp_conv_controller::*;
use super::super::super::super::model::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::super::model::vo::other_part::op_piece_type_vo::*;
use std::collections::HashSet;

pub struct SPPieceTypeSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceTypeSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> SPPieceTypeSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for kms in KMS_ARRAY.iter() {
            num_syugo1.insert(kms_to_num(kms));
        }
        let kms_syugo = SPPieceTypeSetDto {
            num_syugo: num_syugo1,
        };
        kms_syugo
    }
    pub fn remove(&mut self, kms: &PieceType) {
        self.num_syugo.remove(&kms_to_num(kms));
    }
}
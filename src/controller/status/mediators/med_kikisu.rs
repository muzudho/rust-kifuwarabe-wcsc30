//!
//! 利き数
//!
use super::super::super::super::controller::common::conv::*;
use super::super::super::super::controller::consoles::asserts::*;
use super::super::super::super::controller::movement_generation::mg_sub_part::*;
use super::super::super::super::model::master::phase::*;
use super::super::super::super::model::master::piece::*;
use super::super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::super::model::master::square::*;
use super::super::super::super::model::universe::*;
use std::collections::HashSet;

/// 盤上の利き升調べ
///
/// 用途：自殺手防止他
pub fn update_effect_count(universe: &mut Universe) {
    // ゼロ・リセット
    for pc in KM_ARRAY.iter() {
        &universe.get_search_part_mut().effect_count_by_piece
            [PieceStruct::from_piece((*pc).clone()).serial_piece_number()]
        .clear();
    }

    for sn in SN_ARRAY.iter() {
        &universe.get_search_part_mut().effect_count_by_phase[sn_to_num(sn)].clear();
    }

    // カウント
    for km_dst in KM_ARRAY.iter() {
        let ps_dst = PieceStruct::from_piece((*km_dst).clone());

        for x in SUJI_1..SUJI_10 {
            // 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let sq_dst = Square::from_file_rank(x, y);
                assert_banjo_sq(&sq_dst, "think 利き調べ");

                // 移動元の升
                let mut mv_src_hashset: HashSet<Square> = HashSet::<Square>::new();
                make_no_promotion_source_by_square_and_piece(
                    &sq_dst,
                    &ps_dst,
                    &universe.get_search_part(),
                    |square| {
                        mv_src_hashset.insert(square);
                    },
                );
                make_before_promotion_source_by_square_piece(
                    &sq_dst,
                    &ps_dst,
                    &universe.get_application_part(),
                    &universe.get_search_part(),
                    |square| {
                        mv_src_hashset.insert(square);
                    },
                );
                // 打は考えない。盤上の利き数なので
                let kikisu = mv_src_hashset.len();

                // 駒別
                universe.get_search_part_mut().effect_count_by_piece[ps_dst.serial_piece_number()]
                    .add_su_by_sq(&sq_dst, kikisu as i8);

                // 先後別
                universe.get_search_part_mut().effect_count_by_phase[sn_to_num(&ps_dst.phase())]
                    .add_su_by_sq(&sq_dst, kikisu as i8);
            }
        }
    }
}

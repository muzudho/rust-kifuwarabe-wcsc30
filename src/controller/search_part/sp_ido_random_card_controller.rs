#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::controller::common_use::cu_asserts_controller::*;
use super::super::super::controller::common_use::cu_random_move_controller;
use super::super::super::controller::movement_generation::mg_choicing_controller::*;
use super::super::super::controller::movement_generation::mg_main_controller::*;
use super::super::super::controller::search_part::sp_jisatusyu_result_controller::*;
use super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use std::collections::HashSet;

/**
 * ランダム移動
 *
 * piece_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(
    ml_dto: &MLDto,
    piece_dst: &OPPieceVo,
    speed_of_light: &MLSpeedOfLightVo,
) -> MLMovementDto {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let sq_dst = cu_random_move_controller::random_square();
        assert_banjo_sq(&sq_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &ml_dto.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &ml_dto.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1movement_from_hashset(&ss_hashset);

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    MLMovementDto::default()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(ml_dto: &MLDto, speed_of_light: &MLSpeedOfLightVo) -> MLMovementDto {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let sq_dst = cu_random_move_controller::random_square();
        assert_banjo_sq(&sq_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let ps_dst = speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo_by_phase_and_piece_type(
                &ml_dto.get_search_part().get_phase(&Person::Ji),
                *cu_random_move_controller::random_piece_type(),
            );
        let piece_dst = ps_dst.piece();

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &ml_dto.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &ml_dto.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1movement_from_hashset(&ss_hashset);

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&ml_dto, &ss, speed_of_light) {
            continue 'random;
        }

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    MLMovementDto::default()
}

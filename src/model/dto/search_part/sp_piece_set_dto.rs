//!
//! 駒集合
//!

use super::super::super::super::model::dto::search_part::sp_dto::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::super::model::vo::other_part::op_piece_vo::*;
use std::collections::HashSet;

pub struct SPPieceSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all(speed_of_light: &MLSpeedOfLightVo) -> SPPieceSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for piece in KM_ARRAY.iter() {
            let ps = speed_of_light.ml_piece_struct_master_vo.get_piece_vo(piece);
            num_syugo1.insert(ps.serial_piece_number());
        }
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    /**
     * 自分相手
     */
    pub fn new_jiai(
        &self,
        jiai: &Person,
        sp_dto: &SPDto,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> SPPieceSetDto {
        let sn0 = sp_dto.get_phase(&jiai);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let ps = speed_of_light.ml_piece_struct_master_vo.get_piece_vo(km);
            let (sn1, _piece_type) = ps.phase_piece_type();
            if match_sn(&sn0, &sn1) {
                num_syugo1.insert(ps.serial_piece_number());
            }
        }
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece: &OPPieceVo, speed_of_light: &MLSpeedOfLightVo) {
        self.num_syugo.remove(
            &speed_of_light
                .ml_piece_struct_master_vo
                .get_piece_vo(piece)
                .serial_piece_number(),
        );
    }
}

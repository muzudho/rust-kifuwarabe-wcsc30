//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::square::*;
use super::super::super::model::universe::*;
use super::super::super::model::vo::speed_of_light::*;

pub fn is_ji_km_by_sq(sq: &Square, universe: &Universe, speed_of_light: &SpeedOfLight) -> bool {
    match_sn(
        &speed_of_light
            .piece_vo_master
            .get_piece_vo(
                &universe
                    .get_search_part()
                    .get_current_position()
                    .get_piece_by_square(&sq),
            )
            .phase(),
        &universe.get_search_part().get_phase(&Person::Ji),
    )
}

// TODO
pub fn is_ai_kiki_by_sq(_sq: &Square, _uchu: &Universe) -> bool {
    false
}

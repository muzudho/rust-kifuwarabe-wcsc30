//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::place::*;

pub fn is_ji_km_by_ms(ms: umasu, uchu: &Uchu) -> bool {
    match_sn(
        &uchu.ky.get_piece_struct_by_ms(ms).phase(),
        &uchu.get_teban(&Person::Ji),
    )
}

// TODO
pub fn is_ai_kiki_by_ms(_ms: umasu, _uchu: &Uchu) -> bool {
    false
}

#![allow(dead_code)]
//!
//! 頭金仮説
//!
use super::super::super::super::controller::boardmetries::board_metrics;
use super::super::super::super::controller::common::conv::*;
use super::super::super::super::model::combine::multiplication::*;
use super::super::super::super::model::master::person::Person;
use super::super::super::super::model::master::person::*;
use super::super::super::super::model::master::piece_type::PieceType;
use super::super::super::super::model::master::piece_type_set::*;
use super::super::super::super::model::master::square::*;
use super::super::super::super::model::universe::*;

/// 後手視点で、相手らいおんの南側１升に、頭が丸い自駒がない？
pub fn is_s(universe: &Universe) -> bool {
    // 相手玉の位置
    let sq_r = universe.get_search_part().get_king_sq(&Person::Ai).clone();

    let p_r = sq_r.to_point();
    let p_south_r = p_r.to_south();
    if !p_in_ban(&p_south_r) {
        return true;
    }

    let sq_south_r = Square::from_point(&p_south_r);
    let ps = universe
        .get_search_part()
        .get_current_position()
        .get_piece_struct_by_sq(&sq_south_r);
    let jiai_km = universe.get_jiai_by_km(&ps);
    if !match_jiai(&jiai_km, &Person::Ji) {
        return true;
    }

    g_writeln(&format!(
        "info string south of My raion {} = {}. jiai_km={}.",
        sq_r.to_umasu(),
        ps.piece(),
        jiai_km
    ));

    use super::super::super::super::model::master::piece_type::PieceType::*;
    match ps.piece_type() {
        Z | S => {
            return false;
        }
        _ => {}
    }

    return true;
}

/**
 * 頭金か？
 *
 * 三駒定形｛　ms_ai_r、 kms_set_ai_c_r、 ms_atama、 ms_kin、ms_behind、Ｔ　｝において、
 * 　｛　升　×　相手玉　　　　　｝∈ ms_aiR　かつ　、
 * 　｛　升　×　玉以外の相手駒　｝∈ kms_set_ai_c_r　かつ　、
 * 　｛　升　×　駒無し含む玉以外の相手駒　｝∈ ms_atama、
 * 　升　×　｛金、と、杏、圭、全、馬、竜｝　∈ ms_kin、
 * 　　　　　　　　　　　　　　升　×　自駒　∈ ms_behind、
 * 　Ｔは　１手　固定とし、
 *
 * ms_ai_r の下段左中右＝移動不可升
 * かつ、
 * ms_ai_r（北０）ms_atama
 * かつ、
 * ms_behind （利き→）ms_atama、
 * かつ、
 * （
 * 　　（　ms_atama は空升　かつ　｛金｝∈自持駒　）
 * 　　または
 * 　　（　ms_kin （利き→）ms_atama　）
 * ）
 * かつ、
 * kms_set_ai_c_r（利き→） ms_atama でない、
 *
 * のとき、
 *
 * 一手詰め逃さない公理から、
 *
 * ms_ai_r の９近傍非空升　⊂　ms_atama に指す（ ms_ai_r、kms_set_ai_c_r，ms_atama、 ms_kin， ms_behind，Ｔ）
 *
 * が成り立つ
 * FIXME Aが動いたときの、逆王手が未考慮
 */
pub fn is_atamakin(
    _mskms_l: &SqKms,
    _mskms_s: &SqKms,
    _mskms_a: &SqKms,
    _mskms_b: &SqKms,
    universe: &Universe,
) -> bool {
    // 相手らいおんのマス
    let sq_ai_r = universe.get_search_part().get_king_sq(&Person::Ai).clone();

    // らいおん以外の相手の駒種類
    let mut kms_set_ai_c_r = PieceTypeSet::new_all();
    kms_set_ai_c_r.remove(&PieceType::R);

    // kの下段左中右＝移動不可升　※現局面２手先の動き？
    // A が移動することで、利きは変わるか？　玉の下３つは変わらない
    // 単に下３つに移動できるか調べられたらいい。８１升別　利きを作るか？
    // 駒、相手の利き
    let p_k = sq_ai_r.to_point();
    if board_metrics::is_ji_km_by_sq(&Square::from_point(&p_k.to_south_west()), &universe) {}

    if board_metrics::is_ai_kiki_by_sq(&Square::from_point(&p_k.to_south_west()), &universe) {}

    // ms_ai_r （北０） ms_atama
    // if ms_north_of_ms( ms_ai_r, 0, ms_atama ) { }

    // ms_behind （利き→）ms_atama、
    // mskms_attack_to_ms( b, ms_atama ) { }

    // （　ms_atama は空升　かつ　｛金｝∈自持駒　）
    // ( ms_is_empty( ms_atama ) && has_mg( I ) )
    // または
    // ||
    // （　ms_kin （利き→）ms_atama　）
    // mskms_attack_to_ms( ms_kin, ms_atama ) { }

    false
}

//!
//! コマンド一覧
//!

use super::super::super::model::master::phase::Phase;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece::*;
use super::super::super::model::universe::*;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(universe: &Universe) {
    for km in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", km));
        let s = universe.kaku_number_board(&Phase::Owari, &km);
        g_writeln(&s);
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = universe.kaku_number_board(&sn, &Piece::Owari);
        g_writeln(&s);
    }
}

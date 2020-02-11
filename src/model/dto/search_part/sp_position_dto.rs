//!
//! 局面
//!
//! 後手（上手）から見た盤にすると、
//! 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
//!
//! プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
//! 盤を想像すること☆（＾～＾）！
//!

use super::super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::super::model::vo::other_part::op_square_vo::*;

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
pub struct SPPositionDto {
    /// 10の位を筋、1の位を段とする。
    /// 0筋、0段は未使用
    board: [GPPieceVo; BOARD_SIZE],
    /// 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
    /// 増減させたいので、u8 ではなく i8。
    pub hand: [i8; PIECE_LN],
    /// らいおんの位置
    /// [先後]
    square_of_king: [Square; PHASE_LN],
}
impl Default for SPPositionDto {
    fn default() -> Self {
        use super::super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::NonePiece;
        SPPositionDto {
            // 盤上
            board: [
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
                NonePiece, NonePiece,
            ],
            // 持ち駒数
            hand: [
                // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
                0, 0,
            ],
            square_of_king: [
                Square::from_umasu(0),
                Square::from_umasu(0),
                Square::from_umasu(0),
            ],
        }
    }
}
impl SPPositionDto {
    pub fn clear(&mut self) {
        use super::super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::NonePiece;
        self.board = [
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece, NonePiece,
            NonePiece, NonePiece, NonePiece, NonePiece,
        ];
        self.hand = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
            0, 0,
        ];
    }

    /// らいおんの位置
    pub fn get_sq_r(&self, phase_number: usize) -> &Square {
        &self.square_of_king[phase_number]
    }

    /**
     * 歩が置いてあるか確認
     */
    pub fn exists_fu_by_phase_suji(
        &self,
        phase: &Phase,
        suji: i8,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        for dan in DAN_1..DAN_10 {
            let sq = Square::from_file_rank(suji, dan);
            let piece99 = self.get_piece_by_square(&sq);
            let ps100 = speed_of_light.get_piece_struct_vo(piece99);
            let (phase_piece, piece_type) = ps100.phase_piece_type();
            if phase_piece == phase && piece_type == GPPieceTypeVo::Pawn {
                return true;
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn get_piece_by_square(&self, sq: &Square) -> &GPPieceVo {
        &self.board[sq.to_umasu()]
    }
    /// 升で指定して駒を置く
    pub fn set_piece_by_square(&mut self, sq: &Square, piece: &GPPieceVo) {
        self.board[sq.to_umasu()] = piece.clone();

        // 玉の位置を覚え直します。
        use super::super::super::super::model::vo::other_part::op_phase_vo::Phase::*;
        match *piece {
            GPPieceVo::King1 => self.square_of_king[First as usize] = sq.clone(),
            GPPieceVo::King2 => self.square_of_king[Second as usize] = sq.clone(),
            _ => {}
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_hand(&mut self, hand: &GPPieceVo, maisu: i8, speed_of_light: &MLSpeedOfLightVo) {
        self.hand[speed_of_light
            .get_piece_struct_vo(hand)
            .serial_piece_number()] += maisu;
    }
    pub fn get_hand(&self, hand: &GPPieceVo, speed_of_light: &MLSpeedOfLightVo) -> i8 {
        self.hand[speed_of_light
            .get_piece_struct_vo(hand)
            .serial_piece_number()]
    }

    /**
     * 指定の升に駒があれば真
     */
    pub fn exists_km(&self, sq: &Square, speed_of_light: &MLSpeedOfLightVo) -> bool {
        !speed_of_light
            .get_piece_struct_vo(self.get_piece_by_square(&sq))
            .equals_piece(&speed_of_light.get_piece_struct_vo(&GPPieceVo::NonePiece))
    }

    /// 指定の升に指定の駒があれば真
    pub fn has_sq_km(
        &self,
        sq: &Square,
        piece: &GPPieceVo,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        speed_of_light
            .get_piece_struct_vo(self.get_piece_by_square(&sq))
            .equals_piece(&speed_of_light.get_piece_struct_vo(piece))
    }

    /// 指定の升にある駒の先後、または空升
    pub fn get_phase_by_sq(&self, sq: &Square, speed_of_light: &MLSpeedOfLightVo) -> Phase {
        speed_of_light
            .get_piece_struct_vo(self.get_piece_by_square(sq))
            .phase()
    }

    /// 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
    pub fn is_natta(
        &self,
        sq_src: &Square,
        sq_dst: &Square,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> bool {
        let km_src = self.get_piece_by_square(&sq_src);

        let ps_src = speed_of_light.get_piece_struct_vo(km_src);
        let km_dst = self.get_piece_by_square(&sq_dst);

        let ps_dst = speed_of_light.get_piece_struct_vo(km_dst);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = ps_dst.is_promoted();
        let pro_src = ps_src.is_promoted();

        // 成り
        pro_dst && !pro_src
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, ml_universe_dto: &MLDto, speed_of_light: &MLSpeedOfLightVo) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BOARD_SIZE {
            let i_sq = Square::from_umasu(i_ms as umasu);
            let km = self.get_piece_by_square(&i_sq);
            let num_km = speed_of_light.get_piece_struct_vo(km).serial_piece_number();
            hash ^= ml_universe_dto.get_position_hash_seed().km[i_ms][num_km];
        }

        // 持ち駒ハッシュ
        for km in &KM_ARRAY {
            let num_km = speed_of_light.get_piece_struct_vo(km).serial_piece_number();

            let maisu = self.get_hand(km, &speed_of_light);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                km,
                maisu,
                MG_MAX
            );

            hash ^= ml_universe_dto.get_position_hash_seed().mg[num_km][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}

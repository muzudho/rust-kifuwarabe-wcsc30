//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::smart::features::{HandAddressType, PieceMeaning};
use crate::cosmic::toy_box::PieceNum;
use crate::law::speed_of_light::SpeedOfLight;

/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: isize = -300;

pub struct Evaluation {
    // 盤面をカバーする利きの多さの重み☆（＾～＾）1000分率☆（＾～＾）
    board_coverage_weight: isize,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    komawari_weight: isize,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    promotion_weight: isize,
    // 駒割だぜ☆（＾～＾）
    piece_allocation_value: isize,
    /// 成り駒ボーナスだぜ☆（＾～＾）
    promotion_value: isize,
}
impl Evaluation {
    pub fn new(
        board_coverage_weight: isize,
        komawari_weight: isize,
        promotion_weight: isize,
    ) -> Self {
        Evaluation {
            board_coverage_weight: board_coverage_weight,
            komawari_weight: komawari_weight,
            promotion_weight: promotion_weight,
            piece_allocation_value: 0,
            promotion_value: 0,
        }
    }
    pub fn centi_pawn(&self, board_coverage_value: isize) -> isize {
        self.board_coverage(board_coverage_value) + self.komawari() + self.promotion()
    }
    pub fn board_coverage(&self, board_coverage_value: isize) -> isize {
        self.board_coverage_weight * board_coverage_value / 1000
    }
    pub fn komawari(&self) -> isize {
        self.komawari_weight * self.piece_allocation_value / 1000
    }
    pub fn promotion(&self) -> isize {
        self.promotion_weight * self.promotion_value / 1000
    }

    pub fn before_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    pub fn after_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    pub fn after_do_move(
        &mut self,
        source_piece: &Option<(PieceMeaning, PieceNum)>,
        captured_piece: &Option<(PieceMeaning, PieceNum)>,
        promotion: bool,
        speed_of_light: &SpeedOfLight,
    ) -> (isize, isize) {
        // 取った駒の価値を評価するぜ☆（＾～＾）
        let delta_captured_piece =
            Evaluation::caputured_piece_value(captured_piece, speed_of_light);
        self.piece_allocation_value += delta_captured_piece;

        // 成り駒を取って降格させたら、成り駒評価値追加だぜ☆（＾～＾）
        let delta_promotion = if let Some(captured_piece_val) = captured_piece {
            if captured_piece_val
                .0
                .r#type(speed_of_light)
                .promoted(speed_of_light)
            {
                Evaluation::promotion_value(captured_piece_val.0.hand_address(&speed_of_light).r#type(&speed_of_light))
            } else {
                0
            }
        } else {
            0
        }
        // 進めた駒が成っても、評価値追加だぜ☆（＾～＾）
        +
        if let Some(source_piece_val) = source_piece {
            if promotion {
                Evaluation::promotion_value(source_piece_val.0.hand_address(speed_of_light).r#type(&speed_of_light))
            } else {
                0
            }
        } else {
            // 打なら成りは無いぜ☆（＾～＾）
            0
        };
        self.promotion_value += delta_promotion;

        (delta_captured_piece, delta_promotion)
    }

    pub fn before_undo_move(&mut self, delta_captured_piece: isize, delta_promotion: isize) {
        // 1手戻すぜ☆（＾～＾）
        self.piece_allocation_value -= delta_captured_piece;
        self.promotion_value -= delta_promotion;
    }

    /// 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    /// 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    pub fn promotion_value(adr: HandAddressType) -> isize {
        match adr {
            HandAddressType::King => 0,
            HandAddressType::Rook => 1,
            HandAddressType::Bishop => 1,
            HandAddressType::Gold => 0,
            HandAddressType::Silver => 0,
            HandAddressType::Knight => 1,
            HandAddressType::Lance => 1,
            HandAddressType::Pawn => 1,
        }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないのは、駒割ではなく、別の方法で対応してくれだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// Centi pawn.
    fn caputured_piece_value(
        captured_piece: &Option<(PieceMeaning, PieceNum)>,
        speed_of_light: &SpeedOfLight,
    ) -> isize {
        if let Some(captured_piece_val) = captured_piece {
            match captured_piece_val
                .0
                .hand_address(speed_of_light)
                .r#type(speed_of_light)
            {
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                HandAddressType::King => 0,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                HandAddressType::Rook => 1000,
                HandAddressType::Bishop => 900,
                HandAddressType::Gold => 600,
                HandAddressType::Silver => 500,
                HandAddressType::Knight => 300,
                HandAddressType::Lance => 200,
                HandAddressType::Pawn => 100,
            }
        } else {
            0
        }
    }
}

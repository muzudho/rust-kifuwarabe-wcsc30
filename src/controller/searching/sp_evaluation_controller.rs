//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::model::univ::speed_of_light::MLSpeedOfLightVo;

pub struct SPEvaluationController {}
impl SPEvaluationController {
    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// ライオンを取ったら勝ちだぜ☆（＾～＾）
    pub fn evaluate(
        captured_piece_o: Option<Piece>,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> (i16, bool) {
        if let Some(captured_piece) = captured_piece_o {
            let piece_struct = speed_of_light.get_piece_struct(&captured_piece);
            match piece_struct.phase_piece_type.1 {
                PieceType::King => (25000, true),
                PieceType::Rook => (1000, false),
                PieceType::Bishop => (900, false),
                PieceType::Gold => (600, false),
                PieceType::Silver => (500, false),
                PieceType::Knight => (300, false),
                PieceType::Lance => (200, false),
                PieceType::Pawn => (100, false),
                PieceType::Dragon => (2000, false),
                PieceType::Horse => (1900, false),
                PieceType::PromotedSilver => (500, false),
                PieceType::PromotedKnight => (300, false),
                PieceType::PromotedLance => (200, false),
                PieceType::PromotedPawn => (100, false),
            }
        } else {
            (0, false)
        }
    }
}
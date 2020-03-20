//! 探索部
use crate::controller::search_part::sp_number_board_controller::*;
use crate::model::dto::search_part::sp_info::SPInfo;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::movement::*;
use crate::model::univ::gam::movement_builder::*;
use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece::*;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::gam::square::*;
use crate::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::model::vo::other_part::op_person_vo::*;
use crate::model::vo::other_part::op_ply_vo::*;

/// ミュータブルなオブジェクト☆（＾～＾）
pub struct Position {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    ply: i16,

    /// 現局面
    current_board: Board,

    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hash_history: [u64; PLY_LN],

    /// 取った駒
    pub captured_piece_history: [Piece; PLY_LN],

    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements_history: [Movement; PLY_LN],

    /// 現在の指し手を作成中。
    pub current_movement_builder: MovementBuilder,

    /// 利きの数（先後別）
    pub control_count_by_phase: [NumberBoard; PHASE_LN],

    /// 利きの数（先後付き駒別）
    pub control_count_by_piece: [NumberBoard; PIECE_LN],
    // ビジョン・ツリー
    // pub vision_tree_by_phase: [VisionTree; PHASE_LN],
    /// 情報表示担当
    pub info: SPInfo,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            ply: 0,
            // 現局面
            current_board: Board::default(),
            position_hash_history: [0; PLY_LN],
            /// 取った駒
            captured_piece_history: [Piece::NonePiece; PLY_LN],
            movements_history: [Movement::default(); PLY_LN],
            /// 現在の指し手を作成中。
            current_movement_builder: MovementBuilder::default(),
            /// 利き数（先後別）
            control_count_by_phase: [
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
            ],
            // 利き数（駒別なので３０個ある）
            control_count_by_piece: [
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
            ],
            // vision_tree_by_phase: [VisionTree::default(), VisionTree::default(), VisionTree::default()],
            info: SPInfo::default(),
        }
    }
}
impl Position {
    pub fn add_ply(&mut self, ply1: i16) {
        self.ply += ply1
    }
    pub fn get_ply(&self) -> i16 {
        self.ply
    }
    pub fn set_ply(&mut self, ply1: i16) {
        self.ply = ply1
    }

    /// 手番
    pub fn get_phase(&self, person: &Person) -> Phase {
        use super::super::super::super::model::vo::other_part::op_person_vo::Person::*;
        match *person {
            None => Phase::None,
            Friend => {
                // 手番
                if self.ply % 2 == 0 {
                    Phase::First
                } else {
                    Phase::Second
                }
            }
            Opponent => {
                // 相手番
                if self.ply % 2 == 0 {
                    Phase::Second
                } else {
                    Phase::First
                }
            }
        }
    }

    pub fn get_current_board(&self) -> &Board {
        &self.current_board
    }
    pub fn get_current_board_mut(&mut self) -> &mut Board {
        &mut self.current_board
    }

    pub fn get_position_hash_history(&self) -> &[u64; PLY_LN] {
        &self.position_hash_history
    }
    pub fn get_position_hash_history_mut(&mut self) -> &mut [u64; PLY_LN] {
        &mut self.position_hash_history
    }

    pub fn get_current_position_hash(&mut self) -> u64 {
        self.position_hash_history[self.ply as usize]
    }
    pub fn set_current_position_hash(&mut self, hash: u64) {
        self.position_hash_history[self.ply as usize] = hash;
    }

    /**
     * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     */
    pub fn undo_move(&mut self, movement: &Movement, speed_of_light: &MLSpeedOfLightVo) {
        let phase = self.get_phase(&Person::Friend);
        let cap = self.captured_piece_history[self.get_ply() as usize].clone();

        // 移動先の駒
        let piece186 = if movement.source.to_usquare() == SQUARE_DROP {
            // 打なら
            let piece679 = Piece::from_phase_and_piece_type(&phase, movement.drop);
            // 自分の持ち駒を増やす
            //let mg = km_to_mg(km);
            //self.add_hand(mg,1);
            self.current_board.add_hand(&piece679, 1, speed_of_light);
            piece679
        } else {
            // 打で無ければ
            if movement.promote {
                // 成ったなら、成る前へ
                speed_of_light
                    .get_piece_struct_vo(
                        self.current_board
                            .get_piece_by_square(&movement.destination),
                    )
                    .demote()
                    .clone()
            } else {
                self.current_board
                    .get_piece_by_square(&movement.destination)
                    .clone()
            }
        };

        // 移動先の駒を、取った駒（あるいは空）に戻す
        self.current_board
            .set_piece_by_square(&movement.destination, &cap);
        match cap {
            Piece::NonePiece => {}
            _ => {
                // 自分の持ち駒を減らす
                self.current_board.add_hand(
                    speed_of_light.get_piece_struct_vo(&cap).capture(),
                    -1,
                    speed_of_light,
                );
            }
        }

        // 移動元升に、動かした駒を置く
        self.current_board
            .set_piece_by_square(&movement.source, &piece186);
    }

    /// らいおんの位置
    pub fn get_king_sq(&self, person: &Person) -> &Square {
        &self
            .current_board
            .get_sq_r(phase_to_num(&self.get_phase(person)))
    }

    /// 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
    /// 手目のカウントが増えたりはしないぜ☆（＾～＾）
    ///
    /// return : 取った駒
    pub fn do_move(&mut self, movement: &Movement, speed_of_light: &MLSpeedOfLightVo) -> Piece {
        let phase = self.get_phase(&Person::Friend);

        // 取った駒
        let cap;

        {
            // 動かす駒
            let piece144 = if movement.source.to_usquare() == SQUARE_DROP {
                // 打なら
                // 自分の持ち駒を減らす
                let piece734 = Piece::from_phase_and_piece_type(&phase, movement.drop);
                self.current_board.add_hand(&piece734, -1, speed_of_light);
                piece734
            } else {
                // 打で無ければ、元の升の駒を消す。
                let piece152 = if movement.promote {
                    // 成りなら
                    speed_of_light
                        .get_piece_struct_vo(
                            self.current_board.get_piece_by_square(&movement.source),
                        )
                        .promote()
                        .clone()
                } else {
                    self.current_board
                        .get_piece_by_square(&movement.source)
                        .clone()
                };

                self.current_board
                    .set_piece_by_square(&movement.source, &Piece::NonePiece);

                piece152
            };

            // 移動先升に駒があるかどうか
            cap = if let Piece::NonePiece = self
                .current_board
                .get_piece_by_square(&movement.destination)
            {
                Piece::NonePiece
            } else {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let cap764;
                {
                    cap764 = self
                        .current_board
                        .get_piece_by_square(&movement.destination)
                        .clone();
                }

                {
                    let cap773;
                    {
                        cap773 = speed_of_light
                            .get_piece_struct_vo(&cap764)
                            .capture()
                            .clone();
                    }
                    self.current_board.add_hand(&cap773, 1, speed_of_light);
                }
                cap764
            };

            // 移動先升に駒を置く
            self.current_board
                .set_piece_by_square(&movement.destination, &piece144);
        }

        cap
    }

    pub fn get_moves_history(&self) -> &[Movement; PLY_LN] {
        &self.movements_history
    }

    /// 棋譜の作成
    pub fn set_current_movement(&mut self, movement: &Movement) {
        self.movements_history[self.get_ply() as usize] = movement.clone()
    }
    pub fn build_current_movement(&mut self) {
        self.movements_history[self.get_ply() as usize] =
            Movement::new(&self.current_movement_builder)
    }
    pub fn set_current_movement_source_temporary(&mut self, src: &Square) {
        self.current_movement_builder.src = src.clone()
    }
    pub fn set_current_movement_destination_temporary(&mut self, dst: &Square) {
        self.current_movement_builder.dst = dst.clone()
    }
    pub fn set_current_movement_promote_temporary(&mut self, pro: bool) {
        self.current_movement_builder.pro = pro
    }
    pub fn set_current_movement_drop_temporary(&mut self, piece_type: PieceType) {
        self.current_movement_builder.drop = piece_type
    }
    pub fn set_cap(&mut self, ply1: usize, km: Piece) {
        self.captured_piece_history[ply1] = km
    }
    pub fn get_move(&self) -> &Movement {
        &self.movements_history[self.get_ply() as usize]
    }

    /// 棋譜☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.get_ply() {
            let movement = &self.movements_history[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    /*
    /// 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.vision_tree_by_phase[phase_to_num(phase)]
            .set_ai_r(&self.current_position.get_sq_r(phase_to_num(opponent_phase)));
    }
    */
}
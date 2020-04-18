use crate::cosmic::game::board::board::Board;
use crate::cosmic::game::board::square::*;
use crate::cosmic::game::game::Game;
use crate::cosmic::game::movement::movement_builder::MovementBuilder;
use crate::cosmic::game::piece::piece::*;
use crate::cosmic::game::piece::piece_type::*;
use crate::cosmic::game::position::person::Person;
use crate::cosmic::game::position::phase::Phase;
use crate::cosmic::game::position::position::Position;
use crate::law::generate_move::movement_generator::*;
use crate::law::generate_move::squares::*;
use crate::law::speed_of_light::*;

pub struct MGMovements {}
impl MGMovements {
    /// 盤上の駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movements_on_board<F1>(
        friend: Phase,
        current_board: &Board,
        speed_of_light: &SpeedOfLight,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        // 盤上の駒☆（＾～＾）
        MGSquares::for_all(&mut |source| {
            MGMovements::make_a_movement_on_board(
                friend,
                &source,
                current_board,
                speed_of_light,
                callback_movement,
            )
        });
    }

    /// 盤上の駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    fn make_a_movement_on_board<F1>(
        friend: Phase,
        source: &Square,
        current_board: &Board,
        speed_of_light: &SpeedOfLight,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        let callback_next = &mut |destination, promotability, _agility| {
            use crate::cosmic::game::board::board::ThingsInTheSquare::*;
            use crate::law::generate_move::squares::Promotability::*;
            let things_in_the_square =
                current_board.what_is_in_the_square(friend, &destination, speed_of_light);
            match things_in_the_square {
                Space | Opponent => {
                    // 成れるかどうかの判定☆（＾ｑ＾）
                    let promotion = match &promotability {
                        Forced => true,
                        _ => false,
                    };
                    match &promotability {
                        Any => {
                            callback_movement(
                                MovementBuilder {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: false,
                                    drop: None,
                                }
                                .to_hash(speed_of_light),
                            );
                            callback_movement(
                                MovementBuilder {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: true,
                                    drop: None,
                                }
                                .to_hash(speed_of_light),
                            );
                        }
                        _ => {
                            callback_movement(
                                MovementBuilder {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: promotion,
                                    drop: None,
                                }
                                .to_hash(speed_of_light),
                            );
                        }
                    };
                }
                Friend => {}
            };

            match things_in_the_square {
                Space => false,
                _ => true,
            }
        };

        if let Some(piece) = current_board.get_piece_by_square(&source) {
            let ps = speed_of_light.get_piece_struct(&piece);

            if friend == ps.phase() {
                NextSquares::looking_for_squares_from_on_board(
                    ps.piece_type(),
                    friend,
                    &source,
                    callback_next,
                );
            }
        }
    }

    /// 持ち駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movements_on_hand<F1>(
        game: &Game,
        speed_of_light: &SpeedOfLight,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        GPHandPieces::for_all(&mut |any_piece_type| {
            let hand_piece = &speed_of_light
                .get_piece_struct_by_phase_and_piece_type(
                    game.history.get_phase(&Person::Friend),
                    any_piece_type,
                )
                .piece;

            if 0 < game
                .position
                .current_board
                .get_hand(hand_piece, speed_of_light)
            {
                // 駒を持っていれば
                use crate::cosmic::game::piece::piece::Piece::*;
                match *hand_piece {
                    // ▲歩、▲香 の打てる範囲は２段目～９段目。
                    Pawn1 | Lance1 => {
                        Squares::for_from_rank2_to_rank9(Phase::First, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // ▲桂 の打てる範囲は３段目～９段目。
                    Knight1 => Squares::for_from_rank3_to_rank9(Phase::First, &mut |destination| {
                        MGMovements::make_hand(
                            &hand_piece,
                            &game.position,
                            speed_of_light,
                            &destination,
                            callback_movement,
                        );
                    }),
                    // ▽歩、▽香 の打てる範囲は１段目～８段目。
                    Pawn2 | Lance2 => {
                        Squares::for_from_rank2_to_rank9(Phase::Second, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // ▲桂 の打てる範囲は１段目～７段目。
                    Knight2 => {
                        Squares::for_from_rank3_to_rank9(Phase::Second, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // それ以外の駒が打てる範囲は盤面全体。
                    _ => {
                        MGSquares::for_all(&mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        });
                    }
                }
            }
        });
    }

    fn make_hand<F1>(
        hand_piece: &Piece,
        position: &Position,
        speed_of_light: &SpeedOfLight,
        destination: &Square,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        if let None = position.current_board.get_piece_by_square(&destination) {
            // 駒が無いところに打つ
            let current_board = &position.current_board;
            let ps_dst = speed_of_light.get_piece_struct(hand_piece);
            let piece_type_dst = ps_dst.piece_type();
            use crate::cosmic::game::piece::piece::Piece::*;
            match *hand_piece {
                Pawn1 | Pawn2 => {
                    // ひよこ　は２歩できない☆（＾～＾）
                    if current_board.exists_fu_by_phase_suji(
                        ps_dst.phase(),
                        destination.get_file(),
                        speed_of_light,
                    ) {
                        return;
                    }
                }
                _ => {}
            }
            callback_movement(
                MovementBuilder {
                    src: Square::from_address(SQUARE_DROP), // 駒台
                    dst: destination.clone(),               // どの升へ行きたいか
                    pro: false,                             // 打に成りは無し
                    drop: num_to_piece_type(
                        speed_of_light
                            .get_piece_type_struct_from_piece_type(&piece_type_dst)
                            .serial_piece_number,
                    ), // 打った駒種類
                }
                .to_hash(speed_of_light),
            );
        }
    }
}
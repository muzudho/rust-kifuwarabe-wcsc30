//! Static exchange evaluation

// use crate::entities::cosmic::daydream::Value;
// use crate::entities::cosmic::playing::Game;
// use crate::cosmic::smart::square::RelAdr;
// use crate::law::generate_move::PieceEx;
// use crate::law::speed_of_light::Movility;
// use crate::spaceship::equipment::Beam;

// pub struct SEE {}
// impl SEE {
//     /// 葉で駒を取ったら、取り返されるのも考慮しないとな☆（＾～＾）
//     ///
//     /// 価値の低い駒から順に使って、取りに行けだぜ☆（＾～＾）
//     pub fn go(_game: &Game, _adr: &Square) -> Value {
//         // // この駒☆（＾～＾）
//         // let mut current_target_piece = game.position.piece_at(adr).unwrap();
//         // let mut centi_pawn = 0;

//         // // この駒の西に相手の駒があって、それが この駒に利いているなら、取りにくると思おうぜ☆（＾～＾）
//         // let recipes = [
//         //     (RelAdr::new(1, 0), Movility::SideBack),          // 西
//         //     (RelAdr::new(1, 1), Movility::BackDiagonally),    // 南西
//         //     (RelAdr::new(0, 1), Movility::SideBack),          // 南
//         //     (RelAdr::new(-1, 1), Movility::BackDiagonally),   // 南東
//         //     (RelAdr::new(-1, 0), Movility::SideBack),         // 東
//         //     (RelAdr::new(-1, -1), Movility::FrontDiagonally), // 北東
//         //     (RelAdr::new(0, -1), Movility::Front),            // 北
//         //     (RelAdr::new(1, -1), Movility::FrontDiagonally),  // 北西
//         //                                                       // TODO 飛び利きにも対応したいぜ☆（＾～＾）
//         // ];
//         // // 移動先升に利きのある駒が無くなるまで繰り返すぜ☆（＾～＾）
//         // for i in 0..102 {
//         //     let mut next_target_piece = None;
//         //     // TODO 相手の駒が、自分の駒を取る全てのケースを一覧しろだぜ☆（＾～＾）
//         //     let attackers = Vec::<PieceEx>::new();
//         //     for recipe in &recipes {
//         //         let mut cur = adr.clone();
//         //         if cur.offset(&recipe.0).legal_cur() {
//         //             let pc_ex = game.position.piece_at(&cur);
//         //             if let Some(piece_val) = pc_ex {
//         //                 if piece_val.piece.phase() != current_target_piece.piece.phase()
//         //                     || piece_val.piece.type_().movility().contains(&recipe.1)
//         //                 {
//         //                     // 敵の駒も西に動けるんだったら、利かされているぜ☆（＾～＾）
//         //                     // 自分の駒が取られるということだぜ☆（＾～＾）
//         //                     attackers.push(piece_val);
//         //                 }
//         //             }
//         //         }
//         //     }

//         //     for recipe in &recipes {
//         //         let mut cur = adr.clone();
//         //         if cur.offset(&recipe.0).legal_cur() {
//         //             let pc_ex = game.position.piece_at(&cur);
//         //             if let Some(piece_val) = pc_ex {
//         //                 if piece_val.piece.phase() != current_target_piece.piece.phase()
//         //                     || piece_val.piece.type_().movility().contains(&recipe.1)
//         //                 {
//         //                     // 敵の駒も西に動けるんだったら、利かされているぜ☆（＾～＾）
//         //                     // 自分の駒が取られるということだぜ☆（＾～＾）
//         //                     centi_pawn -= current_target_piece
//         //                         .piece
//         //                         .hand_piece()
//         //                         .type_()
//         //                         .captured_value();
//         //                     next_target_piece = pc_ex;
//         //                     game.do_move();
//         //                     break;
//         //                 }
//         //             }
//         //         }
//         //     }
//         //     if let Some(pc_ex) = next_target_piece {
//         //         current_target_piece = pc_ex;
//         //     } else {
//         //         break;
//         //     }

//         //     if 100 < i {
//         //         std::panic::panic_any(Beam::trouble(
//         //             "(Err.61) SEEが終わらんぜ☆（＾～＾）バグってんじゃねーの☆（＾～＾）？"
//         //         ))
//         //     }
//         // }
//         // Value::CentiPawn(centi_pawn)
//         Value::CentiPawn(0)
//     }
// }

//!
//! USIプロトコル
//!
use super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_constants_vo::*;
use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::model::vo::other_part::op_square_vo::*;

/**
 * 指し手読取
 * 例: 7g7f
 *
 * 読み取った指し手は、棋譜に入れる。
 * 現在の手目のところに入れ、手目のカウントアップも行う。
 */
pub fn read_sasite(line: &String, starts: &mut usize, len: usize, ml_dto: &mut MLDto) -> bool {
    // 4文字か5文字あるはず。
    if (len - *starts) < 4 {
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return false;
    }

    // 1文字目と2文字目
    match &line[*starts..(*starts + 1)] {
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::K);
        }
        "B" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::Z);
        }
        "G" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::I);
        }
        "S" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::N);
        }
        "N" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::U);
        }
        "L" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::S);
        }
        "P" => {
            *starts += 2;
            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_umasu(0));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::H);
        }
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            let suji;
            let dan;
            match &line[*starts..(*starts + 1)] {
                "1" => {
                    suji = 1;
                    *starts += 1;
                }
                "2" => {
                    suji = 2;
                    *starts += 1;
                }
                "3" => {
                    suji = 3;
                    *starts += 1;
                }
                "4" => {
                    suji = 4;
                    *starts += 1;
                }
                "5" => {
                    suji = 5;
                    *starts += 1;
                }
                "6" => {
                    suji = 6;
                    *starts += 1;
                }
                "7" => {
                    suji = 7;
                    *starts += 1;
                }
                "8" => {
                    suji = 8;
                    *starts += 1;
                }
                "9" => {
                    suji = 9;
                    *starts += 1;
                }
                _ => {
                    g_writeln(&format!("(1) '{}' だった。", &line[*starts..(*starts + 1)]));
                    return false;
                }
            }

            match &line[*starts..(*starts + 1)] {
                "a" => {
                    dan = 1;
                    *starts += 1;
                }
                "b" => {
                    dan = 2;
                    *starts += 1;
                }
                "c" => {
                    dan = 3;
                    *starts += 1;
                }
                "d" => {
                    dan = 4;
                    *starts += 1;
                }
                "e" => {
                    dan = 5;
                    *starts += 1;
                }
                "f" => {
                    dan = 6;
                    *starts += 1;
                }
                "g" => {
                    dan = 7;
                    *starts += 1;
                }
                "h" => {
                    dan = 8;
                    *starts += 1;
                }
                "i" => {
                    dan = 9;
                    *starts += 1;
                }
                _ => {
                    g_writeln(&format!("(2) '{}' だった。", &line[*starts..(*starts + 1)]));
                    return false;
                }
            }

            ml_dto
                .get_search_part_mut()
                .set_move_src(&Square::from_file_rank(suji, dan));
            ml_dto.get_search_part_mut().set_move_drop(PieceType::Kara);
        }
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。
    let suji;
    let dan;

    // 3文字目
    match &line[*starts..(*starts + 1)] {
        "1" => {
            suji = 1;
            *starts += 1;
        }
        "2" => {
            suji = 2;
            *starts += 1;
        }
        "3" => {
            suji = 3;
            *starts += 1;
        }
        "4" => {
            suji = 4;
            *starts += 1;
        }
        "5" => {
            suji = 5;
            *starts += 1;
        }
        "6" => {
            suji = 6;
            *starts += 1;
        }
        "7" => {
            suji = 7;
            *starts += 1;
        }
        "8" => {
            suji = 8;
            *starts += 1;
        }
        "9" => {
            suji = 9;
            *starts += 1;
        }
        _ => {
            g_writeln(&format!("(3) '{}' だった。", &line[*starts..(*starts + 1)]));
            return false;
        }
    }
    // 4文字目
    match &line[*starts..(*starts + 1)] {
        "a" => {
            dan = 1;
            *starts += 1;
        }
        "b" => {
            dan = 2;
            *starts += 1;
        }
        "c" => {
            dan = 3;
            *starts += 1;
        }
        "d" => {
            dan = 4;
            *starts += 1;
        }
        "e" => {
            dan = 5;
            *starts += 1;
        }
        "f" => {
            dan = 6;
            *starts += 1;
        }
        "g" => {
            dan = 7;
            *starts += 1;
        }
        "h" => {
            dan = 8;
            *starts += 1;
        }
        "i" => {
            dan = 9;
            *starts += 1;
        }
        _ => {
            g_writeln(&format!("(4) '{}' だった。", &line[*starts..(*starts + 1)]));
            return false;
        }
    }

    ml_dto
        .get_search_part_mut()
        .set_move_dst(&Square::from_file_rank(suji, dan));
    // 5文字に「+」があれば成り。
    if 0 < (len - *starts) && &line[*starts..(*starts + 1)] == "+" {
        ml_dto.get_search_part_mut().set_move_pro(true);
        *starts += 1;
    } else {
        ml_dto.get_search_part_mut().set_move_pro(false);
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (len - *starts) && &line[*starts..(*starts + 1)] == " " {
        *starts += 1;
    }

    ml_dto.get_search_part_mut().add_ply(1);
    true
}

/**
 * position コマンド 盤上部分のみ 読取
 */
pub fn read_banjo(
    line: &String,
    starts: &mut usize,
    len: usize,
    ml_dto: &mut MLDto,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // 盤部
    let mut suji = SUJI_9; //９筋から右方向へ読取
    let mut dan = DAN_1;
    'ban: while 0 < (len - *starts) {
        match &line[*starts..(*starts + 1)] {
            "/" => {
                *starts += 1;
                suji = SUJI_9;
                dan += 1;
            }
            "1" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
            }
            "2" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
            }
            "3" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                suji -= 1;
            }
            "4" => {
                *starts += 1;
                for _i_kara in 0..4 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "5" => {
                *starts += 1;
                for _i_kara in 0..5 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "6" => {
                *starts += 1;
                for _i_kara in 0..6 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "7" => {
                *starts += 1;
                for _i_kara in 0..7 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "8" => {
                *starts += 1;
                for _i_kara in 0..8 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "9" => {
                *starts += 1;
                for _i_kara in 0..9 {
                    ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Kara);
                    suji -= 1;
                }
            }
            "K" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::King1);
                suji -= 1;
            }
            "R" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Rook1);
                suji -= 1;
            }
            "B" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Bishop1);
                suji -= 1;
            }
            "G" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Gold1);
                suji -= 1;
            }
            "S" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Silver1);
                suji -= 1;
            }
            "N" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Knight1);
                suji -= 1;
            }
            "L" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Lance1);
                suji -= 1;
            }
            "P" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Pawn1);
                suji -= 1;
            }
            "k" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::King2);
                suji -= 1;
            }
            "r" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Rook2);
                suji -= 1;
            }
            "b" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Bishop2);
                suji -= 1;
            }
            "g" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Gold2);
                suji -= 1;
            }
            "s" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Silver2);
                suji -= 1;
            }
            "n" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Knight2);
                suji -= 1;
            }
            "l" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Lance2);
                suji -= 1;
            }
            "p" => {
                *starts += 1;
                ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::Pawn2);
                suji -= 1;
            }
            "+" => {
                *starts += 1;
                match &line[*starts..(*starts + 1)] {
                    "R" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedRook1);
                        suji -= 1;
                    }
                    "B" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedBishop1,
                        );
                        suji -= 1;
                    }
                    "S" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedSilver1,
                        );
                        suji -= 1;
                    }
                    "N" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedKnight1,
                        );
                        suji -= 1;
                    }
                    "L" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedLance1);
                        suji -= 1;
                    }
                    "P" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedPawn1);
                        suji -= 1;
                    }
                    "r" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedRook2);
                        suji -= 1;
                    }
                    "b" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedBishop2,
                        );
                        suji -= 1;
                    }
                    "s" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedSilver2,
                        );
                        suji -= 1;
                    }
                    "n" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(
                            suji,
                            dan,
                            OPPieceVo::PromotedKnight2,
                        );
                        suji -= 1;
                    }
                    "l" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedLance2);
                        suji -= 1;
                    }
                    "p" => {
                        *starts += 1;
                        ml_dto.set_piece_to_starting_position(suji, dan, OPPieceVo::PromotedPawn2);
                        suji -= 1;
                    }
                    _ => {
                        g_writeln(&format!(
                            "盤部(0) '{}' だった。",
                            &line[*starts..(*starts + 1)]
                        ));
                        break 'ban;
                    }
                }
            }
            _ => {
                break 'ban;
            } // 盤部正常終了
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = ml_dto.create_starting_position_hash(speed_of_light);
    ml_dto
        .get_application_part_mut()
        .set_starting_position_hash(ky_hash);
}

/**
 * position コマンド読取
 */
pub fn read_position(line: &String, ml_dto: &mut MLDto, speed_of_light: &MLSpeedOfLightVo) {
    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();

    // 局面をクリアー。手目も 0 に戻します。
    ml_dto.clear_all_positions();

    if 16 < (len - starts) && &line[starts..(starts + 17)] == "position startpos" {
        // 'position startpos' を読み飛ばし
        starts += 17;
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;
        read_banjo(
            &STARTPOS.to_string(),
            &mut local_starts,
            STARTPOS_LN,
            ml_dto,
            speed_of_light,
        );

        if 0 < (len - starts) && &line[starts..(starts + 1)] == " " {
            // ' ' を読み飛ばした。
            starts += 1;
        }
    } else if 13 < (len - starts) && &line[starts..(starts + 14)] == "position sfen " {
        starts += 14; // 'position sfen ' を読み飛ばし
        read_banjo(line, &mut starts, len, ml_dto, speed_of_light);

        if 0 < (len - starts) && &line[starts..(starts + 1)] == " " {
            starts += 1;
        }

        if 0 < (len - starts) && &line[starts..(starts + 1)] == "w" {
            starts += 1;
        } else if 0 < (len - starts) && &line[starts..(starts + 1)] == "b" {
            starts += 1;
        }

        if 0 < (len - starts) && &line[starts..(starts + 1)] == " " {
            starts += 1;
        }

        // 持ち駒の読取
        if 0 < (len - starts) && &line[starts..(starts + 1)] == "-" {
            starts += 1;
        } else {
            'mg: loop {
                if 0 < (len - starts) {
                    let mut maisu = 1;
                    match &line[starts..(starts + 1)] {
                        "1" => {
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            match &line[starts..(starts + 1)] {
                                "0" => {
                                    maisu = 10;
                                    starts += 2;
                                }
                                "1" => {
                                    maisu = 11;
                                    starts += 2;
                                }
                                "2" => {
                                    maisu = 12;
                                    starts += 2;
                                }
                                "3" => {
                                    maisu = 13;
                                    starts += 2;
                                }
                                "4" => {
                                    maisu = 14;
                                    starts += 2;
                                }
                                "5" => {
                                    maisu = 15;
                                    starts += 2;
                                }
                                "6" => {
                                    maisu = 16;
                                    starts += 2;
                                }
                                "7" => {
                                    maisu = 17;
                                    starts += 2;
                                }
                                "8" => {
                                    maisu = 18;
                                    starts += 2;
                                }
                                _ => {
                                    g_writeln(&format!(
                                        "持駒部(0) '{}' だった。",
                                        &line[starts..(starts + 2)]
                                    ));
                                    return;
                                }
                            }
                        }
                        "2" => {
                            maisu = 2;
                            starts += 1;
                        }
                        "3" => {
                            maisu = 3;
                            starts += 1;
                        }
                        "4" => {
                            maisu = 4;
                            starts += 1;
                        }
                        "5" => {
                            maisu = 5;
                            starts += 1;
                        }
                        "6" => {
                            maisu = 6;
                            starts += 1;
                        }
                        "7" => {
                            maisu = 7;
                            starts += 1;
                        }
                        "8" => {
                            maisu = 8;
                            starts += 1;
                        }
                        "9" => {
                            maisu = 9;
                            starts += 1;
                        }
                        _ => {} // 駒の名前か、エラーなら次へ
                    }

                    use super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo::*;
                    let km: OPPieceVo;
                    match &line[starts..(starts + 1)] {
                        "R" => {
                            km = Rook1;
                            starts += 1;
                        }
                        "B" => {
                            km = Bishop1;
                            starts += 1;
                        }
                        "G" => {
                            km = Gold1;
                            starts += 1;
                        }
                        "S" => {
                            km = Silver1;
                            starts += 1;
                        }
                        "N" => {
                            km = Knight1;
                            starts += 1;
                        }
                        "L" => {
                            km = Lance1;
                            starts += 1;
                        }
                        "P" => {
                            km = Pawn1;
                            starts += 1;
                        }
                        "r" => {
                            km = Rook2;
                            starts += 1;
                        }
                        "b" => {
                            km = Bishop2;
                            starts += 1;
                        }
                        "g" => {
                            km = Gold2;
                            starts += 1;
                        }
                        "s" => {
                            km = Silver2;
                            starts += 1;
                        }
                        "n" => {
                            km = Knight2;
                            starts += 1;
                        }
                        "l" => {
                            km = Lance2;
                            starts += 1;
                        }
                        "p" => {
                            km = Pawn2;
                            starts += 1;
                        }
                        _ => {
                            break 'mg;
                        } // 持駒部 正常終了
                    }

                    ml_dto.set_starting_position_hand_piece(km, maisu);
                } //if
            } //loop
        } //else

        if 2 < (len - starts) && &line[starts..(starts + 3)] == " 1 " {
            starts += 3;
        }
    } else {
        g_writeln("'position startpos' でも、'position sfen ' でも始まらなかった。");
        return;
    }

    if 4 < (len - starts) && &line[starts..(starts + 5)] == "moves" {
        starts += 5;
    }

    if 0 < (len - starts) && &line[starts..(starts + 1)] == " " {
        starts += 1;
    }

    // 初期局面を、現局面にコピーします
    ml_dto.copy_starting_position_to_current_position();

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(line, &mut starts, len, ml_dto) {
        // 手目を戻す
        ml_dto.get_search_part_mut().add_ply(-1);
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let ply = ml_dto.get_search_part().get_ply();
        ml_dto.do_ss(
            &ml_dto.get_search_part().get_moves_history()[ply as usize].clone(),
            speed_of_light,
        );

        // 現局面表示
        //let s1 = &ml_dto.kaku_ky( &KyNums::Current );
        //g_writeln( &s1 );
    }
}

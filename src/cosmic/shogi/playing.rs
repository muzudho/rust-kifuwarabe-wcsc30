use crate::cosmic::shogi::recording::{History, Movement};
use crate::cosmic::shogi::state::{Person, PHASE_FIRST, PHASE_LN, PHASE_SECOND};
use crate::cosmic::smart::square::{
    isquare, AbsoluteAddress, BOARD_MEMORY_AREA, SQUARE_DROP, SQUARE_NONE,
};
use crate::cosmic::toy_box::{Board, Piece, MG_MAX, PIECE_LN};
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Info;
use rand::Rng;

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub piece: [[u64; PIECE_LN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hand: [[u64; MG_MAX]; PIECE_LN],
    // 先後
    pub phase: [u64; PHASE_LN],
}

pub struct Game {
    /// 棋譜
    pub history: History,
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 初期局面
    pub starting_board: Board,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現局面
    pub current_board: Board,
    /// 情報表示担当
    pub info: Info,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            history: History::default(),
            starting_position_hash: 0,
            starting_board: Board::default(),
            hash_seed: GameHashSeed {
                // 盤上の駒
                piece: [[0; PIECE_LN]; BOARD_MEMORY_AREA as usize],
                // 持ち駒
                hand: [[0; MG_MAX]; PIECE_LN],
                // 先後
                phase: [0; PHASE_LN],
            },
            current_board: Board::default(),
            info: Info::default(),
        }
    }
}
impl Game {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_square in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.hash_seed.piece[i_square as usize][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_km in 0..PIECE_LN {
            for i_mg in 0..MG_MAX {
                self.hash_seed.hand[i_km][i_mg] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
            self.hash_seed.phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    /// 棋譜の作成
    pub fn set_current_movement(&mut self, movement: &Movement) {
        self.history.movements[self.history.ply as usize] = movement.clone()
    }
    pub fn get_move(&self) -> &Movement {
        &self.history.movements[self.history.ply as usize]
    }
    /// 棋譜☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.history.ply {
            let movement = &self.history.movements[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    pub fn set_current_position_hash(&mut self, hash: u64) {
        self.history.position_hashs[self.history.ply as usize] = hash;
    }
    pub fn set_captured(&mut self, ply1: usize, pc: Option<Piece>) {
        self.history.captured_pieces[ply1] = pc
    }

    pub fn get_board(&self, num: &PosNums) -> &Board {
        match *num {
            PosNums::Current => &self.current_board,
            PosNums::Start => &self.starting_board,
        }
    }
    pub fn get_mut_starting_board(&mut self) -> &mut Board {
        &mut self.starting_board
    }

    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear(&mut self) {
        self.starting_board.clear();
        self.current_board.clear();
        self.history.ply = 0;
    }

    /// 開始局面を、現局面にコピーします
    pub fn copy_starting_position_to_current_position(&mut self) {
        // 盤上の駒。
        for i_adr in 0..BOARD_MEMORY_AREA {
            let i_sq = AbsoluteAddress::from_address(i_adr as isquare);
            // TODO 取得→設定　するとエラーになってしまうので、今んとこ 作成→設定　するぜ☆（＾～＾）
            let piece = self.starting_board.piece_at(&i_sq);
            self.current_board.set_piece_at(&i_sq, piece);
        }

        // 持ち駒
        self.current_board.hand[..PIECE_LN].clone_from_slice(&self.starting_board.hand[..PIECE_LN]);
    }

    /// 局面ハッシュ。
    pub fn get_all_position_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.starting_position_hash));

        for ply in 0..self.history.ply {
            let hash = &self.history.position_hashs[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 初期局面ハッシュを作り直す
    pub fn create_starting_position_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self.starting_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.hash_seed.phase[PHASE_SECOND];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_current_position_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self.current_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use crate::cosmic::shogi::state::Phase::*;
        match self.history.get_phase(Person::Friend) {
            First => hash ^= self.hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.hash_seed.phase[PHASE_SECOND],
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> i8 {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
            if self.history.position_hashs[t as usize]
                == self.history.position_hashs[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.starting_position_hash == self.history.position_hashs[last_ply as usize] {
            count += 1;
        }

        count
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// Captured piece.
    pub fn do_move(&mut self, movement: &Movement, speed_of_light: &SpeedOfLight) -> Option<Piece> {
        // もう入っているかも知れないが、棋譜に入れる☆
        self.set_current_movement(movement);
        let friend = self.history.get_phase(Person::Friend);

        // 取った駒
        let cap: Option<Piece>;
        {
            // 動かす駒
            let moveing_piece: Option<Piece> = if movement.source.address == SQUARE_DROP {
                // 打なら
                // 自分の持ち駒を減らす
                if let Some(drp) = movement.drop {
                    let piece734 = Piece::from_phase_and_piece_type(friend, drp);
                    self.current_board.add_hand(&piece734, -1, speed_of_light);
                    Some(piece734)
                } else {
                    panic!("打なのに駒を指定してないぜ☆（＾～＾）");
                }
            } else {
                // 打でなければ、元の升に駒はあるので、それを消す。
                let piece152 = if movement.promote {
                    // 成りなら
                    if let Some(pc) = self.current_board.piece_at(&movement.source) {
                        // 成り駒をクローン。
                        Some(speed_of_light.piece_chart(&pc).promoted)
                    } else {
                        panic!("成ったのに、元の升に駒がなかった☆（＾～＾）");
                    }
                } else {
                    // 移動元の駒をクローン。
                    self.current_board.piece_at(&movement.source).clone()
                };

                // 移動元を空に。
                self.current_board.set_piece_at(&movement.source, None);

                piece152
            };

            // 移動先升に駒があるかどうか
            cap = if let Some(_) = self.current_board.piece_at(&movement.destination) {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let cap_o764 = { self.current_board.piece_at(&movement.destination) };

                if let Some(cap764) = cap_o764 {
                    let cap773 = speed_of_light.piece_chart(&cap764).captured;
                    self.current_board.add_hand(&cap773, 1, speed_of_light);
                };
                cap_o764
            } else {
                None
            };

            // 移動先升に駒を置く
            self.current_board
                .set_piece_at(&movement.destination, moveing_piece);
        }
        self.set_captured(self.history.ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_current_position_hash(speed_of_light);
        self.set_current_position_hash(ky_hash);

        self.history.ply += 1;
        cap
    }

    pub fn undo_move(&mut self, speed_of_light: &SpeedOfLight) -> bool {
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let movement = &self.get_move().clone();
            {
                let phase = self.history.get_phase(Person::Friend);
                // 取った駒が有ったか。
                let cap_o: Option<Piece> = self.history.captured_pieces[self.history.ply as usize];
                // 動いた駒
                let old_source391_o: Option<Piece> = if movement.source.address == SQUARE_DROP {
                    // 打なら
                    if let Some(drp) = movement.drop {
                        let drop394 = Piece::from_phase_and_piece_type(phase, drp);
                        // 自分の持ち駒を増やす
                        //let mg = km_to_mg(km);
                        //self.add_hand(mg,1);
                        self.current_board.add_hand(&drop394, 1, speed_of_light);
                        Some(drop394)
                    } else {
                        panic!("打なのに駒を指定していないぜ☆（＾～＾）！")
                    }
                } else {
                    // 打でなければ
                    if movement.promote {
                        // 成ったなら、成る前へ
                        if let Some(source409) = self.current_board.piece_at(&movement.destination)
                        {
                            Some(speed_of_light.piece_chart(&source409).demoted)
                        } else {
                            panic!("成ったのに移動先に駒が無いぜ☆（＾～＾）！")
                        }
                    } else {
                        self.current_board.piece_at(&movement.destination).clone()
                    }
                };

                // 移動先の駒を、取った駒（あるいは空）に戻す
                self.current_board
                    .set_piece_at(&movement.destination, cap_o);

                if let Some(cap) = cap_o {
                    let captured = speed_of_light.piece_chart(&cap).captured;
                    // 自分の持ち駒を減らす
                    self.current_board.add_hand(&captured, -1, speed_of_light);
                }
                // 移動元升に、動かした駒を置く
                self.current_board
                    .set_piece_at(&movement.source, old_source391_o);
            }
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}

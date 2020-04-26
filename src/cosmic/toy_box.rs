//!
//! 駒 と 盤
//!
use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Person, Phase};
use crate::cosmic::smart::features::HAND_ADDRESS_LN;
use crate::cosmic::smart::features::HAND_ADDRESS_TYPE_LEN;
use crate::cosmic::smart::features::{
    HandAddress, HandAddresses, PieceMeaning, PieceType, HAND_MAX,
};
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, BOARD_MEMORY_AREA, FILE_0, FILE_1, FILE_10, RANK_0, RANK_1, RANK_10,
};
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// 背番号付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒に背番号を付けたものだぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum PieceNum {
    // 1 先手玉
    King1,
    // 2 後手玉
    King2,
    // 3 金
    Gold3,
    // 4 金
    Gold4,
    // 5 金
    Gold5,
    // 6 金
    Gold6,
    // 7 銀
    Silver7,
    // 8 銀
    Silver8,
    // 9 銀
    Silver9,
    // 10 銀
    Silver10,
    // 11 桂
    Knight11,
    // 12 桂
    Knight12,
    // 13 桂
    Knight13,
    // 14 桂
    Knight14,
    // 15 香
    Lance15,
    // 16 香
    Lance16,
    // 17 香
    Lance17,
    // 18 香
    Lance18,
    // 19 角
    Bishop19,
    // 20 角
    Bishop20,
    // 21 飛
    Rook21,
    // 22 飛
    Rook22,
    // 23 歩
    Pawn23,
    // 24 歩
    Pawn24,
    // 25 歩
    Pawn25,
    // 26 歩
    Pawn26,
    // 27 歩
    Pawn27,
    // 28 歩
    Pawn28,
    // 29 歩
    Pawn29,
    // 30 歩
    Pawn30,
    // 31 歩
    Pawn31,
    // 32 歩
    Pawn32,
    // 33 歩
    Pawn33,
    // 34 歩
    Pawn34,
    // 35 歩
    Pawn35,
    // 36 歩
    Pawn36,
    // 37 歩
    Pawn37,
    // 38 歩
    Pawn38,
    // 39 歩
    Pawn39,
    // 40 歩
    Pawn40,
}

#[derive(Clone, Copy)]
pub enum Location {
    Board(AbsoluteAddress),
    Hand(HandAddress),
    // 作業中のときは、これだぜ☆（＾～＾）
    Busy,
}

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct Board {
    pieces: [Option<(PieceMeaning, PieceNum)>; BOARD_MEMORY_AREA as usize],
    /// 駒の居場所☆（＾～＾）
    location: [Location; PIECE_NUM_LEN],
    hand_index: [usize; HAND_ADDRESS_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: [Vec<(PieceMeaning, PieceNum)>; HAND_ADDRESS_LN],
    /// 指し手生成でその升に移動したら、先手なら＋１、後手なら－１しろだぜ☆（＾～＾）葉で得点化するぜ☆（＾～＾）
    pub control: [i16; BOARD_MEMORY_AREA as usize],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            // 盤上
            pieces: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            location: [Location::Busy; PIECE_NUM_LEN],
            hand_index: [
                PieceNum::King1 as usize,
                PieceNum::Rook21 as usize,
                PieceNum::Bishop19 as usize,
                PieceNum::Gold3 as usize,
                PieceNum::Silver7 as usize,
                PieceNum::Knight11 as usize,
                PieceNum::Lance15 as usize,
                PieceNum::Pawn23 as usize,
            ],
            // 持ち駒
            hands: [
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
                Vec::<(PieceMeaning, PieceNum)>::new(),
            ],
            control: [0; BOARD_MEMORY_AREA as usize],
        }
    }
}
impl Board {
    pub fn clear(&mut self) {
        self.pieces = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.location = [Location::Busy; PIECE_NUM_LEN];
        self.hand_index = [
            PieceNum::King1 as usize,
            PieceNum::Rook21 as usize,
            PieceNum::Bishop19 as usize,
            PieceNum::Gold3 as usize,
            PieceNum::Silver7 as usize,
            PieceNum::Knight11 as usize,
            PieceNum::Lance15 as usize,
            PieceNum::Pawn23 as usize,
        ];
        // 持ち駒☆（＾～＾）
        self.hands = [
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
            Vec::<(PieceMeaning, PieceNum)>::new(),
        ];
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, board: &Board) {
        self.pieces = board.pieces.clone();
        self.location = board.location.clone();
        self.hand_index = board.hand_index.clone();
        self.hands = board.hands.clone();
        self.control = board.control.clone();
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(
        &self,
        phase: Phase,
        file: i8,
        speed_of_light: &SpeedOfLight,
    ) -> bool {
        for rank in RANK_1..RANK_10 {
            let adr = Address::new(file, rank).abs();
            if let Some(piece) = self.piece_at(&adr) {
                if piece.0.phase(speed_of_light) == phase
                    && piece.0.r#type(speed_of_light) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at(&self, adr: &AbsoluteAddress) -> Option<(PieceMeaning, PieceNum)> {
        self.pieces[adr.address() as usize]
    }
    /// 升で指定して駒を置く
    pub fn push_to_board(
        &mut self,
        adr: &AbsoluteAddress,
        piece: Option<(PieceMeaning, PieceNum)>,
    ) {
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = piece;
            self.location[piece_val.1 as usize] = Location::Board(*adr);
        } else {
            self.pieces[adr.address() as usize] = None;
        }
    }
    /*
    /// TODO push_piece 升で指定して駒を置く
    pub fn set_piece_at(&mut self, adr: &AbsoluteAddress, piece: Option<(PieceMeaning, PieceNum)>) {
        if let Some(_x) = piece {
            self.pieces[adr.address() as usize] = piece;
        } else {
            self.pieces[adr.address() as usize] = None;
        }
    }
    */
    /// 盤上から駒を無くし、その駒を返り値で返すぜ☆（＾～＾）
    pub fn pop_from_board(&mut self, adr: &AbsoluteAddress) -> Option<(PieceMeaning, PieceNum)> {
        // 取り出すピースは複製するぜ☆（＾～＾）
        let piece = self.pieces[adr.address() as usize].clone();
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = None;
            self.location[piece_val.1 as usize] = Location::Busy;
        }
        piece
    }
    /// 盤に駒か空升を置いていきます。
    pub fn push_piece_on_init(
        &mut self,
        file: u8,
        rank: u8,
        piece: Option<PieceMeaning>,
        speed_of_light: &SpeedOfLight,
    ) {
        if !((FILE_0 as u8) < file
            && file < FILE_10 as u8
            && (RANK_0 as u8) < rank
            && rank < RANK_10 as u8)
        {
            panic!(Beam::trouble(&format!(
                "(Err.323) 盤上の初期化で盤の外を指定するのは止めろだぜ☆（＾～＾）！ ({}, {})",
                file, rank
            )))
        }

        if let Some(piece_meaning) = piece {
            let source = Address::new(file as i8, rank as i8).abs();
            let piece_num = match piece_meaning {
                PieceMeaning::King1 => {
                    self.location[PieceNum::King1 as usize] = Location::Board(source);
                    PieceNum::King1
                }
                PieceMeaning::King2 => {
                    self.location[PieceNum::King2 as usize] = Location::Board(source);
                    PieceNum::King2
                }
                _ => {
                    let hand_type = piece_meaning
                        .hand_address(speed_of_light)
                        .r#type(speed_of_light);
                    self.location[self.hand_index[hand_type as usize]] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.hand_index[hand_type as usize]).unwrap();
                    self.hand_index[hand_type as usize] += 1;
                    pn
                }
            };
            self.push_to_board(
                &Address::new(file as i8, rank as i8).abs(),
                Some((piece_meaning, piece_num)),
            );
        }
    }
    /// 駒台に置く
    pub fn push_hand_on_init(
        &mut self,
        piece_meaning: PieceMeaning,
        number: i8,
        speed_of_light: &SpeedOfLight,
    ) {
        for _i in 0..number {
            let adr = piece_meaning.hand_address(speed_of_light);
            let hand_type = piece_meaning
                .hand_address(speed_of_light)
                .r#type(speed_of_light);
            let cursor = self.hand_index[hand_type as usize];
            self.location[cursor] = Location::Hand(adr);
            self.hands[cursor].push((piece_meaning, PieceNum::from_usize(cursor).unwrap()));
            self.hand_index[hand_type as usize] += 1;
        }
    }
    pub fn push_hand(&mut self, hand: &(PieceMeaning, PieceNum), speed_of_light: &SpeedOfLight) {
        let adr = hand.0.hand_address(speed_of_light);
        self.hands[adr as usize].push(*hand);
        self.location[hand.1 as usize] = Location::Hand(adr);
    }
    pub fn pop_hand(
        &mut self,
        hand: PieceMeaning,
        speed_of_light: &SpeedOfLight,
    ) -> Option<(PieceMeaning, PieceNum)> {
        let adr = hand.hand_address(speed_of_light);
        let piece = self.hands[adr as usize].pop();
        self.location[piece.unwrap().1 as usize] = Location::Busy;
        piece
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, adr: HandAddress) -> Option<&(PieceMeaning, PieceNum)> {
        self.hands[adr as usize].last()
    }
    pub fn count_hand(&self, adr: HandAddress) -> usize {
        self.hands[adr as usize].len()
    }

    /// 升には何がありますか？
    pub fn what_is_in_the_square(
        &self,
        phase: Phase,
        adr: &AbsoluteAddress,
        speed_of_light: &SpeedOfLight,
    ) -> Option<Person> {
        // TODO 範囲外チェックは？行わない？
        if let Some(piece) = self.piece_at(&adr) {
            if piece.0.phase(speed_of_light) == phase {
                return Some(Person::Friend);
            }
            Some(Person::Opponent)
        } else {
            None
        }
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, game: &Game, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let ab_adr = &Address::new(file, rank).abs();
                if let Some(piece) = self.piece_at(ab_adr) {
                    hash ^= game.hash_seed.piece[ab_adr.address() as usize]
                        [piece.0.serial_number(speed_of_light)];
                }
            }
        }

        // 持ち駒ハッシュ
        HandAddresses::for_all(&mut |adr| {
            let count = self.count_hand(adr);
            debug_assert!(
                count <= HAND_MAX,
                "持ち駒 {:?} の枚数 {} <= {}",
                adr,
                count,
                HAND_MAX
            );
            hash ^= game.hash_seed.hands[adr as usize][count as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }

    /// 良ければ総量はプラスだぜ☆（＾～＾）
    pub fn control_value(&self) -> i16 {
        self.control.iter().sum()
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_board<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress>, Option<(PieceMeaning, PieceNum)>),
    {
        for (i, location) in self.location.iter().enumerate() {
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(adr).unwrap();
                    piece_get(i, Some(adr), Some(piece));
                }
                Location::Hand(_adr) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.624) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(
        &self,
        friend: Phase,
        speed_of_light: &SpeedOfLight,
        piece_get: &mut F,
    ) where
        F: FnMut(Location, (PieceMeaning, PieceNum)),
    {
        for location in self.location.iter() {
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(adr).unwrap();
                    if piece.0.phase(speed_of_light) == friend {
                        piece_get(*location, piece);
                    }
                }
                Location::Hand(_adr) => {
                    // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.650) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }

        let list = match friend {
            Phase::First => [
                HandAddress::Rook1,
                HandAddress::Bishop1,
                HandAddress::Gold1,
                HandAddress::Silver1,
                HandAddress::Knight1,
                HandAddress::Lance1,
                HandAddress::Pawn1,
            ],
            Phase::Second => [
                HandAddress::Rook2,
                HandAddress::Bishop2,
                HandAddress::Gold2,
                HandAddress::Silver2,
                HandAddress::Knight2,
                HandAddress::Lance2,
                HandAddress::Pawn2,
            ],
        };
        for adr in &list {
            if let Some(piece) = self.last_hand(*adr) {
                piece_get(Location::Hand(*adr), *piece);
            }
        }
    }

    pub fn location_of(&self, piece_num: PieceNum) -> Location {
        self.location[piece_num as usize]
    }
}

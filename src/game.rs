#[derive(Copy, Clone, PartialEq)]
enum PieceType {
    King,   // 王
    Rook,   // 飛
    Dragon, // 龍
    Bishop, // 角
    Horse,  // 馬
    Gold,   // 金
    Silver, // 銀
    Knight, // 桂
    Lance,  // 香
    Pawn,   // 歩
    None,   // 駒なし
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let character = match self {
            PieceType::King => "王",
            PieceType::Rook => "飛",
            PieceType::Dragon => "龍",
            PieceType::Bishop => "角",
            PieceType::Horse => "馬",
            PieceType::Gold => "金",
            PieceType::Silver => "銀",
            PieceType::Knight => "桂",
            PieceType::Lance => "香",
            PieceType::Pawn => "歩",
            _ => "",
        };

        write!(f, "{}", character)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    First,   // 先手
    Second,  // 後手
    Neutral, // 中立
}

#[derive(Copy, Clone)]
struct Piece {
    piece: PieceType, // 駒
    owner: Player,    // 所有者
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let character = match self.piece {
            PieceType::King => {
                if self.owner == Player::First {
                    "王"
                } else {
                    "玉"
                }
            }
            PieceType::Rook => "飛",
            PieceType::Dragon => "龍",
            PieceType::Bishop => "角",
            PieceType::Horse => "馬",
            PieceType::Gold => "金",
            PieceType::Silver => "銀",
            PieceType::Knight => "桂",
            PieceType::Lance => "香",
            PieceType::Pawn => "歩",
            _ => "　",
        };

        let color = match self.owner {
            Player::First => 30,
            Player::Second => 31,
            Player::Neutral => 30,
        };

        write!(f, "\x1b[{}m\x1b[47m{}\x1b[m", color, character)
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let character = match self.piece {
            PieceType::King => "王",
            PieceType::Rook => "飛",
            PieceType::Dragon => "龍",
            PieceType::Bishop => "角",
            PieceType::Horse => "馬",
            PieceType::Gold => "金",
            PieceType::Silver => "銀",
            PieceType::Knight => "桂",
            PieceType::Lance => "香",
            PieceType::Pawn => "歩",
            _ => "",
        };

        write!(f, "{}", character)
    }
}

#[derive(Copy, Clone)]
pub struct Game {
    first_player_hand: [PieceType; 30],  // 先手持ち駒
    second_player_hand: [PieceType; 30], // 後手持ち駒
    turn: Player,                        // 現在の打ち手
    field: [[Piece; 9]; 9],              // 盤面
}

impl Game {
    pub fn new() -> Self {
        Game {
            first_player_hand: [PieceType::None; 30],
            second_player_hand: [PieceType::None; 30],
            turn: Player::First,
            field: [
                [
                    Piece {
                        piece: PieceType::Lance,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Knight,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Silver,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Gold,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::King,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Gold,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Silver,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Knight,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Lance,
                        owner: Player::Second,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::Rook,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::Bishop,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::Second,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Pawn,
                        owner: Player::First,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::Bishop,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                    Piece {
                        piece: PieceType::Rook,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::None,
                        owner: Player::Neutral,
                    },
                ],
                [
                    Piece {
                        piece: PieceType::Lance,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Knight,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Silver,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Gold,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::King,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Gold,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Silver,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Knight,
                        owner: Player::First,
                    },
                    Piece {
                        piece: PieceType::Lance,
                        owner: Player::First,
                    },
                ],
            ],
        }
    }

    pub fn update(&mut self, before: (usize, usize), after: (usize, usize), nari: bool) {
        let p = self.field[before.1 - 1][8 - (before.0 - 1)].clone();
        self.field[before.1 - 1][8 - (before.0 - 1)] = Piece {
            piece: PieceType::None,
            owner: Player::Neutral,
        };
        self.field[after.1 - 1][8 - (after.0 - 1)] = p;
    }

    pub fn print(self) {
        print!("\x1b[2J\x1b[0;0H");
        let row = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];

        println!("");
        print!("　▽　");
        for i in 0..self.second_player_hand.len() {
            if self.second_player_hand[i] != PieceType::None {
                print!("{}　", self.second_player_hand[i]);
            }
        }
        println!("");
        println!("");
        println!("　９８７６５４３２１");
        println!("　ーーーーーーーーー　");
        for i in 0..9 {
            print!("｜");
            for j in 0..9 {
                print!("{}", self.field[i][j]);
            }
            println!("｜{}", row[i]);
        }
        println!("　ーーーーーーーーー　");
        print!("　▲　");
        for i in 0..self.first_player_hand.len() {
            if self.first_player_hand[i] != PieceType::None {
                print!("{}　", self.first_player_hand[i]);
            }
        }
        println!("");
    }
}

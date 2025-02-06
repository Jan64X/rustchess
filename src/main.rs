use colored::*;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

#[derive(Clone, Copy, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

#[derive(Clone)]
struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            squares: [[None; 8]; 8],
        };
        
        // Initialize pieces
        board.init_pieces();
        board
    }

    fn init_pieces(&mut self) {
        // Set up white pieces
        self.squares[7][0] = Some(Piece { piece_type: PieceType::Rook, color: PieceColor::White });
        self.squares[7][1] = Some(Piece { piece_type: PieceType::Knight, color: PieceColor::White });
        self.squares[7][2] = Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::White });
        self.squares[7][3] = Some(Piece { piece_type: PieceType::Queen, color: PieceColor::White });
        self.squares[7][4] = Some(Piece { piece_type: PieceType::King, color: PieceColor::White });
        self.squares[7][5] = Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::White });
        self.squares[7][6] = Some(Piece { piece_type: PieceType::Knight, color: PieceColor::White });
        self.squares[7][7] = Some(Piece { piece_type: PieceType::Rook, color: PieceColor::White });

        // Set up black pieces
        self.squares[0][0] = Some(Piece { piece_type: PieceType::Rook, color: PieceColor::Black });
        self.squares[0][1] = Some(Piece { piece_type: PieceType::Knight, color: PieceColor::Black });
        self.squares[0][2] = Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::Black });
        self.squares[0][3] = Some(Piece { piece_type: PieceType::Queen, color: PieceColor::Black });
        self.squares[0][4] = Some(Piece { piece_type: PieceType::King, color: PieceColor::Black });
        self.squares[0][5] = Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::Black });
        self.squares[0][6] = Some(Piece { piece_type: PieceType::Knight, color: PieceColor::Black });
        self.squares[0][7] = Some(Piece { piece_type: PieceType::Rook, color: PieceColor::Black });

        // Set up pawns
        for i in 0..8 {
            self.squares[1][i] = Some(Piece { piece_type: PieceType::Pawn, color: PieceColor::Black });
            self.squares[6][i] = Some(Piece { piece_type: PieceType::Pawn, color: PieceColor::White });
        }
    }

    fn display(&self) {
        println!("  a b c d e f g h");
        println!("  ─────────────");
        for i in 0..8 {
            print!("{} ", 8 - i);
            for j in 0..8 {
                let piece_str = match &self.squares[i][j] {
                    Some(piece) => {
                        let symbol = match (piece.piece_type, piece.color) {
                            (PieceType::King, PieceColor::White) => "♔",
                            (PieceType::Queen, PieceColor::White) => "♕",
                            (PieceType::Rook, PieceColor::White) => "♖",
                            (PieceType::Bishop, PieceColor::White) => "♗",
                            (PieceType::Knight, PieceColor::White) => "♘",
                            (PieceType::Pawn, PieceColor::White) => "♙",
                            (PieceType::King, PieceColor::Black) => "♚",
                            (PieceType::Queen, PieceColor::Black) => "♛",
                            (PieceType::Rook, PieceColor::Black) => "♜",
                            (PieceType::Bishop, PieceColor::Black) => "♝",
                            (PieceType::Knight, PieceColor::Black) => "♞",
                            (PieceType::Pawn, PieceColor::Black) => "♟",
                        };
                        if piece.color == PieceColor::White {
                            symbol.white().to_string()
                        } else {
                            symbol.black().to_string()
                        }
                    }
                    None => "·".to_string(),
                };
                print!("{} ", piece_str);
            }
            println!("{}", 8 - i);
        }
        println!("  ─────────────");
        println!("  a b c d e f g h");
    }

    fn is_valid_position(x: i32, y: i32) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8
    }

    fn is_king_in_check(&self, color: PieceColor) -> bool {
        // Find king position
        let mut king_pos = None;
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        king_pos = Some((i, j));
                        break;
                    }
                }
            }
        }

        if let Some((king_i, king_j)) = king_pos {
            // Check if any opponent piece can capture the king
            for i in 0..8 {
                for j in 0..8 {
                    if let Some(piece) = self.squares[i][j] {
                        if piece.color != color {
                            let from = format!("{}{}", (b'a' + j as u8) as char, 8 - i);
                            let to = format!("{}{}", (b'a' + king_j as u8) as char, 8 - king_i);
                            if self.is_valid_move(&from, &to) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn is_checkmate(&self, color: PieceColor) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }

        // Try all possible moves to see if any can get out of check
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.color == color {
                        let from = format!("{}{}", (b'a' + j as u8) as char, 8 - i);
                        for to_i in 0..8 {
                            for to_j in 0..8 {
                                let to = format!("{}{}", (b'a' + to_j as u8) as char, 8 - to_i);
                                if self.is_valid_move(&from, &to) {
                                    let mut new_board = self.clone();
                                    if new_board.make_move(&from, &to) {
                                        if !new_board.is_king_in_check(color) {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }

    fn is_stalemate(&self, color: PieceColor) -> bool {
        if self.is_king_in_check(color) {
            return false;
        }

        // Check if any legal move exists
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.color == color {
                        let from = format!("{}{}", (b'a' + j as u8) as char, 8 - i);
                        for to_i in 0..8 {
                            for to_j in 0..8 {
                                let to = format!("{}{}", (b'a' + to_j as u8) as char, 8 - to_i);
                                if self.is_valid_move(&from, &to) {
                                    let mut new_board = self.clone();
                                    if new_board.make_move(&from, &to) {
                                        if !new_board.is_king_in_check(color) {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }

    fn is_valid_move(&self, from: &str, to: &str) -> bool {
        let (from_x, from_y) = match parse_position(from) {
            (Some(x), Some(y)) => (x, y),
            _ => return false,
        };
        
        let (to_x, to_y) = match parse_position(to) {
            (Some(x), Some(y)) => (x, y),
            _ => return false,
        };

        let piece = match self.squares[from_y][from_x] {
            Some(p) => p,
            None => return false,
        };

        // Check if destination has a piece of the same color
        if let Some(dest_piece) = self.squares[to_y][to_x] {
            if dest_piece.color == piece.color {
                return false;
            }
        }

        let basic_valid = match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from_x, from_y, to_x, to_y, piece.color),
            PieceType::Rook => self.is_valid_rook_move(from_x, from_y, to_x, to_y),
            PieceType::Knight => self.is_valid_knight_move(from_x, from_y, to_x, to_y),
            PieceType::Bishop => self.is_valid_bishop_move(from_x, from_y, to_x, to_y),
            PieceType::Queen => self.is_valid_queen_move(from_x, from_y, to_x, to_y),
            PieceType::King => self.is_valid_king_move(from_x, from_y, to_x, to_y),
        };

        if !basic_valid {
            return false;
        }

        // Check if move puts or leaves own king in check
        let mut new_board = self.clone();
        new_board.squares[to_y][to_x] = new_board.squares[from_y][from_x];
        new_board.squares[from_y][from_x] = None;
        !new_board.is_king_in_check(piece.color)
    }

    fn is_valid_pawn_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize, color: PieceColor) -> bool {
        let direction = if color == PieceColor::White { -1 } else { 1 };
        let start_rank = if color == PieceColor::White { 6 } else { 1 };
        
        // Convert to signed integers for calculations
        let dx = (to_x as i32) - (from_x as i32);
        let dy = (to_y as i32) - (from_y as i32);

        // Basic forward movement
        if dx == 0 {
            // Single square forward
            if dy == direction {
                return self.squares[to_y][to_x].is_none();
            }
            // Double square forward from starting position
            if from_y == start_rank && dy == 2 * direction {
                let intermediate_y = (from_y as i32 + direction) as usize;
                return self.squares[intermediate_y][from_x].is_none() 
                    && self.squares[to_y][to_x].is_none();
            }
        }
        // Capture diagonally
        else if dx.abs() == 1 && dy == direction {
            return self.squares[to_y][to_x].is_some();
        }

        false
    }

    fn is_valid_rook_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if from_x != to_x && from_y != to_y {
            return false;
        }

        let (start, end, is_vertical) = if from_x == to_x {
            (from_y.min(to_y) + 1, from_y.max(to_y), true)
        } else {
            (from_x.min(to_x) + 1, from_x.max(to_x), false)
        };

        // Check if path is clear
        for i in start..end {
            if is_vertical {
                if self.squares[i][from_x].is_some() {
                    return false;
                }
            } else {
                if self.squares[from_y][i].is_some() {
                    return false;
                }
            }
        }
        true
    }

    fn is_valid_knight_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        let dx = (to_x as i32 - from_x as i32).abs();
        let dy = (to_y as i32 - from_y as i32).abs();
        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }

    fn is_valid_bishop_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        let dx = (to_x as i32 - from_x as i32).abs();
        let dy = (to_y as i32 - from_y as i32).abs();
        
        if dx != dy {
            return false;
        }

        let x_direction = if to_x > from_x { 1 } else { -1 };
        let y_direction = if to_y > from_y { 1 } else { -1 };

        let mut x = from_x as i32 + x_direction;
        let mut y = from_y as i32 + y_direction;

        while x != to_x as i32 {
            if Board::is_valid_position(x, y) && self.squares[y as usize][x as usize].is_some() {
                return false;
            }
            x += x_direction;
            y += y_direction;
        }
        true
    }

    fn is_valid_queen_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        self.is_valid_rook_move(from_x, from_y, to_x, to_y) || 
        self.is_valid_bishop_move(from_x, from_y, to_x, to_y)
    }

    fn is_valid_king_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        let dx = (to_x as i32 - from_x as i32).abs();
        let dy = (to_y as i32 - from_y as i32).abs();
        dx <= 1 && dy <= 1
    }

    fn make_move(&mut self, from: &str, to: &str) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }

        let (from_x, from_y) = match parse_position(from) {
            (Some(x), Some(y)) => (x, y),
            _ => return false,
        };

        let (to_x, to_y) = match parse_position(to) {
            (Some(x), Some(y)) => (x, y),
            _ => return false,
        };

        self.squares[to_y][to_x] = self.squares[from_y][from_x];
        self.squares[from_y][from_x] = None;
        
        self.check_pawn_promotion(to_x, to_y);
        true
    }

    fn check_pawn_promotion(&mut self, to_x: usize, to_y: usize) {
        if let Some(piece) = self.squares[to_y][to_x] {
            if piece.piece_type == PieceType::Pawn {
                // Check if pawn reached the opposite end
                if (piece.color == PieceColor::White && to_y == 0) ||
                   (piece.color == PieceColor::Black && to_y == 7) {
                    // For AI, automatically promote to Queen
                    // For human players, ask for choice
                    let promotion = if piece.color == PieceColor::Black {
                        PieceType::Queen
                    } else {
                        println!("Promote pawn to:");
                        println!("1. Queen");
                        println!("2. Rook");
                        println!("3. Bishop");
                        println!("4. Knight");
                        print!("Choose promotion (1-4): ");
                        io::stdout().flush().unwrap();
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        match input.trim() {
                            "2" => PieceType::Rook,
                            "3" => PieceType::Bishop,
                            "4" => PieceType::Knight,
                            _ => PieceType::Queen,
                        }
                    };
                    
                    self.squares[to_y][to_x] = Some(Piece {
                        piece_type: promotion,
                        color: piece.color,
                    });
                }
            }
        }
    }
}

fn parse_position(pos: &str) -> (Option<usize>, Option<usize>) {
    if pos.len() != 2 {
        return (None, None);
    }

    let chars: Vec<char> = pos.chars().collect();
    let file = chars[0];
    let rank = chars[1];

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return (None, None);
    }

    let x = (file as u8 - b'a') as usize;
    let y = (8 - (rank as u8 - b'0')) as usize;

    (Some(x), Some(y))
}

struct ChessAI {
    color: PieceColor,
}

impl ChessAI {
    const MAX_DEPTH: i32 = 3;  // Increase for stronger but slower AI

    fn new(color: PieceColor) -> Self {
        ChessAI { color }
    }

    fn evaluate_position(&self, board: &Board) -> i32 {
        let mut score = 0;
        
        // Piece position tables for improved evaluation
        let pawn_position = [
            0,  0,  0,  0,  0,  0,  0,  0,
            50, 50, 50, 50, 50, 50, 50, 50,
            10, 10, 20, 30, 30, 20, 10, 10,
            5,  5, 10, 25, 25, 10,  5,  5,
            0,  0,  0, 20, 20,  0,  0,  0,
            5, -5,-10,  0,  0,-10, -5,  5,
            5, 10, 10,-20,-20, 10, 10,  5,
            0,  0,  0,  0,  0,  0,  0,  0
        ];

        let knight_position = [
            -50,-40,-30,-30,-30,-30,-40,-50,
            -40,-20,  0,  0,  0,  0,-20,-40,
            -30,  0, 10, 15, 15, 10,  0,-30,
            -30,  5, 15, 20, 20, 15,  5,-30,
            -30,  0, 15, 20, 20, 15,  0,-30,
            -30,  5, 10, 15, 15, 10,  5,-30,
            -40,-20,  0,  5,  5,  0,-20,-40,
            -50,-40,-30,-30,-30,-30,-40,-50
        ];

        let bishop_position = [
            -20,-10,-10,-10,-10,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5, 10, 10,  5,  0,-10,
            -10,  5,  5, 10, 10,  5,  5,-10,
            -10,  0, 10, 10, 10, 10,  0,-10,
            -10, 10, 10, 10, 10, 10, 10,-10,
            -10,  5,  0,  0,  0,  0,  5,-10,
            -20,-10,-10,-10,-10,-10,-10,-20
        ];

        let rook_position = [
            0,  0,  0,  0,  0,  0,  0,  0,
            5, 10, 10, 10, 10, 10, 10,  5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            0,  0,  0,  5,  5,  0,  0,  0
        ];

        let queen_position = [
            -20,-10,-10, -5, -5,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5,  5,  5,  5,  0,-10,
            -5,  0,  5,  5,  5,  5,  0, -5,
            0,  0,  5,  5,  5,  5,  0, -5,
            -10,  5,  5,  5,  5,  5,  0,-10,
            -10,  0,  5,  0,  0,  0,  0,-10,
            -20,-10,-10, -5, -5,-10,-10,-20
        ];

        let king_position = [
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -20,-30,-30,-40,-40,-30,-30,-20,
            -10,-20,-20,-20,-20,-20,-20,-10,
            20, 20,  0,  0,  0,  0, 20, 20,
            20, 30, 10,  0,  0, 10, 30, 20
        ];

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = board.squares[i][j] {
                    let position_value = match piece.piece_type {
                        PieceType::Pawn => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            pawn_position[idx]
                        },
                        PieceType::Knight => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            knight_position[idx]
                        },
                        PieceType::Bishop => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            bishop_position[idx]
                        },
                        PieceType::Rook => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            rook_position[idx]
                        },
                        PieceType::Queen => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            queen_position[idx]
                        },
                        PieceType::King => {
                            let idx = if piece.color == self.color {
                                i * 8 + j
                            } else {
                                (7 - i) * 8 + j
                            };
                            king_position[idx]
                        },
                    };

                    let piece_value = match piece.piece_type {
                        PieceType::Pawn => 100,
                        PieceType::Knight => 320,
                        PieceType::Bishop => 330,
                        PieceType::Rook => 500,
                        PieceType::Queen => 900,
                        PieceType::King => 20000,
                    };

                    // Add mobility bonus
                    let mobility_bonus = if piece.color == self.color {
                        self.count_legal_moves(board, i, j) as i32 * 10
                    } else {
                        -(self.count_legal_moves(board, i, j) as i32 * 10)
                    };

                    if piece.color == self.color {
                        score += piece_value + position_value + mobility_bonus;
                    } else {
                        score -= piece_value + position_value - mobility_bonus;
                    }
                }
            }
        }
        score
    }

    fn count_legal_moves(&self, board: &Board, i: usize, j: usize) -> usize {
        let from = format!("{}{}", (b'a' + j as u8) as char, 8 - i);
        let mut count = 0;
        
        for to_i in 0..8 {
            for to_j in 0..8 {
                let to = format!("{}{}", (b'a' + to_j as u8) as char, 8 - to_i);
                if board.is_valid_move(&from, &to) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_all_possible_moves(&self, board: &Board) -> Vec<(String, String)> {
        let mut moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = board.squares[i][j] {
                    if piece.color == self.color {
                        let from = format!("{}{}", 
                            (b'a' + j as u8) as char,
                            8 - i
                        );
                        
                        for to_i in 0..8 {
                            for to_j in 0..8 {
                                let to = format!("{}{}", 
                                    (b'a' + to_j as u8) as char,
                                    8 - to_i
                                );
                                if board.is_valid_move(&from, &to) {
                                    moves.push((from.clone(), to));
                                }
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn minimax(&self, board: &Board, depth: i32, alpha: i32, beta: i32, maximizing: bool) -> i32 {
        if depth == 0 {
            return self.evaluate_position(board);
        }

        let moves = self.get_all_possible_moves(board);
        if moves.is_empty() {
            return if maximizing { -1000 } else { 1000 };
        }

        if maximizing {
            let mut max_eval = i32::MIN;
            for (from, to) in moves {
                let mut new_board = board.clone();
                if new_board.make_move(&from, &to) {
                    let eval = self.minimax(&new_board, depth - 1, alpha, beta, false);
                    max_eval = max_eval.max(eval);
                    if max_eval >= beta {
                        break;
                    }
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for (from, to) in moves {
                let mut new_board = board.clone();
                if new_board.make_move(&from, &to) {
                    let eval = self.minimax(&new_board, depth - 1, alpha, beta, true);
                    min_eval = min_eval.min(eval);
                    if min_eval <= alpha {
                        break;
                    }
                }
            }
            min_eval
        }
    }

    fn make_move(&self, board: &Board) -> Option<(String, String)> {
        let moves = self.get_all_possible_moves(board);
        let mut best_move = None;
        let mut best_eval = i32::MIN;

        for (from, to) in moves {
            let mut new_board = board.clone();
            if new_board.make_move(&from, &to) {
                let eval = self.minimax(&new_board, ChessAI::MAX_DEPTH - 1, i32::MIN, i32::MAX, false);
                if eval > best_eval {
                    best_eval = eval;
                    best_move = Some((from, to));
                }
            }
        }

        best_move
    }
}

#[tokio::main]
async fn main() {
    println!("Welcome to RustChess!");
    println!("1. Play against AI");
    println!("2. Play against another player");
    println!("3. Watch AI vs AI");
    print!("Choose game mode (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let game_mode = input.trim();

    let mut board = Board::new();
    let mut current_turn = PieceColor::White;
    
    // Initialize AIs based on game mode
    let (white_ai, black_ai) = match game_mode {
        "1" => {
            println!("You'll play as White against the AI (Black)");
            (None, Some(ChessAI::new(PieceColor::Black)))
        }
        "3" => {
            println!("Watch two AIs play against each other!");
            println!("Game will advance automatically with 1 second delay between moves.");
            println!("Press Ctrl+C to end the game.");
            (Some(ChessAI::new(PieceColor::White)), Some(ChessAI::new(PieceColor::Black)))
        }
        _ => (None, None)
    };

    loop {
        board.display();
        
        let turn_str = if current_turn == PieceColor::White {
            "White"
        } else {
            "Black"
        };

        // Check for checkmate and stalemate
        if board.is_checkmate(current_turn) {
            println!("Checkmate! {} wins!", if current_turn == PieceColor::White { "Black" } else { "White" });
            break;
        }

        if board.is_stalemate(current_turn) {
            println!("Stalemate! The game is a draw!");
            break;
        }

        // Show if the king is in check
        if board.is_king_in_check(current_turn) {
            println!("{} is in check!", turn_str);
        }

        // Handle AI moves
        let current_ai = match current_turn {
            PieceColor::White => white_ai.as_ref(),
            PieceColor::Black => black_ai.as_ref(),
        };

        if let Some(ai) = current_ai {
            println!("{} AI is thinking...", turn_str);
            if let Some((from, to)) = ai.make_move(&board) {
                println!("{} AI moves: {} to {}", turn_str, from, to);
                
                // In AI vs AI mode, wait for 1 second before next move
                if game_mode == "3" {
                    sleep(Duration::from_secs(1)).await;
                }

                if board.make_move(&from, &to) {
                    current_turn = if current_turn == PieceColor::White {
                        PieceColor::Black
                    } else {
                        PieceColor::White
                    };
                }
            } else {
                println!("AI couldn't find a valid move!");
                break;
            }
            continue;
        }
        
        // Handle human moves
        print!("{}'s turn (e.g., 'e2 e4' or 'quit'): ", turn_str);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input format. Use 'from to' (e.g., 'e2 e4')");
            continue;
        }

        if board.make_move(parts[0], parts[1]) {
            current_turn = if current_turn == PieceColor::White {
                PieceColor::Black
            } else {
                PieceColor::White
            };
        } else {
            println!("Invalid move!");
        }
    }
}

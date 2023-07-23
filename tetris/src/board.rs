//! ## 方向
//!
//! * 0: 初始方向
//! * 1: 向右（顺时针旋转 90 度）
//! * 2: 向下（旋转 180 度）
//! * 3: 向左（逆时针旋转 90 度。）
//!
//! ## 旋转
//! * 0: 不旋转
//! * 1: 顺时针旋转 90 度
//! * 2: 旋转 180 度
//! * 3: 逆时针旋转 90 度。
//!
//! ## 踢墙表
//! 注意：dx 为正表示向右偏移，dy 为正表示向上偏移（因为是从 Tetris Wiki 搬过来的）

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

/// 四连方块的基本数据
pub struct PieceData {
    /// 名称（未使用）
    pub name: String,
    /// 初始状态宽度：用于确定四连方块进场的位置
    pub initial_width: usize,
    /// 不同朝向的形态数据。第一层是方块的当前方向，第二层是每个小块的偏移。
    pub orientation: Vec<Vec<(usize, usize)>>,
    /// 踢墙表。第一层是方块的当前方向，第二层是将要如何旋转，第三层是多个测试位置。
    pub test_table: Vec<Vec<Vec<(isize, isize)>>>,
}

/// 版面
pub struct Board {
    /// 版面宽度
    pub width: usize,
    /// 版面高度
    pub height: usize,
    /// 版面：22 行 10 列。None 表示没有填充。usize 表示一个颜色
    pub board: Vec<Option<usize>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: vec![None; width * height],
            width,
            height,
        }
    }

    /// 清空版面
    pub fn clear(&mut self) {
        self.board.fill(None);
    }

    /// 获取一个小块的状态，第一个 Option 是当前 (x,y) 是否存在，第二个 Option 是当前小块是否已填充
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Option<usize>> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.board.get(y * self.width + x)
    }

    /// 获取一个小块的状态，第一个 Option 是当前 (x,y) 是否存在，第二个 Option 是当前小块是否已填充
    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Option<usize>> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.board.get_mut(y * self.width + x)
    }

    /// 获取一行中已填充的小块的数量
    pub fn get_row_filled_count(&self, y: usize) -> usize {
        let mut count = 0;
        for x in 0..self.width {
            if let Some(cell) = self.get_cell(x, y) {
                if cell.is_some() {
                    count += 1;
                }
            }
        }
        count
    }

    /// 复制行
    pub fn copy_row(&mut self, src_y: usize, dst_y: usize) {
        for x in 0..self.width {
            self.board[dst_y * self.width + x] = self.board[src_y * self.width + x];
        }
    }

    /// 清除行，不下落
    pub fn clear_row(&mut self, y: usize) {
        for x in 0..self.width {
            if let Some(cell) = self.get_cell_mut(x, y) {
                *cell = None
            }
        }
    }

    /// 消除所有填满的行，并下落
    ///
    /// returns: 返回清除的行数和剩余的非空行数
    pub fn clear_filled_rows(&mut self) -> (usize, usize) {
        let mut cleared_row_count = 0;
        let mut non_empty_row_count = 0;
        let mut dst_y = self.height - 1;
        for src_y in (0..self.height).rev() {
            let filled_count = self.get_row_filled_count(src_y);
            if filled_count != 0 {
                if filled_count != self.width {
                    self.copy_row(src_y, dst_y);
                    dst_y -= 1;
                    non_empty_row_count += 1;
                } else {
                    cleared_row_count += 1;
                }
            }
        }
        loop {
            self.clear_row(dst_y);
            if dst_y == 0 {
                break;
            }
            dst_y -= 1;
        }
        (cleared_row_count, non_empty_row_count)
    }

    /// 尝试四连方块能否以指定朝向放入指定位置
    pub fn test_piece(&self, piece_data: &PieceData, (x, y): (isize, isize), orientation: usize) -> bool {
        if let Some(cells) = piece_data.orientation.get(orientation) {
            for (cell_dx, cell_dy) in cells {
                match (cell_dx.checked_add_signed(x), cell_dy.checked_add_signed(y)) {
                    (Some(x2), Some(y2)) => {
                        if let Some(cell) = self.get_cell(x2, y2) {
                            if let Some(_) = cell {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// 尝试平移和旋转，并返回移动后的位置。
    /// 请注意，顺序是先平移后旋转。
    ///
    /// # Arguments
    ///
    /// * `piece_data`: 四连方块数据
    /// * `(x, y)`: 当前位置
    /// * `orientation`: 当前方向
    /// * `(dx, dy)`: 平移
    /// * `rotation`: 旋转
    ///
    /// returns: Option<((isize, isize), usize)> 新的位置, 新的朝向
    pub fn test_move_piece(&self, piece_data: &PieceData, (x, y): (isize, isize), orientation: usize, (dx, dy): (isize, isize), rotation: usize) -> Option<((isize, isize), usize)> {
        if let Some(orientation_data) = piece_data.test_table.get(orientation) {
            if let Some(test_table) = orientation_data.get(rotation) {
                let new_orientation = (orientation + rotation) % 4;
                for (test_dx, test_dy) in test_table {
                    let new_position = (x + dx + *test_dx, y + dy - *test_dy);
                    if self.test_piece(piece_data, new_position, new_orientation) {
                        return Some((new_position, new_orientation));
                    }
                }
            }
        }
        None
    }

    /// 锁定（将四连方块填充到版面中）
    pub fn lock_piece(&mut self, piece_data: &PieceData, (x, y): (isize, isize), orientation: usize, color_id: usize) {
        if let Some(cells) = piece_data.orientation.get(orientation) {
            for (cell_dx, cell_dy) in cells {
                match (cell_dx.checked_add_signed(x), cell_dy.checked_add_signed(y)) {
                    (Some(x2), Some(y2)) => {
                        if let Some(cell) = self.get_cell_mut(x2, y2) {
                            *cell = Some(color_id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/// 四连方块当前状态
pub struct Piece {
    /// 类型（对应 ExtendedBoard 的 Vec<PieceData> 的下标）
    pub typ: usize,
    /// 位置
    pub position: (isize, isize),
    /// 方向
    pub orientation: usize,
}

/// 带当前四连方块的版面
pub struct ExtendedBoard {
    /// 四连方块数据表
    pub piece_data: Vec<PieceData>,
    /// 版面
    pub board: Board,
    /// 当前四连方块
    pub current_piece: Option<Piece>,
    /// 初始行
    pub start_y: isize,
}

impl ExtendedBoard {
    pub fn new(width: usize, height: usize, piece_data: Vec<PieceData>, start_y: isize) -> Self {
        Self {
            piece_data,
            board: Board::new(width, height),
            current_piece: None,
            start_y,
        }
    }

    pub fn reset(&mut self) {
        self.board.clear();
        self.current_piece = None;
    }

    pub fn add_piece(&mut self, typ: usize, position: (isize, isize)) -> bool {
        if let Some(piece_data) = self.piece_data.get(typ) {
            if self.board.test_piece(piece_data, position, 0) {
                self.current_piece = Some(Piece {
                    typ,
                    position,
                    orientation: 0,
                });
                return true;
            }
        }
        false
    }

    /// 在默认位置添加一个指定类型的方块
    pub fn add_piece_default_position(&mut self, typ: usize) -> bool {
        if let Some(piece_data) = self.piece_data.get(typ) {
            let start_x = (self.board.width - piece_data.initial_width) / 2;
            self.add_piece(typ, (start_x as isize, self.start_y))
        } else {
            false
        }
    }

    /// 平移+旋转
    ///
    /// # Arguments
    ///
    /// * `translation`: 平移
    /// * `rotation`: 旋转方向，0: 不旋转，1: 顺时针旋转 90 度，2: 旋转 180 度，3: 逆时针旋转 90 度。
    ///
    /// returns: bool 是否移动过
    pub fn move_piece(&mut self, translation: (isize, isize), rotation: usize) -> bool {
        if let Some(current_piece) = &mut self.current_piece {
            if let Some(piece_data) = self.piece_data.get(current_piece.typ) {
                if let Some((new_position, new_orientation)) = self.board.test_move_piece(piece_data, current_piece.position, current_piece.orientation, translation, rotation) {
                    current_piece.orientation = new_orientation;
                    current_piece.position = new_position;
                    return true;
                }
            }
        }
        false
    }

    /// 快速降落（不锁定）
    pub fn fast_drop(&mut self) {
        let dy = self.test_fast_drop();
        if dy > 0 {
            self.move_piece((0, dy), 0);
        }
    }

    /// 测试快速降落需要移动多少格
    pub fn test_fast_drop(&self) -> isize {
        let mut dy = 0;
        if let Some(current_piece) = &self.current_piece {
            if let Some(piece_data) = self.piece_data.get(current_piece.typ) {
                while let Some(_) = self.board.test_move_piece(piece_data, current_piece.position, current_piece.orientation, (0, dy + 1), 0) {
                    dy += 1;
                }
            }
        }
        dy
    }

    /// 是否已经降落，没有方块时视
    pub fn is_land(&self) -> bool {
        if let Some(_) = &self.current_piece {
            self.test_fast_drop() == 0
        } else {
            false
        }
    }

    /// 锁定
    pub fn lock_piece(&mut self) {
        if let Some(current_piece) = &self.current_piece {
            if let Some(piece_data) = self.piece_data.get(current_piece.typ) {
                self.board.lock_piece(piece_data, current_piece.position, current_piece.orientation, current_piece.typ);
            }
        }
        self.current_piece = None;
    }
}

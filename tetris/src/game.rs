use crate::board::ExtendedBoard;

pub enum Event {
    /// Tick 事件，每帧需要调用一次。
    /// Tick 会计算重力下落、软降下落、左右 Auto Shift 或者 Auto Repeat、落地锁定。
    /// 调用 Tick 之后应该立刻渲染当前状态
    /// 之后的操作都算作下一帧的操作。
    Tick,
    RotateLeft,
    RotateRight,
    Rotate180,
    /// 暂存当前块，或者交换当前块与暂存块（交换可能造成 Game Over!）
    Hold,
    /// 硬降
    HardDrop,
    /// 快速降落
    SoftDropFast,
    /// 放弃
    Forfeit,
    MoveLeftBegin,
    MoveLeftEnd,
    MoveRightBegin,
    MoveRightEnd,
    /// 软降按下
    SoftDropBegin,
    /// 软降抬起
    SoftDropEnd,
}

pub struct Config {
    /// Delay Auto Shift (frames)
    pub das: f32,
    /// Auto Repeat Rate (frames (per block))
    pub arr: f32,
    /// Soft Drop Factor (gravity 的倍数)
    pub sdf: f32,
    /// 重力 (blocks per frame)
    pub gravity: f32,
    /// 锁定延迟 (frames)
    pub lock_delay: usize,
    /// 最大重置次数
    pub max_reset_times: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            das: 7.0,
            arr: 0.0,
            sdf: 10000.0,
            gravity: 0.02,
            lock_delay: 30,
            max_reset_times: 15,
        }
    }
}

pub struct Game<G> {
    /// 游戏设置
    pub config: Config,
    /// 版面
    pub board: ExtendedBoard,
    /// 四连方块的序列产生器
    pub rng: G,

    /// 当前 Hold 的四连方块
    pub hold: Option<usize>,
    /// 已经使用了 Hold 机会
    pub hold_used: bool,

    /// 当前 tick 序号
    pub current_tick: usize,

    /// 上次重力触发移动的 tick
    pub gravity_last_tick: f32,

    /// 左移按键按下
    pub move_left_down: bool,
    /// 右移按键按下
    pub move_right_down: bool,
    /// 当前移动方向：0 不动 -1 向左，1 向右
    pub move_direction: isize,
    /// 上次触发移动的 tick
    pub move_last_tick: f32,
    /// 移动已经完成 Delay Auto Shift 延迟，已经进入 Auto Repeat 阶段
    pub move_auto_repeat: bool,

    /// 软降按键按下
    pub soft_drop_down: bool,
    /// 软降按键按下的 tick
    pub soft_drop_last_tick: f32,

    /// 已落地 tick 数
    pub land_tick_count: usize,
    /// 已重置锁定次数
    pub reset_times: usize,
}

impl<G> Game<G> {
    pub fn new(config: Config, board: ExtendedBoard, rng: G) -> Self {
        Self {
            config,
            board,
            rng,
            hold: None,
            hold_used: false,
            current_tick: 0,
            gravity_last_tick: 0.0,
            move_left_down: false,
            move_right_down: false,
            move_direction: 0,
            move_last_tick: 0.0,
            move_auto_repeat: false,
            soft_drop_down: false,
            soft_drop_last_tick: 0.0,
            land_tick_count: 0,
            reset_times: 0,
        }
    }
}

impl<G> Game<G>
    where
        G: Iterator<Item=usize>
{
    pub fn on_event(&mut self, event: Event) -> Result<(), ()> {
        match event {
            Event::Tick => {
                if self.config.gravity > 0.0 {
                    let tick_per_block = if self.soft_drop_down {
                        1.0 / self.config.gravity * self.config.sdf
                    } else {
                        1.0 / self.config.gravity
                    };
                    while self.soft_drop_last_tick + tick_per_block < self.current_tick as _ {
                        self.soft_drop_last_tick += tick_per_block;
                        let moved = self.move_piece((0, 1), 0)?;
                        if !moved {
                            break;
                        }
                    }
                }

                if self.move_direction != 0 {
                    if !self.move_auto_repeat {
                        if self.move_last_tick + self.config.das >= self.current_tick as _ {
                            self.move_piece((self.move_direction, 0), 0)?;
                            self.move_last_tick = self.current_tick as _;
                            self.move_auto_repeat = true;
                        }
                    }
                    if self.move_auto_repeat {
                        while self.move_last_tick + self.config.arr < self.current_tick as _ {
                            self.move_last_tick += self.config.arr;
                            let moved = self.move_piece((self.move_direction, 0), 0)?;
                            if !moved {
                                break;
                            }
                        }
                    }
                }

                if self.board.is_land() {
                    self.land_tick_count += 1;
                } else {
                    self.land_tick_count = 0;
                }
                if self.land_tick_count >= self.config.lock_delay {
                    self.lock_and_add_next_piece()?;
                }
                self.current_tick += 1;
            }
            Event::RotateLeft => {
                self.move_piece((0, 0), 3)?;
            }
            Event::RotateRight => {
                self.move_piece((0, 0), 1)?;
            }
            Event::Rotate180 => {
                self.move_piece((0, 0), 2)?;
            }
            Event::Hold => {
                if let Some(piece) = &self.board.current_piece {
                    if !self.hold_used {
                        let current_typ = piece.typ;
                        if let Some(hold) = self.hold {
                            self.add_type_piece(hold)?;
                        } else {
                            self.add_next_piece()?;
                        }
                        self.hold = Some(current_typ);
                        self.hold_used = true;
                    }
                }
            }
            Event::HardDrop => {
                self.board.fast_drop();
                self.lock_and_add_next_piece()?;
            }
            Event::SoftDropFast => {
                self.board.fast_drop();
                self.land_tick_count = 0;
            }
            Event::Forfeit => {
                return Err(());
            }
            Event::MoveLeftBegin => {
                self.move_left_down = true;
                self.move_direction = -1;
                self.move_auto_repeat = false;
                self.move_piece((self.move_direction, 0), 0)?;
                self.move_last_tick = self.current_tick as _;
            }
            Event::MoveLeftEnd => {
                self.move_left_down = false;
                if self.move_right_down {
                    self.move_direction = 1;
                    self.move_piece((self.move_direction, 0), 0)?;
                    self.move_last_tick = self.current_tick as _;
                } else {
                    self.move_direction = 0;
                }
            }
            Event::MoveRightBegin => {
                self.move_right_down = true;
                self.move_direction = 1;
                self.move_auto_repeat = false;
                self.move_piece((self.move_direction, 0), 0)?;
                self.move_last_tick = self.current_tick as _;
            }
            Event::MoveRightEnd => {
                self.move_right_down = false;
                if self.move_left_down {
                    self.move_direction = -1;
                    self.move_piece((self.move_direction, 0), 0)?;
                    self.move_last_tick = self.current_tick as _;
                } else {
                    self.move_direction = 0;
                }
            }
            Event::SoftDropBegin => {
                self.soft_drop_down = true;
                self.move_piece((0, 1), 0)?;
                self.soft_drop_last_tick = self.current_tick as _;
            }
            Event::SoftDropEnd => {
                self.soft_drop_down = false;
            }
        }
        Ok(())
    }

    /// 在默认位置添加一个指定块
    ///
    /// 如果添加不了，则将 game_over 设为 true
    pub fn add_type_piece(&mut self, typ: usize) -> Result<(), ()> {
        let added = self.board.add_piece_default_position(typ);
        if !added {
            return Err(());
        }
        self.hold_used = false;
        self.land_tick_count = 0;
        self.reset_times = 0;
        Ok(())
    }

    /// 在默认位置添加一个新块
    pub fn add_next_piece(&mut self) -> Result<(), ()> {
        if let Some(typ) = self.rng.next() {
            self.add_type_piece(typ)?;
        } else {
            return Err(());
        }
        Ok(())
    }

    /// 锁定当前方块，清除填满的行，并添加下一个方块
    pub fn lock_and_add_next_piece(&mut self) -> Result<(), ()> {
        self.board.lock_piece();
        self.board.board.clear_filled_rows();
        self.add_next_piece()?;
        Ok(())
    }

    /// 水平移动、垂直移动、或者旋转当前块
    ///
    /// 会自动更新 reset_times
    ///
    /// reset_times 达到 max_reset_times 时会自动锁定。并添加下一个块
    ///
    /// 如果无法添加下一个块则会触发 game_over
    pub fn move_piece(&mut self, translation: (isize, isize), rotation: usize) -> Result<bool, ()> {
        let prev_is_land = self.board.is_land();
        let moved = self.board.move_piece(translation, rotation);
        if moved {
            if prev_is_land {
                self.reset_times += 1;
            }
            let is_land = self.board.is_land();
            if is_land {
                self.land_tick_count = 0;
                if self.reset_times >= self.config.max_reset_times {
                    self.lock_and_add_next_piece()?;
                }
            }
        }
        Ok(moved)
    }
}
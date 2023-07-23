#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use tetris::{Config, Event, ExtendedBoard, Game, new_default_piece_data, PreviewGenerator, SevenBagGenerator};
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::proto::rng::Rng;

fn get_random_u32(rng: &mut Rng) -> u32 {
    let mut buf = [0; 4];
    rng.get_rng(None, &mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn run_game(system_table: &SystemTable<Boot>) -> Result<(), ()>
{
    let input_handle = system_table.boot_services().get_handle_for_protocol::<Input>().unwrap();
    let mut input = system_table.boot_services().open_protocol_exclusive::<Input>(input_handle).unwrap();
    let gop_handle = system_table.boot_services().get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = system_table.boot_services().open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
    let rng_handle = system_table.boot_services().get_handle_for_protocol::<Rng>().unwrap();
    let mut rng = system_table.boot_services().open_protocol_exclusive::<Rng>(rng_handle).unwrap();

    let seed = get_random_u32(&mut rng);
    let board = ExtendedBoard::new(10, 22, new_default_piece_data(), 0);
    let types = board.piece_data.len();
    let mut game = Game::new(Config::default(), board, PreviewGenerator::new(SevenBagGenerator::new(seed, types), 5));
    game.add_next_piece().unwrap();

    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    let mut first_piece_dropped = false;

    let mut counter = 0;
    loop {
        if let Ok(key) = input.read_key() {
            match key {
                Some(key) => {
                    match key {
                        Key::Printable(c) => {
                            match unsafe { char::from_u32_unchecked(u16::from(c) as u32) } {
                                'r' => game.on_event(Event::Forfeit)?,
                                'z' => game.on_event(Event::Hold)?,
                                'x' => game.on_event(Event::RotateLeft)?,
                                'c' => game.on_event(Event::RotateRight)?,
                                's' => game.on_event(Event::Rotate180)?,
                                ' ' => game.on_event(Event::HardDrop)?,
                                _ => {}
                            }
                        }
                        Key::Special(key) => {
                            match key {
                                ScanCode::DOWN => game.on_event(Event::SoftDropFast)?,
                                ScanCode::RIGHT => {
                                    game.on_event(Event::MoveRightBegin)?;
                                    game.on_event(Event::MoveRightEnd)?;
                                }
                                ScanCode::LEFT => {
                                    game.on_event(Event::MoveLeftBegin)?;
                                    game.on_event(Event::MoveLeftEnd)?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                None => {
                    system_table.boot_services().stall(1_000);
                    counter += 1;
                    if counter >= 15 {
                        game.on_event(Event::Tick)?;
                        render(&mut game, &mut buffer);
                        buffer.blit(&mut gop).unwrap();
                        counter = 0;

                        if game.board.board.get_row_filled_count(game.board.board.height - 1) == 0 { // 全清则退出
                            if first_piece_dropped {
                                break;
                            }
                        } else { // 最下面一行有块表示第一块已放下
                            first_piece_dropped = true;
                        }
                    }
                }
            };
        }
    }
    Ok(())
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    loop {
        match run_game(&system_table) {
            Ok(_) => break,
            Err(_) => {}
        }
    }
    Status::SUCCESS
}

pub const COLOR_TABLE: &[BltPixel] = &[
    BltPixel::new(15, 155, 215), // 青 I
    BltPixel::new(33, 65, 198), // 蓝 J
    BltPixel::new(227, 91, 2), // 橙 L
    BltPixel::new(227, 159, 2), // 黄 O
    BltPixel::new(89, 177, 1), // 绿 S
    BltPixel::new(175, 41, 138), // 紫 T
    BltPixel::new(215, 15, 55), // 红 Z
];

fn render<G>(game: &mut Game<PreviewGenerator<usize, G>>, buffer: &mut Buffer)
    where
        G: Iterator<Item=usize>
{
    let cell_size = buffer.height / (game.board.board.height + 4);
    let start_x = buffer.width / 2 - game.board.board.width * cell_size / 2;
    let start_y = buffer.height / 2 - game.board.board.height * cell_size / 2;
    for y_index in 0..game.board.board.height {
        for x_index in 0..game.board.board.width {
            let x = start_x + x_index * cell_size;
            let y = start_y + y_index * cell_size;
            if let Some(cell) = game.board.board.get_cell(x_index, y_index) {
                if let Some(typ) = cell {
                    buffer.fill_rect(x, y, cell_size, cell_size, COLOR_TABLE[*typ]);
                } else {
                    buffer.fill_rect(x, y, cell_size, cell_size, BltPixel::new(0, 0, 0));
                }
            }
        }
    }
    if let Some(current_piece) = &game.board.current_piece {
        if let Some(piece_data) = game.board.piece_data.get(current_piece.typ) {
            // 半透明颜色
            let x_index = current_piece.position.0;
            let y_index = current_piece.position.1 + game.board.test_fast_drop();
            let color = COLOR_TABLE[current_piece.typ];
            let color = BltPixel::new(color.red / 2, color.green / 2, color.blue / 2);
            for (x_offset, y_offset) in &piece_data.orientation[current_piece.orientation] {
                let x = (start_x as isize + (x_index + *x_offset as isize) * cell_size as isize) as usize;
                let y = (start_y as isize + (y_index + *y_offset as isize) * cell_size as isize) as usize;
                buffer.fill_rect(x, y, cell_size, cell_size, color);
            }

            // 正常颜色
            let x_index = current_piece.position.0;
            let y_index = current_piece.position.1;
            let color = COLOR_TABLE[current_piece.typ];
            for (x_offset, y_offset) in &piece_data.orientation[current_piece.orientation] {
                let x = (start_x as isize + (x_index + *x_offset as isize) * cell_size as isize) as usize;
                let y = (start_y as isize + (y_index + *y_offset as isize) * cell_size as isize) as usize;
                buffer.fill_rect(x, y, cell_size, cell_size, color);
            }
        }
    }

    buffer.fill_rect(start_x + 11 * cell_size, start_y + 2 * cell_size, 4 * cell_size, 15 * cell_size, BltPixel::new(0, 0, 0));

    let mut y_index = 2;
    for typ in game.rng.preview() {
        if let Some(piece_data) = game.board.piece_data.get(*typ) {
            let x_index = game.board.board.width as isize + 1 + (4 - piece_data.initial_width as isize) / 2;
            let color = COLOR_TABLE[*typ];
            for (x_offset, y_offset) in &piece_data.orientation[0] {
                let x = (start_x as isize + (x_index + *x_offset as isize) * cell_size as isize) as usize;
                let y = (start_y as isize + (y_index + *y_offset as isize) * cell_size as isize) as usize;
                buffer.fill_rect(x, y, cell_size, cell_size, color);
            }
            y_index += 3;
        }
    }

    buffer.fill_rect(start_x - 5 * cell_size, start_y + 2 * cell_size, 4 * cell_size, 2 * cell_size, BltPixel::new(0, 0, 0));

    if let Some(typ) = game.hold {
        if let Some(piece_data) = game.board.piece_data.get(typ) {
            let x_index = -5;
            let y_index = 2;
            let color = COLOR_TABLE[typ];
            for (x_offset, y_offset) in &piece_data.orientation[0] {
                let x = (start_x as isize + (x_index + *x_offset as isize) * cell_size as isize) as usize;
                let y = (start_y as isize + (y_index + *y_offset as isize) * cell_size as isize) as usize;
                buffer.fill_rect(x, y, cell_size, cell_size, color);
            }
        }
    }

    buffer.fill_rect(start_x - 1, start_y + 2 * cell_size, 1, (game.board.board.height - 2) * cell_size + 1, BltPixel::new(255, 255, 255));
    buffer.fill_rect(start_x - 1, start_y + 2 * cell_size, game.board.board.width * cell_size + 2, 1, BltPixel::new(255, 255, 255));
    buffer.fill_rect(start_x - 1, start_y + game.board.board.height * cell_size + 1, game.board.board.width * cell_size + 2, 1, BltPixel::new(255, 255, 255));
    buffer.fill_rect(start_x + game.board.board.width * cell_size + 1, start_y + 2 * cell_size, 1, (game.board.board.height - 2) * cell_size + 1, BltPixel::new(255, 255, 255));
}

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<BltPixel>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    fn fill_rect(&mut self, x: usize, y: usize, cx: usize, cy: usize, color: BltPixel) {
        for y1 in y..(y + cy).min(self.height) {
            for x1 in x..(x + cx).min(self.width) {
                if let Some(pixel) = self.pixel(x1, y1) {
                    *pixel = color;
                }
            }
        }
    }

    fn blit(&self, gop: &mut GraphicsOutput) -> uefi::Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}

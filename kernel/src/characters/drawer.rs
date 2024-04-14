mod screen;

use bootloader_api::info::FrameBuffer;
use screen::screenwriter;

pub fn init(framebuffer: &'static mut FrameBuffer) {
    screen::init(framebuffer);
    screenwriter().clear();
}

pub fn clear_screen() {
    screenwriter().clear();
}

pub fn draw_rec(top_left: &(i16, i16), bottom_right: &(i16, i16), r: u8, g: u8, b: u8) {
    for x in top_left.0..bottom_right.0 {
        for y in top_left.1..bottom_right.1 {
            screenwriter().draw_pixel(x as usize, y as usize, r, g, b);
        }
    }
}

pub fn draw_arena(arena_size: &(i16, i16), lives: u8) {
    //- Border
    draw_rec(&(0, 0), &(2, arena_size.1), 0xff, 0xff, 0xff);
    draw_rec(
        &(0, arena_size.1 - 2),
        &(arena_size.0, arena_size.1),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(&(0, 0), &(arena_size.0, 2), 0xff, 0xff, 0xff);
    draw_rec(
        &(arena_size.0 - 2, 0),
        &(arena_size.0, arena_size.1),
        0xff,
        0xff,
        0xff,
    );
    //- Bottom panel
    draw_rec(
        &(0, arena_size.1),
        &(2, arena_size.1 + 100),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(0, arena_size.1 + 100),
        &(arena_size.0, arena_size.1 + 102),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(arena_size.0 - 2, arena_size.1),
        &(arena_size.0, arena_size.1 + 100),
        0xff,
        0xff,
        0xff,
    );
    //- Lives
    match lives {
        0 => {
            clear_player(&(30, arena_size.1 + 70));
            clear_player(&(120, arena_size.1 + 70));
            clear_player(&(210, arena_size.1 + 70));
        }
        1 => {
            draw_player(&(30, arena_size.1 + 70));
            clear_player(&(120, arena_size.1 + 70));
            clear_player(&(210, arena_size.1 + 70));
        }
        2 => {
            draw_player(&(30, arena_size.1 + 70));
            draw_player(&(120, arena_size.1 + 70));
            clear_player(&(210, arena_size.1 + 70));
        }
        3 => {
            draw_player(&(30, arena_size.1 + 70));
            draw_player(&(120, arena_size.1 + 70));
            draw_player(&(210, arena_size.1 + 70));
        }
        _ => {}
    }
}

pub fn draw_win_screen(arena_size: &(i16, i16)) {
    draw_rec(
        &(arena_size.0 / 2 - 150, arena_size.1 / 2 - 50),
        &(arena_size.0 / 2 + 150, arena_size.1 / 2 + 50),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(arena_size.0 / 2 - 148, arena_size.1 / 2 - 48),
        &(arena_size.0 / 2 + 148, arena_size.1 / 2 + 48),
        0,
        0,
        0,
    );
    let writer = screenwriter();
    writer.set_cursor(
        arena_size.0 as usize / 2 - 35,
        arena_size.1 as usize / 2 - 30,
    );
    writer.write_str("YOU WIN!");
    writer.set_cursor(
        arena_size.0 as usize / 2 - 110,
        arena_size.1 as usize / 2 + 10,
    );
    writer.write_str("Press Enter to play again");
}

pub fn draw_lose_screen(arena_size: &(i16, i16)) {
    draw_rec(
        &(arena_size.0 / 2 - 150, arena_size.1 / 2 - 50),
        &(arena_size.0 / 2 + 150, arena_size.1 / 2 + 50),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(arena_size.0 / 2 - 148, arena_size.1 / 2 - 48),
        &(arena_size.0 / 2 + 148, arena_size.1 / 2 + 48),
        0,
        0,
        0,
    );
    let writer = screenwriter();
    writer.set_cursor(
        arena_size.0 as usize / 2 - 35,
        arena_size.1 as usize / 2 - 30,
    );
    writer.write_str("YOU LOSE!");
    writer.set_cursor(
        arena_size.0 as usize / 2 - 110,
        arena_size.1 as usize / 2 + 10,
    );
    writer.write_str("Press Enter to play again");
}

pub fn draw_player(position: &(i16, i16)) {
    // Base layer 1
    draw_rec(
        &(position.0, position.1 - 15),
        &(position.0 + 60, position.1),
        0,
        0xff,
        0,
    );
    // Base layer 2
    draw_rec(
        &(position.0 + 5, position.1 - 20),
        &(position.0 + 55, position.1 - 15),
        0,
        0xff,
        0,
    );
    // Gun
    draw_rec(
        &(position.0 + 22, position.1 - 30),
        &(position.0 + 37, position.1 - 20),
        0,
        0xff,
        0,
    );
    // Gun barrel
    draw_rec(
        &(position.0 + 28, position.1 - 35),
        &(position.0 + 32, position.1 - 30),
        0,
        0xff,
        0,
    );
}

pub fn draw_enemy(position: &(i16, i16)) {
    // Left fang
    draw_rec(
        &(position.0 + 15, position.1 - 5),
        &(position.0 + 20, position.1),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 10, position.1 - 10),
        &(position.0 + 15, position.1 - 5),
        0xff,
        0xff,
        0xff,
    );
    //Right fang
    draw_rec(
        &(position.0 + 35, position.1 - 5),
        &(position.0 + 40, position.1),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 10),
        &(position.0 + 45, position.1 - 5),
        0xff,
        0xff,
        0xff,
    );
    // Left cheek
    draw_rec(
        &(position.0, position.1 - 20),
        &(position.0 + 5, position.1 - 5),
        0xff,
        0xff,
        0xff,
    );
    // Right cheek
    draw_rec(
        &(position.0 + 50, position.1 - 20),
        &(position.0 + 55, position.1 - 5),
        0xff,
        0xff,
        0xff,
    );
    //Body layer 1
    draw_rec(
        &(position.0 + 10, position.1 - 15),
        &(position.0 + 45, position.1 - 10),
        0xff,
        0xff,
        0xff,
    );
    //Body layer 2
    draw_rec(
        &(position.0 + 5, position.1 - 20),
        &(position.0 + 50, position.1 - 15),
        0xff,
        0xff,
        0xff,
    );
    //Body layer 3 (eyes)
    draw_rec(
        &(position.0 + 5, position.1 - 25),
        &(position.0 + 15, position.1 - 20),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 20, position.1 - 25),
        &(position.0 + 35, position.1 - 20),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 25),
        &(position.0 + 50, position.1 - 20),
        0xff,
        0xff,
        0xff,
    );
    //Body layer 4
    draw_rec(
        &(position.0 + 10, position.1 - 30),
        &(position.0 + 45, position.1 - 25),
        0xff,
        0xff,
        0xff,
    );
    //Left antenna
    draw_rec(
        &(position.0 + 15, position.1 - 35),
        &(position.0 + 20, position.1 - 30),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 10, position.1 - 40),
        &(position.0 + 15, position.1 - 35),
        0xff,
        0xff,
        0xff,
    );
    //Left antenna
    draw_rec(
        &(position.0 + 35, position.1 - 35),
        &(position.0 + 40, position.1 - 30),
        0xff,
        0xff,
        0xff,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 40),
        &(position.0 + 45, position.1 - 35),
        0xff,
        0xff,
        0xff,
    );
}

pub fn draw_wall(position: &(i16, i16)) {
    // Layer 1
    draw_rec(
        &(position.0, position.1 - 5),
        &(position.0 + 10, position.1),
        0,
        0xff,
        0,
    );
    draw_rec(
        &(position.0 + 60, position.1 - 5),
        &(position.0 + 70, position.1),
        0,
        0xff,
        0,
    );
    // Layer 2
    draw_rec(
        &(position.0, position.1 - 10),
        &(position.0 + 15, position.1 - 5),
        0,
        0xff,
        0,
    );
    draw_rec(
        &(position.0 + 55, position.1 - 10),
        &(position.0 + 70, position.1 - 5),
        0,
        0xff,
        0,
    );
    // Layer 3
    draw_rec(
        &(position.0, position.1 - 15),
        &(position.0 + 20, position.1 - 10),
        0,
        0xff,
        0,
    );
    draw_rec(
        &(position.0 + 50, position.1 - 15),
        &(position.0 + 70, position.1 - 10),
        0,
        0xff,
        0,
    );
    // Layer 4
    draw_rec(
        &(position.0, position.1 - 20),
        &(position.0 + 25, position.1 - 15),
        0,
        0xff,
        0,
    );
    draw_rec(
        &(position.0 + 45, position.1 - 20),
        &(position.0 + 70, position.1 - 15),
        0,
        0xff,
        0,
    );
    // Layer 5-7
    draw_rec(
        &(position.0, position.1 - 40),
        &(position.0 + 70, position.1 - 20),
        0,
        0xff,
        0,
    );
}

pub fn draw_bullet(position: &(i16, i16)) {
    draw_rec(
        &(position.0, position.1 - 25),
        &(position.0 + 5, position.1),
        0xff,
        0xff,
        0xff,
    );
}

pub fn draw_enemy_bullet(position: &(i16, i16)) {
    draw_rec(
        &(position.0, position.1 - 25),
        &(position.0 + 5, position.1),
        0xff,
        0,
        0,
    );
}

pub fn clear_player(position: &(i16, i16)) {
    draw_rec(
        &(position.0, position.1 - 15),
        &(position.0 + 60, position.1),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 5, position.1 - 20),
        &(position.0 + 55, position.1 - 15),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 22, position.1 - 30),
        &(position.0 + 37, position.1 - 20),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 28, position.1 - 35),
        &(position.0 + 32, position.1 - 30),
        0,
        0,
        0,
    );
}

pub fn clear_enemy(position: &(i16, i16)) {
    // Left fang
    draw_rec(
        &(position.0 + 15, position.1 - 5),
        &(position.0 + 20, position.1),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 10, position.1 - 10),
        &(position.0 + 15, position.1 - 5),
        0,
        0,
        0,
    );
    //Right fang
    draw_rec(
        &(position.0 + 35, position.1 - 5),
        &(position.0 + 40, position.1),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 10),
        &(position.0 + 45, position.1 - 5),
        0,
        0,
        0,
    );
    // Left cheek
    draw_rec(
        &(position.0, position.1 - 20),
        &(position.0 + 5, position.1 - 5),
        0,
        0,
        0,
    );
    // Right cheek
    draw_rec(
        &(position.0 + 50, position.1 - 20),
        &(position.0 + 55, position.1 - 5),
        0,
        0,
        0,
    );
    //Body layer 1
    draw_rec(
        &(position.0 + 10, position.1 - 15),
        &(position.0 + 45, position.1 - 10),
        0,
        0,
        0,
    );
    //Body layer 2
    draw_rec(
        &(position.0 + 5, position.1 - 20),
        &(position.0 + 50, position.1 - 15),
        0,
        0,
        0,
    );
    //Body layer 3 (eyes)
    draw_rec(
        &(position.0 + 5, position.1 - 25),
        &(position.0 + 15, position.1 - 20),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 20, position.1 - 25),
        &(position.0 + 35, position.1 - 20),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 25),
        &(position.0 + 50, position.1 - 20),
        0,
        0,
        0,
    );
    //Body layer 4
    draw_rec(
        &(position.0 + 10, position.1 - 30),
        &(position.0 + 45, position.1 - 25),
        0,
        0,
        0,
    );
    //Left antenna
    draw_rec(
        &(position.0 + 15, position.1 - 35),
        &(position.0 + 20, position.1 - 30),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 10, position.1 - 40),
        &(position.0 + 15, position.1 - 35),
        0,
        0,
        0,
    );
    //Left antenna
    draw_rec(
        &(position.0 + 35, position.1 - 35),
        &(position.0 + 40, position.1 - 30),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 40, position.1 - 40),
        &(position.0 + 45, position.1 - 35),
        0,
        0,
        0,
    );
}

pub fn clear_wall(position: &(i16, i16)) {
    // Layer 1
    draw_rec(
        &(position.0, position.1 - 5),
        &(position.0 + 10, position.1),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 60, position.1 - 5),
        &(position.0 + 70, position.1),
        0,
        0,
        0,
    );
    // Layer 2
    draw_rec(
        &(position.0, position.1 - 10),
        &(position.0 + 15, position.1 - 5),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 55, position.1 - 10),
        &(position.0 + 70, position.1 - 5),
        0,
        0,
        0,
    );
    // Layer 3
    draw_rec(
        &(position.0, position.1 - 15),
        &(position.0 + 20, position.1 - 10),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 50, position.1 - 15),
        &(position.0 + 70, position.1 - 10),
        0,
        0,
        0,
    );
    // Layer 4
    draw_rec(
        &(position.0, position.1 - 20),
        &(position.0 + 25, position.1 - 15),
        0,
        0,
        0,
    );
    draw_rec(
        &(position.0 + 45, position.1 - 20),
        &(position.0 + 70, position.1 - 15),
        0,
        0,
        0,
    );
    // Layer 5-7
    draw_rec(
        &(position.0, position.1 - 40),
        &(position.0 + 70, position.1 - 20),
        0,
        0,
        0,
    );
}

pub fn clear_bullet(position: &(i16, i16)) {
    draw_rec(
        &(position.0, position.1 - 25),
        &(position.0 + 5, position.1),
        0,
        0,
        0,
    );
}

pub fn draw_score(score: &u32, x: i16, y: i16) {
    let writer = screenwriter();
    writer.set_cursor(x as usize, y as usize);
    writer.write_str("Score: ");
    writer.write_number(score);
}

pub fn draw_win_lose(win: &u32, lose: &u32, x: i16, y: i16) {
    let writer = screenwriter();
    writer.set_cursor(x as usize, y as usize);
    writer.write_str("Win: ");
    writer.write_number(win);
    writer.set_cursor(x as usize, (y + 20) as usize);
    writer.write_str("Lose: ");
    writer.write_number(lose);
}

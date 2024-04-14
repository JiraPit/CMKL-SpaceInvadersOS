#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#[macro_use]
extern crate lazy_static;
extern crate alloc;
extern crate spin;

mod allocator;
mod characters;

use alloc::vec::Vec;
use bootloader_api::config::Mapping::Dynamic;
use bootloader_api::info::MemoryRegionKind;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use characters::{collider as col, drawer as drw};
use core::fmt::Write;
use kernel::{serial, HandlerTable};
use pc_keyboard::{DecodedKey, KeyCode};
use spin::Mutex;

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Dynamic); // obtain physical memory offset
    config.kernel_stack_size = 1024 * 1024; // 1 MB stack
    config
};
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

/// Player component
struct Player {
    position: (i16, i16),
    health: u8,
    bullet: Bullet,
    is_visible: bool,
}

impl Player {
    fn new(position: (i16, i16), health: u8) -> Player {
        Player {
            position,
            health,
            is_visible: false,
            bullet: Bullet {
                position,
                shooting: false,
            },
        }
    }

    fn shoot(&mut self) {
        self.bullet.position = (
            self.position.0 + col::PLAYER_COLLIDER_SIZE.0 / 2,
            self.position.1 - col::PLAYER_COLLIDER_SIZE.1,
        );
        self.bullet.shooting = true;
        self.bullet.draw();
    }

    fn draw(&mut self) {
        if !self.is_visible {
            self.is_visible = true;
            drw::draw_player(&self.position);
        }
    }

    fn clear(&mut self) {
        if self.is_visible {
            self.is_visible = false;
            drw::clear_player(&self.position);
        }
    }

    fn collider(&self) -> col::Collider {
        col::player_collider(&self.position)
    }
}

/// Enemy component
struct Enemy {
    position: (i16, i16),
    health: u8,
    bullet: EnemyBullet,
    is_visible: bool,
}

impl Enemy {
    fn new(position: (i16, i16), health: u8) -> Enemy {
        Enemy {
            position,
            health,
            is_visible: false,
            bullet: EnemyBullet {
                position,
                shooting: false,
            },
        }
    }

    fn shoot(&mut self) {
        self.bullet.position = (
            self.position.0 + col::ENEMY_COLLIDER_SIZE.1 / 2,
            self.position.1 + col::ENEMY_COLLIDER_SIZE.1,
        );
        self.bullet.shooting = true;
        self.bullet.draw();
    }

    fn draw(&mut self) {
        if !self.is_visible {
            self.is_visible = true;
            drw::draw_enemy(&self.position);
        }
    }

    fn clear(&mut self) {
        if self.is_visible {
            self.is_visible = false;
            drw::clear_enemy(&self.position);
        }
    }

    fn collider(&self) -> col::Collider {
        col::enemy_collider(&self.position)
    }
}

/// Wall component
struct Wall {
    position: (i16, i16),
    health: u8,
    is_visible: bool,
}

impl Wall {
    fn new(position: (i16, i16), health: u8) -> Wall {
        Wall {
            position,
            health,
            is_visible: false,
        }
    }

    fn draw(&mut self) {
        if !self.is_visible {
            self.is_visible = true;
            drw::draw_wall(&self.position);
        }
    }

    fn clear(&mut self) {
        if self.is_visible {
            self.is_visible = false;
            drw::clear_wall(&self.position);
        }
    }

    fn collider(&self) -> col::Collider {
        col::wall_collider(&self.position)
    }
}

/// Bullet component
struct Bullet {
    position: (i16, i16),
    shooting: bool,
}

impl Bullet {
    fn draw(&self) {
        drw::draw_bullet(&self.position);
    }

    fn clear(&self) {
        drw::clear_bullet(&self.position);
    }

    fn collider(&self) -> col::Collider {
        col::bullet_collider(&self.position)
    }
}

struct EnemyBullet {
    position: (i16, i16),
    shooting: bool,
}

impl EnemyBullet {
    fn draw(&self) {
        drw::draw_enemy_bullet(&self.position);
    }

    fn clear(&self) {
        drw::clear_bullet(&self.position);
    }

    fn collider(&self) -> col::Collider {
        col::bullet_collider(&self.position)
    }
}

lazy_static! {
    //- Stats & Config initialization
    static ref ARENA_SIZE: (i16, i16) = (780, 525);
    static ref FRAME_COUNT: Mutex<u16> = Mutex::new(500);
    static ref SCORE: Mutex<u32> = Mutex::new(0);
    static ref WIN: Mutex<u32> = Mutex::new(0);
    static ref LOSE: Mutex<u32> = Mutex::new(0);
    static ref IS_RUNNING: Mutex<bool> = Mutex::new(true);

    //- Components initialization with none
    static ref PLAYER: Mutex<Option<Player>> = Mutex::new(None);
    static ref WALLS: Mutex<Option<Vec<Wall>>> = Mutex::new(None);
    static ref ENEMIES: Mutex<Option<Vec<Enemy>>> = Mutex::new(None);
    static ref ENEMY_DIRECTION: Mutex<Option<(i16,i16)>> = Mutex::new(None);
}

/// Kernel entry point
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    writeln!(serial(), "Entered kernel with boot info: {boot_info:?}").unwrap();

    //- Memory Initialization
    let usable_region = boot_info
        .memory_regions
        .iter()
        .filter(|x| x.kind == MemoryRegionKind::Usable)
        .last()
        .unwrap();
    let physical_offset = boot_info.physical_memory_offset.into_option().unwrap();
    allocator::init_heap(
        (physical_offset + usable_region.start) as usize,
        (physical_offset + usable_region.end) as usize,
    );

    //- Screen Initialization
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    drw::init(framebuffer);
    setup_game();

    //- Start game
    HandlerTable::new().timer(update).keyboard(keyboard).start();
}

/// Setup the game, called once at the beginning of each game
fn setup_game() {
    let mut is_running = IS_RUNNING.lock();
    let mut player = PLAYER.lock();
    let mut frame_count = FRAME_COUNT.lock();
    let mut enemies = ENEMIES.lock();
    let mut walls = WALLS.lock();
    let mut enemy_direction = ENEMY_DIRECTION.lock();
    let score = SCORE.lock();
    let win = WIN.lock();
    let lose = LOSE.lock();

    //- Components initialization
    *frame_count = 500;
    *player = Some(Player::new((285, 505), 3));
    *enemies = Some(Vec::from([
        // Row 4
        Enemy::new((30, 95), 1),
        Enemy::new((105, 95), 1),
        Enemy::new((180, 95), 1),
        Enemy::new((255, 95), 1),
        Enemy::new((330, 95), 1),
        Enemy::new((405, 95), 1),
        // Row 3
        Enemy::new((30, 155), 1),
        Enemy::new((105, 155), 1),
        Enemy::new((180, 155), 1),
        Enemy::new((255, 155), 1),
        Enemy::new((330, 155), 1),
        Enemy::new((405, 155), 1),
        // Row 2
        Enemy::new((30, 215), 1),
        Enemy::new((105, 215), 1),
        Enemy::new((180, 215), 1),
        Enemy::new((255, 215), 1),
        Enemy::new((330, 215), 1),
        Enemy::new((405, 215), 1),
        // Row 1
        Enemy::new((30, 275), 1),
        Enemy::new((105, 275), 1),
        Enemy::new((180, 275), 1),
        Enemy::new((255, 275), 1),
        Enemy::new((330, 275), 1),
        Enemy::new((405, 275), 1),
    ]));
    *walls = Some(Vec::from([
        Wall::new((100, 435), 20),
        Wall::new((270, 435), 20),
        Wall::new((440, 435), 20),
        Wall::new((610, 435), 20),
    ]));
    *enemy_direction = Some((4, 0));

    //- Render game
    drw::clear_screen();
    drw::draw_arena(&ARENA_SIZE, player.as_mut().unwrap().health);
    drw::draw_score(&score, ARENA_SIZE.0 - 130, ARENA_SIZE.1 + 20);
    drw::draw_win_lose(&win, &lose, ARENA_SIZE.0 - 130, ARENA_SIZE.1 + 40);
    player.as_mut().unwrap().draw();
    for wall in walls.as_mut().unwrap().iter_mut() {
        wall.draw();
    }
    for enemy in enemies.as_mut().unwrap().iter_mut() {
        enemy.draw();
    }
    *is_running = true;
}

/// Update the game, called every frame while the game is running
fn update() {
    let mut frame_count = FRAME_COUNT.lock();
    //- Update frame count
    *frame_count += 1;
    if *frame_count == u16::MAX {
        *frame_count = 500;
    }
    //- Check if the game is running
    let mut is_running = IS_RUNNING.lock();
    if *is_running && *frame_count % 2 == 0 {
        let mut score = SCORE.lock();
        let mut lose = LOSE.lock();
        let mut win = WIN.lock();
        let mut player = PLAYER.lock();
        let mut enemies = ENEMIES.lock();
        let mut walls = WALLS.lock();
        let mut enemy_direction = ENEMY_DIRECTION.lock();

        let player = player.as_mut().unwrap();
        let enemies = enemies.as_mut().unwrap();
        let walls = walls.as_mut().unwrap();
        let enemy_direction = enemy_direction.as_mut().unwrap();

        //- Update enemy direction
        for enemy in enemies.iter_mut() {
            if enemy.position.0 > ARENA_SIZE.0 - col::ENEMY_COLLIDER_SIZE.0 - 10 {
                enemy_direction.0 = -4;
                enemy_direction.1 = 20;
                break;
            }
            if enemy.position.0 < 10 {
                enemy_direction.0 = 4;
                enemy_direction.1 = 20;
                break;
            }
        }

        //- Check if the player has won
        if enemies.is_empty() {
            *win += 1;
            *is_running = false;
            drw::clear_screen();
            drw::draw_win_screen(&ARENA_SIZE);
            return;
        }

        //- Move enemies and their bullets
        for (i, enemy) in enemies.iter_mut().enumerate() {
            enemy.clear();
            enemy.position.0 += enemy_direction.0;
            enemy.position.1 += enemy_direction.1;
            enemy.draw();
            //- Check enemy collision on walls
            for wall in walls.iter_mut() {
                if enemy.collider().collides_with(&wall.collider()) {
                    wall.health = 0;
                    wall.clear();
                }
            }
            walls.retain(|x| x.health > 0);
            //- Check enemy collision on player
            if player.health > 0 && enemy.collider().collides_with(&player.collider()) {
                player.health = 0;
                player.clear();
                *lose += 1;
                *is_running = false;
                if *score >= 120 {
                    *score -= 120;
                } else {
                    *score = 0;
                }
                drw::clear_screen();
                drw::draw_lose_screen(&ARENA_SIZE);
                return;
            }
            //- Move enemy bullet and check for collision
            if enemy.bullet.shooting {
                enemy.bullet.clear();
                enemy.bullet.position.1 += 20;
                if enemy.bullet.position.1 > ARENA_SIZE.1 - 10 {
                    enemy.bullet.shooting = false;
                } else {
                    enemy.bullet.draw();
                    //- Check enemy bullet collision on walls
                    for wall in walls.iter_mut() {
                        if enemy.bullet.collider().collides_with(&wall.collider()) {
                            enemy.bullet.clear();
                            enemy.bullet.shooting = false;
                            wall.health -= 1;
                            if wall.health == 0 {
                                wall.clear();
                            }
                            break;
                        }
                    }
                    walls.retain(|x| x.health > 0);
                    //- Check enemy bullet collision on player
                    if player.health > 0
                        && enemy.bullet.collider().collides_with(&player.collider())
                    {
                        enemy.bullet.clear();
                        enemy.bullet.shooting = false;
                        player.health -= 1;
                        drw::draw_arena(&ARENA_SIZE, player.health);
                        if player.health == 0 {
                            player.clear();
                            *lose += 1;
                            *is_running = false;
                            if *score >= 120 {
                                *score -= 120;
                            } else {
                                *score = 0;
                            }
                            drw::clear_screen();
                            drw::draw_lose_screen(&ARENA_SIZE);
                            return;
                        }
                        break;
                    }
                }
            //- Shoot enemy bullet
            } else if *frame_count % ((i as u16 + 20) * 5) == 0 && enemy.health != 0 {
                enemy.shoot();
            }
        }
        //- Clear enemy direction on Y axis
        enemy_direction.1 = 0;
        //- Move player bullet and check for collision
        if player.bullet.shooting {
            player.bullet.clear();
            player.bullet.position.1 -= 50;
            if player.bullet.position.1 > col::BULLET_COLLIDER_SIZE.1 {
                player.bullet.draw();
            } else {
                player.bullet.shooting = false;
            }
            //- Check for bullet collision on enemies
            for enemy in enemies.iter_mut() {
                if player.bullet.collider().collides_with(&enemy.collider()) {
                    player.bullet.clear();
                    player.bullet.shooting = false;
                    enemy.health -= 1;
                    if enemy.health == 0 {
                        if *score > 994 {
                            *score = 999;
                        } else {
                            *score += 5;
                        }
                        enemy.clear();
                        enemy.bullet.clear();
                        drw::draw_score(&score, ARENA_SIZE.0 - 130, ARENA_SIZE.1 + 20);
                    }
                    break;
                }
            }
            enemies.retain(|x| x.health > 0);
        }
        //- Check for bullet collision on walls
        if player.bullet.shooting {
            for wall in walls.iter_mut() {
                if player.bullet.collider().collides_with(&wall.collider()) {
                    player.bullet.clear();
                    player.bullet.shooting = false;
                    wall.health -= 1;
                    if wall.health == 0 {
                        wall.clear();
                    }
                    break;
                }
            }
        }
        walls.retain(|x| x.health > 0);
    }
}

/// Handle keyboard input
fn keyboard(key: DecodedKey) {
    match key {
        DecodedKey::RawKey(key_code) => match key_code {
            KeyCode::ArrowLeft => {
                let mut player = PLAYER.lock();
                let is_running = IS_RUNNING.lock();
                if *is_running
                    && player.as_ref().unwrap().health != 0
                    && player.as_ref().unwrap().position.0 > 15
                {
                    player.as_mut().unwrap().clear();
                    player.as_mut().unwrap().position.0 -= 15;
                    player.as_mut().unwrap().draw();
                }
            }
            KeyCode::ArrowRight => {
                let mut player = PLAYER.lock();
                let is_running = IS_RUNNING.lock();
                if *is_running
                    && player.as_ref().unwrap().health != 0
                    && player.as_ref().unwrap().position.0
                        < ARENA_SIZE.0 - col::PLAYER_COLLIDER_SIZE.0 - 15
                {
                    player.as_mut().unwrap().clear();
                    player.as_mut().unwrap().position.0 += 15;
                    player.as_mut().unwrap().draw();
                }
            }
            _ => {}
        },
        DecodedKey::Unicode(character) => match character {
            ' ' => {
                let mut player = PLAYER.lock();
                let is_running = IS_RUNNING.lock();
                if *is_running
                    && player.as_ref().unwrap().health != 0
                    && !player.as_ref().unwrap().bullet.shooting
                {
                    player.as_mut().unwrap().shoot();
                }
            }
            '\n' => {
                let is_running = IS_RUNNING.lock();
                if !*is_running {
                    drop(is_running);
                    setup_game();
                }
            }
            _ => {}
        },
    }
}

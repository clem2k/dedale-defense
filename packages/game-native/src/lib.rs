use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, set_sound_volume,
};
use macroquad::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

include!(concat!(env!("OUT_DIR"), "/content.rs"));
const COUNT: usize = COLS * ROWS;
const BOARD_W: f32 = COLS as f32 * CELL;
const BOARD_H: f32 = ROWS as f32 * CELL;
const BOARD_X: f32 = 18.0;
const BOARD_Y: f32 = 74.0;

fn window_size() -> Vec2 {
    vec2(
        BOARD_X + BOARD_W + 382.0,
        (BOARD_Y + BOARD_H + 126.0)
            .max(640.0)
            .max(tower_info_y() + 320.0)
            .max(308.0 + Mode::ALL.len() as f32 * 46.0),
    )
}

pub fn window_conf() -> Conf {
    let size = window_size();
    Conf {
        window_title: "Dedale Defense".to_owned(),
        window_width: size.x as i32,
        window_height: size.y as i32,
        ..Default::default()
    }
}

struct Viewport {
    offset: Vec2,
    size: Vec2,
    scale: f32,
}

impl Viewport {
    fn fit(screen: Vec2, logical: Vec2) -> Self {
        let scale = (screen.x / logical.x).min(screen.y / logical.y);
        let size = logical * scale;
        Self {
            offset: (screen - size) / 2.0,
            size,
            scale,
        }
    }

    fn pointer(&self, position: Vec2) -> Vec2 {
        (position - self.offset) / self.scale
    }
}

fn ui_width() -> f32 {
    if cfg!(target_os = "android") {
        window_size().x
    } else {
        screen_width()
    }
}

fn ui_height() -> f32 {
    if cfg!(target_os = "android") {
        window_size().y
    } else {
        screen_height()
    }
}

fn pointer_position() -> (f32, f32) {
    let (x, y) = mouse_position();
    if cfg!(target_os = "android") {
        let viewport = Viewport::fit(vec2(screen_width(), screen_height()), window_size());
        let position = viewport.pointer(vec2(x, y));
        (position.x, position.y)
    } else {
        (x, y)
    }
}

fn begin_ui() {
    set_default_camera();
    clear_background(Color::new(0.035, 0.055, 0.085, 1.0));
    if cfg!(target_os = "android") {
        let size = window_size();
        let framebuffer = vec2(screen_width(), screen_height()) * screen_dpi_scale();
        let viewport = Viewport::fit(framebuffer, size);
        set_camera(&Camera2D {
            viewport: Some((
                viewport.offset.x.round() as i32,
                viewport.offset.y.round() as i32,
                viewport.size.x.round() as i32,
                viewport.size.y.round() as i32,
            )),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, size.x, size.y))
        });
    }
}

const FX_BYTES: [&[u8]; 7] = [
    include_bytes!(concat!(env!("OUT_DIR"), "/01.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/02.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/03.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/04.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/05.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/06.wav")),
    include_bytes!(concat!(env!("OUT_DIR"), "/HIT.wav")),
];

fn tower_fx(kind: TowerKind) -> usize {
    TOWERS[kind.index()].fx
}

struct AudioSettings {
    volume: u8,
    muted: bool,
}

impl AudioSettings {
    fn effective_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.volume as f32 / 100.0
        }
    }
}

struct FxAudio {
    sounds: Vec<Sound>,
    settings: AudioSettings,
    options_open: bool,
    dragging: bool,
}

impl FxAudio {
    async fn load() -> Self {
        let mut sounds = Vec::new();
        for bytes in FX_BYTES {
            sounds.push(
                load_sound_from_bytes(bytes)
                    .await
                    .expect("cannot load embedded FX"),
            );
        }
        Self {
            sounds,
            settings: AudioSettings {
                volume: 50,
                muted: false,
            },
            options_open: false,
            dragging: false,
        }
    }

    fn play_pending(&self, pending: &mut Vec<usize>) {
        let volume = self.settings.effective_volume();
        for index in pending.drain(..) {
            if volume > 0.0 {
                play_sound(
                    &self.sounds[index],
                    PlaySoundParams {
                        looped: false,
                        volume,
                    },
                );
            }
        }
    }

    fn draw_controls(&mut self) {
        let previous = self.settings.effective_volume();
        if button(
            Rect::new(
                ui_width() - 230.0,
                18.0,
                100.0,
                if cfg!(target_os = "android") {
                    44.0
                } else {
                    32.0
                },
            ),
            "Options",
            false,
            !self.options_open,
        ) {
            self.options_open = true;
        }
        if button(
            Rect::new(
                ui_width() - 120.0,
                18.0,
                100.0,
                if cfg!(target_os = "android") {
                    44.0
                } else {
                    32.0
                },
            ),
            if self.settings.muted {
                "Mute ON"
            } else {
                "Mute OFF"
            },
            self.settings.muted,
            !self.options_open,
        ) {
            self.settings.muted = !self.settings.muted;
        }
        if self.options_open {
            draw_rectangle(
                0.0,
                0.0,
                ui_width(),
                ui_height(),
                Color::new(0.02, 0.04, 0.07, 0.9),
            );
            let x = (ui_width() - 400.0) / 2.0;
            let y = (ui_height() - 250.0) / 2.0;
            draw_rectangle(x, y, 400.0, 250.0, PANEL);
            draw_rectangle_lines(x, y, 400.0, 250.0, 1.0, LINE);
            draw_text("OPTIONS", x + 24.0, y + 36.0, 24.0, TEXT);
            draw_text(
                "Partie suspendue pendant les reglages.",
                x + 24.0,
                y + 64.0,
                16.0,
                MUTED,
            );
            draw_text(
                &format!("Volume FX : {} %", self.settings.volume),
                x + 24.0,
                y + 100.0,
                20.0,
                TEXT,
            );
            let slider = Rect::new(x + 24.0, y + 118.0, 352.0, 24.0);
            let mouse = pointer_position();
            if is_mouse_button_pressed(MouseButton::Left) {
                self.dragging = slider.contains(vec2(mouse.0, mouse.1));
            }
            if self.dragging && is_mouse_button_down(MouseButton::Left) {
                self.settings.volume =
                    (((mouse.0 - slider.x) / slider.w).clamp(0.0, 1.0) * 100.0).round() as u8;
            }
            if is_mouse_button_released(MouseButton::Left) {
                self.dragging = false;
            }
            if is_key_pressed(KeyCode::Left) {
                self.settings.volume = self.settings.volume.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Right) {
                self.settings.volume = self.settings.volume.saturating_add(1).min(100);
            }
            if is_key_pressed(KeyCode::Home) {
                self.settings.volume = 0;
            }
            if is_key_pressed(KeyCode::End) {
                self.settings.volume = 100;
            }
            draw_rectangle(slider.x, slider.y + 10.0, slider.w, 4.0, LINE);
            draw_circle(
                slider.x + slider.w * self.settings.volume as f32 / 100.0,
                slider.y + 12.0,
                8.0,
                GOLD,
            );
            draw_text(
                if cfg!(target_os = "android") {
                    "Glissez pour regler le volume."
                } else {
                    "Fleches : volume. Le mute garde ce reglage."
                },
                x + 24.0,
                y + 172.0,
                15.0,
                MUTED,
            );
            if button(
                Rect::new(x + 140.0, y + 194.0, 120.0, 32.0),
                "Fermer",
                true,
                true,
            ) || is_key_pressed(KeyCode::Escape)
            {
                self.options_open = false;
                self.dragging = false;
            }
        }
        let volume = self.settings.effective_volume();
        if volume != previous {
            for sound in &self.sounds {
                set_sound_volume(sound, volume);
            }
        }
    }
}
#[derive(Clone, Copy)]
struct SpriteDef {
    texture: usize,
    x: f32,
    y: f32,
    frame_size: f32,
    scale: f32,
}

const BG: Color = Color::new(0.055, 0.085, 0.13, 1.0);
const PANEL: Color = Color::new(0.09, 0.13, 0.19, 1.0);
const LINE: Color = Color::new(0.20, 0.26, 0.34, 1.0);
const TEXT: Color = Color::new(0.91, 0.93, 0.96, 1.0);
const MUTED: Color = Color::new(0.58, 0.64, 0.72, 1.0);
const GOLD: Color = Color::new(0.98, 0.77, 0.08, 1.0);
const GREEN: Color = Color::new(0.23, 0.80, 0.50, 1.0);
const RED: Color = Color::new(0.92, 0.22, 0.32, 1.0);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Target {
    Right,
    Bottom,
}

#[derive(Clone)]
struct Tower {
    id: u64,
    kind: TowerKind,
    cx: usize,
    cy: usize,
    level: usize,
    cooldown: f32,
    angle: f32,
    spent: i32,
}

#[derive(Clone)]
struct Creep {
    id: u64,
    kind: EnemyKind,
    x: f32,
    y: f32,
    hp: f32,
    max_hp: f32,
    bounty: f32,
    target: Target,
    flying: bool,
    cell: Option<usize>,
    next_cell: Option<usize>,
    wx: f32,
    wy: f32,
    exiting: bool,
    slow_until: f32,
    slow_factor: f32,
    stun_until: f32,
    dead: bool,
}

struct Spawner {
    def: WaveDef,
    left: usize,
    spawned: usize,
    next_in: f32,
}

struct Shot {
    target_id: u64,
    x: f32,
    y: f32,
    speed: f32,
    damage: f32,
    color: Color,
    origin: (f32, f32),
    range: f32,
    special: &'static str,
    factor: f32,
    kind: TowerKind,
}

struct Game {
    mode: Mode,
    lives: i32,
    gold: i32,
    score: i32,
    kills: u32,
    wave: u32,
    timer: f32,
    started: bool,
    paused: bool,
    over: bool,
    won: bool,
    game_time: f32,
    towers: Vec<Tower>,
    occupancy: Vec<Option<u64>>,
    creeps: Vec<Creep>,
    spawners: Vec<Spawner>,
    shots: Vec<Shot>,
    field_right: Vec<u32>,
    field_bottom: Vec<u32>,
    next_id: u64,
    build_kind: Option<TowerKind>,
    selected_id: Option<u64>,
    keep_building: bool,
    speed: f32,
    message: String,
    message_time: f32,
    start_hint_until: Option<Instant>,
    pending_audio: Vec<usize>,
}

impl Game {
    fn new(mode: Mode) -> Self {
        let mut game = Self {
            mode,
            lives: mode.lives(),
            gold: mode.gold(),
            score: 0,
            kills: 0,
            wave: 0,
            timer: 0.0,
            started: false,
            paused: false,
            over: false,
            won: false,
            game_time: 0.0,
            towers: Vec::new(),
            occupancy: vec![None; COUNT],
            creeps: Vec::new(),
            spawners: Vec::new(),
            shots: Vec::new(),
            field_right: Vec::new(),
            field_bottom: Vec::new(),
            next_id: 1,
            build_kind: None,
            selected_id: None,
            keep_building: false,
            speed: 1.0,
            message: String::new(),
            message_time: 0.0,
            start_hint_until: Some(Instant::now() + Duration::from_secs(5)),
            pending_audio: Vec::new(),
        };
        game.refresh_fields();
        game
    }

    fn show_start_hint(&self, now: Instant) -> bool {
        !self.started && !self.over && self.start_hint_until.is_some_and(|deadline| now < deadline)
    }

    fn dismiss_start_hint(&mut self) {
        self.start_hint_until = None;
    }

    fn towers_at(&self, x: usize, y: usize) -> Option<u64> {
        if x >= COLS || y >= ROWS {
            return None;
        }
        self.occupancy[index(x, y)]
    }

    fn is_wall(x: usize, y: usize) -> bool {
        let edge = x == 0 || y == 0 || x == COLS - 1 || y == ROWS - 1;
        let open_lr = (x == 0 || x == COLS - 1) && OPEN_R.contains(&y);
        let open_tb = (y == 0 || y == ROWS - 1) && OPEN_C.contains(&x);
        edge && !open_lr && !open_tb
    }

    fn free(&self, x: isize, y: isize, blocked: &[usize]) -> bool {
        if x < 0 || y < 0 || x >= COLS as isize || y >= ROWS as isize {
            return false;
        }
        let i = index(x as usize, y as usize);
        !Self::is_wall(x as usize, y as usize)
            && self.occupancy[i].is_none()
            && !blocked.contains(&i)
    }

    fn compute_field(&self, targets: &[usize], blocked: &[usize]) -> Vec<u32> {
        const INF: u32 = u32::MAX / 4;
        const NEIGHBORS: [(isize, isize, u32); 8] = [
            (1, 0, 10),
            (-1, 0, 10),
            (0, 1, 10),
            (0, -1, 10),
            (1, 1, 14),
            (1, -1, 14),
            (-1, 1, 14),
            (-1, -1, 14),
        ];
        let mut distance = vec![INF; COUNT];
        let mut heap = BinaryHeap::new();
        for &target in targets {
            distance[target] = 0;
            heap.push(Reverse((0_u32, target)));
        }
        while let Some(Reverse((cost, current))) = heap.pop() {
            if cost > distance[current] {
                continue;
            }
            let x = (current % COLS) as isize;
            let y = (current / COLS) as isize;
            for (dx, dy, step) in NEIGHBORS {
                let nx = x + dx;
                let ny = y + dy;
                if !self.free(nx, ny, blocked) {
                    continue;
                }
                if dx != 0
                    && dy != 0
                    && (!self.free(x + dx, y, blocked) || !self.free(x, y + dy, blocked))
                {
                    continue;
                }
                let next = index(nx as usize, ny as usize);
                let new_cost = cost + step;
                if new_cost < distance[next] {
                    distance[next] = new_cost;
                    heap.push(Reverse((new_cost, next)));
                }
            }
        }
        distance
    }

    fn refresh_fields(&mut self) {
        let right: Vec<usize> = OPEN_R.iter().map(|&y| index(COLS - 1, y)).collect();
        let bottom: Vec<usize> = OPEN_C.iter().map(|&x| index(x, ROWS - 1)).collect();
        self.field_right = self.compute_field(&right, &[]);
        self.field_bottom = self.compute_field(&bottom, &[]);
    }

    fn placement_cells(cx: usize, cy: usize) -> [usize; 4] {
        [
            index(cx, cy),
            index(cx + 1, cy),
            index(cx, cy + 1),
            index(cx + 1, cy + 1),
        ]
    }

    fn can_place(&self, cx: usize, cy: usize) -> Result<(), &'static str> {
        if cx < 1 || cy < 1 || cx + 1 > COLS - 2 || cy + 1 > ROWS - 2 {
            return Err("Hors du plateau");
        }
        let cells = Self::placement_cells(cx, cy);
        for &cell in &cells {
            let x = cell % COLS;
            let y = cell / COLS;
            if self.occupancy[cell].is_some() || Self::is_wall(x, y) {
                return Err("Case occupee");
            }
        }
        if self.creeps.iter().any(|creep| {
            !creep.flying
                && (creep.cell.is_some_and(|c| cells.contains(&c))
                    || creep.next_cell.is_some_and(|c| cells.contains(&c)))
        }) {
            return Err("Un ennemi est la");
        }
        let right = self.compute_field(
            &OPEN_R
                .iter()
                .map(|&y| index(COLS - 1, y))
                .collect::<Vec<_>>(),
            &cells,
        );
        let bottom = self.compute_field(
            &OPEN_C
                .iter()
                .map(|&x| index(x, ROWS - 1))
                .collect::<Vec<_>>(),
            &cells,
        );
        let entries_reachable = OPEN_R.iter().all(|&y| right[index(0, y)] < u32::MAX / 4)
            && OPEN_C.iter().all(|&x| bottom[index(x, 0)] < u32::MAX / 4);
        let creeps_reachable = self.creeps.iter().all(|creep| {
            if creep.flying {
                return true;
            }
            let Some(cell) = creep.next_cell else {
                return true;
            };
            let field = if creep.target == Target::Right {
                &right
            } else {
                &bottom
            };
            field[cell] < u32::MAX / 4
        });
        if !entries_reachable || !creeps_reachable {
            return Err("Ne bloquez pas le passage !");
        }
        Ok(())
    }

    fn place_tower(&mut self, cx: usize, cy: usize) {
        if self.over {
            return;
        }
        let Some(kind) = self.build_kind else {
            return;
        };
        let def = TOWERS[kind.index()];
        if self.gold < def.cost {
            self.flash("Pas assez d'or");
            return;
        }
        if let Err(message) = self.can_place(cx, cy) {
            self.flash(message);
            return;
        }
        let id = self.next_id;
        self.next_id += 1;
        for cell in Self::placement_cells(cx, cy) {
            self.occupancy[cell] = Some(id);
        }
        self.towers.push(Tower {
            id,
            kind,
            cx,
            cy,
            level: 0,
            cooldown: 0.0,
            angle: -std::f32::consts::FRAC_PI_2,
            spent: def.cost,
        });
        self.gold -= def.cost;
        self.refresh_fields();
        if !self.keep_building {
            self.build_kind = None;
        }
    }

    fn upgrade_selected(&mut self) {
        if self.over {
            return;
        }
        let Some(id) = self.selected_id else { return };
        let Some(index) = self.towers.iter().position(|tower| tower.id == id) else {
            self.selected_id = None;
            return;
        };
        let tower = &self.towers[index];
        if tower.level >= TOWERS[tower.kind.index()].upgrades.len() {
            return;
        }
        let cost = TOWERS[tower.kind.index()].upgrades[tower.level];
        if self.gold < cost {
            self.flash("Pas assez d'or");
            return;
        }
        self.gold -= cost;
        self.towers[index].spent += cost;
        self.towers[index].level += 1;
    }

    fn sell_selected(&mut self) {
        if self.over {
            return;
        }
        let Some(id) = self.selected_id.take() else {
            return;
        };
        let Some(position) = self.towers.iter().position(|tower| tower.id == id) else {
            return;
        };
        let tower = self.towers.remove(position);
        let refund = if self.started {
            TOWERS[tower.kind.index()].sell[tower.level]
        } else {
            tower.spent
        };
        self.gold += refund;
        for cell in Self::placement_cells(tower.cx, tower.cy) {
            self.occupancy[cell] = None;
        }
        self.refresh_fields();
    }

    fn flash(&mut self, message: &str) {
        self.message = message.to_owned();
        self.message_time = 1.6;
    }

    fn wave_def(wave: u32, mode: Mode) -> WaveDef {
        assert!(wave > 0 && wave <= mode.waves(), "invalid wave for level");
        mode.def().wave_set[(wave - 1) as usize]
    }

    fn send_wave(&mut self) {
        if self.over || self.wave >= self.mode.waves() {
            return;
        }
        if self.started && self.timer > 0.0 {
            self.score += self.timer.ceil() as i32 * (self.wave as i32 + 1);
        }
        self.started = true;
        self.wave += 1;
        let def = Self::wave_def(self.wave, self.mode);
        self.spawners.push(Spawner {
            def,
            left: def.count,
            spawned: 0,
            next_in: 0.0,
        });
        self.timer = self.mode.wave_gap();
    }

    fn spawn(&mut self, kind: EnemyKind, base_hp: f32, bounty: f32, target: Target) {
        let lanes = if target == Target::Right {
            &OPEN_R[..]
        } else {
            &OPEN_C[..]
        };
        let lane = lanes[macroquad::rand::gen_range(0, lanes.len())];
        let (x, y, cell) = if target == Target::Right {
            (-CELL * 0.6, lane as f32 * CELL + CELL * 0.5, index(0, lane))
        } else {
            (lane as f32 * CELL + CELL * 0.5, -CELL * 0.6, index(lane, 0))
        };
        let flying = kind.is_flying();
        let id = self.next_id;
        self.next_id += 1;
        self.creeps.push(Creep {
            id,
            kind,
            x,
            y,
            hp: base_hp * kind.hp_factor() * self.mode.hp_factor(),
            max_hp: base_hp * kind.hp_factor() * self.mode.hp_factor(),
            bounty,
            target,
            flying,
            cell: None,
            next_cell: if flying { None } else { Some(cell) },
            wx: if flying && target == Target::Right {
                BOARD_W + CELL
            } else if flying {
                x
            } else {
                (cell % COLS) as f32 * CELL + CELL * 0.5
            },
            wy: if flying && target == Target::Bottom {
                BOARD_H + CELL
            } else if flying {
                y
            } else {
                (cell / COLS) as f32 * CELL + CELL * 0.5
            },
            exiting: false,
            slow_until: 0.0,
            slow_factor: 0.0,
            stun_until: 0.0,
            dead: false,
        });
    }

    fn remaining(&self, creep: &Creep) -> f32 {
        if creep.flying {
            return if creep.target == Target::Right {
                BOARD_W - creep.x
            } else {
                BOARD_H - creep.y
            };
        }
        if creep.exiting {
            return 0.0;
        }
        let field = if creep.target == Target::Right {
            &self.field_right
        } else {
            &self.field_bottom
        };
        creep
            .next_cell
            .map(|cell| {
                field[cell] as f32 / 10.0 * CELL + (creep.wx - creep.x).hypot(creep.wy - creep.y)
            })
            .unwrap_or(f32::INFINITY)
    }

    fn next_step(&self, field: &[u32], cell: usize) -> Option<usize> {
        const NEIGHBORS: [(isize, isize, u32); 8] = [
            (1, 0, 10),
            (-1, 0, 10),
            (0, 1, 10),
            (0, -1, 10),
            (1, 1, 14),
            (1, -1, 14),
            (-1, 1, 14),
            (-1, -1, 14),
        ];
        let x = (cell % COLS) as isize;
        let y = (cell / COLS) as isize;
        NEIGHBORS
            .iter()
            .filter_map(|(dx, dy, cost)| {
                let nx = x + dx;
                let ny = y + dy;
                if !self.free(nx, ny, &[]) {
                    return None;
                }
                if *dx != 0
                    && *dy != 0
                    && (!self.free(x + dx, y, &[]) || !self.free(x, y + dy, &[]))
                {
                    return None;
                }
                let next = index(nx as usize, ny as usize);
                Some((field[next].saturating_add(*cost), next))
            })
            .min_by_key(|(cost, _)| *cost)
            .and_then(|(cost, next)| (cost < u32::MAX / 4).then_some(next))
    }

    fn move_creep(&mut self, index: usize, dt: f32) {
        if self.creeps[index].dead || self.game_time < self.creeps[index].stun_until {
            return;
        }
        let mut speed = self.creeps[index].kind.speed()
            * if self.creeps[index].target == Target::Right {
                ENEMY_SPEED_X
            } else {
                ENEMY_SPEED_Y
            };
        if self.game_time < self.creeps[index].slow_until {
            speed *= 1.0 - self.creeps[index].slow_factor;
        }
        let mut step = speed * dt;
        let mut guard = 0;
        while step > 0.0 && guard < 8 {
            guard += 1;
            let (x, y, wx, wy) = {
                let creep = &self.creeps[index];
                (creep.x, creep.y, creep.wx, creep.wy)
            };
            let dx = wx - x;
            let dy = wy - y;
            let distance = dx.hypot(dy);
            if distance > step {
                self.creeps[index].x += dx / distance * step;
                self.creeps[index].y += dy / distance * step;
                break;
            }
            self.creeps[index].x = wx;
            self.creeps[index].y = wy;
            step -= distance;
            if self.creeps[index].flying || self.creeps[index].exiting {
                self.leak(index);
                return;
            }
            let cell = self.creeps[index].next_cell.unwrap();
            self.creeps[index].cell = Some(cell);
            let field = if self.creeps[index].target == Target::Right {
                &self.field_right
            } else {
                &self.field_bottom
            };
            if field[cell] == 0 {
                self.creeps[index].exiting = true;
                self.creeps[index].next_cell = None;
                if self.creeps[index].target == Target::Right {
                    self.creeps[index].wx = BOARD_W + CELL;
                } else {
                    self.creeps[index].wy = BOARD_H + CELL;
                }
                continue;
            }
            if let Some(next) = self.next_step(field, cell) {
                let creep = &mut self.creeps[index];
                creep.next_cell = Some(next);
                creep.wx = (next % COLS) as f32 * CELL + CELL * 0.5;
                creep.wy = (next / COLS) as f32 * CELL + CELL * 0.5;
            } else {
                break;
            }
        }
    }

    fn leak(&mut self, index: usize) {
        let loss = self.creeps[index].kind.def().life_loss;
        self.creeps[index].dead = true;
        self.lives = (self.lives - loss).max(0);
        self.pending_audio.push(6);
        if self.lives == 0 {
            self.finish(false);
        }
    }

    fn hurt(&mut self, id: u64, damage: f32) {
        let Some(index) = self
            .creeps
            .iter()
            .position(|creep| creep.id == id && !creep.dead)
        else {
            return;
        };
        self.creeps[index].hp -= damage;
        if self.creeps[index].hp > 0.0 {
            return;
        }
        let creep = self.creeps[index].clone();
        let bounty = creep.bounty;
        let kind = creep.kind;
        self.creeps[index].dead = true;
        let reward = (bounty.round() as i32).max(1);
        self.gold += reward;
        self.score += reward * KILL_SCORE;
        self.kills += 1;
        if let Some(split) = kind.def().split {
            for _ in 0..split.count {
                self.spawn_child(split, &creep);
            }
        }
    }

    fn spawn_child(&mut self, split: SplitDef, parent: &Creep) {
        let kind = split.kind;
        let hp = parent.max_hp * split.hp;
        let x = parent.x + macroquad::rand::gen_range(-0.5, 0.5) * SPLIT_JITTER;
        let y = parent.y + macroquad::rand::gen_range(-0.5, 0.5) * SPLIT_JITTER;
        let flying = kind.is_flying();
        let id = self.next_id;
        self.next_id += 1;
        self.creeps.push(Creep {
            id,
            kind,
            x,
            y,
            hp,
            max_hp: hp,
            bounty: parent.bounty * split.bounty,
            target: parent.target,
            flying,
            cell: None,
            next_cell: if flying { None } else { parent.next_cell },
            wx: if flying {
                if parent.target == Target::Right {
                    BOARD_W + CELL
                } else {
                    x
                }
            } else {
                parent.wx
            },
            wy: if flying {
                if parent.target == Target::Bottom {
                    BOARD_H + CELL
                } else {
                    y
                }
            } else {
                parent.wy
            },
            exiting: !flying && parent.exiting,
            slow_until: 0.0,
            slow_factor: 0.0,
            stun_until: 0.0,
            dead: false,
        });
    }

    fn tower_stats(tower: &Tower) -> (f32, f32, f32, f32) {
        let def = TOWERS[tower.kind.index()];
        (
            def.damage[tower.level],
            def.range[tower.level] * RANGE_SCALE,
            def.rate[tower.level],
            if def.special[tower.level] == "slow" {
                0.5
            } else {
                0.0
            },
        )
    }

    fn fire_towers(&mut self, dt: f32) {
        let mut shots = Vec::new();
        let creep_targets: Vec<(f32, u64, f32, f32, bool, f32)> = self
            .creeps
            .iter()
            .filter(|creep| !creep.dead)
            .map(|creep| {
                (
                    self.remaining(creep),
                    creep.id,
                    creep.x,
                    creep.y,
                    creep.flying,
                    creep.kind.radius(),
                )
            })
            .collect();
        for tower in &mut self.towers {
            tower.cooldown -= dt;
            if tower.cooldown > 0.0 {
                continue;
            }
            let (damage, range, rate, _) = Self::tower_stats(tower);
            let def = TOWERS[tower.kind.index()];
            let x = (tower.cx as f32 + 1.0) * CELL;
            let y = (tower.cy as f32 + 1.0) * CELL;
            let mut targets: Vec<(f32, u64, f32, f32)> = creep_targets
                .iter()
                .filter(|(_, _, cx, cy, flying, _)| {
                    (if *flying { def.air } else { def.ground })
                        && (*cx - x).hypot(*cy - y) <= range
                })
                .map(|(remaining, id, cx, cy, _, _)| (*remaining, *id, *cx, *cy))
                .collect();
            targets.sort_by(|a, b| a.0.total_cmp(&b.0));
            if targets.is_empty() {
                continue;
            }
            tower.angle = (targets[0].3 - y).atan2(targets[0].2 - x);
            self.pending_audio.push(tower_fx(tower.kind));
            for (_, id, _, _) in targets.into_iter().take(def.multi) {
                shots.push(Shot {
                    target_id: id,
                    x: x + tower.angle.cos() * 8.0,
                    y: y + tower.angle.sin() * 8.0,
                    speed: def.speed,
                    damage,
                    color: def.accent,
                    origin: (x, y),
                    range,
                    special: def.special[tower.level],
                    factor: def.factor[tower.level],
                    kind: tower.kind,
                });
            }
            tower.cooldown = rate;
        }
        self.shots.extend(shots);
    }

    fn update_shots(&mut self, dt: f32) {
        let mut pending = Vec::new();
        let mut impacts = Vec::new();
        for mut shot in self.shots.drain(..) {
            if let Some(target) = self
                .creeps
                .iter()
                .find(|creep| creep.id == shot.target_id && !creep.dead)
            {
                let dx = target.x - shot.x;
                let dy = target.y - shot.y;
                let distance = dx.hypot(dy);
                let step = shot.speed * dt;
                if distance <= step + 2.0 {
                    impacts.push((target.x, target.y, shot));
                } else if distance > 0.0 {
                    shot.x += dx / distance * step;
                    shot.y += dy / distance * step;
                    pending.push(shot);
                }
            }
        }
        self.shots = pending;
        for (x, y, shot) in impacts {
            self.impact(&shot, x, y);
        }
    }

    fn impact(&mut self, shot: &Shot, x: f32, y: f32) {
        let def = TOWERS[shot.kind.index()];
        let targets: Vec<u64> = self
            .creeps
            .iter()
            .filter(|c| {
                !c.dead
                    && (if c.flying { def.air } else { def.ground })
                    && (c.x - x).hypot(c.y - y) <= shot.range
                    && (c.x - shot.origin.0).hypot(c.y - shot.origin.1) <= shot.range
            })
            .map(|c| c.id)
            .collect();
        for id in targets {
            if shot.special == "spread" && id != shot.target_id {
                self.hurt(id, shot.factor);
            }
            if let Some(c) = self.creeps.iter_mut().find(|c| c.id == id && !c.dead) {
                if !c.kind.is_immune()
                    && matches!(shot.special, "slow" | "stun")
                    && macroquad::rand::gen_range(0.0, 1.0) < shot.factor
                {
                    if shot.special == "slow" {
                        c.slow_factor = 0.5;
                        c.slow_until = self.game_time + SLOW_DURATION;
                    } else {
                        c.stun_until = self.game_time + STUN_DURATION;
                    }
                }
            }
        }
        self.hurt(shot.target_id, shot.damage);
    }

    fn update(&mut self, dt: f32) {
        if self.paused || self.over {
            return;
        }
        self.game_time += dt;
        if self.message_time > 0.0 {
            self.message_time -= dt;
        }
        if self.started && self.wave < self.mode.waves() {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.timer = 0.0;
                self.send_wave();
            }
        }
        let mut spawns = Vec::new();
        for spawner in &mut self.spawners {
            spawner.next_in -= dt;
            while spawner.left > 0 && spawner.next_in <= 0.0 {
                spawns.push((
                    spawner.def.kind,
                    spawner.def.hp,
                    spawner.def.bounty,
                    spawner.def.targets[spawner.spawned % spawner.def.targets.len()],
                ));
                spawner.left -= 1;
                spawner.spawned += 1;
                spawner.next_in += spawner.def.gap;
            }
        }
        self.spawners.retain(|spawner| spawner.left > 0);
        for (kind, base_hp, bounty, target) in spawns {
            self.spawn(kind, base_hp, bounty, target);
        }
        let creep_count = self.creeps.len();
        for i in 0..creep_count {
            self.move_creep(i, dt);
            if self.over {
                return;
            }
        }
        self.fire_towers(dt);
        self.update_shots(dt);
        self.creeps.retain(|creep| !creep.dead);
        if self.wave >= self.mode.waves() && self.spawners.is_empty() && self.creeps.is_empty() {
            self.finish(true);
        }
    }

    fn finish(&mut self, won: bool) {
        if self.over {
            return;
        }
        self.over = true;
        self.won = won;
        if won {
            self.score += self.lives * WIN_SCORE;
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        if self.over {
            return;
        }
        let shortcut = match key {
            KeyCode::Key1 => Some('1'),
            KeyCode::Key2 => Some('2'),
            KeyCode::Key3 => Some('3'),
            KeyCode::Key4 => Some('4'),
            KeyCode::Key5 => Some('5'),
            KeyCode::Key6 => Some('6'),
            KeyCode::Key7 => Some('7'),
            KeyCode::Key8 => Some('8'),
            KeyCode::Key9 => Some('9'),
            _ => None,
        };
        if let Some(kind) = TowerKind::ALL
            .iter()
            .find(|kind| Some(TOWERS[kind.index()].key) == shortcut)
        {
            self.build_kind = Some(*kind);
            self.selected_id = None;
            return;
        }
        match key {
            KeyCode::Key0 | KeyCode::Escape => {
                self.build_kind = None;
                self.selected_id = None;
            }
            KeyCode::U => self.upgrade_selected(),
            KeyCode::S => self.sell_selected(),
            KeyCode::Space => self.paused = !self.paused,
            KeyCode::N if !self.paused => self.send_wave(),
            KeyCode::F => {
                self.speed = if self.speed == 1.0 {
                    2.0
                } else if self.speed == 2.0 {
                    3.0
                } else {
                    1.0
                }
            }
            _ => {}
        }
    }
}

fn index(x: usize, y: usize) -> usize {
    y * COLS + x
}

fn button(rect: Rect, label: &str, active: bool, enabled: bool) -> bool {
    let mouse = pointer_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    let fill = if !enabled {
        Color::new(0.14, 0.17, 0.22, 1.0)
    } else if active {
        Color::new(0.79, 0.52, 0.04, 1.0)
    } else if hovered {
        Color::new(0.24, 0.31, 0.41, 1.0)
    } else {
        Color::new(0.18, 0.23, 0.31, 1.0)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, LINE);
    let dims = measure_text(label, None, 16, 1.0);
    draw_text(
        label,
        rect.x + (rect.w - dims.width) * 0.5,
        rect.y + (rect.h + dims.height * 0.65) * 0.5,
        16.0,
        if enabled { TEXT } else { MUTED },
    );
    hovered && is_mouse_button_pressed(MouseButton::Left) && enabled
}

fn tower_sprite_frame(kind: TowerKind, angle: f32) -> Rect {
    let direction = (angle / std::f32::consts::FRAC_PI_4 + 0.5)
        .floor()
        .rem_euclid(8.0);
    let sprite = TOWER_SPRITES[kind.index()];
    Rect::new(
        sprite.x + direction * sprite.frame_size,
        sprite.y,
        sprite.frame_size,
        sprite.frame_size,
    )
}

fn enemy_sprite_frame(kind: EnemyKind) -> Rect {
    let sprite = ENEMY_SPRITES[kind.index()];
    Rect::new(sprite.x, sprite.y, sprite.frame_size, sprite.frame_size)
}

fn draw_tower_shape(
    textures: &[Texture2D],
    kind: TowerKind,
    position: Vec2,
    angle: f32,
    size: f32,
    level: usize,
) {
    let Vec2 { x, y } = position;
    let sprite = TOWER_SPRITES[kind.index()];
    let width = sprite.scale * size;
    draw_texture_ex(
        &textures[sprite.texture],
        x - width / 2.0,
        y - width / 2.0,
        WHITE,
        DrawTextureParams {
            source: Some(tower_sprite_frame(kind, angle)),
            dest_size: Some(vec2(width, width)),
            ..Default::default()
        },
    );
    for i in 0..level {
        draw_circle(
            x - 12.0 * size + i as f32 * 6.0 * size,
            y + 14.0 * size,
            1.8 * size,
            GOLD,
        );
    }
}

fn placement_anchor(position: Vec2) -> (usize, usize) {
    (
        ((position.x / CELL).round() as usize).saturating_sub(1),
        ((position.y / CELL).round() as usize).saturating_sub(1),
    )
}

fn draw_board(game: &Game, mouse: Vec2, textures: &[Texture2D]) -> Option<Vec2> {
    draw_rectangle(BOARD_X, BOARD_Y, BOARD_W, BOARD_H, BG);
    for y in 0..ROWS {
        for x in 0..COLS {
            let px = BOARD_X + x as f32 * CELL;
            let py = BOARD_Y + y as f32 * CELL;
            if Game::is_wall(x, y) {
                draw_rectangle(px, py, CELL, CELL, Color::new(0.04, 0.07, 0.11, 1.0));
                draw_line(
                    px,
                    py + CELL,
                    px + CELL,
                    py,
                    1.0,
                    Color::new(0.16, 0.20, 0.25, 1.0),
                );
            } else {
                let shade = if (x + y) % 2 == 0 { 0.12 } else { 0.105 };
                draw_rectangle(
                    px,
                    py,
                    CELL,
                    CELL,
                    Color::new(shade, shade + 0.025, shade + 0.045, 1.0),
                );
                let vertical = if x % 2 == 1 {
                    LINE
                } else {
                    Color::new(LINE.r, LINE.g, LINE.b, 0.3)
                };
                let horizontal = if y % 2 == 1 {
                    LINE
                } else {
                    Color::new(LINE.r, LINE.g, LINE.b, 0.3)
                };
                draw_line(px, py, px, py + CELL, 0.5, vertical);
                draw_line(px, py, px + CELL, py, 0.5, horizontal);
            }
        }
    }
    for y in OPEN_R {
        draw_rectangle(
            BOARD_X,
            BOARD_Y + y as f32 * CELL,
            CELL,
            CELL,
            Color::new(0.10, 0.35, 0.25, 0.55),
        );
        draw_rectangle(
            BOARD_X + (COLS - 1) as f32 * CELL,
            BOARD_Y + y as f32 * CELL,
            CELL,
            CELL,
            Color::new(0.42, 0.12, 0.17, 0.55),
        );
    }
    for x in OPEN_C {
        draw_rectangle(
            BOARD_X + x as f32 * CELL,
            BOARD_Y,
            CELL,
            CELL,
            Color::new(0.10, 0.35, 0.25, 0.55),
        );
        draw_rectangle(
            BOARD_X + x as f32 * CELL,
            BOARD_Y + (ROWS - 1) as f32 * CELL,
            CELL,
            CELL,
            Color::new(0.42, 0.12, 0.17, 0.55),
        );
    }

    for tower in &game.towers {
        let x = BOARD_X + (tower.cx as f32 + 1.0) * CELL;
        let y = BOARD_Y + (tower.cy as f32 + 1.0) * CELL;
        draw_tower_shape(
            textures,
            tower.kind,
            vec2(x, y),
            tower.angle,
            1.0,
            tower.level,
        );
        if game.selected_id == Some(tower.id) {
            draw_rectangle_lines(x - CELL, y - CELL, CELL * 2.0, CELL * 2.0, 2.0, GOLD);
            let (_, range, _, _) = Game::tower_stats(tower);
            draw_circle_lines(x, y, range, 1.0, Color::new(0.98, 0.77, 0.08, 0.32));
        }
    }

    if let Some(kind) = game.build_kind {
        if mouse.x >= BOARD_X
            && mouse.x < BOARD_X + BOARD_W
            && mouse.y >= BOARD_Y
            && mouse.y < BOARD_Y + BOARD_H
        {
            let (cx, cy) = placement_anchor(mouse - vec2(BOARD_X, BOARD_Y));
            let valid = game.can_place(cx, cy).is_ok() && game.gold >= TOWERS[kind.index()].cost;
            let x = BOARD_X + (cx as f32 + 1.0) * CELL;
            let y = BOARD_Y + (cy as f32 + 1.0) * CELL;
            draw_circle_lines(
                x,
                y,
                TOWERS[kind.index()].range[0] * RANGE_SCALE,
                1.0,
                if valid { GREEN } else { RED },
            );
            draw_tower_shape(
                textures,
                kind,
                vec2(x, y),
                -std::f32::consts::FRAC_PI_2,
                1.0,
                0,
            );
            draw_rectangle(
                BOARD_X + cx as f32 * CELL,
                BOARD_Y + cy as f32 * CELL,
                CELL * 2.0,
                CELL * 2.0,
                Color::new(0.92, 0.22, 0.32, 0.12),
            );
        }
    }

    for creep in game.creeps.iter().filter(|creep| !creep.flying) {
        draw_creep(creep, game.game_time, textures);
    }
    for creep in game.creeps.iter().filter(|creep| creep.flying) {
        draw_creep(creep, game.game_time, textures);
    }
    for shot in &game.shots {
        draw_circle(BOARD_X + shot.x, BOARD_Y + shot.y, 3.0, shot.color);
    }
    if game.paused || game.show_start_hint(Instant::now()) {
        draw_rectangle(
            BOARD_X,
            BOARD_Y,
            BOARD_W,
            BOARD_H,
            Color::new(0.02, 0.04, 0.07, 0.52),
        );
        let label = if game.paused {
            "PAUSE - Espace pour reprendre"
        } else {
            "Construisez votre labyrinthe puis lancez la vague (N)"
        };
        let dims = measure_text(label, None, 20, 1.0);
        draw_text(
            label,
            BOARD_X + (BOARD_W - dims.width) / 2.0,
            BOARD_Y + BOARD_H / 2.0,
            20.0,
            TEXT,
        );
    }
    if game.over {
        draw_rectangle(
            BOARD_X,
            BOARD_Y,
            BOARD_W,
            BOARD_H,
            Color::new(0.02, 0.04, 0.07, 0.75),
        );
        let label = if game.won {
            "VICTOIRE !"
        } else {
            "PARTIE TERMINEE"
        };
        let dims = measure_text(label, None, 36, 1.0);
        draw_text(
            label,
            BOARD_X + (BOARD_W - dims.width) / 2.0,
            BOARD_Y + 195.0,
            36.0,
            GOLD,
        );
        let summary = format!(
            "Score : {}  |  Ennemis arretes : {}",
            game.score, game.kills
        );
        let dims = measure_text(&summary, None, 18, 1.0);
        draw_text(
            &summary,
            BOARD_X + (BOARD_W - dims.width) / 2.0,
            BOARD_Y + 226.0,
            18.0,
            TEXT,
        );
    }
    if game.message_time > 0.0 {
        let dims = measure_text(&game.message, None, 16, 1.0);
        draw_rectangle(
            BOARD_X + 100.0,
            BOARD_Y + BOARD_H - 37.0,
            BOARD_W - 200.0,
            28.0,
            PANEL,
        );
        draw_text(
            &game.message,
            BOARD_X + (BOARD_W - dims.width) / 2.0,
            BOARD_Y + BOARD_H - 18.0,
            16.0,
            RED,
        );
    }
    if mouse.x >= BOARD_X
        && mouse.x < BOARD_X + BOARD_W
        && mouse.y >= BOARD_Y
        && mouse.y < BOARD_Y + BOARD_H
    {
        Some(mouse - vec2(BOARD_X, BOARD_Y))
    } else {
        None
    }
}

fn draw_creep(creep: &Creep, time: f32, textures: &[Texture2D]) {
    let x = BOARD_X + creep.x;
    let y = BOARD_Y
        + creep.y
        + if creep.flying {
            (time * 6.0).sin() * 1.5
        } else {
            0.0
        };
    let radius = creep.kind.radius();
    let sprite = ENEMY_SPRITES[creep.kind.index()];
    let size = radius * sprite.scale;
    draw_texture_ex(
        &textures[sprite.texture],
        x - size / 2.0,
        y - size / 2.0,
        WHITE,
        DrawTextureParams {
            source: Some(enemy_sprite_frame(creep.kind)),
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
    if time < creep.slow_until {
        draw_circle_lines(x, y, radius + 2.0, 1.5, Color::new(0.37, 0.88, 0.88, 1.0));
    }
    if creep.hp < creep.max_hp {
        let width = (radius * 2.0 + 2.0).max(12.0);
        draw_rectangle(
            x - width / 2.0,
            y - radius - 6.0,
            width,
            3.0,
            Color::new(0.02, 0.02, 0.02, 0.8),
        );
        draw_rectangle(
            x - width / 2.0,
            y - radius - 6.0,
            width * (creep.hp / creep.max_hp).max(0.0),
            3.0,
            GREEN,
        );
    }
}

fn tower_info_y() -> f32 {
    200.0 + TowerKind::ALL.len().div_ceil(3) as f32 * 58.0
}

fn draw_sidebar(game: &Game, textures: &[Texture2D]) {
    let x = BOARD_X + BOARD_W + 20.0;
    let width = 344.0;
    let height = ui_height() - 100.0;
    draw_rectangle(x, 74.0, width, height, PANEL);
    draw_rectangle_lines(x, 74.0, width, height, 1.0, LINE);
    draw_text("STATISTIQUES", x + 12.0, 94.0, 14.0, MUTED);
    draw_text(
        &format!("Vague  {}/{}", game.wave, game.mode.waves()),
        x + 12.0,
        119.0,
        20.0,
        TEXT,
    );
    draw_text(
        &format!("Vies  {}", game.lives),
        x + 178.0,
        119.0,
        20.0,
        if game.lives <= 5 { RED } else { TEXT },
    );
    draw_text(&format!("Or  {}", game.gold), x + 12.0, 146.0, 20.0, GOLD);
    draw_text(
        &format!("Score  {}", game.score),
        x + 178.0,
        146.0,
        20.0,
        TEXT,
    );

    let shortcuts: String = TOWERS.iter().map(|def| def.key).collect();
    draw_text(
        &format!("CHOISIR UNE TOUR ({shortcuts})"),
        x + 12.0,
        177.0,
        14.0,
        MUTED,
    );
    for (i, kind) in TowerKind::ALL.iter().enumerate() {
        let row = i / 3;
        let col = i % 3;
        let bx = x + 12.0 + col as f32 * 106.0;
        let by = 188.0 + row as f32 * 58.0;
        let def = TOWERS[kind.index()];
        let rect = Rect::new(bx, by, 98.0, 50.0);
        let mouse = pointer_position();
        let hovered = rect.contains(vec2(mouse.0, mouse.1));
        let selected = game.build_kind == Some(*kind);
        draw_rectangle(
            bx,
            by,
            rect.w,
            rect.h,
            if selected {
                Color::new(0.26, 0.20, 0.08, 1.0)
            } else {
                Color::new(0.13, 0.18, 0.25, 1.0)
            },
        );
        draw_rectangle_lines(
            bx,
            by,
            rect.w,
            rect.h,
            1.0,
            if selected || hovered { GOLD } else { LINE },
        );
        draw_tower_shape(
            textures,
            *kind,
            vec2(bx + 18.0, by + 24.0),
            -std::f32::consts::FRAC_PI_2,
            0.45,
            0,
        );
        draw_text(
            &format!("{} {}", i + 1, def.name),
            bx + 36.0,
            by + 21.0,
            13.0,
            TEXT,
        );
        draw_text(
            &format!("{} or", def.cost),
            bx + 36.0,
            by + 39.0,
            12.0,
            GOLD,
        );
    }

    let mut y = tower_info_y();
    if let Some(id) = game.selected_id {
        if let Some(tower) = game.towers.iter().find(|tower| tower.id == id) {
            let def = TOWERS[tower.kind.index()];
            let (damage, _, rate, _) = Game::tower_stats(tower);
            draw_text(
                &format!(
                    "{} - niveau {}/{}",
                    def.name,
                    tower.level + 1,
                    def.damage.len()
                ),
                x + 12.0,
                y,
                18.0,
                TEXT,
            );
            y += 22.0;
            draw_text(
                &format!(
                    "Degats {}   Portee {}",
                    damage as i32, def.range[tower.level] as i32
                ),
                x + 12.0,
                y,
                14.0,
                MUTED,
            );
            y += 18.0;
            draw_text(
                &format!(
                    "Cadence {:.2}s   Vise {}",
                    rate,
                    match (def.ground, def.air) {
                        (true, true) => "sol + air",
                        (true, false) => "sol",
                        (false, true) => "air",
                        _ => unreachable!("validated targeting flags"),
                    }
                ),
                x + 12.0,
                y,
                13.0,
                MUTED,
            );
            y += 18.0;
            if def.special[tower.level] != "none" {
                let label = match def.special[tower.level] {
                    "spread" => format!("Zone : {} degats", def.factor[tower.level]),
                    "slow" => format!(
                        "Ralentit x0.5 : {}% (2s)",
                        (def.factor[tower.level] * 100.0).round()
                    ),
                    "stun" => format!(
                        "Etourdit : {}% (2s)",
                        (def.factor[tower.level] * 100.0).round()
                    ),
                    _ => unreachable!("validated special effect"),
                };
                draw_text(&label, x + 12.0, y, 13.0, MUTED);
                y += 19.0;
            }
            let can_upgrade =
                tower.level < def.upgrades.len() && game.gold >= def.upgrades[tower.level];
            let upgrade_label = if tower.level >= def.upgrades.len() {
                "Niveau maximum (U)".to_owned()
            } else {
                format!("Ameliorer {} or (U)", def.upgrades[tower.level])
            };
            if button(
                Rect::new(x + 12.0, y + 3.0, 174.0, 30.0),
                &upgrade_label,
                false,
                can_upgrade,
            ) {
                // Input is handled in the main loop.
            }
            if button(
                Rect::new(x + 196.0, y + 3.0, 136.0, 30.0),
                "Vendre (S)",
                false,
                true,
            ) {
                // Input is handled in the main loop.
            }
            y += 43.0;
        }
    } else {
        draw_text(
            "Cliquez une tour posee pour la gerer.",
            x + 12.0,
            y,
            14.0,
            MUTED,
        );
        y += 23.0;
    }
    draw_text("CONTROLES", x + 12.0, y + 3.0, 14.0, MUTED);
    draw_text(
        if cfg!(target_os = "android") {
            "Touchez une tour puis le plateau."
        } else {
            "N : vague suivante   Espace : pause"
        },
        x + 12.0,
        y + 23.0,
        13.0,
        TEXT,
    );
    draw_text(
        if cfg!(target_os = "android") {
            "Touchez une tour posee pour la gerer."
        } else {
            "U : ameliorer   S : vendre   F : vitesse"
        },
        x + 12.0,
        y + 42.0,
        13.0,
        TEXT,
    );
    draw_text(
        if cfg!(target_os = "android") {
            "Commandes tactiles sous le plateau."
        } else {
            "0 / Echap : annuler   Maj : placer plusieurs"
        },
        x + 12.0,
        y + 61.0,
        12.0,
        MUTED,
    );

    let next_wave = (game.wave + 1).min(game.mode.waves());
    if next_wave <= game.mode.waves() {
        let def = Game::wave_def(next_wave, game.mode);
        let kind = def.kind;
        draw_text("PROCHAINE VAGUE", x + 12.0, y + 96.0, 14.0, MUTED);
        draw_text(
            &format!(
                "{} - {} : {} ennemis, ~{} PV",
                next_wave,
                kind.name(),
                def.count,
                (def.hp * kind.hp_factor() * game.mode.hp_factor()).round() as i32
            ),
            x + 12.0,
            y + 118.0,
            14.0,
            kind.color(),
        );
        draw_text(
            &format!("Intervalle : {:.0}s", game.timer.max(0.0)),
            x + 12.0,
            y + 139.0,
            13.0,
            MUTED,
        );
    }
}

fn board_click(game: &mut Game, position: Vec2, shift: bool) {
    game.dismiss_start_hint();
    if game.build_kind.is_some() {
        let (cx, cy) = placement_anchor(position);
        game.keep_building = shift;
        game.place_tower(cx, cy);
        return;
    }
    game.selected_id = game.towers_at(
        (position.x / CELL).floor() as usize,
        (position.y / CELL).floor() as usize,
    );
}

fn draw_start_screen(enabled: bool) -> Option<Mode> {
    draw_rectangle(
        0.0,
        0.0,
        ui_width(),
        ui_height(),
        Color::new(0.02, 0.04, 0.07, 0.94),
    );
    let title = "DEDALE DEFENSE";
    let dims = measure_text(title, None, 42, 1.0);
    draw_text(title, (ui_width() - dims.width) / 2.0, 190.0, 42.0, TEXT);
    let subtitle = "Tower defense natif en Rust - construisez un labyrinthe de tours";
    let dims = measure_text(subtitle, None, 18, 1.0);
    draw_text(
        subtitle,
        (ui_width() - dims.width) / 2.0,
        229.0,
        18.0,
        MUTED,
    );
    for (i, mode) in Mode::ALL.iter().enumerate() {
        let bx = ui_width() / 2.0 - 175.0;
        let rect = Rect::new(bx, 268.0 + i as f32 * 46.0, 350.0, 38.0);
        if button(
            rect,
            &format!(
                "{} - {} vagues, {}s, {} vies",
                mode.name(),
                mode.waves(),
                mode.wave_gap() as i32,
                mode.lives()
            ),
            *mode == Mode::Normal,
            enabled,
        ) {
            return Some(*mode);
        }
    }
    None
}

fn draw_touch_controls(game: &mut Game, enabled: bool) -> bool {
    let y = BOARD_Y + BOARD_H + 16.0;
    if button(
        Rect::new(BOARD_X, y, 150.0, 48.0),
        if game.over {
            "Rejouer"
        } else {
            "Vague suivante"
        },
        false,
        enabled && (game.over || !game.paused),
    ) {
        if game.over {
            *game = Game::new(game.mode);
        } else {
            game.send_wave();
        }
    }
    if button(
        Rect::new(BOARD_X + 158.0, y, 90.0, 48.0),
        if game.paused { "Reprendre" } else { "Pause" },
        game.paused,
        enabled && !game.over,
    ) {
        game.paused = !game.paused;
    }
    if button(
        Rect::new(BOARD_X + 256.0, y, 80.0, 48.0),
        &format!("x{}", game.speed),
        false,
        enabled && !game.over,
    ) {
        game.handle_key(KeyCode::F);
    }
    if button(
        Rect::new(BOARD_X + 344.0, y, 108.0, 48.0),
        "Annuler",
        false,
        enabled && !game.over,
    ) {
        game.handle_key(KeyCode::Escape);
    }
    button(
        Rect::new(BOARD_X + 460.0, y, 100.0, 48.0),
        "Menu",
        false,
        enabled,
    )
}

pub async fn run() {
    let mut audio = FxAudio::load().await;
    let textures: Vec<_> = SPRITE_IMAGES
        .iter()
        .map(|bytes| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            texture
        })
        .collect();
    let mut game: Option<Game> = None;
    loop {
        begin_ui();
        let input_enabled = !audio.options_open;
        if game.is_none() {
            if let Some(mode) = draw_start_screen(input_enabled) {
                game = Some(Game::new(mode));
            }
        } else if let Some(current) = &mut game {
            if is_mouse_button_pressed(MouseButton::Left)
                || is_mouse_button_pressed(MouseButton::Right)
                || is_mouse_button_pressed(MouseButton::Middle)
            {
                current.dismiss_start_hint();
            }
            let mut reset_mode = None;
            for key in [
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::Key5,
                KeyCode::Key6,
                KeyCode::Key7,
                KeyCode::Key8,
                KeyCode::Key9,
                KeyCode::Key0,
                KeyCode::Escape,
                KeyCode::U,
                KeyCode::S,
                KeyCode::Space,
                KeyCode::N,
                KeyCode::F,
            ] {
                if input_enabled && is_key_pressed(key) {
                    current.handle_key(key);
                }
            }
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                current.keep_building = true;
            } else {
                current.keep_building = false;
            }
            let mouse = pointer_position();
            let mouse_vec = vec2(mouse.0, mouse.1);
            draw_text("DEDALE DEFENSE", 18.0, 42.0, 28.0, TEXT);
            draw_text(
                &format!("Mode {}", current.mode.name()),
                258.0,
                40.0,
                15.0,
                MUTED,
            );
            let cell = draw_board(current, mouse_vec, &textures);
            draw_sidebar(current, &textures);
            if input_enabled && is_mouse_button_pressed(MouseButton::Right) {
                current.build_kind = None;
                current.selected_id = None;
            }
            if input_enabled && is_mouse_button_pressed(MouseButton::Left) && !current.over {
                if let Some(position) = cell {
                    board_click(current, position, current.keep_building);
                } else {
                    let x = BOARD_X + BOARD_W + 20.0;
                    for (i, kind) in TowerKind::ALL.iter().enumerate() {
                        let col = i % 3;
                        let row = i / 3;
                        let rect = Rect::new(
                            x + 12.0 + col as f32 * 106.0,
                            188.0 + row as f32 * 58.0,
                            98.0,
                            50.0,
                        );
                        if rect.contains(mouse_vec) {
                            current.build_kind = if current.build_kind == Some(*kind) {
                                None
                            } else {
                                Some(*kind)
                            };
                            current.selected_id = None;
                        }
                    }
                    if let Some(id) = current.selected_id {
                        let tower = current.towers.iter().find(|tower| tower.id == id);
                        if let Some(tower) = tower {
                            let y = tower_info_y()
                                + 22.0
                                + 18.0
                                + 18.0
                                + if TOWERS[tower.kind.index()].special[tower.level] != "none" {
                                    19.0
                                } else {
                                    0.0
                                };
                            if Rect::new(x + 12.0, y + 3.0, 174.0, 30.0).contains(mouse_vec) {
                                current.upgrade_selected();
                            } else if Rect::new(x + 196.0, y + 3.0, 136.0, 30.0).contains(mouse_vec)
                            {
                                current.sell_selected();
                            }
                        }
                    }
                }
            }
            let back_to_menu =
                cfg!(target_os = "android") && draw_touch_controls(current, input_enabled);
            let dt = get_frame_time().min(0.1) * current.speed;
            if input_enabled {
                current.update(dt);
            }
            audio.play_pending(&mut current.pending_audio);
            if current.over {
                let label = if cfg!(target_os = "android") {
                    "Touchez Rejouer ou Menu"
                } else {
                    "N : nouvelle partie"
                };
                let dims = measure_text(label, None, 16, 1.0);
                draw_text(
                    label,
                    BOARD_X + (BOARD_W - dims.width) / 2.0,
                    BOARD_Y + 280.0,
                    16.0,
                    TEXT,
                );
                if input_enabled && is_key_pressed(KeyCode::N) {
                    reset_mode = Some(current.mode);
                }
            }
            if let Some(mode) = reset_mode {
                game = Some(Game::new(mode));
            } else if back_to_menu {
                game = None;
            }
        }
        audio.draw_controls();
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_viewport_preserves_board_coordinates_on_different_screens() {
        let logical = window_size();
        let board_point = vec2(BOARD_X + CELL * 5.5, BOARD_Y + CELL * 7.5);
        for screen in [
            vec2(1280.0, 720.0),
            vec2(2400.0, 1080.0),
            vec2(1920.0, 1200.0),
            vec2(800.0, 1280.0),
        ] {
            let viewport = Viewport::fit(screen, logical);
            assert!(viewport.size.x <= screen.x + 0.001);
            assert!(viewport.size.y <= screen.y + 0.001);
            let physical = viewport.offset + board_point * viewport.scale;
            let restored = viewport.pointer(physical);
            assert!((restored - board_point).length() < 0.001);
            assert_eq!(
                placement_anchor(restored - vec2(BOARD_X, BOARD_Y)),
                placement_anchor(board_point - vec2(BOARD_X, BOARD_Y))
            );
            assert!((viewport.pointer(viewport.offset + viewport.size) - logical).length() < 0.001);
        }
    }

    #[test]
    fn touches_in_mobile_letterboxing_do_not_hit_the_board() {
        let viewport = Viewport::fit(vec2(2400.0, 1080.0), window_size());
        assert!(viewport.pointer(vec2(0.0, 540.0)).x < 0.0);
        assert!(viewport.pointer(vec2(2400.0, 540.0)).x > window_size().x);
    }

    #[test]
    fn fx_volume_and_mute_remain_independent() {
        let mut settings = AudioSettings {
            volume: 50,
            muted: false,
        };
        assert_eq!(settings.effective_volume(), 0.5);
        settings.muted = true;
        assert_eq!(settings.effective_volume(), 0.0);
        settings.volume = 27;
        assert_eq!(settings.effective_volume(), 0.0);
        settings.muted = false;
        assert_eq!(settings.effective_volume(), 0.27);
        settings.volume = 0;
        assert_eq!(settings.effective_volume(), 0.0);
    }

    #[test]
    fn embedded_fx_and_combat_events_cover_all_towers_and_life_loss() {
        for bytes in FX_BYTES {
            assert_eq!(&bytes[..4], b"RIFF");
            assert_eq!(&bytes[8..12], b"WAVE");
            assert!(bytes.len() > 44);
        }
        for kind in TowerKind::ALL {
            let mut game = Game::new(Mode::UncleScrooge);
            game.build_kind = Some(kind);
            game.place_tower(5, 5);
            game.fire_towers(0.0);
            assert!(game.pending_audio.is_empty());
            game.spawn(
                if TOWERS[kind.index()].ground {
                    EnemyKind::Walker
                } else {
                    EnemyKind::Flyer
                },
                100.0,
                1.0,
                Target::Right,
            );
            game.creeps[0].x = 6.0 * CELL;
            game.creeps[0].y = 6.0 * CELL;
            game.fire_towers(0.0);
            assert_eq!(game.pending_audio, vec![tower_fx(kind)]);
            assert_eq!(tower_fx(kind), kind.index());
            game.pending_audio.clear();
            game.fire_towers(0.0);
            assert!(game.pending_audio.is_empty());
            game.lives = 1;
            game.leak(0);
            assert_eq!(game.pending_audio, vec![6]);
            assert!(game.over);
        }
    }

    #[test]
    fn csv_sprites_select_the_configured_image_frame_and_size() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/content.json"))).unwrap();
        let images: Vec<_> = SPRITE_IMAGES
            .iter()
            .map(|bytes| Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap())
            .collect();
        for (kind, expected) in EnemyKind::ALL
            .into_iter()
            .zip(data["enemies"].as_object().unwrap().values())
        {
            let sprite = ENEMY_SPRITES[kind.index()];
            let frame = enemy_sprite_frame(kind);
            check_sprite(sprite, frame, &expected["sprite"], "drawScale", &images);
        }
        for kind in TowerKind::ALL {
            let sprite = TOWER_SPRITES[kind.index()];
            let expected = &data["towers"][kind.index()]["sprite"];
            check_sprite(
                sprite,
                tower_sprite_frame(kind, 0.0),
                expected,
                "drawSize",
                &images,
            );
            for direction in 0..8 {
                for turns in -2..=2 {
                    for offset in [-0.1, 0.0, 0.1] {
                        let angle = direction as f32 * std::f32::consts::FRAC_PI_4
                            + turns as f32 * std::f32::consts::TAU
                            + offset;
                        let frame = tower_sprite_frame(kind, angle);
                        assert_eq!(frame.x, sprite.x + direction as f32 * sprite.frame_size);
                        assert_eq!(frame.y, sprite.y);
                    }
                }
                let frame =
                    tower_sprite_frame(kind, direction as f32 * std::f32::consts::FRAC_PI_4);
                check_frame(&images[sprite.texture], frame);
            }
        }
    }

    fn check_sprite(
        sprite: SpriteDef,
        frame: Rect,
        expected: &serde_json::Value,
        scale: &str,
        images: &[Image],
    ) {
        let size = expected["frameSize"].as_f64().unwrap() as f32;
        assert_eq!(frame.x, expected["column"].as_f64().unwrap() as f32 * size);
        assert_eq!(frame.y, expected["row"].as_f64().unwrap() as f32 * size);
        assert_eq!(frame.w, size);
        assert_eq!(frame.h, size);
        assert_eq!(sprite.scale, expected[scale].as_f64().unwrap() as f32);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(expected["image"].as_str().unwrap());
        assert_eq!(SPRITE_IMAGES[sprite.texture], std::fs::read(path).unwrap());
        check_frame(&images[sprite.texture], frame);
    }

    fn check_frame(image: &Image, frame: Rect) {
        assert!(frame.x + frame.w <= image.width as f32);
        assert!(frame.y + frame.h <= image.height as f32);
        let mut visible = 0;
        for y in 0..frame.h as u32 {
            for x in 0..frame.w as u32 {
                let alpha = image.get_pixel(frame.x as u32 + x, frame.y as u32 + y).a;
                if x == 0 || y == 0 || x == frame.w as u32 - 1 || y == frame.h as u32 - 1 {
                    assert_eq!(alpha, 0.0, "sprite must have transparent padding");
                }
                if alpha > 0.0 {
                    visible += 1;
                }
            }
        }
        assert!(visible > 1000, "missing sprite artwork");
    }

    #[test]
    fn construction_hint_expires_and_first_click_still_places_a_tower() {
        let mut game = Game::new(Mode::Normal);
        let deadline = game.start_hint_until.unwrap();
        assert!(game.show_start_hint(deadline - Duration::from_millis(1)));
        assert!(!game.show_start_hint(deadline));
        assert!(!game.show_start_hint(deadline + Duration::from_secs(1)));
        game = Game::new(Mode::Normal);
        assert!(game.show_start_hint(Instant::now()));
        game.build_kind = Some(TowerKind::Stone);
        board_click(&mut game, vec2(120.0, 120.0), false);
        assert!(!game.show_start_hint(Instant::now()));
        assert_eq!(game.towers.len(), 1);
        assert!(!game.started);
        game = Game::new(Mode::Normal);
        assert!(game.show_start_hint(Instant::now()));
    }

    #[test]
    fn board_starts_with_open_routes() {
        let game = Game::new(Mode::Normal);
        assert!(
            OPEN_R
                .iter()
                .all(|&y| game.field_right[index(0, y)] < u32::MAX / 4)
        );
        assert!(
            OPEN_C
                .iter()
                .all(|&x| game.field_bottom[index(x, 0)] < u32::MAX / 4)
        );
    }

    #[test]
    fn placement_keeps_both_routes_open() {
        let mut game = Game::new(Mode::Normal);
        assert!(game.can_place(5, 5).is_ok());
        game.build_kind = Some(TowerKind::Stone);
        game.place_tower(5, 5);
        assert!(game.field_right[index(0, 10)] < u32::MAX / 4);
        assert!(game.field_bottom[index(13, 0)] < u32::MAX / 4);
    }

    #[test]
    fn tower_sale_refunds_all_before_first_wave() {
        let mut game = Game::new(Mode::Normal);
        game.build_kind = Some(TowerKind::Stone);
        game.place_tower(5, 5);
        game.selected_id = game.towers.first().map(|tower| tower.id);
        game.sell_selected();
        assert_eq!(game.gold, Mode::Normal.gold());
    }

    #[test]
    fn wave_progression_uses_expected_enemy_cycle() {
        assert_eq!(Game::wave_def(1, Mode::Normal).kind, EnemyKind::Walker);
        assert_eq!(Game::wave_def(4, Mode::Normal).kind, EnemyKind::Flyer);
        assert_eq!(Game::wave_def(10, Mode::Normal).kind, EnemyKind::Boss);
    }

    #[test]
    fn every_level_and_wave_matches_shared_json() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/content.json"))).unwrap();
        let levels = data["levels"].as_array().unwrap();
        assert_eq!(Mode::ALL.len(), levels.len());
        for (mode, level) in Mode::ALL.iter().zip(levels) {
            assert_eq!(mode.name(), level["name"].as_str().unwrap());
            assert_eq!(mode.lives() as i64, level["lives"].as_i64().unwrap());
            assert_eq!(mode.gold() as i64, level["gold"].as_i64().unwrap());
            assert_eq!(mode.hp_factor(), level["hp"].as_f64().unwrap() as f32);
            assert_eq!(mode.wave_gap(), level["gap"].as_f64().unwrap() as f32);
            assert_eq!(mode.waves() as u64, level["waves"].as_u64().unwrap());
            let game = Game::new(*mode);
            assert_eq!(game.lives, mode.lives());
            assert_eq!(game.gold, mode.gold());
            let waves = &data["waveSets"][level["waveSet"].as_str().unwrap()];
            for n in 1..=mode.waves() {
                let actual = Game::wave_def(n, *mode);
                let expected = &waves[(n - 1) as usize];
                let enemy = &data["enemies"][expected["type"].as_str().unwrap()];
                assert_eq!(actual.kind.name(), enemy["name"].as_str().unwrap());
                assert_eq!(actual.hp, expected["hp"].as_f64().unwrap() as f32);
                assert_eq!(actual.count as u64, enemy["count"].as_u64().unwrap());
                assert_eq!(actual.gap, enemy["gap"].as_f64().unwrap() as f32);
                assert_eq!(
                    actual.bounty,
                    (enemy["bounty"].as_f64().unwrap()
                        * expected["bountyMultiplier"].as_f64().unwrap())
                        as f32
                );
                let targets: Vec<_> = expected["targets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|t| {
                        if t == "R" {
                            Target::Right
                        } else {
                            Target::Bottom
                        }
                    })
                    .collect();
                assert!(actual.targets == targets);
            }
        }
    }

    #[test]
    fn game_ends_after_its_mode_wave_count() {
        let mut game = Game::new(Mode::Rush);
        for _ in 0..Mode::Rush.waves() {
            game.send_wave();
        }
        assert_eq!(game.wave, Mode::Rush.waves());
        game.send_wave(); // one more is refused
        assert_eq!(game.wave, Mode::Rush.waves());

        let mut marathon = Game::new(Mode::Marathon);
        marathon.send_wave();
        assert_eq!(marathon.wave, 1);
        assert!(!marathon.won);
    }

    #[test]
    fn marathon_grows_enemy_health_more_slowly() {
        let normal = Game::wave_def(20, Mode::Normal).hp;
        let marathon = Game::wave_def(20, Mode::Marathon).hp;
        assert!(marathon < normal);
    }

    #[test]
    fn flyers_travel_toward_their_assigned_exit() {
        let mut game = Game::new(Mode::Normal);
        game.spawn(EnemyKind::Flyer, 10.0, 1.0, Target::Bottom);
        let flyer = &game.creeps[0];
        assert_eq!(flyer.wx, flyer.x);
        assert_eq!(flyer.wy, BOARD_H + CELL);

        game.spawn(EnemyKind::Flyer, 10.0, 1.0, Target::Right);
        let flyer = &game.creeps[1];
        assert_eq!(flyer.wx, BOARD_W + CELL);
        assert_eq!(flyer.wy, flyer.y);
    }

    #[test]
    fn mixed_waves_alternate_entrances_per_enemy() {
        let mut game = Game::new(Mode::Normal);
        game.wave = 14;
        game.send_wave();
        game.update(0.0);
        game.update(Game::wave_def(15, Mode::Normal).gap);
        assert_eq!(game.creeps.len(), 2);
        assert!(game.creeps[0].target == Target::Right);
        assert!(game.creeps[1].target == Target::Bottom);
    }

    #[test]
    fn split_children_inherit_the_parent_route_and_shared_stats() {
        let mut game = Game::new(Mode::Normal);
        game.spawn(EnemyKind::Splitter, 100.0, 8.0, Target::Bottom);
        game.creeps[0].exiting = true;
        game.creeps[0].next_cell = None;
        game.creeps[0].wx = 120.0;
        game.creeps[0].wy = BOARD_H + CELL;
        let parent = game.creeps[0].clone();
        game.hurt(parent.id, parent.hp);
        let split = parent.kind.def().split.unwrap();
        assert_eq!(game.creeps.len(), 1 + split.count);
        for child in &game.creeps[1..] {
            assert_eq!(child.kind, split.kind);
            assert_eq!(child.hp, parent.max_hp * split.hp);
            assert_eq!(child.bounty, parent.bounty * split.bounty);
            assert!(child.target == parent.target);
            assert!(child.exiting);
            assert_eq!(child.next_cell, None);
            assert_eq!(child.wx, parent.wx);
            assert_eq!(child.wy, parent.wy);
            assert!((child.x - parent.x).abs() <= SPLIT_JITTER / 2.0);
            assert!((child.y - parent.y).abs() <= SPLIT_JITTER / 2.0);
        }
    }

    #[test]
    fn slow_effect_resets_after_expiry_and_preserves_immunity() {
        let mut game = Game::new(Mode::Normal);
        for kind in [EnemyKind::Walker, EnemyKind::Armored] {
            game.spawn(kind, 100.0, 1.0, Target::Right);
            let creep = game.creeps.last_mut().unwrap();
            creep.slow_until = -1.0;
            creep.slow_factor = 0.8;
            let frost = TOWERS[TowerKind::Frost.index()];
            game.shots.push(Shot {
                target_id: creep.id,
                x: creep.x,
                y: creep.y,
                speed: frost.speed,
                damage: frost.damage[0],
                color: frost.accent,
                origin: (creep.x, creep.y),
                range: frost.range[0] * RANGE_SCALE,
                special: "slow",
                factor: 1.0,
                kind: TowerKind::Frost,
            });
            game.update_shots(0.0);
            let creep = game.creeps.last().unwrap();
            if kind.is_immune() {
                assert_eq!(creep.slow_until, -1.0);
            } else {
                assert_eq!(creep.slow_factor, 0.5);
                assert_eq!(creep.slow_until, game.game_time + SLOW_DURATION);
            }
        }
    }

    #[test]
    fn tower_shortcuts_follow_shared_content() {
        let mut game = Game::new(Mode::Normal);
        for kind in TowerKind::ALL {
            let key = match TOWERS[kind.index()].key {
                '1' => KeyCode::Key1,
                '2' => KeyCode::Key2,
                '3' => KeyCode::Key3,
                '4' => KeyCode::Key4,
                '5' => KeyCode::Key5,
                '6' => KeyCode::Key6,
                '7' => KeyCode::Key7,
                '8' => KeyCode::Key8,
                '9' => KeyCode::Key9,
                _ => panic!("unsupported shortcut"),
            };
            game.handle_key(key);
            assert_eq!(game.build_kind, Some(kind));
        }
    }

    #[test]
    fn grid_and_all_six_tower_levels_use_csv_units() {
        assert_eq!((COLS - 2) / 2, 13);
        assert_eq!((ROWS - 2) / 2, 11);
        assert!((180.0 * RANGE_SCALE * 2.0 - (COLS - 2) as f32 * CELL).abs() < 0.001);
        let mut game = Game::new(Mode::UncleScrooge);
        assert!(game.can_place(25, 21).is_ok());
        assert!(game.can_place(24, 20).is_ok());
        for kind in TowerKind::ALL {
            game = Game::new(Mode::UncleScrooge);
            game.build_kind = Some(kind);
            game.place_tower(5, 5);
            game.selected_id = Some(game.towers[0].id);
            for level in 0..6 {
                let def = TOWERS[kind.index()];
                let (damage, range, rate, _) = Game::tower_stats(&game.towers[0]);
                assert_eq!(damage, def.damage[level]);
                assert_eq!(range, def.range[level] * RANGE_SCALE);
                assert_eq!(rate, def.rate[level]);
                if level < 5 {
                    game.upgrade_selected();
                }
            }
            assert_eq!(game.towers[0].level, 5);
            game.upgrade_selected();
            assert_eq!(game.towers[0].level, 5);
            game.sell_selected();
            assert_eq!(game.gold, Mode::UncleScrooge.gold());
        }
    }

    #[test]
    fn half_tower_input_and_placement_keep_small_corridors_open() {
        let mut game = Game::new(Mode::UncleScrooge);
        for (cx, cy) in [(5, 5), (8, 6)] {
            let position = vec2((cx + 1) as f32 * CELL, (cy + 1) as f32 * CELL);
            for offset in [-9.0, 0.0, 9.0] {
                assert_eq!(placement_anchor(position + vec2(offset, offset)), (cx, cy));
            }
            game.build_kind = Some(TowerKind::Stone);
            board_click(&mut game, position, false);
            assert_eq!(game.towers.last().map(|t| (t.cx, t.cy)), Some((cx, cy)));
        }
        assert_eq!(game.towers.len(), 2);
        assert!(game.can_place(6, 5).is_err());
        for kind in EnemyKind::ALL.into_iter().filter(|kind| !kind.is_flying()) {
            game.spawn(kind, 100.0, 0.0, Target::Right);
            let creep = game.creeps.last_mut().unwrap();
            creep.x = 7.5 * CELL;
            creep.y = 5.5 * CELL;
            creep.next_cell = Some(index(7, 6));
            creep.wx = creep.x;
            creep.wy = 6.5 * CELL;
            let i = game.creeps.len() - 1;
            game.move_creep(i, CELL / kind.speed());
            assert!((game.creeps[i].y - 6.5 * CELL).abs() < 0.001);
            assert!(!game.creeps[i].dead);
            assert_eq!(game.creeps[i].cell, Some(index(7, 6)));
        }
        let mut field = vec![200; COUNT];
        field[index(5, 4)] = 0;
        field[index(4, 4)] = 100;
        assert_eq!(game.next_step(&field, index(4, 5)), Some(index(4, 4)));
    }

    #[test]
    fn entrances_and_enemy_sizes_match_reference_proportions() {
        assert_eq!(OPEN_R.len(), 6);
        assert_eq!(OPEN_C.len(), 8);
        assert_eq!(OPEN_R[0] + OPEN_R[OPEN_R.len() - 1] + 1, ROWS);
        assert_eq!(OPEN_C[0] + OPEN_C[OPEN_C.len() - 1] + 1, COLS);
        let game = Game::new(Mode::Normal);
        for kind in EnemyKind::ALL {
            assert!(kind.radius() * 2.0 < CELL);
        }
        for y in 0..ROWS {
            assert_eq!(!Game::is_wall(0, y), OPEN_R.contains(&y));
            assert_eq!(!Game::is_wall(COLS - 1, y), OPEN_R.contains(&y));
            if OPEN_R.contains(&y) {
                assert!(game.field_right[index(0, y)] < u32::MAX / 4);
            }
        }
        for x in 0..COLS {
            assert_eq!(!Game::is_wall(x, 0), OPEN_C.contains(&x));
            assert_eq!(!Game::is_wall(x, ROWS - 1), OPEN_C.contains(&x));
            if OPEN_C.contains(&x) {
                assert!(game.field_bottom[index(x, 0)] < u32::MAX / 4);
            }
        }
    }

    #[test]
    fn area_effects_require_both_ranges_and_compatible_targets() {
        for (kind, special) in [
            (TowerKind::Needle, "spread"),
            (TowerKind::Frost, "slow"),
            (TowerKind::Pulse, "stun"),
        ] {
            let mut game = Game::new(Mode::Normal);
            for (enemy, x) in [
                (EnemyKind::Walker, 30.0),
                (EnemyKind::Walker, 40.0),
                (EnemyKind::Walker, 60.0),
                (EnemyKind::Walker, -30.0),
                (EnemyKind::Armored, 40.0),
                (EnemyKind::Flyer, 40.0),
            ] {
                game.spawn(enemy, 10000.0, 0.0, Target::Right);
                let c = game.creeps.last_mut().unwrap();
                c.x = x;
                c.y = 0.0;
            }
            let before: Vec<f32> = game.creeps.iter().map(|c| c.hp).collect();
            let shot = Shot {
                target_id: game.creeps[0].id,
                x: 30.0,
                y: 0.0,
                speed: 1.0,
                damage: 8.0,
                color: TEXT,
                origin: (0.0, 0.0),
                range: 50.0,
                special,
                factor: if special == "spread" { 4.0 } else { 1.0 },
                kind,
            };
            game.impact(&shot, 30.0, 0.0);
            assert_eq!(game.creeps[0].hp, before[0] - 8.0);
            for i in [2, 3] {
                assert_eq!(game.creeps[i].hp, before[i]);
                assert_eq!(game.creeps[i].slow_until, 0.0);
                assert_eq!(game.creeps[i].stun_until, 0.0);
            }
            if special == "spread" {
                assert_eq!(game.creeps[1].hp, before[1] - 4.0);
                assert_eq!(game.creeps[5].hp, before[5]);
            } else {
                assert_eq!(game.creeps[4].slow_until, 0.0);
                assert_eq!(game.creeps[4].stun_until, 0.0);
                if special == "slow" {
                    assert_eq!(game.creeps[1].slow_factor, 0.5);
                    assert_eq!(game.creeps[1].slow_until, 2.0);
                    assert_eq!(game.creeps[5].slow_until, 2.0);
                } else {
                    assert_eq!(game.creeps[1].stun_until, 2.0);
                    assert_eq!(game.creeps[5].stun_until, 0.0);
                }
            }
        }
    }

    #[test]
    fn towers_and_enemies_match_csv_content() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/content.json"))).unwrap();
        for (actual, expected) in TOWERS.iter().zip(data["towers"].as_array().unwrap()) {
            assert_eq!(actual.name, expected["name"].as_str().unwrap());
            assert_eq!(actual.cost as i64, expected["cost"].as_i64().unwrap());
            assert_eq!(actual.speed, expected["speed"].as_f64().unwrap() as f32);
            assert_eq!(actual.multi as u64, expected["multi"].as_u64().unwrap());
            assert_eq!(actual.shot, expected["shot"].as_str().unwrap());
            assert_eq!(actual.ground, expected["ground"].as_bool().unwrap());
            assert_eq!(actual.air, expected["air"].as_bool().unwrap());
            for i in 0..6 {
                assert_eq!(
                    actual.range[i],
                    expected["range"][i].as_f64().unwrap() as f32
                );
                assert_eq!(actual.rate[i], expected["rate"][i].as_f64().unwrap() as f32);
                assert_eq!(actual.sell[i] as i64, expected["sell"][i].as_i64().unwrap());
                assert_eq!(actual.special[i], expected["special"][i].as_str().unwrap());
                assert_eq!(
                    actual.factor[i],
                    expected["factor"][i].as_f64().unwrap() as f32
                );
                assert_eq!(
                    actual.damage[i],
                    expected["dmg"][i].as_f64().unwrap() as f32
                );
                if i < 5 {
                    assert_eq!(
                        actual.upgrades[i] as i64,
                        expected["ups"][i].as_i64().unwrap()
                    );
                }
            }
        }
        for (kind, expected) in EnemyKind::ALL
            .iter()
            .zip(data["enemies"].as_object().unwrap().values())
        {
            assert_eq!(kind.name(), expected["name"].as_str().unwrap());
            assert_eq!(kind.hp_factor(), expected["hp"].as_f64().unwrap() as f32);
            assert_eq!(kind.speed(), expected["speed"].as_f64().unwrap() as f32);
            assert_eq!(kind.radius(), expected["r"].as_f64().unwrap() as f32);
            assert_eq!(kind.is_flying(), expected["flying"].as_bool().unwrap());
            assert_eq!(kind.is_immune(), expected["immune"].as_bool().unwrap());
            assert_eq!(
                kind.def().life_loss as i64,
                expected["lifeLoss"].as_i64().unwrap()
            );
            assert_eq!(kind.def().split.is_some(), !expected["split"].is_null());
            if let Some(split) = kind.def().split {
                assert_eq!(
                    split.kind.name(),
                    data["enemies"][expected["split"]["type"].as_str().unwrap()]["name"]
                        .as_str()
                        .unwrap()
                );
                assert_eq!(
                    split.count as u64,
                    expected["split"]["count"].as_u64().unwrap()
                );
                assert_eq!(split.hp, expected["split"]["hp"].as_f64().unwrap() as f32);
                assert_eq!(
                    split.bounty,
                    expected["split"]["bounty"].as_f64().unwrap() as f32
                );
            }
        }
    }

    #[test]
    fn csv_refunds_apply_at_every_level_after_the_first_wave() {
        for (index, def) in TOWERS.iter().enumerate() {
            for level in 0..6 {
                let mut game = Game::new(Mode::UncleScrooge);
                game.build_kind = Some(TowerKind::ALL[index]);
                game.place_tower(5, 5);
                game.selected_id = Some(game.towers[0].id);
                for _ in 0..level {
                    game.upgrade_selected();
                }
                game.send_wave();
                let gold = game.gold;
                game.sell_selected();
                assert_eq!(game.gold - gold, def.sell[level]);
            }
        }
    }
}

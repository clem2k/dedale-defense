use macroquad::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const CELL: f32 = 20.0;
const COLS: usize = 28;
const ROWS: usize = 22;
const COUNT: usize = COLS * ROWS;
const BOARD_W: f32 = COLS as f32 * CELL;
const BOARD_H: f32 = ROWS as f32 * CELL;
const BOARD_X: f32 = 18.0;
const BOARD_Y: f32 = 74.0;
const TOTAL_WAVES: u32 = 50;

const BG: Color = Color::new(0.055, 0.085, 0.13, 1.0);
const PANEL: Color = Color::new(0.09, 0.13, 0.19, 1.0);
const LINE: Color = Color::new(0.20, 0.26, 0.34, 1.0);
const TEXT: Color = Color::new(0.91, 0.93, 0.96, 1.0);
const MUTED: Color = Color::new(0.58, 0.64, 0.72, 1.0);
const GOLD: Color = Color::new(0.98, 0.77, 0.08, 1.0);
const GREEN: Color = Color::new(0.23, 0.80, 0.50, 1.0);
const RED: Color = Color::new(0.92, 0.22, 0.32, 1.0);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Easy,
    Normal,
    Hard,
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Easy => "Facile",
            Self::Normal => "Normal",
            Self::Hard => "Difficile",
        }
    }

    fn lives(self) -> i32 {
        match self {
            Self::Easy => 30,
            Self::Normal => 20,
            Self::Hard => 10,
        }
    }

    fn gold(self) -> i32 {
        match self {
            Self::Easy => 70,
            Self::Normal => 60,
            Self::Hard => 50,
        }
    }

    fn hp_factor(self) -> f32 {
        match self {
            Self::Easy => 0.75,
            Self::Normal => 1.0,
            Self::Hard => 1.35,
        }
    }

    fn wave_gap(self) -> f32 {
        match self {
            Self::Easy => 28.0,
            Self::Normal => 24.0,
            Self::Hard => 20.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TowerKind {
    Stone,
    Burst,
    Needle,
    Volley,
    Frost,
    Pulse,
}

impl TowerKind {
    const ALL: [Self; 6] = [
        Self::Stone,
        Self::Burst,
        Self::Needle,
        Self::Volley,
        Self::Frost,
        Self::Pulse,
    ];

    fn index(self) -> usize {
        match self {
            Self::Stone => 0,
            Self::Burst => 1,
            Self::Needle => 2,
            Self::Volley => 3,
            Self::Frost => 4,
            Self::Pulse => 5,
        }
    }
}

#[derive(Clone, Copy)]
struct TowerDef {
    name: &'static str,
    cost: i32,
    upgrades: [i32; 4],
    damage: [f32; 5],
    range: f32,
    rate: f32,
    ground: bool,
    air: bool,
    speed: f32,
    splash: f32,
    multi: usize,
    slow: [f32; 5],
    color: Color,
    accent: Color,
}

const TOWERS: [TowerDef; 6] = [
    TowerDef {
        name: "Caillou",
        cost: 5,
        upgrades: [5, 10, 20, 40],
        damage: [5.0, 10.0, 20.0, 40.0, 80.0],
        range: 70.0,
        rate: 0.5,
        ground: true,
        air: true,
        speed: 340.0,
        splash: 0.0,
        multi: 1,
        slow: [0.0; 5],
        color: Color::new(0.60, 0.64, 0.70, 1.0),
        accent: Color::new(0.89, 0.91, 0.94, 1.0),
    },
    TowerDef {
        name: "Eclat",
        cost: 15,
        upgrades: [15, 30, 60, 120],
        damage: [8.0, 16.0, 32.0, 64.0, 128.0],
        range: 62.0,
        rate: 0.9,
        ground: true,
        air: false,
        speed: 240.0,
        splash: 30.0,
        multi: 1,
        slow: [0.0; 5],
        color: Color::new(0.31, 0.63, 1.0, 1.0),
        accent: Color::new(0.73, 0.86, 1.0, 1.0),
    },
    TowerDef {
        name: "Aiguille",
        cost: 25,
        upgrades: [25, 50, 100, 200],
        damage: [25.0, 50.0, 100.0, 200.0, 400.0],
        range: 130.0,
        rate: 1.4,
        ground: true,
        air: true,
        speed: 520.0,
        splash: 0.0,
        multi: 1,
        slow: [0.0; 5],
        color: Color::new(0.75, 0.52, 0.98, 1.0),
        accent: Color::new(0.91, 0.82, 1.0, 1.0),
    },
    TowerDef {
        name: "Volee",
        cost: 30,
        upgrades: [30, 60, 120, 240],
        damage: [12.0, 24.0, 48.0, 96.0, 192.0],
        range: 105.0,
        rate: 0.85,
        ground: false,
        air: true,
        speed: 300.0,
        splash: 0.0,
        multi: 4,
        slow: [0.0; 5],
        color: Color::new(0.98, 0.44, 0.40, 1.0),
        accent: Color::new(1.0, 0.81, 0.79, 1.0),
    },
    TowerDef {
        name: "Givre",
        cost: 20,
        upgrades: [20, 40, 80, 160],
        damage: [2.0, 4.0, 8.0, 16.0, 32.0],
        range: 72.0,
        rate: 0.6,
        ground: true,
        air: true,
        speed: 320.0,
        splash: 0.0,
        multi: 1,
        slow: [0.35, 0.40, 0.45, 0.50, 0.60],
        color: Color::new(0.37, 0.88, 0.88, 1.0),
        accent: Color::new(0.83, 0.98, 0.98, 1.0),
    },
    TowerDef {
        name: "Onde",
        cost: 40,
        upgrades: [40, 80, 160, 320],
        damage: [20.0, 40.0, 80.0, 160.0, 320.0],
        range: 46.0,
        rate: 1.2,
        ground: true,
        air: false,
        speed: 0.0,
        splash: 0.0,
        multi: 1,
        slow: [0.0; 5],
        color: Color::new(0.99, 0.69, 0.13, 1.0),
        accent: Color::new(1.0, 0.91, 0.64, 1.0),
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnemyKind {
    Walker,
    Runner,
    Swarm,
    Flyer,
    Armored,
    Splitter,
    Mini,
    Boss,
}

impl EnemyKind {
    fn name(self) -> &'static str {
        match self {
            Self::Walker => "Marcheurs",
            Self::Runner => "Coureurs",
            Self::Swarm => "Essaim",
            Self::Flyer => "Volants",
            Self::Armored => "Blindes",
            Self::Splitter => "Scindeurs",
            Self::Mini => "Eclats",
            Self::Boss => "Colosse",
        }
    }

    fn hp_factor(self) -> f32 {
        match self {
            Self::Walker => 1.0,
            Self::Runner => 0.7,
            Self::Swarm => 0.4,
            Self::Flyer => 0.85,
            Self::Armored => 1.5,
            Self::Splitter => 1.05,
            Self::Mini => 0.3,
            Self::Boss => 14.0,
        }
    }

    fn speed(self) -> f32 {
        match self {
            Self::Walker | Self::Splitter => 40.0,
            Self::Runner => 78.0,
            Self::Swarm => 46.0,
            Self::Flyer => 46.0,
            Self::Armored => 34.0,
            Self::Mini => 52.0,
            Self::Boss => 28.0,
        }
    }

    fn radius(self) -> f32 {
        match self {
            Self::Walker | Self::Flyer => 6.0,
            Self::Runner | Self::Mini => 5.0,
            Self::Swarm => 4.0,
            Self::Armored | Self::Splitter => 7.0,
            Self::Boss => 11.0,
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Walker => Color::new(0.65, 0.94, 0.40, 1.0),
            Self::Runner => Color::new(0.99, 0.94, 0.54, 1.0),
            Self::Swarm => Color::new(0.98, 0.64, 0.60, 1.0),
            Self::Flyer => Color::new(0.49, 0.83, 0.99, 1.0),
            Self::Armored => Color::new(0.74, 0.70, 0.99, 1.0),
            Self::Splitter | Self::Mini => Color::new(0.97, 0.66, 0.83, 1.0),
            Self::Boss => Color::new(0.94, 0.27, 0.22, 1.0),
        }
    }

    fn is_flying(self) -> bool {
        self == Self::Flyer
    }

    fn is_immune(self) -> bool {
        self == Self::Armored
    }

    fn splits(self) -> bool {
        self == Self::Splitter
    }

    fn count(self) -> usize {
        match self {
            Self::Walker | Self::Runner => 10,
            Self::Swarm => 22,
            Self::Flyer | Self::Armored | Self::Splitter => 8,
            Self::Mini => 0,
            Self::Boss => 1,
        }
    }

    fn spawn_gap(self) -> f32 {
        match self {
            Self::Walker => 0.65,
            Self::Runner => 0.55,
            Self::Swarm => 0.30,
            Self::Flyer => 0.9,
            Self::Armored => 0.9,
            Self::Splitter => 0.85,
            Self::Mini => 0.0,
            Self::Boss => 1.0,
        }
    }

    fn bounty(self) -> f32 {
        match self {
            Self::Walker | Self::Runner | Self::Splitter => 1.0,
            Self::Swarm | Self::Mini => 0.5,
            Self::Flyer | Self::Armored => 1.5,
            Self::Boss => 25.0,
        }
    }
}

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
    built_started: bool,
}

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
    kind: EnemyKind,
    base_hp: f32,
    bounty: f32,
    target: Target,
    left: usize,
    next_in: f32,
}

struct Shot {
    target_id: u64,
    x: f32,
    y: f32,
    speed: f32,
    damage: f32,
    splash: f32,
    slow: f32,
    color: Color,
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
        };
        game.refresh_fields();
        game
    }

    fn towers_at(&self, x: usize, y: usize) -> Option<u64> {
        if x >= COLS || y >= ROWS {
            return None;
        }
        self.occupancy[index(x, y)]
    }

    fn is_wall(x: usize, y: usize) -> bool {
        let edge = x == 0 || y == 0 || x == COLS - 1 || y == ROWS - 1;
        let open_lr = (x == 0 || x == COLS - 1) && (9..=12).contains(&y);
        let open_tb = (y == 0 || y == ROWS - 1) && (12..=15).contains(&x);
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
        let right: Vec<usize> = (9..=12).map(|y| index(COLS - 1, y)).collect();
        let bottom: Vec<usize> = (12..=15).map(|x| index(x, ROWS - 1)).collect();
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
            &(9..=12).map(|y| index(COLS - 1, y)).collect::<Vec<_>>(),
            &cells,
        );
        let bottom = self.compute_field(
            &(12..=15).map(|x| index(x, ROWS - 1)).collect::<Vec<_>>(),
            &cells,
        );
        let entries_reachable = (9..=12).all(|y| right[index(0, y)] < u32::MAX / 4)
            && (12..=15).all(|x| bottom[index(x, 0)] < u32::MAX / 4);
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
            built_started: self.started,
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
        if tower.level >= 4 {
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
        let refund = if tower.built_started {
            (tower.spent as f32 * 0.75).floor() as i32
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

    fn wave_def(wave: u32) -> (EnemyKind, f32, usize, f32) {
        const CYCLE: [EnemyKind; 10] = [
            EnemyKind::Walker,
            EnemyKind::Runner,
            EnemyKind::Walker,
            EnemyKind::Flyer,
            EnemyKind::Swarm,
            EnemyKind::Armored,
            EnemyKind::Walker,
            EnemyKind::Splitter,
            EnemyKind::Runner,
            EnemyKind::Boss,
        ];
        let kind = if wave == 1 {
            EnemyKind::Walker
        } else {
            CYCLE[((wave - 1) as usize) % CYCLE.len()]
        };
        let base_hp = 12.0 * 1.115_f32.powi((wave - 1) as i32) + 4.0 * wave as f32;
        let bounty = kind.bounty() * (2 + (wave / 6)) as f32;
        (kind, base_hp, kind.count(), bounty)
    }

    fn send_wave(&mut self) {
        if self.over || self.wave >= TOTAL_WAVES {
            return;
        }
        if self.started && self.timer > 0.0 {
            self.score += self.timer.ceil() as i32 * (self.wave as i32 + 1);
        }
        self.started = true;
        self.wave += 1;
        let (kind, base_hp, count, bounty) = Self::wave_def(self.wave);
        let target = if self.wave >= 15 {
            if self.wave % 2 == 0 {
                Target::Right
            } else {
                Target::Bottom
            }
        } else if self.wave % 2 == 1 {
            Target::Right
        } else {
            Target::Bottom
        };
        self.spawners.push(Spawner {
            kind,
            base_hp,
            bounty,
            target,
            left: count,
            next_in: 0.0,
        });
        self.timer = self.mode.wave_gap();
    }

    fn spawn(&mut self, kind: EnemyKind, base_hp: f32, bounty: f32, target: Target) {
        let lane = macroquad::rand::gen_range(0, 4) as usize;
        let (x, y, cell) = if target == Target::Right {
            (
                -CELL * 0.6,
                (9 + lane) as f32 * CELL + CELL * 0.5,
                index(0, 9 + lane),
            )
        } else {
            (
                (12 + lane) as f32 * CELL + CELL * 0.5,
                -CELL * 0.6,
                index(12 + lane, 0),
            )
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
        let mut speed = self.creeps[index].kind.speed();
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
        let loss = if self.creeps[index].kind == EnemyKind::Boss {
            2
        } else {
            1
        };
        self.creeps[index].dead = true;
        self.lives = (self.lives - loss).max(0);
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
        let creep = &self.creeps[index];
        let bounty = creep.bounty;
        let kind = creep.kind;
        let max_hp = creep.max_hp;
        let target = creep.target;
        let x = creep.x;
        let y = creep.y;
        self.creeps[index].dead = true;
        let reward = (bounty.round() as i32).max(1);
        self.gold += reward;
        self.score += reward * 10;
        self.kills += 1;
        if kind.splits() {
            for _ in 0..2 {
                self.spawn_at(
                    kind,
                    EnemyKind::Mini,
                    max_hp * 0.3,
                    bounty * 0.5,
                    target,
                    x,
                    y,
                );
            }
        }
    }

    fn spawn_at(
        &mut self,
        _parent: EnemyKind,
        kind: EnemyKind,
        hp: f32,
        bounty: f32,
        target: Target,
        x: f32,
        y: f32,
    ) {
        let cell = index(
            ((x / CELL).floor() as usize).min(COLS - 1),
            ((y / CELL).floor() as usize).min(ROWS - 1),
        );
        let id = self.next_id;
        self.next_id += 1;
        self.creeps.push(Creep {
            id,
            kind,
            x,
            y,
            hp,
            max_hp: hp,
            bounty,
            target,
            flying: false,
            cell: Some(cell),
            next_cell: Some(cell),
            wx: (cell % COLS) as f32 * CELL + CELL * 0.5,
            wy: (cell / COLS) as f32 * CELL + CELL * 0.5,
            exiting: false,
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
            def.range * (1.0 + 0.08 * tower.level as f32),
            def.rate * (1.0 - 0.04 * tower.level as f32),
            def.slow[tower.level],
        )
    }

    fn fire_towers(&mut self, dt: f32) {
        let mut shots = Vec::new();
        let mut hits = Vec::new();
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
            let (damage, range, rate, slow) = Self::tower_stats(tower);
            let def = TOWERS[tower.kind.index()];
            let x = (tower.cx as f32 + 1.0) * CELL;
            let y = (tower.cy as f32 + 1.0) * CELL;
            let mut targets: Vec<(f32, u64, f32, f32)> = creep_targets
                .iter()
                .filter(|(_, _, cx, cy, flying, radius)| {
                    (if *flying { def.air } else { def.ground })
                        && (*cx - x).hypot(*cy - y) <= range + radius
                })
                .map(|(remaining, id, cx, cy, _, _)| (*remaining, *id, *cx, *cy))
                .collect();
            targets.sort_by(|a, b| a.0.total_cmp(&b.0));
            if targets.is_empty() {
                continue;
            }
            if tower.kind == TowerKind::Pulse {
                for (_, id, _, _) in targets {
                    hits.push((id, damage));
                }
                tower.cooldown = rate;
                continue;
            }
            tower.angle = (targets[0].3 - y).atan2(targets[0].2 - x);
            for (_, id, _, _) in targets.into_iter().take(def.multi) {
                shots.push(Shot {
                    target_id: id,
                    x: x + tower.angle.cos() * 8.0,
                    y: y + tower.angle.sin() * 8.0,
                    speed: def.speed,
                    damage,
                    splash: def.splash,
                    slow,
                    color: def.accent,
                });
            }
            tower.cooldown = rate;
        }
        for (id, damage) in hits {
            self.hurt(id, damage);
            if let Some(creep) = self
                .creeps
                .iter_mut()
                .find(|creep| creep.id == id && !creep.dead)
            {
                if !creep.kind.is_immune() && macroquad::rand::gen_range(0.0, 1.0) < 0.15 {
                    creep.stun_until = self.game_time + 0.6;
                }
            }
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
                    impacts.push((
                        shot.target_id,
                        target.x,
                        target.y,
                        shot.damage,
                        shot.splash,
                        shot.slow,
                    ));
                } else if distance > 0.0 {
                    shot.x += dx / distance * step;
                    shot.y += dy / distance * step;
                    pending.push(shot);
                }
            }
        }
        self.shots = pending;
        for (id, x, y, damage, splash, slow) in impacts {
            if splash > 0.0 {
                let targets: Vec<u64> = self
                    .creeps
                    .iter()
                    .filter(|creep| {
                        !creep.dead
                            && !creep.flying
                            && (creep.x - x).hypot(creep.y - y) <= splash + creep.kind.radius()
                    })
                    .map(|creep| creep.id)
                    .collect();
                for target in targets {
                    self.hurt(target, damage);
                }
            } else {
                if let Some(creep) = self
                    .creeps
                    .iter_mut()
                    .find(|creep| creep.id == id && !creep.dead)
                {
                    if slow > 0.0 && !creep.kind.is_immune() {
                        creep.slow_until = self.game_time + 2.0;
                        creep.slow_factor = creep.slow_factor.max(slow);
                    }
                }
                self.hurt(id, damage);
            }
        }
    }

    fn update(&mut self, dt: f32) {
        if self.paused || self.over {
            return;
        }
        self.game_time += dt;
        if self.message_time > 0.0 {
            self.message_time -= dt;
        }
        if self.started && self.wave < TOTAL_WAVES {
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
                    spawner.kind,
                    spawner.base_hp,
                    spawner.bounty,
                    spawner.target,
                ));
                spawner.left -= 1;
                spawner.next_in += spawner.kind.spawn_gap();
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
        if self.wave >= TOTAL_WAVES && self.spawners.is_empty() && self.creeps.is_empty() {
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
            self.score += self.lives * 100;
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        if self.over {
            return;
        }
        match key {
            KeyCode::Key1 => {
                self.build_kind = Some(TowerKind::Stone);
                self.selected_id = None;
            }
            KeyCode::Key2 => {
                self.build_kind = Some(TowerKind::Burst);
                self.selected_id = None;
            }
            KeyCode::Key3 => {
                self.build_kind = Some(TowerKind::Needle);
                self.selected_id = None;
            }
            KeyCode::Key4 => {
                self.build_kind = Some(TowerKind::Volley);
                self.selected_id = None;
            }
            KeyCode::Key5 => {
                self.build_kind = Some(TowerKind::Frost);
                self.selected_id = None;
            }
            KeyCode::Key6 => {
                self.build_kind = Some(TowerKind::Pulse);
                self.selected_id = None;
            }
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
    let mouse = mouse_position();
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

fn draw_tower_shape(kind: TowerKind, x: f32, y: f32, angle: f32, size: f32, level: usize) {
    let def = TOWERS[kind.index()];
    draw_rectangle(
        x - 18.0 * size,
        y - 18.0 * size,
        36.0 * size,
        36.0 * size,
        Color::new(0.15, 0.21, 0.31, 1.0),
    );
    draw_rectangle_lines(
        x - 18.0 * size,
        y - 18.0 * size,
        36.0 * size,
        36.0 * size,
        1.0,
        Color::new(0.46, 0.53, 0.63, 0.8),
    );
    match kind {
        TowerKind::Stone => draw_circle(x, y, 9.0 * size, def.color),
        TowerKind::Burst => draw_poly(x, y, 6, 11.0 * size, 0.0, def.color),
        TowerKind::Needle => {
            draw_poly(x, y, 4, 12.0 * size, std::f32::consts::FRAC_PI_4, def.color)
        }
        TowerKind::Volley => {
            draw_rectangle(
                x - 10.0 * size,
                y - 10.0 * size,
                20.0 * size,
                20.0 * size,
                def.color,
            );
            for (dx, dy) in [(-5.0, -5.0), (5.0, -5.0), (-5.0, 5.0), (5.0, 5.0)] {
                draw_circle(x + dx * size, y + dy * size, 2.2 * size, PANEL);
            }
        }
        TowerKind::Frost => {
            for i in 0..3 {
                let a = i as f32 * std::f32::consts::PI / 3.0;
                draw_line(
                    x - a.cos() * 11.0 * size,
                    y - a.sin() * 11.0 * size,
                    x + a.cos() * 11.0 * size,
                    y + a.sin() * 11.0 * size,
                    3.0 * size,
                    def.color,
                );
            }
        }
        TowerKind::Pulse => {
            draw_circle(x, y, 10.0 * size, def.color);
            draw_circle_lines(x, y, 6.0 * size, 2.0 * size, PANEL);
            draw_circle_lines(x, y, 2.0 * size, 1.0 * size, PANEL);
        }
    }
    if kind != TowerKind::Pulse && kind != TowerKind::Frost {
        let end_x = x + angle.cos() * 14.0 * size;
        let end_y = y + angle.sin() * 14.0 * size;
        draw_line(x, y, end_x, end_y, 4.0 * size, def.accent);
    }
    for i in 0..level {
        draw_circle(
            x - 12.0 * size + i as f32 * 6.0 * size,
            y + 14.0 * size,
            1.8 * size,
            GOLD,
        );
    }
}

fn draw_board(game: &Game, mouse: Vec2) -> Option<(usize, usize)> {
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
                draw_rectangle_lines(px, py, CELL, CELL, 0.5, Color::new(0.28, 0.37, 0.49, 0.20));
            }
        }
    }
    for y in 9..=12 {
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
    for x in 12..=15 {
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
        draw_tower_shape(tower.kind, x, y, tower.angle, 1.0, tower.level);
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
            let cell_x = ((mouse.x - BOARD_X) / CELL).floor() as usize;
            let cell_y = ((mouse.y - BOARD_Y) / CELL).floor() as usize;
            let cx = cell_x.saturating_sub(1);
            let cy = cell_y.saturating_sub(1);
            let valid = game.can_place(cx, cy).is_ok() && game.gold >= TOWERS[kind.index()].cost;
            let x = BOARD_X + (cx as f32 + 1.0) * CELL;
            let y = BOARD_Y + (cy as f32 + 1.0) * CELL;
            draw_circle_lines(
                x,
                y,
                TOWERS[kind.index()].range,
                1.0,
                if valid { GREEN } else { RED },
            );
            draw_tower_shape(kind, x, y, -std::f32::consts::FRAC_PI_2, 0.75, 0);
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
        draw_creep(creep, game.game_time);
    }
    for creep in game.creeps.iter().filter(|creep| creep.flying) {
        draw_creep(creep, game.game_time);
    }
    for shot in &game.shots {
        draw_circle(BOARD_X + shot.x, BOARD_Y + shot.y, 3.0, shot.color);
    }
    if game.paused || !game.started {
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
        Some((
            ((mouse.x - BOARD_X) / CELL).floor() as usize,
            ((mouse.y - BOARD_Y) / CELL).floor() as usize,
        ))
    } else {
        None
    }
}

fn draw_creep(creep: &Creep, time: f32) {
    let x = BOARD_X + creep.x;
    let y = BOARD_Y
        + creep.y
        + if creep.flying {
            (time * 6.0).sin() * 1.5
        } else {
            0.0
        };
    let radius = creep.kind.radius();
    let color = creep.kind.color();
    match creep.kind {
        EnemyKind::Runner | EnemyKind::Flyer => draw_poly(x, y, 3, radius + 1.0, 0.0, color),
        EnemyKind::Armored => {
            draw_rectangle(x - radius, y - radius, radius * 2.0, radius * 2.0, color)
        }
        EnemyKind::Splitter | EnemyKind::Mini => {
            draw_poly(x, y, 4, radius + 1.0, std::f32::consts::FRAC_PI_4, color)
        }
        EnemyKind::Boss => draw_poly(x, y, 10, radius, time, color),
        _ => draw_circle(x, y, radius, color),
    }
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

fn draw_sidebar(game: &Game) {
    let x = 598.0;
    let width = 344.0;
    draw_rectangle(x, 74.0, width, 540.0, PANEL);
    draw_rectangle_lines(x, 74.0, width, 540.0, 1.0, LINE);
    draw_text("STATISTIQUES", x + 12.0, 94.0, 14.0, MUTED);
    draw_text(
        &format!("Vague  {}/{}", game.wave, TOTAL_WAVES),
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

    draw_text("CHOISIR UNE TOUR (1-6)", x + 12.0, 177.0, 14.0, MUTED);
    for (i, kind) in TowerKind::ALL.iter().enumerate() {
        let row = i / 3;
        let col = i % 3;
        let bx = x + 12.0 + col as f32 * 106.0;
        let by = 188.0 + row as f32 * 58.0;
        let def = TOWERS[kind.index()];
        let rect = Rect::new(bx, by, 98.0, 50.0);
        let mouse = mouse_position();
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
            *kind,
            bx + 18.0,
            by + 24.0,
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

    let mut y = 316.0;
    if let Some(id) = game.selected_id {
        if let Some(tower) = game.towers.iter().find(|tower| tower.id == id) {
            let def = TOWERS[tower.kind.index()];
            let (damage, range, rate, slow) = Game::tower_stats(tower);
            draw_text(
                &format!("{} - niveau {}/5", def.name, tower.level + 1),
                x + 12.0,
                y,
                18.0,
                TEXT,
            );
            y += 22.0;
            draw_text(
                &format!("Degats {}   Portee {}", damage as i32, range as i32),
                x + 12.0,
                y,
                14.0,
                MUTED,
            );
            y += 18.0;
            draw_text(
                &format!(
                    "Cadence {:.2}s   Cible sol/air{}",
                    rate,
                    if def.air { "" } else { " (sol)" }
                ),
                x + 12.0,
                y,
                13.0,
                MUTED,
            );
            y += 18.0;
            if slow > 0.0 {
                draw_text(
                    &format!("Ralentissement {}%", (slow * 100.0) as i32),
                    x + 12.0,
                    y,
                    13.0,
                    MUTED,
                );
                y += 19.0;
            }
            let can_upgrade = tower.level < 4 && game.gold >= def.upgrades[tower.level];
            let upgrade_label = if tower.level >= 4 {
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
        "N : vague suivante   Espace : pause",
        x + 12.0,
        y + 23.0,
        13.0,
        TEXT,
    );
    draw_text(
        "U : ameliorer   S : vendre   F : vitesse",
        x + 12.0,
        y + 42.0,
        13.0,
        TEXT,
    );
    draw_text(
        "0 / Echap : annuler   Maj : placer plusieurs",
        x + 12.0,
        y + 61.0,
        12.0,
        MUTED,
    );

    let next_wave = (game.wave + 1).min(TOTAL_WAVES);
    if next_wave <= TOTAL_WAVES {
        let (kind, hp, count, _) = Game::wave_def(next_wave);
        draw_text("PROCHAINE VAGUE", x + 12.0, y + 96.0, 14.0, MUTED);
        draw_text(
            &format!(
                "{} - {} ennemis, ~{} PV",
                next_wave,
                kind.name(),
                (hp * kind.hp_factor() * game.mode.hp_factor()).round() as i32
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
        let _ = count;
    }
}

fn board_click(game: &mut Game, cell: (usize, usize), shift: bool) {
    let (x, y) = cell;
    if game.build_kind.is_some() {
        let cx = x.saturating_sub(1);
        let cy = y.saturating_sub(1);
        game.keep_building = shift;
        game.place_tower(cx, cy);
        return;
    }
    game.selected_id = game.towers_at(x, y);
}

fn draw_start_screen() -> Option<Mode> {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.02, 0.04, 0.07, 0.94),
    );
    let title = "DEDALE DEFENSE";
    let dims = measure_text(title, None, 42, 1.0);
    draw_text(
        title,
        (screen_width() - dims.width) / 2.0,
        190.0,
        42.0,
        TEXT,
    );
    let subtitle = "Tower defense natif en Rust - construisez un labyrinthe de tours";
    let dims = measure_text(subtitle, None, 18, 1.0);
    draw_text(
        subtitle,
        (screen_width() - dims.width) / 2.0,
        229.0,
        18.0,
        MUTED,
    );
    for (i, mode) in [Mode::Easy, Mode::Normal, Mode::Hard].iter().enumerate() {
        let bx = screen_width() / 2.0 - 155.0;
        let rect = Rect::new(bx, 280.0 + i as f32 * 58.0, 310.0, 44.0);
        if button(
            rect,
            &format!("{}  -  {} vies", mode.name(), mode.lives()),
            *mode == Mode::Normal,
            true,
        ) {
            return Some(*mode);
        }
    }
    None
}

#[macroquad::main("Dedale Defense")]
async fn main() {
    request_new_screen_size(960.0, 640.0);
    let mut game: Option<Game> = None;
    loop {
        clear_background(Color::new(0.035, 0.055, 0.085, 1.0));
        if game.is_none() {
            if let Some(mode) = draw_start_screen() {
                game = Some(Game::new(mode));
            }
        } else if let Some(current) = &mut game {
            let mut reset_mode = None;
            for key in [
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::Key5,
                KeyCode::Key6,
                KeyCode::Key0,
                KeyCode::Escape,
                KeyCode::U,
                KeyCode::S,
                KeyCode::Space,
                KeyCode::N,
                KeyCode::F,
            ] {
                if is_key_pressed(key) {
                    current.handle_key(key);
                }
            }
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                current.keep_building = true;
            } else {
                current.keep_building = false;
            }
            let mouse = mouse_position();
            let mouse_vec = vec2(mouse.0, mouse.1);
            draw_text("DEDALE DEFENSE", 18.0, 42.0, 28.0, TEXT);
            draw_text(
                &format!("Mode {}", current.mode.name()),
                258.0,
                40.0,
                15.0,
                MUTED,
            );
            let cell = draw_board(current, mouse_vec);
            draw_sidebar(current);
            if is_mouse_button_pressed(MouseButton::Right) {
                current.build_kind = None;
                current.selected_id = None;
            }
            if is_mouse_button_pressed(MouseButton::Left) && !current.over {
                if let Some((x, y)) = cell {
                    board_click(current, (x, y), current.keep_building);
                } else {
                    let x = 598.0;
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
                            let y = 316.0
                                + 22.0
                                + 18.0
                                + 18.0
                                + if tower.kind == TowerKind::Frost {
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
            let dt = get_frame_time().min(0.1) * current.speed;
            current.update(dt);
            if current.over {
                let label = "N : nouvelle partie";
                let dims = measure_text(label, None, 16, 1.0);
                draw_text(
                    label,
                    BOARD_X + (BOARD_W - dims.width) / 2.0,
                    BOARD_Y + 280.0,
                    16.0,
                    TEXT,
                );
                if is_key_pressed(KeyCode::N) {
                    reset_mode = Some(current.mode);
                }
            }
            if let Some(mode) = reset_mode {
                game = Some(Game::new(mode));
            }
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_starts_with_open_routes() {
        let game = Game::new(Mode::Normal);
        assert!((9..=12).all(|y| game.field_right[index(0, y)] < u32::MAX / 4));
        assert!((12..=15).all(|x| game.field_bottom[index(x, 0)] < u32::MAX / 4));
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
        assert_eq!(Game::wave_def(1).0, EnemyKind::Walker);
        assert_eq!(Game::wave_def(4).0, EnemyKind::Flyer);
        assert_eq!(Game::wave_def(10).0, EnemyKind::Boss);
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
}

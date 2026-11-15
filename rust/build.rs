use serde_json::Value;
use std::{env, fs, path::PathBuf};

fn field<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.get(key)
        .unwrap_or_else(|| panic!("game-content.json: missing {key}"))
}

fn text(v: &Value) -> &str {
    let s = v.as_str().expect("game-content.json: expected string");
    assert!(!s.is_empty(), "game-content.json: empty string");
    s
}

fn uint(v: &Value) -> u64 {
    v.as_u64()
        .expect("game-content.json: expected nonnegative integer")
}

fn num(v: &Value) -> f64 {
    let n = v.as_f64().expect("game-content.json: expected number");
    assert!(
        n.is_finite() && n >= 0.0 && n <= f32::MAX as f64,
        "game-content.json: invalid number"
    );
    n
}

fn array(v: &Value) -> &[Value] {
    v.as_array().expect("game-content.json: expected array")
}

fn boolean(v: &Value) -> bool {
    v.as_bool().expect("game-content.json: expected boolean")
}

fn variant(id: &str) -> String {
    assert!(
        !id.is_empty()
            && id.as_bytes()[0].is_ascii_lowercase()
            && id
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_'),
        "game-content.json: invalid identifier {id}"
    );
    id.split('_')
        .map(|word| {
            assert!(
                !word.is_empty(),
                "game-content.json: empty identifier segment"
            );
            let mut chars = word.chars();
            chars.next().unwrap().to_ascii_uppercase().to_string() + chars.as_str()
        })
        .collect()
}

fn enumeration(name: &str, ids: &[&str]) -> String {
    let variants: Vec<_> = ids.iter().map(|id| variant(id)).collect();
    let mut unique = std::collections::HashSet::new();
    assert!(
        variants.iter().all(|v| unique.insert(v)),
        "game-content.json: duplicate {name}"
    );
    assert!(!variants.is_empty(), "game-content.json: empty {name}");
    let entries = variants.join(", ");
    let all = variants
        .iter()
        .map(|v| format!("Self::{v}"))
        .collect::<Vec<_>>()
        .join(", ");
    let all_attribute = if name == "EnemyKind" {
        "#[cfg(test)]"
    } else {
        ""
    };
    format!(
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nenum {name} {{ {entries} }}\n\
         impl {name} {{ {all_attribute} const ALL: [Self; {len}] = [{all}]; fn index(self) -> usize {{ self as usize }} }}\n",
        len = ids.len()
    )
}

fn color(v: &Value) -> String {
    let hex = text(v);
    assert!(
        hex.len() == 7 && hex.starts_with('#'),
        "game-content.json: invalid color {hex}"
    );
    let rgb = u32::from_str_radix(&hex[1..], 16).expect("game-content.json: invalid hex color");
    format!(
        "Color::new({:?}_f32, {:?}_f32, {:?}_f32, 1.0)",
        ((rgb >> 16) & 255) as f64 / 255.0,
        ((rgb >> 8) & 255) as f64 / 255.0,
        (rgb & 255) as f64 / 255.0
    )
}

fn floats(v: &Value, length: usize) -> String {
    assert_eq!(
        array(v).len(),
        length,
        "game-content.json: invalid upgrade array length"
    );
    format!(
        "[{}]",
        array(v)
            .iter()
            .map(|n| format!("{:?}_f32", num(n)))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn ints(v: &Value) -> String {
    format!(
        "[{}]",
        array(v)
            .iter()
            .map(|n| uint(n).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn main() {
    println!("cargo:rerun-if-changed=../game-content.json");
    let data: Value = serde_json::from_str(
        &fs::read_to_string("../game-content.json").expect("cannot read game-content.json"),
    )
    .expect("invalid game-content.json");
    assert_eq!(
        uint(field(&data, "version")),
        1,
        "unsupported game-content.json version"
    );
    let board = field(&data, "board");
    let cols = uint(field(board, "cols"));
    let rows = uint(field(board, "rows"));
    assert!(
        cols >= 4 && rows >= 4 && num(field(board, "cell")) >= 1.0,
        "game-content.json: invalid board"
    );
    for (key, limit) in [("openRows", rows), ("openCols", cols)] {
        let lanes = array(field(board, key));
        let mut unique = std::collections::HashSet::new();
        assert!(
            !lanes.is_empty()
                && lanes.iter().all(|n| {
                    let n = uint(n);
                    n > 0 && n < limit - 1 && unique.insert(n)
                }),
            "game-content.json: invalid {key}"
        );
    }
    let mut code = format!(
        "const CELL: f32 = {:?}_f32;\nconst COLS: usize = {cols};\nconst ROWS: usize = {rows};\n\
         const OPEN_R: [usize; {}] = {};\nconst OPEN_C: [usize; {}] = {};\n",
        num(field(board, "cell")),
        array(field(board, "openRows")).len(),
        ints(field(board, "openRows")),
        array(field(board, "openCols")).len(),
        ints(field(board, "openCols"))
    );
    let rules = field(&data, "rules");
    for (key, constant) in [
        ("rangeGrowth", "RANGE_GROWTH"),
        ("rateReduction", "RATE_REDUCTION"),
        ("sellRefund", "SELL_REFUND"),
        ("slowDuration", "SLOW_DURATION"),
        ("stunDuration", "STUN_DURATION"),
        ("splitJitter", "SPLIT_JITTER"),
    ] {
        code += &format!(
            "const {constant}: f32 = {:?}_f32;\n",
            num(field(rules, key))
        );
    }
    for (key, constant) in [("killScore", "KILL_SCORE"), ("winScore", "WIN_SCORE")] {
        let n = uint(field(rules, key));
        assert!(
            n <= i32::MAX as u64,
            "game-content.json: score multiplier too large"
        );
        code += &format!("const {constant}: i32 = {n};\n");
    }
    assert!(
        num(field(rules, "sellRefund")) <= 1.0
            && 1.0 - 4.0 * num(field(rules, "rateReduction")) > 0.0,
        "game-content.json: invalid refund or firing rate"
    );
    let levels = array(field(&data, "levels"));
    let towers = array(field(&data, "towers"));
    let enemies = field(&data, "enemies")
        .as_object()
        .expect("game-content.json: expected enemies object");
    let level_ids: Vec<_> = levels.iter().map(|l| text(field(l, "id"))).collect();
    assert!(
        level_ids.contains(&"normal"),
        "game-content.json: missing normal level"
    );
    let tower_ids: Vec<_> = towers.iter().map(|t| text(field(t, "id"))).collect();
    let enemy_ids: Vec<_> = enemies.keys().map(String::as_str).collect();
    code += &enumeration("Mode", &level_ids);
    code += &enumeration("TowerKind", &tower_ids);
    code += &enumeration("EnemyKind", &enemy_ids);
    code += "#[derive(Clone, Copy)]\nstruct SplitDef { kind: EnemyKind, count: usize, hp: f32, bounty: f32 }\n\
        #[derive(Clone, Copy)]\nstruct EnemyDef { name: &'static str, hp: f32, speed: f32, radius: f32, color: Color,\n\
        flying: bool, immune: bool, life_loss: i32, shape: &'static str, split: Option<SplitDef> }\n";
    code += &format!("const ENEMIES: [EnemyDef; {}] = [\n", enemies.len());
    for (id, e) in enemies {
        assert!(
            num(field(e, "hp")) > 0.0
                && num(field(e, "speed")) > 0.0
                && num(field(e, "r")) > 0.0
                && uint(field(e, "lifeLoss")) > 0
                && (uint(field(e, "count")) == 0 || num(field(e, "gap")) > 0.0),
            "game-content.json: invalid enemy {id}"
        );
        let split = field(e, "split");
        let split_code = if split.is_null() {
            "None".to_owned()
        } else {
            let kind = text(field(split, "type"));
            assert!(
                enemies.contains_key(kind)
                    && uint(field(split, "count")) > 0
                    && num(field(split, "hp")) > 0.0,
                "game-content.json: invalid split {id}"
            );
            format!(
                "Some(SplitDef {{ kind: EnemyKind::{}, count: {}, hp: {:?}_f32, bounty: {:?}_f32 }})",
                variant(kind),
                uint(field(split, "count")),
                num(field(split, "hp")),
                num(field(split, "bounty"))
            )
        };
        let shape = text(field(e, "shape"));
        assert!(
            ["circle", "tri", "wing", "square", "diamond", "boss"].contains(&shape),
            "game-content.json: invalid shape"
        );
        code += &format!(
            "EnemyDef {{ name: {:?}, hp: {:?}_f32, speed: {:?}_f32, radius: {:?}_f32, color: {}, flying: {}, immune: {},\n\
             life_loss: {}, shape: {:?}, split: {} }},\n",
            text(field(e, "name")),
            num(field(e, "hp")),
            num(field(e, "speed")),
            num(field(e, "r")),
            color(field(e, "color")),
            boolean(field(e, "flying")),
            boolean(field(e, "immune")),
            uint(field(e, "lifeLoss")),
            shape,
            split_code
        );
    }
    code += "];\nimpl EnemyKind {\n\
        fn def(self) -> &'static EnemyDef { &ENEMIES[self.index()] }\n\
        fn name(self) -> &'static str { self.def().name }\n\
        fn hp_factor(self) -> f32 { self.def().hp }\n\
        fn speed(self) -> f32 { self.def().speed }\n\
        fn radius(self) -> f32 { self.def().radius }\n\
        fn color(self) -> Color { self.def().color }\n\
        fn is_flying(self) -> bool { self.def().flying }\n\
        fn is_immune(self) -> bool { self.def().immune }\n}\n";
    code += "#[derive(Clone, Copy)]\nstruct TowerDef { name: &'static str, cost: i32, upgrades: [i32; 4],\n\
        damage: [f32; 5], range: f32, rate: f32, ground: bool, air: bool, speed: f32, splash: f32,\n\
        multi: usize, slow: [f32; 5], stun: f32, shot: &'static str, key: char, color: Color, accent: Color }\n";
    code += &format!("const TOWERS: [TowerDef; {}] = [\n", towers.len());
    let mut keys = std::collections::HashSet::new();
    for t in towers {
        let key = text(field(t, "key"));
        assert!(
            key.len() == 1 && matches!(key.as_bytes()[0], b'1'..=b'9') && keys.insert(key),
            "game-content.json: invalid or duplicate tower shortcut"
        );
        let shot = text(field(t, "shot"));
        assert!(
            ["pellet", "shell", "dart", "missile", "ice", "pulse"].contains(&shot)
                && (shot == "pulse" || num(field(t, "speed")) > 0.0)
                && num(field(t, "range")) >= 1.0
                && num(field(t, "rate")) > 0.0
                && uint(field(t, "cost")) > 0
                && uint(field(t, "multi")) > 0
                && num(field(t, "stun")) <= 1.0
                && (boolean(field(t, "ground")) || boolean(field(t, "air")))
                && array(field(t, "ups")).len() == 4
                && array(field(t, "ups")).iter().all(|n| uint(n) > 0)
                && array(field(t, "slow")).iter().all(|n| num(n) < 1.0),
            "game-content.json: invalid tower"
        );
        code += &format!(
            "TowerDef {{ name: {:?}, cost: {}, upgrades: {}, damage: {}, range: {:?}_f32, rate: {:?}_f32,\n\
             ground: {}, air: {}, speed: {:?}_f32, splash: {:?}_f32, multi: {}, slow: {}, stun: {:?}_f32,\n\
             shot: {:?}, key: {:?}, color: {}, accent: {} }},\n",
            text(field(t, "name")),
            uint(field(t, "cost")),
            ints(field(t, "ups")),
            floats(field(t, "dmg"), 5),
            num(field(t, "range")),
            num(field(t, "rate")),
            boolean(field(t, "ground")),
            boolean(field(t, "air")),
            num(field(t, "speed")),
            num(field(t, "splash")),
            uint(field(t, "multi")),
            floats(field(t, "slow"), 5),
            num(field(t, "stun")),
            shot,
            key.chars().next().unwrap(),
            color(field(t, "color")),
            color(field(t, "accent"))
        );
    }
    code += "];\n#[derive(Clone, Copy)]\nstruct WaveDef { kind: EnemyKind, hp: f32, count: usize, bounty: f32,\n\
        gap: f32, targets: &'static [Target] }\n";
    let sets = field(&data, "waveSets")
        .as_object()
        .expect("game-content.json: expected waveSets object");
    let mut set_names = std::collections::HashSet::new();
    for (id, waves) in sets {
        let name = variant(id).to_ascii_uppercase();
        assert!(
            set_names.insert(name.clone()),
            "game-content.json: duplicate wave set identifier"
        );
        assert!(
            !array(waves).is_empty(),
            "game-content.json: empty wave set"
        );
        code += &format!(
            "const WAVES_{name}: [WaveDef; {}] = [\n",
            array(waves).len()
        );
        for wave in array(waves) {
            let kind = text(field(wave, "type"));
            let e = enemies
                .get(kind)
                .expect("game-content.json: unknown wave enemy");
            assert!(
                uint(field(e, "count")) > 0 && num(field(wave, "hp")) > 0.0,
                "game-content.json: invalid wave"
            );
            let targets = array(field(wave, "targets"));
            assert!(
                !targets.is_empty(),
                "game-content.json: missing wave targets"
            );
            let targets = targets
                .iter()
                .map(|t| match text(t) {
                    "R" => "Target::Right",
                    "B" => "Target::Bottom",
                    _ => panic!("game-content.json: unknown target"),
                })
                .collect::<Vec<_>>()
                .join(", ");
            code += &format!(
                "WaveDef {{ kind: EnemyKind::{}, hp: {:?}_f32, count: {}, bounty: {:?}_f32, gap: {:?}_f32, targets: &[{}] }},\n",
                variant(kind),
                num(field(wave, "hp")),
                uint(field(e, "count")),
                num(field(e, "bounty")) * num(field(wave, "bountyMultiplier")),
                num(field(e, "gap")),
                targets
            );
        }
        code += "];\n";
    }
    code += "struct ModeDef { name: &'static str, lives: i32, gold: i32, hp: f32, gap: f32, waves: u32,\n\
        wave_set: &'static [WaveDef] }\n";
    code += &format!("const MODES: [ModeDef; {}] = [\n", levels.len());
    for l in levels {
        let set = text(field(l, "waveSet"));
        let waves = uint(field(l, "waves"));
        assert!(
            waves > 0
                && waves
                    <= array(sets.get(set).expect("game-content.json: unknown wave set")).len()
                        as u64
                && uint(field(l, "lives")) > 0
                && num(field(l, "hp")) > 0.0
                && num(field(l, "gap")) > 0.0,
            "game-content.json: invalid level"
        );
        code += &format!(
            "ModeDef {{ name: {:?}, lives: {}, gold: {}, hp: {:?}_f32, gap: {:?}_f32, waves: {}, wave_set: &WAVES_{} }},\n",
            text(field(l, "name")),
            uint(field(l, "lives")),
            uint(field(l, "gold")),
            num(field(l, "hp")),
            num(field(l, "gap")),
            waves,
            variant(set).to_ascii_uppercase()
        );
    }
    code += "];\nimpl Mode {\n\
        fn def(self) -> &'static ModeDef { &MODES[self.index()] }\n\
        fn name(self) -> &'static str { self.def().name }\n\
        fn lives(self) -> i32 { self.def().lives }\n\
        fn gold(self) -> i32 { self.def().gold }\n\
        fn hp_factor(self) -> f32 { self.def().hp }\n\
        fn wave_gap(self) -> f32 { self.def().gap }\n\
        fn waves(self) -> u32 { self.def().waves }\n}\n";
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("missing OUT_DIR")).join("content.rs");
    fs::write(output, code).expect("cannot write generated game content");
}

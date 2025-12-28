use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};

mod attack;
use crate::attack::{
    AttackRoll, DexterityDecorator, DieRoll, ProficiencyDecorator, StrengthDecorator,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DamageType {
    #[default]
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Cold,
    Lightning,
    Acid,
    Poison,
    Radiant,
    Necrotic,
    Force,
    Psychic,
    Thunder,
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DamageType::Slashing => "Slashing".white(),
            DamageType::Piercing => "Piercing".white(),
            DamageType::Bludgeoning => "Bludgeoning".white(),
            DamageType::Fire => "Fire".red(),
            DamageType::Cold => "Cold".blue(),
            DamageType::Lightning => "Lightning".bright_blue(),
            DamageType::Acid => "Acid".green(),
            DamageType::Poison => "Poison".bright_green(),
            DamageType::Radiant => "Radiant".yellow(),
            DamageType::Necrotic => "Necrotic".magenta(),
            DamageType::Force => "Force".bright_magenta(),
            DamageType::Psychic => "Psychic".bright_cyan(),
            DamageType::Thunder => "Thunder".cyan(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub count: u32,
    pub sides: u32,
}

impl Dice {
    pub fn new(count: u32, sides: u32) -> Self {
        Self { count, sides }
    }

    pub fn min(&self) -> f32 {
        self.count as f32
    }

    pub fn max(&self) -> f32 {
        (self.count * self.sides) as f32
    }

    pub fn avg(&self, savage: bool) -> f32 {
        let base_avg = self.count as f32 * (self.sides as f32 + 1.0) / 2.0;
        if savage {
            let n = self.sides as f32;
            let single_die_avg = (n + 1.0) * (2.0 * n + 1.0) / (6.0 * n);
            self.count as f32 * single_die_avg
        } else {
            base_avg
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSource {
    pub name: String,
    pub dice: Option<Dice>,
    pub flat: i32,
    pub damage_type: DamageType,
}

impl DamageSource {
    pub fn min(&self) -> f32 {
        let d = self.dice.as_ref().map_or(0.0, |d| d.min());
        d + self.flat as f32
    }

    pub fn max(&self) -> f32 {
        let d = self.dice.as_ref().map_or(0.0, |d| d.max());
        d + self.flat as f32
    }

    pub fn avg(&self, savage: bool) -> f32 {
        let d = self.dice.as_ref().map_or(0.0, |d| d.avg(savage));
        d + self.flat as f32
    }

    pub fn crit_avg(&self, savage: bool) -> f32 {
        let d = self.dice.as_ref().map_or(0.0, |d| d.avg(savage) * 2.0);
        d + self.flat as f32
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Strength score
    #[arg(short, long, default_value_t = 10)]
    strength: i32,

    /// Dexterity score
    #[arg(short, long, default_value_t = 10)]
    dexterity: i32,

    /// Proficiency Bonus
    #[arg(short, long, default_value_t = 2)]
    proficiency: i32,

    /// Weapon name
    #[arg(short, long, default_value = "Longsword")]
    weapon: String,

    /// Weapon base damage (e.g. 1d8)
    #[arg(long, default_value = "1d8")]
    base_damage: String,

    /// Weapon damage type
    #[arg(long, default_value = "Slashing")]
    damage_type: String,

    /// Weapon enchantment (+1, +2, etc.)
    #[arg(short, long, default_value_t = 0)]
    enchantment: i32,

    /// Is the weapon finesse?
    #[arg(long)]
    finesse: bool,

    /// Great Weapon Master feat (+10 damage, -5 attack)
    #[arg(long)]
    gwm: bool,

    /// Dueling Fighting Style (+2 damage)
    #[arg(long)]
    dueling: bool,

    /// Savage Attacker feat (reroll damage dice)
    #[arg(long)]
    savage: bool,

    /// Extra damage from gear (format: "Name:Dice:Type,Name:Flat:Type")
    #[arg(long)]
    gear: Option<String>,

    /// Target Armor Class (AC) to calculate expected damage
    #[arg(long)]
    target_ac: Option<i32>,

    /// Attack has Advantage
    #[arg(long)]
    advantage: bool,

    /// Attack has Disadvantage
    #[arg(long)]
    disadvantage: bool,

    /// Number of attacks per turn
    #[arg(short, long, default_value_t = 1)]
    attacks: u32,

    /// Critical hit threshold (default 20)
    #[arg(long, default_value_t = 20)]
    crit_threshold: i32,

    /// Output in JSON format
    #[arg(long)]
    json: bool,

    /// Run the decorator pattern demo for attack rolls
    #[arg(long)]
    roll_demo: bool,
}

#[derive(Serialize)]
struct Report {
    character: CharacterInfo,
    weapon: WeaponInfo,
    sources: Vec<DamageSource>,
    summary: Summary,
    combat: Option<CombatResult>,
}

#[derive(Serialize)]
struct CharacterInfo {
    strength: i32,
    dexterity: i32,
    proficiency: i32,
}

#[derive(Serialize)]
struct WeaponInfo {
    name: String,
    enchantment: i32,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct Summary {
    min: f32,
    max: f32,
    avg: f32,
    crit_avg: f32,
}

#[derive(Serialize)]
struct CombatResult {
    target_ac: i32,
    attack_bonus: i32,
    hit_chance: f32,
    crit_chance: f32,
    expected_dmg_per_attack: f32,
    expected_dmg_per_turn: f32,
}

fn parse_dice(s: &str) -> (Option<Dice>, i32) {
    if !s.contains('d') {
        return (None, s.parse().unwrap_or(0));
    }
    let parts: Vec<&str> = s.split('d').collect();
    if parts.len() != 2 {
        return (None, 0);
    }
    let count = parts[0].parse().unwrap_or(1);
    let sides = parts[1].parse().unwrap_or(0);
    (Some(Dice::new(count, sides)), 0)
}

fn parse_damage_type(s: &str) -> DamageType {
    match s.to_lowercase().as_str() {
        "slashing" => DamageType::Slashing,
        "piercing" => DamageType::Piercing,
        "bludgeoning" => DamageType::Bludgeoning,
        "fire" => DamageType::Fire,
        "cold" => DamageType::Cold,
        "lightning" => DamageType::Lightning,
        "acid" => DamageType::Acid,
        "poison" => DamageType::Poison,
        "radiant" => DamageType::Radiant,
        "necrotic" => DamageType::Necrotic,
        "force" => DamageType::Force,
        "psychic" => DamageType::Psychic,
        "thunder" => DamageType::Thunder,
        _ => DamageType::Slashing,
    }
}

fn calculate_hit_chance(
    bonus: i32,
    ac: i32,
    adv: bool,
    dis: bool,
    crit_threshold: i32,
) -> (f32, f32) {
    let target_roll = ac - bonus;
    let target_roll = target_roll.clamp(2, 20);

    let p_single = (21.0 - target_roll as f32) / 20.0;
    let p_crit_single = (21.0 - crit_threshold as f32) / 20.0;

    let (p_hit, p_crit) = if adv && !dis {
        (
            1.0 - (1.0 - p_single).powi(2),
            1.0 - (1.0 - p_crit_single).powi(2),
        )
    } else if dis && !adv {
        (p_single.powi(2), p_crit_single.powi(2))
    } else {
        (p_single, p_crit_single)
    };

    (p_hit, p_crit)
}

fn main() {
    let args = Args::parse();

    let str_mod = (args.strength - 10) / 2;
    let dex_mod = (args.dexterity - 10) / 2;
    let ability_mod = if args.finesse {
        std::cmp::max(str_mod, dex_mod)
    } else {
        str_mod
    };

    let mut sources = Vec::new();
    let (base_dice, _) = parse_dice(&args.base_damage);
    let base_type = parse_damage_type(&args.damage_type);

    let mut weapon_flat = ability_mod + args.enchantment;
    if args.gwm {
        weapon_flat += 10;
    }
    if args.dueling {
        weapon_flat += 2;
    }

    sources.push(DamageSource {
        name: args.weapon.clone(),
        dice: base_dice,
        flat: weapon_flat,
        damage_type: base_type,
    });

    if let Some(gear_str) = args.gear {
        for item in gear_str.split(',') {
            let parts: Vec<&str> = item.split(':').collect();
            if parts.len() == 3 {
                let name = parts[0].trim();
                let (dice, flat) = parse_dice(parts[1]);
                let dtype = parse_damage_type(parts[2]);
                sources.push(DamageSource {
                    name: name.to_string(),
                    dice,
                    flat,
                    damage_type: dtype,
                });
            }
        }
    }

    let mut total_min = 0.0;
    let mut total_max = 0.0;
    let mut total_avg = 0.0;
    let mut total_crit_avg = 0.0;

    for src in &sources {
        total_min += src.min();
        total_max += src.max();
        total_avg += src.avg(args.savage);
        total_crit_avg += src.crit_avg(args.savage);
    }

    let mut combat_result = None;
    if let Some(ac) = args.target_ac {
        let mut attack_bonus = ability_mod + args.proficiency + args.enchantment;
        if args.gwm {
            attack_bonus -= 5;
        }
        let (p_hit, p_crit) = calculate_hit_chance(
            attack_bonus,
            ac,
            args.advantage,
            args.disadvantage,
            args.crit_threshold,
        );
        let expected_per_attack = (p_hit - p_crit) * total_avg + p_crit * total_crit_avg;
        combat_result = Some(CombatResult {
            target_ac: ac,
            attack_bonus,
            hit_chance: p_hit,
            crit_chance: p_crit,
            expected_dmg_per_attack: expected_per_attack,
            expected_dmg_per_turn: expected_per_attack * args.attacks as f32,
        });
    }

    if args.json {
        let report = Report {
            character: CharacterInfo {
                strength: args.strength,
                dexterity: args.dexterity,
                proficiency: args.proficiency,
            },
            weapon: WeaponInfo {
                name: args.weapon,
                enchantment: args.enchantment,
                tags: vec![], // Could add tags here
            },
            sources,
            summary: Summary {
                min: total_min,
                max: total_max,
                avg: total_avg,
                crit_avg: total_crit_avg,
            },
            combat: combat_result,
        };
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return;
    }

    // Normal Output
    println!(
        "\n{}",
        "=== Baldur's Gate 3 Damage Report ==="
            .bold()
            .bright_yellow()
    );
    println!(
        "{}: {} {} / {} {} (+{} prof)",
        "Character Stats".bold(),
        "STR".red(),
        args.strength,
        "DEX".blue(),
        args.dexterity,
        args.proficiency
    );
    println!(
        "{}: {} (+{} enchantment)",
        "Weapon".bold(),
        args.weapon.green(),
        args.enchantment
    );

    if args.gwm {
        println!("- {}", "Great Weapon Master (+10 dmg, -5 attack)".red());
    }
    if args.dueling {
        println!("- {}", "Dueling (+2 dmg)".green());
    }
    if args.savage {
        println!("- {}", "Savage Attacker (dmg dice reroll)".cyan());
    }
    if args.advantage {
        println!("- {}", "Advantage".bright_yellow());
    }
    if args.disadvantage {
        println!("- {}", "Disadvantage".bright_red());
    }
    println!("");

    println!(
        "{:<25} {:<15} {:<15}",
        "Source".bold(),
        "Damage".bold(),
        "Type".bold()
    );
    println!(
        "{}",
        "------------------------------------------------------------".white()
    );

    for src in &sources {
        let dmg_str = if let Some(d) = &src.dice {
            format!("{}d{} + {}", d.count, d.sides, src.flat)
        } else {
            format!("{}", src.flat)
        };
        println!(
            "{:<25} {:<15} {:<15}",
            src.name.dimmed(),
            dmg_str,
            format!("{}", src.damage_type)
        );
    }

    println!(
        "{}",
        "------------------------------------------------------------".white()
    );
    println!(
        "{:<25} {:>15.1}-{:<4.1} (Avg: {:.2})",
        "TOTAL DAMAGE (per hit)".bold().bright_white(),
        total_min,
        total_max,
        total_avg
    );
    println!(
        "{:<25} {:>15.2}",
        "AVERAGE CRIT".bold().red(),
        total_crit_avg
    );

    if let Some(res) = combat_result {
        println!("\n{}", "=== Combat Simulation ===".bold().bright_cyan());
        println!("{:<25} {:<15}", "Target AC:".dimmed(), res.target_ac);
        println!("{:<25} {:<15}", "Attack Bonus:".dimmed(), res.attack_bonus);
        println!(
            "{:<25} {:>14.1}%",
            "Hit Chance:".dimmed(),
            res.hit_chance * 100.0
        );
        println!(
            "{:<25} {:>14.1}%",
            "Crit Chance:".dimmed(),
            res.crit_chance * 100.0
        );
        println!(
            "{:<25} {:>15.2}",
            "Expected Dmg/Attack:".bold().bright_green(),
            res.expected_dmg_per_attack
        );
        if args.attacks > 1 {
            println!(
                "{:<25} {:>15.2}",
                format!("Expected Dmg/Turn ({}x):", args.attacks)
                    .bold()
                    .bright_green(),
                res.expected_dmg_per_turn
            );
        }
    }

    if args.roll_demo {
        println!(
            "\n{}",
            "=== Decorator Pattern Demo: Attack Rolls ==="
                .bold()
                .bright_magenta()
        );

        // Strength Decorator
        let str_roll = StrengthDecorator {
            inner: DieRoll::new(&args.weapon),
            strength_score: args.strength,
        };

        // Dexterity Decorator
        let dex_roll = DexterityDecorator {
            inner: DieRoll::new(&args.weapon),
            dexterity_score: args.dexterity,
        };

        println!(
            "{:<25}: {} -> Result: {}",
            "Strength-based".cyan(),
            str_roll.description(),
            str_roll.roll().to_string().green()
        );

        println!(
            "{:<25}: {} -> Result: {}",
            "Dexterity-based".cyan(),
            dex_roll.description(),
            dex_roll.roll().to_string().green()
        );

        // Stacked Example: Finesse + Proficiency
        let finesse_prof = ProficiencyDecorator {
            inner: DexterityDecorator {
                inner: DieRoll::new(&args.weapon),
                dexterity_score: args.dexterity,
            },
            bonus: args.proficiency,
        };

        println!(
            "{:<25}: {} -> Result: {}",
            "Finesse + Proficiency".cyan(),
            finesse_prof.description(),
            finesse_prof.roll().to_string().green()
        );

        println!("\n{}", "Note: The decorator pattern allows us to wrap rolls with any number of modifiers dynamically.".italic().dimmed());
    }
}

use serde_json::Value::*;
use serde_json::json;

mod aland_functions;
mod helpers;

use aland_functions::*;
use helpers::*;

use std::thread;
use std::time::Duration;

// Make a list of potions from largest to smallest
fn pot_insert(
    mut pots: Vec<(u64, u64, std::string::String)>,
    new: (u64, u64, std::string::String),
) -> Vec<(u64, u64, std::string::String)> {
    if let Some(index) = pots.iter().position(|x| x.0 < new.0) {
        let ins_index = if index <= 0 { 0 } else { index - 1 };
        pots.insert(ins_index, new)
    } else {
        pots.push(new)
    }
    pots
}

#[allow(unreachable_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    info_local("rust start");

    let mut said_target = false;
    let mut said_attacking = false;
    let mut is_fleeing = false;
    let mut said_fleeing = false;
    let mut is_rehoming = false;
    let mut said_rehoming = false;

    // TODO: make a type for locations; would also affect find_npc and smart_move, if we wanted to
    // do it properly
    let areas = vec![
        (
            "Spooky Forest Snakes",
            json!({"map":"halloween", "in":"halloween", "x":-583, "y":-217}),
        ),
        (
            "Mainland Crabs",
            json!({"map":"main", "in":"main", "x":-1157, "y":-58}),
        ),
        (
            "Mainland Bees",
            json!({"map":"main", "in":"main", "x":313, "y":1143}),
        ),
    ];
    let mut next_area: usize = 0;

    // Clear any target from a previous run
    change_target(&Null)?;

    // Grab the big data piles; there's so much more here, see `grep '\bG\.' js/html.js` in the
    // aland repo
    let g_items = get_js_var("G.items".to_owned())?;
    let g_npcs = get_js_var("G.npcs".to_owned())?;
    let g_levels_raw = get_js_var("G.levels".to_owned())?;
    let g_levels = g_levels_raw.as_object().unwrap();

    loop {
        let character = get_js_var("character".to_owned())?;

        // Stored from largest to smallest
        let mut hp_potions: Vec<(u64, u64, std::string::String)> = vec![];
        let mut mp_potions: Vec<(u64, u64, std::string::String)> = vec![];
        let mut total_hp_potions_count: u64 = 0;

        // Find all our potions
        for (index, item) in character["items"].as_array().unwrap().iter().enumerate() {
            if !item.is_null() {
                let item_name = item["name"].as_str().unwrap();
                let g_item = &g_items[item_name];
                if !g_item["gives"].is_null() {
                    for giver in g_item["gives"].as_array().unwrap() {
                        if giver[0].as_str().unwrap() == "hp" {
                            total_hp_potions_count += item["q"].as_u64().unwrap();
                            hp_potions = pot_insert(
                                hp_potions,
                                (
                                    giver[1].as_u64().unwrap(),
                                    index as u64,
                                    item_name.to_owned(),
                                ),
                            );
                        }
                        if giver[0].as_str().unwrap() == "mp" {
                            mp_potions = pot_insert(
                                mp_potions,
                                (
                                    giver[1].as_u64().unwrap(),
                                    index as u64,
                                    item_name.to_owned(),
                                ),
                            );
                        }
                    }
                }
            }
        }

        for potion in hp_potions {
            if (character["max_hp"].as_u64().unwrap() - character["hp"].as_u64().unwrap())
                >= potion.0
                || character["hp"].as_u64().unwrap() < 500
            {
                if !is_on_cooldown(json!("use_hp"))?.as_bool().unwrap() {
                    info_both(&format!(
                        "Using HP potion {} in slot {} for {} HP",
                        potion.2, potion.1, potion.0
                    ))?;
                    equip(json!(potion.1), Null)?;
                }
                break;
            }
        }

        for potion in mp_potions {
            if (character["max_mp"].as_u64().unwrap() - character["mp"].as_u64().unwrap())
                >= potion.0
                || character["mp"].as_u64().unwrap() < 50
            {
                if !is_on_cooldown(json!("use_mp"))?.as_bool().unwrap() {
                    info_both(&format!(
                        "Using MP potion {} in slot {} for {} MP",
                        potion.2, potion.1, potion.0
                    ))?;
                    equip(json!(potion.1), Null)?;
                }
                break;
            }
        }

        // Skip if moving *after* potions
        if is_moving(&character)?.as_bool().unwrap() {
            continue;
        }

        // *** Conditions For Fleeing
        if character["hp"].as_f64().unwrap() < (character["max_hp"].as_f64().unwrap() * 0.75) {
            is_fleeing = true;
        }

        if character["mp"].as_f64().unwrap() < (character["max_mp"].as_f64().unwrap() * 0.25) {
            is_fleeing = true;
        }

        if total_hp_potions_count < 10 {
            is_fleeing = true;
        }

        // Flee *after* potions
        if is_fleeing {
            if !said_fleeing {
                info_both("Fleeing.")?;
                said_fleeing = true;
            }
            let coords = find_npc(json!("fancypots"))?;

            if simple_distance(&character, &coords)?.as_f64().unwrap() < 200.0 {
                info_both("Done fleeing.")?;
                is_fleeing = false;
                said_fleeing = false;
                if total_hp_potions_count < 10 {
                    info_both(
                        "Done fleeing but you have no potions; sleeping, kill the script when you see this.",
                    )?;
                    thread::sleep(Duration::from_secs(99999999));
                } else {
                    // Go back to current target area
                    is_rehoming = true;
                }
            } else {
                smart_move(coords, Null)?;
                // Don't bother to do other stuff if we're focused on moving
                continue;
            }
        }

        if is_rehoming {
            if !said_rehoming {
                info_both(&format!("Rehoming to {}.", areas[next_area].0))?;
                said_rehoming = true;
            }

            let coords = areas[next_area].1.clone();

            if simple_distance(&character, &coords)?.as_f64().unwrap() < 50.0 {
                info_both(&format!("Done rehoming to {}.", areas[next_area].0))?;
                is_rehoming = false;
                said_rehoming = false;
                next_area = (next_area + 1) % areas.len();
            } else {
                smart_move(coords, Null)?;
                // Don't bother to do other stuff if we're focused on moving
                continue;
            }
        }

        loot(Null)?;

        if character["rip"] != json!(0) && character["rip"] != json!(false) {
            info_both("Character died; sleeping, kill the script when you see this.")?;
            thread::sleep(Duration::from_secs(99999999));
        }

        let mut target = get_targeted_monster()?;

        // TODO: add up the attack totals of all things targetting us, flee as appropriate
        if target.is_null() {
            info_both("No current target, looking for new target.")?;
            said_target = false;
            said_attacking = false;

            loot(Null)?;

            // Look for something that:
            // - has a max attack no higher than twice the character's
            // - has at least 1% of the charaacter's XP to next level
            // - has the highest XP per hit (assuming that one hit does exactly the character's
            // attack damage); XP per hit is XP divided by the number of hits it takes to kill

            // TODO: start with monsters that are attacking us
            // TODO: scrub monsters that are attacking someone else

            let mut best_monster_xp_per_hit = 0.0;

            let nearby_hostiles = get_nearby_hostiles(json!({"range": 100000, "limit": 50}))?;

            let character_level = character["level"].as_u64().unwrap() as usize;
            let one_percent_xp_to_next_level =
                g_levels[&character_level.to_string()].as_f64().unwrap() / 100.0;

            for hostile in nearby_hostiles.as_array().unwrap() {
                // Avoid monsters with high DPS
                if (hostile["attack"].as_f64().unwrap() * hostile["frequency"].as_f64().unwrap())
                    > (character["max_hp"].as_f64().unwrap() * 0.1)
                {
                    continue;
                }

                if hostile["xp"].as_f64().unwrap() < one_percent_xp_to_next_level {
                    continue;
                }

                // Skip target dummies
                if hostile["mtype"].as_str().unwrap().contains("target") {
                    continue;
                }

                let cur_hits_to_kill =
                    hostile["hp"].as_f64().unwrap() / character["attack"].as_f64().unwrap();

                if cur_hits_to_kill > 100.0 {
                    continue;
                }

                let cur_xp_per_hit = hostile["xp"].as_f64().unwrap() / cur_hits_to_kill;

                if cur_xp_per_hit > best_monster_xp_per_hit {
                    best_monster_xp_per_hit = cur_xp_per_hit;
                    target = hostile.clone();
                }
            }

            if target.is_null() {
                info_both("No matching monsters found.")?;
                is_rehoming = true;
                continue;
            } else if target.is_object() {
                eprintln!("Changing to new target {}", target["name"]);

                said_target = true;
                change_target(&target)?;
            } else {
                panic!("target is neither null nor object??");
            }
        } else if target.is_object() {
            if !said_target {
                info_both(&format!("Current target is {}", target["name"]))?;
                said_target = true;
            }
        } else {
            panic!("target is neither null nor object??");
        }

        if !is_in_range(&target, Null)?.as_bool().unwrap() {
            info_both(&format!("Moving towards {}", target["name"]))?;

            smart_move(json!({"x": target["x"], "y": target["y"]}), Null)?;
        } else if can_attack(&target)?.as_bool().unwrap() {
            if !said_attacking {
                info_both(&format!("Attacking {}", target["name"]))?;
                said_attacking = true;
            }
            attack(&target)?;
        }
    }

    Ok(())
}

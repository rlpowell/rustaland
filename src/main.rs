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
    let mut is_banking = false;
    let mut said_banking = false;

    // Clear any target from a previous run
    change_target(&Null)?;

    // Grab the big data piles; there's so much more here, see `grep '\bG\.' js/html.js` in the
    // aland repo
    let g_items = get_js_var("G.items".to_owned())?;
    let g_npcs = get_js_var("G.npcs".to_owned())?;
    let g_maps = get_js_var("G.maps".to_owned())?;
    let g_levels_raw = get_js_var("G.levels".to_owned())?;
    let g_levels = g_levels_raw.as_object().unwrap();

    // In order of preference
    let monsters_to_kill = vec![
        ("any", "pinkgoo"),
        ("halloween", "osnake"),
        ("halloween", "snake"),
        ("halloween", "minimush"),
        ("main", "croc"),
        ("main", "armadillo"),
        ("main", "snake"),
        ("main", "crab"),
        ("main", "bee"),
        ("main", "goo"),
    ];

    // TODO: make a type for locations; would also affect find_npc and smart_move, if we wanted to
    // do it properly
    let mut areas: Vec<(std::string::String, serde_json::Value)> = vec![];

    // Build up a list of area targets to check for monsters
    for monster in &monsters_to_kill {
        for map_name in g_maps.as_object().unwrap().keys() {
            let mut count = 1;
            let map_monsters = &g_maps[map_name]["monsters"];
            if map_monsters.is_array() {
                for map_monster in g_maps[map_name]["monsters"].as_array().unwrap() {
                    if (monster.0 == "any" || map_name == monster.0)
                        && map_monster["type"].as_str().unwrap() == monster.1
                    {
                        let boundary = map_monster["boundary"].as_array().unwrap();
                        let mmx = boundary[0].as_f64().unwrap();
                        let mmy = boundary[1].as_f64().unwrap();
                        areas.push((
                            format!("{} area {} in map {}", monster.1, count, map_name),
                            json!({"map":map_name, "in":map_name, "x":mmx, "y":mmy}),
                        ));
                        count += 1;
                    }
                }
            }
        }
    }

    let mut next_area: usize = 0;

    loop {
        let character = get_js_var("character".to_owned())?;

        if character["rip"] != json!(0) && character["rip"] != json!(false) {
            info_both("Character died; sleeping, kill the script when you see this.")?;
            thread::sleep(Duration::from_secs(99999999));
        }

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
                if !is_on_cooldown(json!("use_hp"))?.as_bool().unwrap_or(true) {
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
                if !is_on_cooldown(json!("use_mp"))?.as_bool().unwrap_or(true) {
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

        // Do we need to bank items?
        //
        // TODO: this is terrible in a way that appling a type to the character's items would
        // almost certainly fix
        if character["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.is_object())
            .collect::<Vec<&serde_json::Value>>()
            .len()
            > 35
        {
            is_banking = true;
        }

        // TODO: fleeing and rehoming and banking all look very similar dontcha think?

        if is_banking {
            if !said_banking {
                info_both("Banking.")?;
                said_banking = true;
            }
            let coords = find_npc(json!("items0"))?;

            if simple_distance(&character, &coords)?.as_f64().unwrap() < 200.0 {
                for (index, item) in character["items"].as_array().unwrap().iter().enumerate() {
                    if index > 13 && !item.is_null() {
                        info_both(&format!(
                            "Storing {} in the bank.",
                            g_items[item["name"].as_str().unwrap()]["name"]
                        ))?;
                        bank_store(json!(index), Null, Null)?;
                    }
                }

                info_both("Done banking, restarting monster search from the top.")?;
                is_banking = false;
                said_banking = false;
                if total_hp_potions_count < 10 {
                    info_both(
                        "Done banking but you have no potions; sleeping, kill the script when you see this.",
                    )?;
                    thread::sleep(Duration::from_secs(99999999));
                } else {
                    // Go back to monster finding
                    is_rehoming = true;

                    // Start the monster cycle from the beginning after banking.
                    next_area = 0;
                }
            } else {
                smart_move(coords, Null)?;
                // Don't bother to do other stuff if we're focused on moving
                continue;
            }
        }

        // Flee *after* potions
        if is_fleeing {
            if !said_fleeing {
                info_both("Fleeing.")?;
                said_fleeing = true;
            }

            // Stun everybody so it's easier to get away
            let target = get_nearest_monster(json!(Null))?;

            if !is_on_cooldown(json!("stomp"))?.as_bool().unwrap_or(true) {
                info_both("Using Stomp skill.")?;
                use_skill("stomp", &target, Null)?;
            }

            let coords = find_npc(json!("fancypots"))?;

            if simple_distance(&character, &coords)?.as_f64().unwrap() < 200.0 {
                info_both("Done fleeing, continuing monster search.")?;
                is_fleeing = false;
                said_fleeing = false;
                if total_hp_potions_count < 10 {
                    info_both(
                        "Done fleeing but you have no potions; sleeping, kill the script when you see this.",
                    )?;
                    thread::sleep(Duration::from_secs(99999999));
                } else {
                    // Go back to monster finding
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

        let mut target = get_targeted_monster()?;

        // TODO: add up the attack totals of all things targetting us, flee as appropriate

        if target.is_null() {
            info_both("No current target, looking for new target.")?;
            said_target = false;
            said_attacking = false;

            thread::sleep(Duration::from_millis(500));
            loot(Null)?;

            for monster in &monsters_to_kill {
                // TODO: there must be some better way to structure this
                target = get_nearest_monster(
                    json!({"target":true, "path_check":true, "type":monster.1}),
                )?;

                if target.is_null() {
                    target = get_nearest_monster(
                        json!({"no_target":true, "path_check":true, "type":monster.1}),
                    )?;
                    if !target.is_null() {
                        if target.as_object().unwrap()["level"].as_u64().unwrap() < 3 {
                            target = Null;
                        } else {
                            break;
                        }
                    }
                } else {
                    if target.as_object().unwrap()["level"].as_u64().unwrap() < 3 {
                        target = Null;
                    } else {
                        break;
                    }
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

            // There's XP-based and how-many-hits-to-kill code in some of the earlier checkins, if
            // you need that; search on cur_hits_to_kill
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
            if !is_on_cooldown(json!("charge"))?.as_bool().unwrap_or(true) {
                info_both("Using Charge skill.")?;
                use_skill("charge", &target, Null)?;
            }
            if !is_on_cooldown(json!("attack"))?.as_bool().unwrap_or(true) {
                attack(&target)?;
            }
        }
    }

    Ok(())
}

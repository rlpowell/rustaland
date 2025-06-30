use serde_json::Value;
use serde_json::json;

use std::env;
use std::fs;

mod aland_functions;
mod helpers;

use aland_functions::*;
use helpers::*;

use std::thread;
use std::time::Duration;

// Make a list of potions from largest to smallest
fn pot_insert(
    mut pots: Vec<(u64, u64, String)>,
    new: (u64, u64, String),
) -> Vec<(u64, u64, String)> {
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

    let character = get_js_var("character".to_owned())?;

    let args: Vec<String> = env::args().collect();

    // Various global data setup stuff
    let my_name = character["name"].as_str().unwrap().to_owned();

    let lead_char = if my_name == args[1].clone() {
        true
    } else {
        false
    };

    let mut party_data: PartyData;

    // NOTE: You might need to manually delete this file if things have changed sufficiently
    if fs::exists(party_data_file_name())? {
        let data = fs::read_to_string(party_data_file_name())?;

        party_data = serde_json::from_str(&data)?;
        // Set stuff from the command line
        party_data.set_lead_char(args[1].clone())?;
        party_data.set_all_chars(args[1..].to_vec().clone())?;
    } else {
        party_data = PartyData::new(args[1].clone(), args[1..].to_vec().clone())?;
    }

    let mut personal_data: PersonalData;

    // NOTE: You might need to manually delete this file if things have changed sufficiently
    let pdfname = personal_data_file_name(&my_name);

    if fs::exists(&pdfname)? {
        let data = fs::read_to_string(&pdfname)?;

        personal_data = serde_json::from_str(&data)?;
    } else {
        personal_data = PersonalData::new(my_name.clone())?;
    }

    let mut local_game_data = LocalGameData { in_party: false };

    eprintln!("game_data done");
    eprintln!("{:#?}", &party_data);
    eprintln!("{:#?}", &personal_data);

    // // Clear any target from a previous run
    // change_target(&Value::Null)?;

    // Grab the big data piles; there's so much more here, see `grep '\bG\.' js/html.js` in the
    // aland repo
    let g_items = get_js_var("G.items".to_owned())?;
    let _g_npcs = get_js_var("G.npcs".to_owned())?;
    let g_maps = get_js_var("G.maps".to_owned())?;
    let g_levels_raw = get_js_var("G.levels".to_owned())?;
    let _g_levels = g_levels_raw.as_object().unwrap();

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
    let mut areas: Vec<(String, serde_json::Value)> = vec![];

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

    // Set up the party
    //
    // TODO: The party seems to break sometimes; move this into the loop
    if !local_game_data.in_party {
        let mut party_list = party_data.all_chars().clone();
        party_list.sort();

        loop {
            let get_party = get_party()?;
            let mut get_party_list = get_party
                .as_object()
                .unwrap()
                .keys()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            get_party_list.sort();

            if party_list == get_party_list {
                info_both("Party set up!")?;
                local_game_data.in_party = true;
                break;
            } else {
                info_both("Party not yet set up, retrying.")?;

                if lead_char {
                    for name in party_data.all_chars() {
                        if *name != my_name {
                            send_party_invite(name, Value::Null)?;
                        }
                    }
                } else {
                    accept_party_invite(&party_data.lead_char())?;
                }

                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    loop {
        // Read the party data; the lead_char writes and maintains this and therefore shouldn't
        // need to read it
        //
        // TODO: Check timestamp to skip the file read for efficiency?
        if !lead_char {
            let data = fs::read_to_string(party_data_file_name()).expect("expect 1");

            party_data = serde_json::from_str(&data)?;
        }

        // Get a bunch of data all at once to minimize fetch()es.  No idea why the outer parens are
        // necessary but they ar.
        let mut buncha_data_text =
            format!("({{ \"{}\": character, \"party\": get_party()", &my_name).to_owned();
        for name in party_data.all_chars().iter().filter(|x| x != &&my_name) {
            buncha_data_text += &format!(", \"{}\": parent.entities[\"{}\"]", name, name);
        }
        buncha_data_text += " })";

        let buncha_data = get_js_var(buncha_data_text)?;

        // Split the data up
        let character = buncha_data[&my_name].clone();
        let get_party_data = buncha_data["party"].clone();
        // eprintln!("gpd {:#?}", &get_party_data);

        // TODO: Check that we have buncha_data for all characters

        if character["rip"] != json!(0) && character["rip"] != json!(false) {
            info_both("Character died; sleeping, kill the script when you see this.")?;
            thread::sleep(Duration::from_secs(99999999));
        }

        // Stored from largest to smallest
        let mut hp_potions: Vec<(u64, u64, String)> = vec![];
        let mut mp_potions: Vec<(u64, u64, String)> = vec![];
        let mut total_hp_potions_count: u64 = 0;
        let mut total_mp_potions_count: u64 = 0;

        // Find all our potions; this is hard because the only way to find a potion is to look in
        // our inventory for an item that "gives" hp or mp
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
                            total_mp_potions_count += item["q"].as_u64().unwrap();
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

        if character["ctype"] == "priest" {
            // Heal if needed
            let heal_amount = character["heal"].as_f64().unwrap();
            let need_healing = party_data
                .all_chars()
                .iter()
                .filter(|x| needs_healing(heal_amount, &buncha_data[x]))
                .collect::<Vec<&String>>();

            if need_healing.len() > 1 {
                if !is_on_cooldown("partyheal")?.as_bool().unwrap_or(true) {
                    info_both("Using Party Heal skill.")?;
                    use_skill("partyheal", &Value::Null, Value::Null)?;
                }
            } else if need_healing.len() == 1 {
                approach_and_use_skill(&character, &buncha_data[need_healing[0]], "heal", "Heal")?;
            }
        }

        for potion in hp_potions {
            // Now we have a healer, save potions for emergencies
            //
            // if (character["max_hp"].as_u64().unwrap() - character["hp"].as_u64().unwrap())
            //     >= potion.0
            //     || character["hp"].as_u64().unwrap() < 500
            if (character["max_hp"].as_u64().unwrap() - character["hp"].as_u64().unwrap()) >= 1000 {
                if !is_on_cooldown("use_hp")?.as_bool().unwrap_or(true) {
                    info_both(&format!(
                        "Using HP potion {} in slot {} for {} HP",
                        potion.2, potion.1, potion.0
                    ))?;
                    equip(json!(potion.1), Value::Null)?;
                }
                break;
            }
        }

        for potion in mp_potions {
            if (character["max_mp"].as_u64().unwrap() - character["mp"].as_u64().unwrap())
                >= potion.0
                || character["mp"].as_u64().unwrap() < 50
            {
                if !is_on_cooldown("use_mp")?.as_bool().unwrap_or(true) {
                    info_both(&format!(
                        "Using MP potion {} in slot {} for {} MP",
                        potion.2, potion.1, potion.0
                    ))?;
                    equip(json!(potion.1), Value::Null)?;
                }
                break;
            }
        }

        // Skip if moving *after* potions
        if is_moving(&character)?.as_bool().unwrap() {
            continue;
        }

        // *** Conditions For Fleeing
        if character["hp"].as_f64().unwrap() < (character["max_hp"].as_f64().unwrap() * 0.50) {
            eprintln!("fleeing 1");
            personal_data.change_state(&party_data, false, State::Fleeing)?;
        }

        if character["mp"].as_f64().unwrap() < (character["max_mp"].as_f64().unwrap() * 0.25) {
            eprintln!("fleeing 2");
            personal_data.change_state(&party_data, false, State::Fleeing)?;
        }

        if total_hp_potions_count < 10 {
            eprintln!("fleeing 3");
            personal_data.change_state(&party_data, false, State::Fleeing)?;
        }

        if total_mp_potions_count < 10 {
            eprintln!("fleeing 4");
            personal_data.change_state(&party_data, false, State::Fleeing)?;
        }

        // Do we need to bank items?
        //
        // TODO: this is terrible in a way that applying a type to the character's items would
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
            eprintln!("banking 1");
            personal_data.change_state(&party_data, false, State::Banking)?;
        }

        // Check that the party is still together
        for name in party_data.all_chars() {
            // TODO: fill this in
        }

        // **** Rectify states

        // Check if anyone is fleeing or banking
        let states = everyones_states(&party_data)?;

        if states.iter().any(|x| x == &State::Banking) {
            eprintln!("anyone_banking");
            personal_data.change_state(&party_data, false, State::Banking)?;
        }

        if states.iter().any(|x| x == &State::Fleeing) {
            eprintln!("anyone_fleeing");
            personal_data.change_state(&party_data, false, State::Fleeing)?;
        }

        if lead_char {
            // The lead_char doesn't need to rectify its own state except in the fleeing and
            // banking cases above I don't think?
        } else {
            let data = fs::read_to_string(personal_data_file_name(party_data.lead_char()))
                .expect("expect 4");

            let lead_char_data: PersonalData = serde_json::from_str(&data)?;

            match &personal_data.state() {
                State::Startup | State::Fleeing | State::Banking | State::Gathering => {
                    // Keep going until you're done
                }
                State::Attacking | State::Rehoming | State::Targeting => {
                    // Match the lead character as needed
                    if personal_data.state() != lead_char_data.state() {
                        personal_data.change_state(
                            &party_data,
                            true,
                            lead_char_data.state().clone(),
                        )?;
                    }
                }
            }
        }

        loot(Value::Null)?;

        // Handle state sync
        if personal_data.waiting_for_sync() {
            if personal_data.sync_count() > 10 {
                eprintln!("Too many sync attempts, gathering.");
                personal_data.change_state(&party_data, false, State::Gathering)?;
            } else {
                let states = everyones_states(&party_data)?;

                if states.iter().all(|x| x == personal_data.state()) {
                    personal_data.set_waiting_for_sync(false);
                    personal_data.set_sync_count(0);
                } else {
                    eprintln!("Waiting for sync on {:#?}", personal_data.state());
                    thread::sleep(Duration::from_millis(100));
                }

                personal_data.set_sync_count(personal_data.sync_count() + 1);
            }
        }

        // eprintln!("About to hit the match: {:#?}", &personal_data.state());

        match &personal_data.state() {
            State::Gathering => {
                eprint!("gpd: {:#?}", get_party_data);
                thread::sleep(Duration::from_millis(100));
                if lead_char {
                    eprintln!("Waiting for others to gather.");
                    let mut someone_too_far = false;
                    for name in party_data.all_chars().iter().filter(|x| x != &&my_name) {
                        let map_name = get_party_data[name]["map"].as_str().unwrap();
                        let ocx = my_as_f64(&get_party_data[name]["x"]);
                        let ocy = my_as_f64(&get_party_data[name]["y"]);

                        if simple_distance(
                            &character,
                            &json!({"map":map_name, "in":map_name, "x":ocx, "y":ocy}),
                        )?
                        .as_f64()
                        .unwrap()
                            > 200.0
                        {
                            someone_too_far = true;
                        }
                    }

                    if !someone_too_far {
                        personal_data.change_state(&party_data, true, State::Rehoming)?;
                    }
                } else {
                    let map_name = get_party_data[&party_data.lead_char()]["map"]
                        .as_str()
                        .unwrap();
                    let lcx = my_as_f64(&get_party_data[&party_data.lead_char()]["x"]);
                    let lcy = my_as_f64(&get_party_data[&party_data.lead_char()]["y"]);
                    if still_moving_to_location(
                        &character,
                        json!({"map":map_name, "in":map_name, "x":lcx, "y":lcy}),
                    )? {
                        continue;
                    } else {
                        personal_data.change_state(&party_data, true, State::Rehoming)?;
                    }
                }
            }
            State::Banking => {
                // TODO: use home skill
                personal_data.announce_as_needed("Banking.")?;

                if still_moving_to_npc(&character, "items0")? {
                    continue;
                }

                for (index, item) in character["items"].as_array().unwrap().iter().enumerate() {
                    if index > 13 && !item.is_null() {
                        info_both(&format!(
                            "Storing {} in the bank.",
                            g_items[item["name"].as_str().unwrap()]["name"]
                        ))?;
                        bank_store(json!(index), Value::Null, Value::Null)?;
                    }
                }

                info_both("Done banking, restarting monster search from the top.")?;

                if lead_char {
                    party_data.set_next_area(0)?;
                }

                check_if_bailing(total_hp_potions_count)?;

                personal_data.change_state(&party_data, true, State::Rehoming)?;
            }
            State::Targeting => {
                if lead_char {
                    let mut target = Value::Null;

                    if !party_data.targets().is_empty() {
                        let target_id = party_data.targets()[0].clone();
                        target = get_js_var(format!("parent.entities[{}]", target_id))?;
                    }

                    // TODO: add up the attack totals of all things targetting us, flee as appropriate; see telegram notes

                    if target.is_null() {
                        info_both("No current target, looking for new target.")?;

                        loot(Value::Null)?;

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
                                        target = Value::Null;
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                if target.as_object().unwrap()["level"].as_u64().unwrap() < 3 {
                                    target = Value::Null;
                                } else {
                                    break;
                                }
                            }
                        }

                        if target.is_null() {
                            info_both("No matching monsters found.")?;
                            personal_data.change_state(&party_data, true, State::Rehoming)?;
                        } else if target.is_object() {
                            personal_data.announce_as_needed(&format!(
                                "Changing to new target {}",
                                target["name"]
                            ))?;

                            change_target(&target)?;

                            party_data
                                .set_targets(vec![target["id"].as_str().unwrap().to_owned()])?;

                            personal_data.change_state(&party_data, true, State::Attacking)?;
                        } else {
                            panic!("target is neither null nor object??");
                        }

                        // There's XP-based and how-many-hits-to-kill code in some of the earlier checkins, if
                        // you need that; search on cur_hits_to_kill
                    } else if target.is_object() {
                        personal_data
                            .announce_as_needed(&format!("Current target is {}", target["name"]))?;

                        personal_data.change_state(&party_data, true, State::Attacking)?;
                    } else {
                        panic!("target is neither null nor object??");
                    }
                } else {
                    eprintln!("Waiting on lead char for targetting");
                    thread::sleep(Duration::from_millis(100));
                }
            }
            State::Attacking => {
                // TODO: Kiting for ranged

                let mut target_id = "".to_owned();

                if !party_data.targets().is_empty() {
                    target_id = party_data.targets()[0].clone();
                }

                let target = get_js_var(format!("parent.entities[{}]", target_id))?;

                if target.is_null() {
                    if lead_char {
                        personal_data.change_state(&party_data, true, State::Targeting)?;
                        continue;
                    } else {
                        eprintln!("Waiting on lead char for new target");
                        thread::sleep(Duration::from_millis(100));
                    }
                } else {
                    match character["ctype"].as_str().unwrap() {
                        "priest" => {
                            personal_data
                                .announce_as_needed(&format!("Attacking {}", target["name"]))?;

                            // TODO: Kiting for ranged
                            approach_and_use_skill(&character, &target, "attack", "Attack")?;

                            // TODO: other priest skills
                            if character["level"].as_u64().unwrap() > 55 {
                                // Draw aggro if needed
                                let mut draw_aggro = false;
                                for name in party_data.all_chars().iter().filter(|x| x != &&my_name)
                                {
                                    let gnm = get_nearest_monster(json!({"target": name}))?;
                                    if !gnm.is_null() {
                                        draw_aggro = true;
                                    }
                                }

                                if draw_aggro {
                                    if !is_on_cooldown("absorb")?.as_bool().unwrap_or(true) {
                                        info_both("Using Absorb skill.")?;
                                        use_skill("absorb", &Value::Null, Value::Null)?;
                                    }
                                }
                            }
                        }
                        "warrior" => {
                            // TODO: other warrior skills

                            personal_data
                                .announce_as_needed(&format!("Attacking {}", target["name"]))?;

                            if !is_on_cooldown("charge")?.as_bool().unwrap_or(true) {
                                info_both("Using Charge skill.")?;
                                use_skill("charge", &target, Value::Null)?;
                            }

                            approach_and_use_skill(&character, &target, "attack", "Attack")?;
                        }
                        "ranger" => {
                            // TODO: other ranger skills

                            personal_data
                                .announce_as_needed(&format!("Attacking {}", target["name"]))?;

                            approach_and_use_skill(&character, &target, "attack", "Attack")?;
                        }
                        _ => {
                            todo!("What the hell?");
                        }
                    }
                }
            }
            State::Fleeing => {
                // TODO: use town skill

                personal_data.announce_as_needed("Fleeing.")?;

                // Stun everybody so it's easier to get away
                let target = get_nearest_monster(json!(Value::Null))?;

                if character["ctype"].as_str().unwrap() == "warrior" {
                    if !is_on_cooldown("stomp")?.as_bool().unwrap_or(true) {
                        info_both("Using Stomp skill.")?;
                        use_skill("stomp", &target, Value::Null)?;
                    }
                }

                if still_moving_to_npc(&character, "fancypots")? {
                    continue;
                }

                info_both("Done fleeing, continuing monster search.")?;

                check_if_bailing(total_hp_potions_count)?;

                personal_data.change_state(&party_data, true, State::Rehoming)?;
            }
            State::Rehoming => {
                let next_area = party_data.next_area().clone();

                personal_data
                    .announce_as_needed(&format!("Rehoming to {}.", areas[next_area].0))?;

                if still_moving_to_location(&character, areas[next_area].1.clone())? {
                    continue;
                }

                info_both(&format!("Done rehoming to {}.", areas[next_area].0))?;

                if lead_char {
                    party_data.set_next_area((next_area + 1) % areas.len())?;
                }

                personal_data.change_state(&party_data, true, State::Targeting)?;
            }
            State::Startup => {
                personal_data.change_state(&party_data, true, State::Rehoming)?;
            }
        };
    }

    Ok(())
}

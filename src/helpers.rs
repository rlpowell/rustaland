use crate::aland_functions::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

const DEBUG: bool = false;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum State {
    Attacking,
    Banking,
    Fleeing,
    Rehoming,
    Startup,
    Targeting,
}

// Private so that every write
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct PartyData {
    // FIXME: Most things in here should get proper types some day
    lead_char: String,
    all_chars: Vec<String>,
    next_area: usize,
    targets: Vec<String>,
    state: State,
}

impl PartyData {
    pub fn new(
        lead_char: String,
        all_chars: Vec<String>,
    ) -> Result<PartyData, Box<dyn std::error::Error>> {
        let new_self = PartyData {
            lead_char: lead_char,
            all_chars: all_chars,
            next_area: 0,
            targets: vec![],
            state: State::Startup,
        };

        new_self.write_to_file()?;

        Ok(new_self)
    }

    pub fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pdfname = party_data_file_name();
        let pdfname_temp = format!("{}.temp", pdfname);
        fs::write(&pdfname_temp, serde_json::to_string(&self)?)?;

        // Make the update atomic
        fs::rename(pdfname_temp, &pdfname)?;

        Ok(())
    }

    pub fn set_next_area(&mut self, next_area: usize) -> Result<(), Box<dyn std::error::Error>> {
        info_both(&format!("Changing area to {:#?}", next_area))?;
        self.next_area = next_area;

        self.write_to_file()?;

        Ok(())
    }

    pub fn set_state(&mut self, state: State) -> Result<(), Box<dyn std::error::Error>> {
        self.state = state;

        self.write_to_file()?;

        Ok(())
    }

    pub fn set_targets(&mut self, targets: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.targets = targets;

        self.write_to_file()?;

        Ok(())
    }

    pub fn set_all_chars(
        &mut self,
        all_chars: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.all_chars = all_chars;

        self.write_to_file()?;

        Ok(())
    }

    pub fn set_lead_char(&mut self, lead_char: String) -> Result<(), Box<dyn std::error::Error>> {
        self.lead_char = lead_char;

        self.write_to_file()?;

        Ok(())
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn all_chars(&self) -> &Vec<String> {
        &self.all_chars
    }

    pub fn lead_char(&self) -> &String {
        &self.lead_char
    }

    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    pub fn next_area(&self) -> usize {
        self.next_area
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PersonalData {
    // FIXME: Most things in here should get proper types some day
    name: String,
    last_seen_state: State,
    has_announced: bool,
    has_completed: bool,
    needs_to_bank: bool,
    needs_to_flee: bool,
    map: String,
    x: f64,
    y: f64,
}

impl PersonalData {
    pub fn new(my_name: String) -> Result<PersonalData, Box<dyn std::error::Error>> {
        let new_self = PersonalData {
            name: my_name,
            last_seen_state: State::Startup,
            has_announced: false,
            has_completed: false,
            needs_to_bank: false,
            needs_to_flee: false,
            map: "main".to_owned(),
            x: 0.0,
            y: 0.0,
        };

        new_self.write_to_file()?;

        Ok(new_self)
    }

    pub fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pdfname = personal_data_file_name(&self.name);
        let pdfname_temp = format!("{}.temp", pdfname);
        fs::write(&pdfname_temp, serde_json::to_string(&self)?)?;

        // Make the update atomic
        fs::rename(pdfname_temp, &pdfname)?;

        Ok(())
    }

    pub fn announce_as_needed(
        &mut self,
        announcement: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.has_announced {
            info_both(announcement)?;
            self.has_announced = true;
        }

        self.write_to_file()?;

        Ok(())
    }

    pub fn check_state_change(&mut self, state: State) -> Result<(), Box<dyn std::error::Error>> {
        if self.last_seen_state != state {
            info_both(&format!(
                "Moving from state {:#?} to state {:#?}",
                self.last_seen_state, state,
            ))?;
            self.last_seen_state = state;
            self.has_completed = false;
            self.has_announced = false;
        }

        self.write_to_file()?;

        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn has_completed(&self) -> bool {
        self.has_completed
    }

    pub fn set_completed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.has_completed = true;

        self.write_to_file()?;

        Ok(())
    }

    pub fn needs_to_bank(&self) -> bool {
        self.needs_to_bank
    }

    pub fn set_needs_to_bank(
        &mut self,
        needs_to_bank: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.needs_to_bank = needs_to_bank;

        self.write_to_file()?;

        Ok(())
    }

    pub fn needs_to_flee(&self) -> bool {
        self.needs_to_flee
    }

    pub fn set_needs_to_flee(
        &mut self,
        needs_to_flee: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.needs_to_flee = needs_to_flee;

        self.write_to_file()?;

        Ok(())
    }

    pub fn set_location(&mut self, character: &Value) -> Result<(), Box<dyn std::error::Error>> {
        self.map = character["map"].as_str().unwrap().to_string();
        self.x = my_as_f64(&character["x"]);
        self.y = my_as_f64(&character["y"]);

        self.write_to_file()?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LocalGameData {
    pub in_party: bool,
}

// This function takes an incoming line from server.py and parses it.  Note that
// `io::stdin().read_line` blocks and that is an important and intentional behaviour here; if
// your language doesn't have a blocking read line you'll want to explicitly wait.
//
// The `return_num` argument specifies whether the sequence number should be kept in the result or
// not.
pub fn get_line_from_python(return_num: bool) -> Result<Value, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input = input.trim_end().to_string();

    debug_local(&format!("inp: {input}"));

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&input)?;

    debug_local(&format!("v: {:#?}", v));

    if return_num {
        Ok(v.clone())
    } else {
        Ok(v[1].clone())
    }
}

// This handles the two-step communication flow with server.py; see
// porting-to-other-languages/README.md for details of why it works the way it does.
//
// It is hilarious to me that our sequence number is signed 128 bits, but that's what was available
// in serde_json.  At one command every 50 miliseconds, we could run for
// 276,576,778,149,171,009,759,427,186,908 years :D
pub fn handle_flow(to_send_string: String) -> Result<Value, Box<dyn std::error::Error>> {
    let value = get_line_from_python(true).unwrap();

    if value[1] != json!("ready") {
        panic!(
            "Exepcted a GET/ready line from upstream but didn't get one; something got confused, bailing."
        );
    }

    debug_local(&format!("value: {}", value));

    let newnum = value[0].as_i64().unwrap() + 1i64;

    let string = Value::Array(vec![
        Value::Number(serde_json::Number::from_i128(i128::from(newnum)).unwrap()),
        Value::String(to_send_string),
    ])
    .to_string();

    println!("{}", string);

    get_line_from_python(false)
}

// This is literally just handle_flow, but aliased to make it more clear what it's doing in this
// case; expected usage is like `let character = get_js_var("character");`, where the javascript
// that's run on the other end is literally just the string `character`, which of course evaluates
// to your character object.
pub fn get_js_var(to_send_string: String) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(to_send_string)
}

pub fn info_local(string: &str) {
    eprintln!("{}", string);
}

pub fn debug_local(string: &str) {
    if DEBUG {
        eprintln!("{}", string);
    }
}

// TODO: Print timestamp (just in the local version?)
pub fn info_both(string: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("{}", string);
    // set_message(Value::String("No monsters found.".to_string()), Value::Null)?;
    game_log(json!(string), Value::Null, Value::Null)?;
    Ok(())
}

pub fn get_id(v: &Value) -> &str {
    // We use null to reset our target for example
    if v.is_null() {
        "null"
    } else {
        v["id"].as_str().unwrap()
    }
}

/*
pub fn debug_both(string: &str) -> Result<(), Box<dyn std::error::Error>> {
    if DEBUG {
        eprintln!("{}", string);
        // set_message(Value::String("No monsters found.".to_string()), Value::Null)?;
        game_log(json!(string), Value::Null, Value::Null)?;
    }
    Ok(())
}
*/

// We don't particularly want to be sending entire entity objects back and forth, especially since
// we don't necessarily have an entire copy (we use game_stringify() on the far end to avoid
// dealing with circular objects).
//
// Note that in brief testing sending the whole object *does* seem to work, we just don't want to
//
// So, if it's us we just use the `character` variable name, if it's not we use parent.entities
//
// Hopefully that works forever; if not we'll need to do something else.  Maybe have something on
// the far end that sends a list of the IDs in parent.entities and if it's not in there, send the
// whole object anyway
pub fn deref_entity(v: &Value) -> String {
    if v["me"] == json!(1) {
        "character".to_string()
    } else if v["id"].is_null() {
        // coords obj, hopefully
        v.to_string()
    } else {
        format!(
            "parent.entities[\"{}\"] || G.npcs[\"{}\"]",
            get_id(v),
            get_id(v)
        )
    }
}

pub fn check_if_bailing(total_hp_potions_count: u64) -> Result<(), Box<dyn std::error::Error>> {
    if total_hp_potions_count < 10 {
        info_both("You have no potions; sleeping, kill the script when you see this.")?;
        thread::sleep(Duration::from_secs(99999999));
    }

    Ok(())
}

pub fn still_moving_to_location(
    character: &Value,
    coords: Value,
) -> Result<bool, Box<dyn std::error::Error>> {
    if simple_distance(character, &coords)? > 200.0 {
        half_move(&character, &coords)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn still_moving_to_npc(
    character: &Value,
    npcid: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let coords = find_npc(json!(npcid))?;

    if simple_distance(character, &coords)? > 200.0 {
        half_move(&character, &coords)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn personal_data_file_name(name: &String) -> String {
    format!("/tmp/rustaland_personal_data_{}.json", name)
}

pub fn party_data_file_name() -> String {
    "/tmp/rustaland_party_data.json".to_owned()
}

pub fn everyones_personal_data(
    party_data: &PartyData,
) -> Result<Vec<PersonalData>, Box<dyn std::error::Error>> {
    let mut datas: Vec<PersonalData> = vec![];

    for name in party_data.all_chars() {
        let fname = personal_data_file_name(name);

        while !fs::exists(&fname)? {
            info_both(&format!("Waiting on personal data file for {name}"))?;
            thread::sleep(Duration::from_millis(500));
        }

        // TODO: Could save a lot of file reading by checking timestamps or something
        let data = fs::read_to_string(fname).expect("expect 3");

        let temp_data: PersonalData = serde_json::from_str(&data)?;

        datas.push(temp_data.clone());
    }

    Ok(datas)
}

pub fn my_as_f64(value: &Value) -> f64 {
    if value.is_f64() {
        return value.as_f64().unwrap();
    } else if value.is_i64() {
        return value.as_i64().unwrap() as f64;
    } else {
        // This happens when an object isn't found or something
        // FIXME: Can we re-arrange function calls to move "is this actually valid?" to somewhere
        // that can make a better decision?  Or just make this a Result and catch it properly?
        return 0.0;
        // panic!("bad value {:#?}", value);
    }
}

pub fn needs_healing(heal_amount: f64, character: &Value) -> bool {
    if character["hp"].is_number() && character["max_hp"].is_number() {
        let hp = my_as_f64(&character["hp"]);
        let max_hp = my_as_f64(&character["max_hp"]);

        if hp / max_hp < 0.8 {
            return true;
        }

        if max_hp - hp > heal_amount {
            return true;
        }
    }

    false
}

pub fn half_move(character: &Value, target: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if character["map"].as_str().unwrap() != target["map"].as_str().unwrap() {
        // We somehow got badly out of position; use smart_move
        let tname: String;
        if target["name"].is_string() {
            tname = target["name"].as_str().unwrap().to_owned();
        } else {
            tname = format!("{:#?}", target);
        }
        info_both(&format!(
            "Target {} is on map {} but we're on map {}, using smart_move",
            tname,
            target["map"].as_str().unwrap(),
            character["map"].as_str().unwrap()
        ))?;
        debug_local("smart move due to other map");
        smart_move(&target, Value::Null)?;
        debug_local("after smart move due to other map");
    } else {
        if character["x"].is_number()
            && character["y"].is_number()
            && target["x"].is_number()
            && target["y"].is_number()
        {
            // Walk half way to target
            let c_x = my_as_f64(&character["x"]);
            let c_y = my_as_f64(&character["y"]);
            let t_x = my_as_f64(&target["x"]);
            let t_y = my_as_f64(&target["y"]);

            let dist = simple_distance(character, target)?;
            debug_local(&format!("dist: {:#?}", dist));
            if simple_distance(character, target)? > 1000.0 {
                // We're far enough away that the half way point is not likely to be useful
                debug_local("smart move due to distance");
                smart_move(&target, Value::Null)?;
                debug_local("after smart move due to distance");
            } else {
                if can_move_to(c_x + (t_x - c_x) / 2.0, c_y + (t_y - c_y) / 2.0)?
                    .as_bool()
                    .unwrap()
                {
                    debug_local("move, close");
                    almove(c_x + (t_x - c_x) / 2.0, c_y + (t_y - c_y) / 2.0)?;
                } else {
                    debug_local("smart_move, blocked");
                    smart_move(&target, Value::Null)?;
                    debug_local("after smart_move, blocked");
                }
            }
        } else {
            debug_local(&format!("Bad half_move: {:#?}, {:#?}", character, target));
        }
    }

    Ok(())
}

pub fn approach_and_use_skill(
    character: &Value,
    target: &Value,
    skill: &str,
    skill_pretty: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !is_on_cooldown(skill)?.as_bool().unwrap_or(true) {
        if !is_in_range(&target, skill)?.as_bool().unwrap() {
            info_both(&format!("Moving towards {}", target["name"]))?;

            half_move(&character, &target)?;
        } else {
            info_both(&format!(
                "Using {skill_pretty} skill on {}.",
                target["name"]
            ))?;
            use_skill(skill, target, Value::Null)?;
        }
    }
    Ok(())
}

pub fn handle_sync(
    party_data: &mut PartyData,
    lead_char: bool,
    personal_datas: &Vec<PersonalData>,
    next_state: State,
) -> Result<(), Box<dyn std::error::Error>> {
    if lead_char {
        if personal_datas.iter().all(|x| x.has_completed()) {
            info_both(&format!("Sync completed on {:#?}", party_data.state()))?;
            party_data.set_state(next_state)?;
        } else {
            info_both(&format!("Waiting for sync on {:#?}", party_data.state()))?;
            thread::sleep(Duration::from_millis(100));
        }
    } else {
        info_both(&format!("Waiting for sync on {:#?}", party_data.state()))?;
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

pub fn get_coords_from_pds(
    personal_datas: &Vec<PersonalData>,
    name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut value = Value::Null;

    for pd in personal_datas {
        if name == pd.name {
            value = json!({"map":pd.map, "in":pd.map, "x":pd.x, "y":pd.y});
        }
    }

    Ok(value)
}

pub fn get_coords_from_value(value: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let map_name = value["map"].as_str().unwrap();
    let ocx = my_as_f64(&value["x"]);
    let ocy = my_as_f64(&value["y"]);

    Ok(json!({"map":map_name, "in":map_name, "x":ocx, "y":ocy}))
}

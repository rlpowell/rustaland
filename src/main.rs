use serde_json::Value::*;
use serde_json::json;

mod aland_functions;
mod helpers;

use aland_functions::*;
use helpers::*;

#[allow(unreachable_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    info_local("rust start");

    let mut said_target = false;

    loop {
        use_hp_or_mp()?;
        loot(Null)?;

        let character = get_js_var("character".to_owned())?;

        if character["rip"] != json!(0) {
            info_both("Character died, exiting.")?;
            panic!("Character died, exiting.");
        }

        if is_moving(&character)?.as_bool().unwrap() {
            // info_both("Character is moving.")?;
            continue;
        }

        let mut target = get_targeted_monster()?;

        if target.is_null() {
            info_both("No current target, looking for new target.")?;
            said_target = false;

            target = get_nearest_monster(json!({"min_xp": 100, "max_att": 120}))?;

            if target.is_null() {
                info_both("No matching monsters found.")?;
                continue;
            } else if target.is_object() {
                eprintln!("Changing to new target {}", target["name"]);

                said_target = true;
                change_target(&target)?;
            } else {
                panic!("target is neither null nor object??");
            }
        } else if target.is_object() {
            if said_target {
                info_both(&format!("Current target is {}", target["name"]))?;
                said_target = true;
            }
        } else {
            panic!("target is neither null nor object??");
        }

        if !is_in_range(&target, Null)?.as_bool().unwrap() {
            info_both(&format!("Moving towards {}", target["name"]))?;

            // smart_move(json!({"x": target["x"], "y": target["y"]}), Null)?;

            let c_x = character["x"].as_f64().unwrap();
            let c_y = character["y"].as_f64().unwrap();
            let t_x = target["x"].as_f64().unwrap();
            let t_y = target["y"].as_f64().unwrap();
            // Move half the distance
            almove(
                json!(c_x + (t_x - c_x) / 2.0),
                json!(c_y + (t_y - c_y) / 2.0),
            )?;
        } else if can_attack(&target)?.as_bool().unwrap() {
            info_both(&format!("Attacking {}", target["name"]))?;
            attack(&target)?;
        }
    }

    Ok(())
}

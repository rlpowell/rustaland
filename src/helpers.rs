use crate::aland_functions::*;
use serde_json::Value;
use serde_json::json;
use std::io;

const DBG: bool = false;

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
    if DBG {
        eprintln!("{}", string);
    }
}

pub fn info_both(string: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("{}", string);
    // set_message(Value::String("No monsters found.".to_string()), Value::Null)?;
    game_log(json!(string), Value::Null, Value::Null)?;
    Ok(())
}

pub fn get_id(v: &Value) -> &str {
    v["id"].as_str().unwrap()
}

/*
pub fn debug_both(string: &str) -> Result<(), Box<dyn std::error::Error>> {
    if DBG {
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
    } else {
        format!("parent.entities[{}]", get_id(v))
    }
}

use crate::helpers::*;
use serde_json::Value;

#[allow(dead_code)]
pub fn mode_resolve_all() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("mode_resolve_all();"))
}

#[allow(dead_code)]
pub fn start_character(
    name: Value,
    code_slot_or_name: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let code_slot_or_name_string = code_slot_or_name.to_string();

    handle_flow(format!(
        "start_character({name_string}, {code_slot_or_name_string});"
    ))
}

#[allow(dead_code)]
pub fn stop_character(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("stop_character({name_string});"))
}

#[allow(dead_code)]
pub fn command_character(
    name: Value,
    code_snippet: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let code_snippet_string = code_snippet.to_string();

    handle_flow(format!(
        "command_character({name_string}, {code_snippet_string});"
    ))
}

#[allow(dead_code)]
pub fn get_active_characters() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_active_characters();"))
}

#[allow(dead_code)]
pub fn change_server(region: Value, name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let region_string = region.to_string();
    let name_string = name.to_string();

    handle_flow(format!("change_server({region_string}, {name_string});"))
}

#[allow(dead_code)]
pub fn in_pvp() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("in_pvp();"))
}

#[allow(dead_code)]
pub fn is_npc(entity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();

    handle_flow(format!("is_npc({entity_string});"))
}

#[allow(dead_code)]
pub fn is_monster(entity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();

    handle_flow(format!("is_monster({entity_string});"))
}

#[allow(dead_code)]
pub fn is_character(entity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();

    handle_flow(format!("is_character({entity_string});"))
}

#[allow(dead_code)]
pub fn interact(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("interact({name_string});"))
}

#[allow(dead_code)]
pub fn enter(place: Value, name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let place_string = place.to_string();
    let name_string = name.to_string();

    handle_flow(format!("enter({place_string}, {name_string});"))
}

#[allow(dead_code)]
pub fn join(event: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let event_string = event.to_string();

    handle_flow(format!("join({event_string});"))
}

#[allow(dead_code)]
pub fn use_nearest_door() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("use_nearest_door();"))
}

#[allow(dead_code)]
pub fn activate(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("activate({num_string});"))
}

#[allow(dead_code)]
pub fn shift(num: Value, name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let name_string = name.to_string();

    handle_flow(format!("shift({num_string}, {name_string});"))
}

#[allow(dead_code)]
pub fn throw_item(num: Value, x: Value, y: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let x_string = x.to_string();
    let y_string = y.to_string();

    handle_flow(format!("throw_item({num_string}, {x_string}, {y_string});"))
}

#[allow(dead_code)]
pub fn use_skill(
    name: &str,
    target: &Value,
    extra_arg: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let extra_arg_string = extra_arg.to_string();

    handle_flow(format!(
        "use_skill(\"{name}\", {}, {extra_arg_string});",
        deref_entity(target)
    ))
}

#[allow(dead_code)]
pub fn reduce_cooldown(name: Value, ms: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let ms_string = ms.to_string();

    handle_flow(format!("reduce_cooldown({name_string}, {ms_string});"))
}

#[allow(dead_code)]
pub fn bank_deposit(gold: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let gold_string = gold.to_string();

    handle_flow(format!("bank_deposit({gold_string});"))
}

#[allow(dead_code)]
pub fn bank_withdraw(gold: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let gold_string = gold.to_string();

    handle_flow(format!("bank_withdraw({gold_string});"))
}

#[allow(dead_code)]
pub fn bank_store(
    num: Value,
    pack: Value,
    pack_num: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let pack_string = pack.to_string();
    let pack_num_string = pack_num.to_string();

    handle_flow(format!(
        "bank_store({num_string}, {pack_string}, {pack_num_string});"
    ))
}

#[allow(dead_code)]
pub fn bank_retrieve(
    pack: Value,
    pack_num: Value,
    num: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let pack_string = pack.to_string();
    let pack_num_string = pack_num.to_string();
    let num_string = num.to_string();

    handle_flow(format!(
        "bank_retrieve({pack_string}, {pack_num_string}, {num_string});"
    ))
}

#[allow(dead_code)]
pub fn bank_swap(pack: Value, a: Value, b: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let pack_string = pack.to_string();
    let a_string = a.to_string();
    let b_string = b.to_string();

    handle_flow(format!("bank_swap({pack_string}, {a_string}, {b_string});"))
}

#[allow(dead_code)]
pub fn swap(a: Value, b: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let a_string = a.to_string();
    let b_string = b.to_string();

    handle_flow(format!("swap({a_string}, {b_string});"))
}

#[allow(dead_code)]
pub fn locate_item(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("locate_item({name_string});"))
}

#[allow(dead_code)]
pub fn quantity(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("quantity({name_string});"))
}

#[allow(dead_code)]
pub fn item_properties(item: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item_string = item.to_string();

    handle_flow(format!("item_properties({item_string});"))
}

#[allow(dead_code)]
pub fn item_grade(item: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item_string = item.to_string();

    handle_flow(format!("item_grade({item_string});"))
}

#[allow(dead_code)]
pub fn item_value(item: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item_string = item.to_string();

    handle_flow(format!("item_value({item_string});"))
}

#[allow(dead_code)]
pub fn leave() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("leave();"))
}

#[allow(dead_code)]
pub fn is_paused() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("is_paused();"))
}

#[allow(dead_code)]
pub fn pause() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("pause();"))
}

#[allow(dead_code)]
pub fn get_socket() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_socket();"))
}

#[allow(dead_code)]
pub fn get_map() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_map();"))
}

#[allow(dead_code)]
pub fn set_message(text: Value, color: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let text_string = text.to_string();
    let color_string = color.to_string();

    handle_flow(format!("set_message({text_string}, {color_string});"))
}

#[allow(dead_code)]
pub fn game_log(
    message: Value,
    color: Value,
    x: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let message_string = message.to_string();
    let color_string = color.to_string();
    let x_string = x.to_string();

    handle_flow(format!(
        "game_log({message_string}, {color_string}, {x_string});"
    ))
}

#[allow(dead_code)]
pub fn log(message: Value, color: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let message_string = message.to_string();
    let color_string = color.to_string();

    handle_flow(format!("log({message_string}, {color_string});"))
}

#[allow(dead_code)]
pub fn safe_log(message: Value, color: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let message_string = message.to_string();
    let color_string = color.to_string();

    handle_flow(format!("safe_log({message_string}, {color_string});"))
}

#[allow(dead_code)]
pub fn get_focus() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_focus();"))
}

#[allow(dead_code)]
pub fn get_target_of(entity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();

    handle_flow(format!("get_target_of({entity_string});"))
}

#[allow(dead_code)]
pub fn get_target() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_target();"))
}

#[allow(dead_code)]
pub fn get_targeted_monster() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_targeted_monster();"))
}

#[allow(dead_code)]
pub fn change_target(target: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("change_target({});", deref_entity(target)))
}

#[allow(dead_code)]
pub fn change_target_privately(target: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    // See change_target
    handle_flow(format!("change_target_privately({})", deref_entity(target)))
}

#[allow(dead_code)]
pub fn can_move_to(x: f64, y: f64) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("can_move_to({x}, {y});"))
}

#[allow(dead_code)]
pub fn xmove(x: f64, y: f64) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("xmove({x}, {y});"))
}

#[allow(dead_code)]
// NOTE: This version doesn't allow an empty skill name.
pub fn is_in_range(target: &Value, skill: &str) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!(
        "is_in_range({}, \"{skill}\");",
        deref_entity(target)
    ))
}

#[allow(dead_code)]
pub fn is_on_cooldown(skill: &str) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("is_on_cooldown(\"{skill}\");"))
}

#[allow(dead_code)]
pub fn can_attack(target: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("can_attack({})", deref_entity(target)))
}

#[allow(dead_code)]
pub fn can_heal(t: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let t_string = t.to_string();

    handle_flow(format!("can_heal({t_string});"))
}

#[allow(dead_code)]
pub fn is_moving(entity: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("is_moving({})", deref_entity(entity)))
}

#[allow(dead_code)]
pub fn is_transporting(entity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();

    handle_flow(format!("is_transporting({entity_string});"))
}

#[allow(dead_code)]
pub fn attack(target: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("attack({});", deref_entity(target)))
}

#[allow(dead_code)]
pub fn heal(target: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let target_string = target.to_string();

    handle_flow(format!("heal({target_string});"))
}

#[allow(dead_code)]
pub fn buy(name: Value, quantity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!("buy({name_string}, {quantity_string});"))
}

#[allow(dead_code)]
pub fn buy_with_gold(name: Value, quantity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!("buy_with_gold({name_string}, {quantity_string});"))
}

#[allow(dead_code)]
pub fn buy_with_shells(name: Value, quantity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "buy_with_shells({name_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn split(num: Value, quantity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!("split({num_string}, {quantity_string});"))
}

#[allow(dead_code)]
pub fn sell(num: Value, quantity: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!("sell({num_string}, {quantity_string});"))
}

#[allow(dead_code)]
pub fn consume(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("consume({num_string});"))
}

#[allow(dead_code)]
pub fn equip_batch(data: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let data_string = data.to_string();

    handle_flow(format!("equip_batch({data_string});"))
}

#[allow(dead_code)]
pub fn equip(num: Value, slot: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let slot_string = slot.to_string();

    let res = handle_flow(format!("equip({num_string}, {slot_string});"));

    // Give things a bit to settle
    std::thread::sleep(std::time::Duration::from_millis(100));

    res
}

#[allow(dead_code)]
pub fn unequip(slot: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let slot_string = slot.to_string();

    handle_flow(format!("unequip({slot_string});"))
}

#[allow(dead_code)]
pub fn lock_item(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("lock_item({num_string});"))
}

#[allow(dead_code)]
pub fn seal_item(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("seal_item({num_string});"))
}

#[allow(dead_code)]
pub fn unlock_item(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("unlock_item({num_string});"))
}

#[allow(dead_code)]
pub fn trade(
    num: Value,
    trade_slot: Value,
    price: Value,
    quantity: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();
    let trade_slot_string = trade_slot.to_string();
    let price_string = price.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "trade({num_string}, {trade_slot_string}, {price_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn trade_buy(
    target: Value,
    trade_slot: Value,
    quantity: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let target_string = target.to_string();
    let trade_slot_string = trade_slot.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "trade_buy({target_string}, {trade_slot_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn trade_sell(
    target: Value,
    trade_slot: Value,
    quantity: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let target_string = target.to_string();
    let trade_slot_string = trade_slot.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "trade_sell({target_string}, {trade_slot_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn wishlist(
    trade_slot: Value,
    name: Value,
    price: Value,
    level: Value,
    quantity: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let trade_slot_string = trade_slot.to_string();
    let name_string = name.to_string();
    let price_string = price.to_string();
    let level_string = level.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "wishlist({trade_slot_string}, {name_string}, {price_string}, {level_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn giveaway(
    slot: Value,
    num: Value,
    q: Value,
    minutes: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let slot_string = slot.to_string();
    let num_string = num.to_string();
    let q_string = q.to_string();
    let minutes_string = minutes.to_string();

    handle_flow(format!(
        "giveaway({slot_string}, {num_string}, {q_string}, {minutes_string});"
    ))
}

#[allow(dead_code)]
pub fn join_giveaway(
    name: Value,
    slot: Value,
    rid: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let slot_string = slot.to_string();
    let rid_string = rid.to_string();

    handle_flow(format!(
        "join_giveaway({name_string}, {slot_string}, {rid_string});"
    ))
}

#[allow(dead_code)]
pub fn upgrade(
    item_num: Value,
    scroll_num: Value,
    offering_num: Value,
    only_calculate: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item_num_string = item_num.to_string();
    let scroll_num_string = scroll_num.to_string();
    let offering_num_string = offering_num.to_string();
    let only_calculate_string = only_calculate.to_string();

    handle_flow(format!(
        "upgrade({item_num_string}, {scroll_num_string}, {offering_num_string}, {only_calculate_string});"
    ))
}

#[allow(dead_code)]
pub fn compound(
    item0: Value,
    item1: Value,
    item2: Value,
    scroll_num: Value,
    offering_num: Value,
    only_calculate: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item0_string = item0.to_string();
    let item1_string = item1.to_string();
    let item2_string = item2.to_string();
    let scroll_num_string = scroll_num.to_string();
    let offering_num_string = offering_num.to_string();
    let only_calculate_string = only_calculate.to_string();

    handle_flow(format!(
        "compound({item0_string}, {item1_string}, {item2_string}, {scroll_num_string}, {offering_num_string}, {only_calculate_string});"
    ))
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn craft(
    i0: Value,
    i1: Value,
    i2: Value,
    i3: Value,
    i4: Value,
    i5: Value,
    i6: Value,
    i7: Value,
    i8: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let i0_string = i0.to_string();
    let i1_string = i1.to_string();
    let i2_string = i2.to_string();
    let i3_string = i3.to_string();
    let i4_string = i4.to_string();
    let i5_string = i5.to_string();
    let i6_string = i6.to_string();
    let i7_string = i7.to_string();
    let i8_string = i8.to_string();

    handle_flow(format!(
        "craft({i0_string}, {i1_string}, {i2_string}, {i3_string}, {i4_string}, {i5_string}, {i6_string}, {i7_string}, {i8_string});"
    ))
}

#[allow(dead_code)]
pub fn auto_craft(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("auto_craft({name_string});"))
}

#[allow(dead_code)]
pub fn dismantle(item_num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let item_num_string = item_num.to_string();

    handle_flow(format!("dismantle({item_num_string});"))
}

#[allow(dead_code)]
pub fn exchange_buy(token: Value, name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let token_string = token.to_string();
    let name_string = name.to_string();

    handle_flow(format!("exchange_buy({token_string}, {name_string});"))
}

#[allow(dead_code)]
pub fn say(message: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let message_string = message.to_string();

    handle_flow(format!("say({message_string});"))
}

#[allow(dead_code)]
pub fn party_say(message: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let message_string = message.to_string();

    handle_flow(format!("party_say({message_string});"))
}

#[allow(dead_code)]
pub fn pm(name: Value, message: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let message_string = message.to_string();

    handle_flow(format!("pm({name_string}, {message_string});"))
}

#[allow(dead_code)]
pub fn almove(x: f64, y: f64) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("move({x}, {y});"))
}

#[allow(dead_code)]
pub fn cruise(speed: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let speed_string = speed.to_string();

    handle_flow(format!("cruise({speed_string});"))
}

#[allow(dead_code)]
pub fn equip_cx(slot: Value, cx_name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let slot_string = slot.to_string();
    let cx_name_string = cx_name.to_string();

    handle_flow(format!("equip_cx({slot_string}, {cx_name_string});"))
}

#[allow(dead_code)]
pub fn show_json(e: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let e_string = e.to_string();

    handle_flow(format!("show_json({e_string});"))
}

#[allow(dead_code)]
pub fn get_servers() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_servers();"))
}

#[allow(dead_code)]
pub fn get_characters() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_characters();"))
}

#[allow(dead_code)]
pub fn get_party() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_party();"))
}

#[allow(dead_code)]
pub fn get_monster(id: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();

    handle_flow(format!("get_monster({id_string});"))
}

#[allow(dead_code)]
pub fn get_player(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("get_player({name_string});"))
}

#[allow(dead_code)]
pub fn get_entity(id: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();

    handle_flow(format!("get_entity({id_string});"))
}

#[allow(dead_code)]
pub fn find_npc(npc_id: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let npc_id_string = npc_id.to_string();

    handle_flow(format!("find_npc({npc_id_string});"))
}

#[allow(dead_code)]
pub fn get_nearest_monster(args: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let args_string = args.to_string();

    handle_flow(format!("get_nearest_monster({args_string});"))
}

#[allow(dead_code)]
pub fn get_nearest_hostile(args: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let args_string = args.to_string();

    handle_flow(format!("get_nearest_hostile({args_string});"))
}

#[allow(dead_code)]
pub fn get_nearby_hostiles(args: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let args_string = args.to_string();

    handle_flow(format!("parent.get_nearby_hostiles({args_string});"))
}

#[allow(dead_code)]
pub fn get_nearest_npc() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_nearest_npc();"))
}

#[allow(dead_code)]
pub fn use_hp_or_mp() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("use_hp_or_mp();"))
}

#[allow(dead_code)]
pub fn loot(id_or_arg: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_or_arg_string = id_or_arg.to_string();

    handle_flow(format!("loot({id_or_arg_string});"))
}

#[allow(dead_code)]
pub fn get_chests() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_chests();"))
}

#[allow(dead_code)]
pub fn open_stand(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("open_stand({num_string});"))
}

#[allow(dead_code)]
pub fn close_stand() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("close_stand();"))
}

#[allow(dead_code)]
pub fn send_gold(receiver: Value, gold: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let receiver_string = receiver.to_string();
    let gold_string = gold.to_string();

    handle_flow(format!("send_gold({receiver_string}, {gold_string});"))
}

#[allow(dead_code)]
pub fn send_item(
    receiver: Value,
    num: Value,
    quantity: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let receiver_string = receiver.to_string();
    let num_string = num.to_string();
    let quantity_string = quantity.to_string();

    handle_flow(format!(
        "send_item({receiver_string}, {num_string}, {quantity_string});"
    ))
}

#[allow(dead_code)]
pub fn send_cx(receiver: Value, cx: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let receiver_string = receiver.to_string();
    let cx_string = cx.to_string();

    handle_flow(format!("send_cx({receiver_string}, {cx_string});"))
}

#[allow(dead_code)]
pub fn send_mail(
    to: Value,
    subject: Value,
    message: Value,
    item: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let to_string = to.to_string();
    let subject_string = subject.to_string();
    let message_string = message.to_string();
    let item_string = item.to_string();

    handle_flow(format!(
        "send_mail({to_string}, {subject_string}, {message_string}, {item_string});"
    ))
}

#[allow(dead_code)]
pub fn destroy(num: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let num_string = num.to_string();

    handle_flow(format!("destroy({num_string});"))
}

#[allow(dead_code)]
pub fn send_party_invite(
    name: &String,
    is_request: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let is_request_string = is_request.to_string();

    handle_flow(format!(
        "send_party_invite(\"{name}\", {is_request_string});"
    ))
}

#[allow(dead_code)]
pub fn send_party_request(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("send_party_request({name_string});"))
}

#[allow(dead_code)]
pub fn accept_party_invite(name: &String) -> Result<Value, Box<dyn std::error::Error>> {
    handle_flow(format!("accept_party_invite(\"{name}\");"))
}

#[allow(dead_code)]
pub fn accept_party_request(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("accept_party_request({name_string});"))
}

#[allow(dead_code)]
pub fn leave_party() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("leave_party();"))
}

#[allow(dead_code)]
pub fn kick_party_member(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("kick_party_member({name_string});"))
}

#[allow(dead_code)]
pub fn accept_magiport(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("accept_magiport({name_string});"))
}

#[allow(dead_code)]
pub fn unfriend(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("unfriend({name_string});"))
}

#[allow(dead_code)]
pub fn respawn() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("respawn();"))
}

#[allow(dead_code)]
pub fn set_home() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("set_home();"))
}

#[allow(dead_code)]
pub fn handle_command(command: Value, args: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let command_string = command.to_string();
    let args_string = args.to_string();

    handle_flow(format!("handle_command({command_string}, {args_string});"))
}

#[allow(dead_code)]
pub fn send_server_cm(to: Value, message: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let to_string = to.to_string();
    let message_string = message.to_string();

    handle_flow(format!("send_server_cm({to_string}, {message_string});"))
}

#[allow(dead_code)]
pub fn on_disappear(entity: Value, data: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let entity_string = entity.to_string();
    let data_string = data.to_string();

    handle_flow(format!("on_disappear({entity_string}, {data_string});"))
}

#[allow(dead_code)]
pub fn on_party_invite(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("on_party_invite({name_string});"))
}

#[allow(dead_code)]
pub fn on_party_request(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("on_party_request({name_string});"))
}

#[allow(dead_code)]
pub fn on_magiport(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("on_magiport({name_string});"))
}

#[allow(dead_code)]
pub fn on_map_click(x: Value, y: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let x_string = x.to_string();
    let y_string = y.to_string();

    handle_flow(format!("on_map_click({x_string}, {y_string});"))
}

#[allow(dead_code)]
pub fn on_destroy() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("on_destroy();"))
}

#[allow(dead_code)]
pub fn on_draw() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("on_draw();"))
}

#[allow(dead_code)]
pub fn draw_line(
    x: Value,
    y: Value,
    x2: Value,
    y2: Value,
    size: Value,
    color: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let x_string = x.to_string();
    let y_string = y.to_string();
    let x2_string = x2.to_string();
    let y2_string = y2.to_string();
    let size_string = size.to_string();
    let color_string = color.to_string();

    handle_flow(format!(
        "draw_line({x_string}, {y_string}, {x2_string}, {y2_string}, {size_string}, {color_string});"
    ))
}

#[allow(dead_code)]
pub fn draw_circle(
    x: Value,
    y: Value,
    radius: Value,
    size: Value,
    color: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let x_string = x.to_string();
    let y_string = y.to_string();
    let radius_string = radius.to_string();
    let size_string = size.to_string();
    let color_string = color.to_string();

    handle_flow(format!(
        "draw_circle({x_string}, {y_string}, {radius_string}, {size_string}, {color_string});"
    ))
}

#[allow(dead_code)]
pub fn clear_drawings() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("clear_drawings();"))
}

#[allow(dead_code)]
pub fn add_top_button(
    id: Value,
    value: Value,
    func: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();
    let value_string = value.to_string();
    let func_string = func.to_string();

    handle_flow(format!(
        "add_top_button({id_string}, {value_string}, {func_string});"
    ))
}

#[allow(dead_code)]
pub fn add_bottom_button(
    id: Value,
    value: Value,
    func: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();
    let value_string = value.to_string();
    let func_string = func.to_string();

    handle_flow(format!(
        "add_bottom_button({id_string}, {value_string}, {func_string});"
    ))
}

#[allow(dead_code)]
pub fn set_button_value(id: Value, value: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();
    let value_string = value.to_string();

    handle_flow(format!("set_button_value({id_string}, {value_string});"))
}

#[allow(dead_code)]
pub fn set_button_color(id: Value, color: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();
    let color_string = color.to_string();

    handle_flow(format!("set_button_color({id_string}, {color_string});"))
}

#[allow(dead_code)]
pub fn set_button_onclick(id: Value, func: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let id_string = id.to_string();
    let func_string = func.to_string();

    handle_flow(format!("set_button_onclick({id_string}, {func_string});"))
}

#[allow(dead_code)]
pub fn clear_buttons() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("clear_buttons();"))
}

#[allow(dead_code)]
pub fn trigger_character_event(
    name: Value,
    data: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let data_string = data.to_string();

    handle_flow(format!(
        "trigger_character_event({name_string}, {data_string});"
    ))
}

#[allow(dead_code)]
pub fn trigger_event(name: Value, data: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let data_string = data.to_string();

    handle_flow(format!("trigger_event({name_string}, {data_string});"))
}

#[allow(dead_code)]
pub fn preview_item(def: Value, args: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let def_string = def.to_string();
    let args_string = args.to_string();

    handle_flow(format!("preview_item({def_string}, {args_string});"))
}

#[allow(dead_code)]
pub fn set_skillbar() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("set_skillbar();"))
}

#[allow(dead_code)]
pub fn set_keymap(keymap: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let keymap_string = keymap.to_string();

    handle_flow(format!("set_keymap({keymap_string});"))
}

#[allow(dead_code)]
pub fn map_key(key: Value, skill: Value, code: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let key_string = key.to_string();
    let skill_string = skill.to_string();
    let code_string = code.to_string();

    handle_flow(format!(
        "map_key({key_string}, {skill_string}, {code_string});"
    ))
}

#[allow(dead_code)]
pub fn unmap_key(key: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let key_string = key.to_string();

    handle_flow(format!("unmap_key({key_string});"))
}

#[allow(dead_code)]
pub fn reset_mappings() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("reset_mappings();"))
}

#[allow(dead_code)]
pub fn local_cm_logic() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("local_cm_logic();"))
}

#[allow(dead_code)]
pub fn send_local_cm(name: Value, data: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let data_string = data.to_string();

    handle_flow(format!("send_local_cm({name_string}, {data_string});"))
}

#[allow(dead_code)]
pub fn is_character_local(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("is_character_local({name_string});"))
}

#[allow(dead_code)]
pub fn pset(name: Value, value: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let value_string = value.to_string();

    handle_flow(format!("pset({name_string}, {value_string});"))
}

#[allow(dead_code)]
pub fn pget(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("pget({name_string});"))
}

#[allow(dead_code)]
pub fn set(name: Value, value: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();
    let value_string = value.to_string();

    handle_flow(format!("set({name_string}, {value_string});"))
}

#[allow(dead_code)]
pub fn get(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("get({name_string});"))
}

#[allow(dead_code)]
pub fn load_code(name_or_slot: Value, onerror: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_or_slot_string = name_or_slot.to_string();
    let onerror_string = onerror.to_string();

    handle_flow(format!(
        "load_code({name_or_slot_string}, {onerror_string});"
    ))
}

#[allow(dead_code)]
pub fn require_code(name_or_slot: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_or_slot_string = name_or_slot.to_string();

    handle_flow(format!("require_code({name_or_slot_string});"))
}

#[allow(dead_code)]
pub fn upload_code(
    slot_number: Value,
    slot_name: Value,
    code_string: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let slot_number_string = slot_number.to_string();
    let slot_name_string = slot_name.to_string();
    let code_string_string = code_string.to_string();

    handle_flow(format!(
        "upload_code({slot_number_string}, {slot_name_string}, {code_string_string});"
    ))
}

#[allow(dead_code)]
pub fn get_active_code_slot() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_active_code_slot();"))
}

#[allow(dead_code)]
pub fn get_edited_code_slot() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("get_edited_code_slot();"))
}

#[allow(dead_code)]
pub fn disconnect() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("disconnect();"))
}

#[allow(dead_code)]
pub fn smart_move(
    destination: &Value,
    on_done: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    // If we just send like `parent.entities["$Ernis"]` we don't want to process that further
    let destination_string = if destination.is_string() {
        destination.as_str().unwrap().to_owned()
    } else {
        destination.to_string()
    };

    let on_done_string = on_done.to_string();

    handle_flow(format!(
        "smart_move({destination_string}, {on_done_string});"
    ))
}

#[allow(dead_code)]
pub fn stop(action: Value, second: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let action_string = action.to_string();
    let second_string = second.to_string();

    handle_flow(format!("stop({action_string}, {second_string});"))
}

#[allow(dead_code)]
pub fn plot(index: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let index_string = index.to_string();

    handle_flow(format!("plot({index_string});"))
}

#[allow(dead_code)]
pub fn qpush(node: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let node_string = node.to_string();

    handle_flow(format!("qpush({node_string});"))
}

#[allow(dead_code)]
pub fn smooth_path() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("smooth_path();"))
}

#[allow(dead_code)]
pub fn bfs() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("bfs();"))
}

#[allow(dead_code)]
pub fn start_pathfinding() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("start_pathfinding();"))
}

#[allow(dead_code)]
pub fn cli_smart_move_result(data: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let data_string = data.to_string();

    handle_flow(format!("cli_smart_move_result({data_string});"))
}

#[allow(dead_code)]
pub fn continue_pathfinding() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("continue_pathfinding();"))
}

#[allow(dead_code)]
pub fn smart_move_logic() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("smart_move_logic();"))
}

#[allow(dead_code)]
pub fn proxy(name: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let name_string = name.to_string();

    handle_flow(format!("proxy({name_string});"))
}

#[allow(dead_code)]
pub fn eval_s(code: Value) -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    let code_string = code.to_string();

    handle_flow(format!("eval_s({code_string});"))
}

#[allow(dead_code)]
pub fn performance_trick() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("performance_trick();"))
}

#[allow(dead_code)]
pub fn code_draw() -> Result<Value, Box<dyn std::error::Error>> {
    #[allow(clippy::useless_format)]
    handle_flow(format!("code_draw();"))
}

#[allow(dead_code)]
pub fn simple_distance(a: &Value, b: &Value) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(my_as_f64(&handle_flow(format!(
        "simple_distance({}, {});",
        deref_entity(a),
        deref_entity(b)
    ))?))
}

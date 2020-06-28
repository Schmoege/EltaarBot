use rand::*;

pub fn roll(dice_sides: &u8) -> u8 {
    let roll = rand::thread_rng().gen_range(1, dice_sides);
    return roll;
}

fn main() {
    println!("Hello, world!");
}


/**
 * @notice Request randomness from a user-provided seed
 * @param _seed: seed provided by the NearLott lottery
 */
pub fn get_random_number() -> u32 {
    // generate 15 number position with random position from [1..9]
    let random: Vec<u8> = random_position();
    let rand_array_str = format!("{:?}", &random);

    // convert so string
    let randomness_instr = random
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
    // convert to u64 to prepare for final number
    let randomness = randomness_instr
        .parse::<u128>()
        .expect("Error");

    // determine final number
    let win_number = (1000000 + (randomness % 1000000)) as u32;

    // return
    win_number
}

pub fn random_position() -> Vec<u8> {
    let positions = random_seed();
    let slice: Vec<u8> = positions[0..10].iter().map(|x| x.clone()).collect();
    return slice;
}
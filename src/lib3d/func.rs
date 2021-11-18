/// Original: `RegleTrois32`
pub fn cross_mult_32(val1: u32, val2: u32, nbstep: u32, step: u32) -> u32 {
    let res = (val2 as i32 - val1 as i32) * (step as i32) / (nbstep as i32) + val1 as i32;
    res as u32
}

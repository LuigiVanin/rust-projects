use rand::Rng;

pub fn generate_rnd(start: u8, end: u8) -> u8 {
    return rand::thread_rng().gen_range(start..end);
}

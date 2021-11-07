pub fn rule_30(pattern: u8) -> u8 {
    /* Rule 30 is an elementary cellular automaton introduced by Stephen Wolfram in 1983
     * https://en.wikipedia.org/wiki/Rule_30
     */
    match pattern {
        0b111 => 0b101,
        0b110 => 0b100,
        0b101 => 0b101,
        0b100 => 0b110,
        0b011 => 0b011,
        0b010 => 0b010,
        0b001 => 0b011,
        0b000 => 0b000,
        _ => 0b000,
    }
}

pub fn rule_30_u8(pattern: u8) -> u8 {
    /* Rule 30 is an elementary cellular automaton introduced by Stephen Wolfram in 1983
     * https://en.wikipedia.org/wiki/Rule_30
     */
    match pattern {
        0b111 => 0,
        0b110 => 0,
        0b101 => 0,
        0b100 => 1,
        0b011 => 1,
        0b010 => 1,
        0b001 => 1,
        0b000 => 0,
        _ => 0,
    }
}

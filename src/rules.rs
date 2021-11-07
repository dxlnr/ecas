pub enum Rule {
    R30,
    R90,
    R110,
}

pub fn apply_rule(rule: &Rule, pattern: u8) -> u8 {
    match rule {
        Rule::R30 => rule_30_u8(pattern),
        Rule::R90 => rule_90_u8(pattern),
        Rule::R110 => rule_110_u8(pattern),
    }
}

fn rule_30_u8(pattern: u8) -> u8 {
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

fn rule_90_u8(pattern: u8) -> u8 {
    /* Rule 90 is an elementary cellular automaton introduced by Stephen Wolfram in 1983
     * https://en.wikipedia.org/wiki/Rule_90
     */
    match pattern {
        0b111 => 0,
        0b110 => 1,
        0b101 => 0,
        0b100 => 1,
        0b011 => 1,
        0b010 => 0,
        0b001 => 1,
        0b000 => 0,
        _ => 0,
    }
}

fn rule_110_u8(pattern: u8) -> u8 {
    /* Rule 90 is an elementary cellular automaton introduced by Stephen Wolfram in 1983
     * https://en.wikipedia.org/wiki/Rule_90
     */
    match pattern {
        0b111 => 0,
        0b110 => 1,
        0b101 => 1,
        0b100 => 0,
        0b011 => 1,
        0b010 => 1,
        0b001 => 1,
        0b000 => 0,
        _ => 0,
    }
}

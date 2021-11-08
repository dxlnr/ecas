pub enum Rule {
    R30,
    R45,
    R60,
    R73,
    R86,
    R90,
    R99,
    R110,
    R158,
    R164,
    R184,
    R188,
}

impl Rule {
    pub fn apply_rule(rule: &Rule, pattern: u8) -> u8 {
        match rule {
            Rule::R30 => rule_30_u8(pattern),
            Rule::R45 => rule_45_u8(pattern),
            Rule::R60 => rule_60_u8(pattern),
            Rule::R73 => rule_73_u8(pattern),
            Rule::R86 => rule_86_u8(pattern),
            Rule::R90 => rule_90_u8(pattern),
            Rule::R99 => rule_99_u8(pattern),
            Rule::R110 => rule_110_u8(pattern),
            Rule::R158 => rule_158_u8(pattern),
            Rule::R164 => rule_164_u8(pattern),
            Rule::R184 => rule_184_u8(pattern),
            Rule::R188 => rule_188_u8(pattern),
        }
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

fn rule_45_u8(pattern: u8) -> u8 {
    /* Rule 45 is an elementary cellular automaton. */
    match pattern {
        0b111 => 0,
        0b110 => 0,
        0b101 => 1,
        0b100 => 0,
        0b011 => 1,
        0b010 => 1,
        0b001 => 0,
        0b000 => 1,
        _ => 0,
    }
}

fn rule_60_u8(pattern: u8) -> u8 {
    /* Rule 60 is an elementary cellular automaton.
     * https://de.wikipedia.org/wiki/Sierpinski-Dreieck
     */
    match pattern {
        0b111 => 0,
        0b110 => 0,
        0b101 => 1,
        0b100 => 1,
        0b011 => 1,
        0b010 => 1,
        0b001 => 0,
        0b000 => 0,
        _ => 0,
    }
}

fn rule_73_u8(pattern: u8) -> u8 {
    /* Rule 73 is an elementary cellular automaton. */
    match pattern {
        0b111 => 0,
        0b110 => 1,
        0b101 => 0,
        0b100 => 0,
        0b011 => 1,
        0b010 => 0,
        0b001 => 0,
        0b000 => 1,
        _ => 0,
    }
}

fn rule_86_u8(pattern: u8) -> u8 {
    /* Rule 86 is an elementary cellular automaton. */
    match pattern {
        0b111 => 0,
        0b110 => 1,
        0b101 => 0,
        0b100 => 1,
        0b011 => 0,
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

fn rule_99_u8(pattern: u8) -> u8 {
    /* Rule 99 is an elementary cellular automaton. */
    match pattern {
        0b111 => 0,
        0b110 => 1,
        0b101 => 1,
        0b100 => 0,
        0b011 => 0,
        0b010 => 0,
        0b001 => 1,
        0b000 => 1,
        _ => 0,
    }
}

fn rule_110_u8(pattern: u8) -> u8 {
    /* Rule 90 is an elementary cellular automaton introduced by Stephen Wolfram in 1983
     * https://en.wikipedia.org/wiki/Rule_110
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

fn rule_158_u8(pattern: u8) -> u8 {
    /* Rule 158 is an elementary cellular automaton. */
    match pattern {
        0b111 => 1,
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

fn rule_164_u8(pattern: u8) -> u8 {
    /* Rule 164 is an elementary cellular automaton. */
    match pattern {
        0b111 => 1,
        0b110 => 0,
        0b101 => 1,
        0b100 => 0,
        0b011 => 0,
        0b010 => 1,
        0b001 => 0,
        0b000 => 0,
        _ => 0,
    }
}

fn rule_184_u8(pattern: u8) -> u8 {
    /* Rule 184 is an elementary cellular automaton. */
    match pattern {
        0b111 => 1,
        0b110 => 0,
        0b101 => 1,
        0b100 => 1,
        0b011 => 1,
        0b010 => 0,
        0b001 => 0,
        0b000 => 0,
        _ => 0,
    }
}

fn rule_188_u8(pattern: u8) -> u8 {
    /* Rule 188 is an elementary cellular automaton. */
    match pattern {
        0b111 => 1,
        0b110 => 0,
        0b101 => 1,
        0b100 => 1,
        0b011 => 1,
        0b010 => 1,
        0b001 => 0,
        0b000 => 0,
        _ => 0,
    }
}

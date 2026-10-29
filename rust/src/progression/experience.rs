use crate::characters::components::progression::{Experience, Level};

const BASE_XP: u64 = 100;
const LINEAR_GROWTH: u64 = 50;
const QUADRATIC_GROWTH: u64 = 50;

pub fn experience_to_next_level(level: u32) -> u32 {
    let n = u64::from(level.max(1) - 1);

    let required = BASE_XP
        .saturating_add(LINEAR_GROWTH.saturating_mul(n))
        .saturating_add(QUADRATIC_GROWTH.saturating_mul(n).saturating_mul(n));

    u32::try_from(required).unwrap_or(u32::MAX)
}

pub fn process_level_up(level: &mut Level, experience: &mut Experience) -> u32 {
    let mut levels_gained = 0;

    while level.0 < u32::MAX {
        let required = experience_to_next_level(level.0);

        if experience.0 < required {
            break;
        }

        experience.0 -= required;
        level.0 += 1;
        levels_gained += 1;
    }

    levels_gained
}

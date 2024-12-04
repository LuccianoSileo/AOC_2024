use crate::parser::Instruction;

pub fn calculate_sum(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .filter_map(|instr| match instr {
            Instruction::Mul { x, y, enabled } if *enabled => Some(x * y),
            _ => None,
        })
        .sum()
}

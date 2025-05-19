#![cfg_attr(not(test), no_std)]
use pinocchio::{
    entrypoint::InstructionContext, lazy_program_entrypoint, log, no_allocator,
    nostd_panic_handler, ProgramResult,
};

lazy_program_entrypoint!(process_intruction);
no_allocator!();
nostd_panic_handler!();

fn process_intruction(context: InstructionContext) -> ProgramResult {
    unsafe {
        log::sol_log(core::str::from_utf8_unchecked(context.instruction_data()?));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{instruction::Instruction, pubkey};
    #[test]
    fn test_log() {
        let program_id = pubkey!("22222222222222222222222222222222222222222222");
        let mollusk = Mollusk::new(&program_id, "target/deploy/pinocchio_memo");
        mollusk.process_instruction(
            &Instruction::new_with_bytes(program_id, b"Hello, Scale or Die!", vec![]),
            &[],
        );
    }
}

use gba;

fn get_instruction_regs_from_log(instr_idx: usize, log_bytes: &[u8]) -> [u32; 18] {
    let mut regs = [0u32; 18];
    let raw_bytes = &log_bytes[instr_idx * 4 * 18 .. (instr_idx + 1) * 4 * 18];

    for i in 0 .. 18 {
        regs[i] = u32::from_le_bytes( raw_bytes[i*4 .. (i+1)*4].try_into().unwrap() )
    }

    regs
}

#[test]
fn arm_log() {
    let mut gba = gba::GbaCore::default();
    let bytes = include_bytes!("roms/arm.gba");

    let log_bytes = include_bytes!("arm-log.bin");

    gba.load_rom(bytes);
    gba.skip_bios();

    let mut i = 0;
    loop {

        log::debug!("Tick {i}");
        assert_eq!(gba.regs(), get_instruction_regs_from_log(i, log_bytes));
        gba.tick();

        i += 1;
    }
}

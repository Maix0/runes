{
    let mut cycles = 0;
    if cpu.flags.contains(Flags::C) {
        cycles += 1;
        cpu.addr = cpu.registers.pc + cpu.rel;

        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
            cycles += 1;
        }
        cpu.registers.pc = cpu.addr;
    }
    cycles
}

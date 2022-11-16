{
    cpu.registers.x = cpu.registers.x.wrapping_sub(1);
    cpu.flags.set(Flags::Z, cpu.registers.x == 0);
    cpu.flags.set(Flags::N, cpu.registers.x & 0x80 > 0);
    0
}

{
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    cpu.flags.set(Flags::Z, cpu.registers.y == 0);
    cpu.flags.set(Flags::N, cpu.registers.y & 0x80 > 0);
    0
}

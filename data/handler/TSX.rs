{
    cpu.registers.x = cpu.registers.stack_pointer;
    cpu.flags.set(Flags::Z, cpu.registers.x == 0);
    cpu.flags.set(Flags::N, (cpu.registers.x & 0x80) == 0x80);
    0
}

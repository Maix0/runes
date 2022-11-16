{
    cpu.registers.a ^= data;
    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
    1
}

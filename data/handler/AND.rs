{
    cpu.registers.a &= data;
    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
    1
}

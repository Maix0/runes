{
    cpu.registers.y = data;
    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
    cpu.flags.set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
    1
}

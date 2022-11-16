{
    let tmp = cpu.registers.a & data;
    cpu.flags.set(Flags::Z, tmp == 0);
    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
    cpu.flags.set(Flags::V, (data & 0b10000000) == 0b10000000);
    0
}

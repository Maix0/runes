{
    let sub = cpu.registers.x.wrapping_sub(data);

    cpu.flags.set(Flags::C, cpu.registers.x >= data);
    cpu.flags.set(Flags::Z, sub == 0);
    cpu.flags.set(Flags::N, (sub & 0x80) ==  0x80);
    0
}

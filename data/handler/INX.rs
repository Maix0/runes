{
    let tmp = cpu.registers.x.wrapping_add(1);
    cpu.registers.x = tmp;
    cpu.flags.set(Flags::Z, tmp == 0);
    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
    0
}

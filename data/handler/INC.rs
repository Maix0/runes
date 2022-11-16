{
    let tmp = data.wrapping_add(1);
    bus.write(tmp, cpu.addr);
    cpu.flags.set(Flags::Z, tmp == 0);
    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
    0
}

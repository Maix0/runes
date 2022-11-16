{
    let new_data = data.wrapping_sub(1);
    bus.write(new_data, cpu.addr);
    cpu.flags.set(Flags::Z, new_data == 0);
    cpu.flags.set(Flags::N, new_data & 0x80 > 0);
    0
}

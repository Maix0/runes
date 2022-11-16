{
    let result = ((data as u16) << 1) | if cpu.flags.contains(Flags::C) {1} else {0};
    cpu.flags.set(Flags::Z, result == 0);
    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
    let result_u8 = (result & 0xFF).try_into().unwrap();

    if self.mode() == Mode::Implied {
        cpu.registers.a =  result_u8;
    } else {
        bus.write(result_u8, cpu.addr);
    }
    0
}

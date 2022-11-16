{
    bus.write((cpu.flags | Flags::B | Flags::U).bits(), 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;
    cpu.flags &= !(Flags::B | Flags::U);
    0
}

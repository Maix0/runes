{
    cpu.registers.stack_pointer += 1;
    cpu.registers.a = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
    0
}

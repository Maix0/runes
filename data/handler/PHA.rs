{
    bus.write(cpu.registers.a, 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;
    0
}

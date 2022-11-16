{
    cpu.registers.stack_pointer += 1;
    cpu.flags = Flags::from_bits_truncate(bus.read(0x100 + (cpu.registers.stack_pointer as u16))) | Flags::U;
    
    0
}

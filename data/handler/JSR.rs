{
    cpu.registers.pc -= 1;
    let [high, low] = cpu.registers.pc.to_le_bytes();

    bus.write(high, 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;

    bus.write(low, 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;

    cpu.registers.pc = cpu.addr;

    1
}

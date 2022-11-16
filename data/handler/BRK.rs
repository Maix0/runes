{
    cpu.registers.pc += 1;
    cpu.flags.insert(Flags::I);

    let [high, low] = cpu.registers.pc.to_le_bytes();
    bus.write(high, 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;
    bus.write(low, 0x100 + (cpu.registers.stack_pointer as u16));

    cpu.registers.stack_pointer -= 1;
    cpu.flags.insert(Flags::B);

    bus.write(cpu.flags.bits(), 0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer -= 1;
    cpu.flags.remove(Flags::B);

    cpu.registers.pc = u16::from_be_bytes([bus.read(0xFFFF), bus.read(0xFFFE)]);
    0
}

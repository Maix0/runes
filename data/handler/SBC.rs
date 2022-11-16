{ 
    let (result, carry) = crate::utils::borrowing_sub(cpu.registers.a, data, cpu.flags.contains(Flags::CARRY));
    cpu.flags.set(Flags::C, carry);
    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
    cpu.flags.set(Flags::V, carry);
    cpu.flags.set(Flags::Z, result == 0);
    cpu.registers.a = result;
    1
}

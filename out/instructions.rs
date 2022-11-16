#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Instructions {
    ADCIndirectY = 113u8,
    ADCIndirectX = 97u8,
    ADCAbsoluteY = 121u8,
    ADCAbsoluteX = 125u8,
    ADCAbsolute = 109u8,
    ADCZeroPageX = 117u8,
    ADCZeroPage = 101u8,
    ADCImmediate = 105u8,
    ANDIndirectY = 49u8,
    ANDIndirectX = 33u8,
    ANDAbsoluteY = 57u8,
    ANDAbsoluteX = 61u8,
    ANDAbsolute = 45u8,
    ANDZeroPageX = 53u8,
    ANDZeroPage = 37u8,
    ANDImmediate = 41u8,
    ASLAbsoluteX = 30u8,
    ASLAbsolute = 14u8,
    ASLAccumulator = 10u8,
    ASLZeroPageX = 22u8,
    ASLZeroPage = 6u8,
    BCCRelative = 144u8,
    BCSRelative = 176u8,
    BEQRelative = 240u8,
    BITAbsolute = 44u8,
    BITZeroPage = 36u8,
    BMIRelative = 48u8,
    BNERelative = 208u8,
    BPLRelative = 16u8,
    BRK = 0u8,
    BVCRelative = 80u8,
    BVSRelative = 112u8,
    CLC = 24u8,
    CLD = 216u8,
    CLI = 88u8,
    CLV = 184u8,
    CMPIndirectY = 209u8,
    CMPIndirectX = 193u8,
    CMPAbsoluteY = 217u8,
    CMPAbsoluteX = 221u8,
    CMPAbsolute = 205u8,
    CMPZeroPageX = 213u8,
    CMPZeroPage = 197u8,
    CMPImmediate = 201u8,
    CPXAbsolute = 236u8,
    CPXZeroPage = 228u8,
    CPXImmediate = 224u8,
    CPYAbsolute = 204u8,
    CPYZeroPage = 196u8,
    CPYImmediate = 192u8,
    DECAbsoluteX = 222u8,
    DECAbsolute = 206u8,
    DECZeroPageX = 214u8,
    DECZeroPage = 198u8,
    DEX = 202u8,
    DEY = 136u8,
    EORIndirectY = 81u8,
    EORIndirectX = 65u8,
    EORAbsoluteY = 89u8,
    EORAbsoluteX = 93u8,
    EORAbsolute = 77u8,
    EORZeroPageX = 85u8,
    EORZeroPage = 69u8,
    EORImmediate = 73u8,
    INCAbsoluteX = 254u8,
    INCAbsolute = 238u8,
    INCZeroPageX = 246u8,
    INCZeroPage = 230u8,
    INX = 232u8,
    INY = 200u8,
    JMPIndirect = 108u8,
    JMPAbsolute = 76u8,
    JSR = 32u8,
    LDAIndirectY = 177u8,
    LDAIndirectX = 161u8,
    LDAAbsoluteY = 185u8,
    LDAAbsoluteX = 189u8,
    LDAAbsolute = 173u8,
    LDAZeroPageX = 181u8,
    LDAZeroPage = 165u8,
    LDAImmediate = 169u8,
    LDXAbsoluteY = 190u8,
    LDXAbsolute = 174u8,
    LDXZeroPageY = 182u8,
    LDXZeroPage = 166u8,
    LDXImmediate = 162u8,
    LDYAbsoluteX = 188u8,
    LDYAbsolute = 172u8,
    LDYZeroPageX = 180u8,
    LDYZeroPage = 164u8,
    LDYImmediate = 160u8,
    LSRAbsoluteX = 94u8,
    LSRAbsolute = 78u8,
    LSRAccumulator = 74u8,
    LSRZeroPageX = 86u8,
    LSRZeroPage = 70u8,
    NOP = 234u8,
    ORAIndirectY = 17u8,
    ORAIndirectX = 1u8,
    ORAAbsoluteY = 25u8,
    ORAAbsoluteX = 29u8,
    ORAAbsolute = 13u8,
    ORAZeroPageX = 21u8,
    ORAZeroPage = 5u8,
    ORAImmediate = 9u8,
    PHA = 72u8,
    PHP = 8u8,
    PLA = 104u8,
    PLP = 40u8,
    ROLAbsoluteX = 62u8,
    ROLAbsolute = 46u8,
    ROLAccumulator = 42u8,
    ROLZeroPageX = 54u8,
    ROLZeroPage = 38u8,
    RORAbsoluteX = 126u8,
    RORAbsolute = 110u8,
    RORAccumulator = 106u8,
    RORZeroPageX = 118u8,
    RORZeroPage = 102u8,
    RTI = 64u8,
    RTS = 96u8,
    SBCIndirectY = 241u8,
    SBCIndirectX = 225u8,
    SBCAbsoluteY = 249u8,
    SBCAbsoluteX = 253u8,
    SBCAbsolute = 237u8,
    SBCZeroPageX = 245u8,
    SBCZeroPage = 229u8,
    SBCImmediate = 233u8,
    SEC = 56u8,
    SED = 248u8,
    SEI = 120u8,
    STAIndirectY = 145u8,
    STAIndirectX = 129u8,
    STAAbsoluteY = 153u8,
    STAAbsoluteX = 157u8,
    STAAbsolute = 141u8,
    STAZeroPageX = 149u8,
    STAZeroPage = 133u8,
    STXAbsolute = 142u8,
    STXZeroPageY = 150u8,
    STXZeroPage = 134u8,
    STYAbsolute = 140u8,
    STYZeroPageX = 148u8,
    STYZeroPage = 132u8,
    TAX = 170u8,
    TAY = 168u8,
    TSX = 186u8,
    TXA = 138u8,
    TXS = 154u8,
    TYA = 152u8,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Cycles {
    Normal(u8),
    Page(u8),
    Branch(u8),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Mode {
    Immediate,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}
impl Instructions {
    pub fn cycles(&self) -> Cycles {
        match *self {
            Self::ADCIndirectY => Cycles::Page(5u8),
            Self::ADCIndirectX => Cycles::Normal(6u8),
            Self::ADCAbsoluteY => Cycles::Page(4u8),
            Self::ADCAbsoluteX => Cycles::Page(4u8),
            Self::ADCAbsolute => Cycles::Normal(4u8),
            Self::ADCZeroPageX => Cycles::Normal(4u8),
            Self::ADCZeroPage => Cycles::Normal(3u8),
            Self::ADCImmediate => Cycles::Normal(2u8),
            Self::ANDIndirectY => Cycles::Page(5u8),
            Self::ANDIndirectX => Cycles::Normal(6u8),
            Self::ANDAbsoluteY => Cycles::Page(4u8),
            Self::ANDAbsoluteX => Cycles::Page(4u8),
            Self::ANDAbsolute => Cycles::Normal(4u8),
            Self::ANDZeroPageX => Cycles::Normal(4u8),
            Self::ANDZeroPage => Cycles::Normal(3u8),
            Self::ANDImmediate => Cycles::Normal(2u8),
            Self::ASLAbsoluteX => Cycles::Normal(7u8),
            Self::ASLAbsolute => Cycles::Normal(6u8),
            Self::ASLAccumulator => Cycles::Normal(2u8),
            Self::ASLZeroPageX => Cycles::Normal(6u8),
            Self::ASLZeroPage => Cycles::Normal(5u8),
            Self::BCCRelative => Cycles::Branch(2u8),
            Self::BCSRelative => Cycles::Branch(2u8),
            Self::BEQRelative => Cycles::Branch(2u8),
            Self::BITAbsolute => Cycles::Normal(4u8),
            Self::BITZeroPage => Cycles::Normal(3u8),
            Self::BMIRelative => Cycles::Branch(2u8),
            Self::BNERelative => Cycles::Branch(2u8),
            Self::BPLRelative => Cycles::Branch(2u8),
            Self::BRK => Cycles::Normal(7u8),
            Self::BVCRelative => Cycles::Branch(2u8),
            Self::BVSRelative => Cycles::Branch(2u8),
            Self::CLC => Cycles::Normal(2u8),
            Self::CLD => Cycles::Normal(2u8),
            Self::CLI => Cycles::Normal(2u8),
            Self::CLV => Cycles::Normal(2u8),
            Self::CMPIndirectY => Cycles::Page(5u8),
            Self::CMPIndirectX => Cycles::Normal(6u8),
            Self::CMPAbsoluteY => Cycles::Page(4u8),
            Self::CMPAbsoluteX => Cycles::Page(4u8),
            Self::CMPAbsolute => Cycles::Normal(4u8),
            Self::CMPZeroPageX => Cycles::Normal(4u8),
            Self::CMPZeroPage => Cycles::Normal(3u8),
            Self::CMPImmediate => Cycles::Normal(2u8),
            Self::CPXAbsolute => Cycles::Normal(4u8),
            Self::CPXZeroPage => Cycles::Normal(3u8),
            Self::CPXImmediate => Cycles::Normal(2u8),
            Self::CPYAbsolute => Cycles::Normal(4u8),
            Self::CPYZeroPage => Cycles::Normal(3u8),
            Self::CPYImmediate => Cycles::Normal(2u8),
            Self::DECAbsoluteX => Cycles::Normal(7u8),
            Self::DECAbsolute => Cycles::Normal(6u8),
            Self::DECZeroPageX => Cycles::Normal(6u8),
            Self::DECZeroPage => Cycles::Normal(5u8),
            Self::DEX => Cycles::Normal(2u8),
            Self::DEY => Cycles::Normal(2u8),
            Self::EORIndirectY => Cycles::Page(5u8),
            Self::EORIndirectX => Cycles::Normal(6u8),
            Self::EORAbsoluteY => Cycles::Page(4u8),
            Self::EORAbsoluteX => Cycles::Page(4u8),
            Self::EORAbsolute => Cycles::Normal(4u8),
            Self::EORZeroPageX => Cycles::Normal(4u8),
            Self::EORZeroPage => Cycles::Normal(3u8),
            Self::EORImmediate => Cycles::Normal(2u8),
            Self::INCAbsoluteX => Cycles::Normal(7u8),
            Self::INCAbsolute => Cycles::Normal(6u8),
            Self::INCZeroPageX => Cycles::Normal(6u8),
            Self::INCZeroPage => Cycles::Normal(5u8),
            Self::INX => Cycles::Normal(2u8),
            Self::INY => Cycles::Normal(2u8),
            Self::JMPIndirect => Cycles::Normal(5u8),
            Self::JMPAbsolute => Cycles::Normal(3u8),
            Self::JSR => Cycles::Normal(6u8),
            Self::LDAIndirectY => Cycles::Page(5u8),
            Self::LDAIndirectX => Cycles::Normal(6u8),
            Self::LDAAbsoluteY => Cycles::Page(4u8),
            Self::LDAAbsoluteX => Cycles::Page(4u8),
            Self::LDAAbsolute => Cycles::Normal(4u8),
            Self::LDAZeroPageX => Cycles::Normal(4u8),
            Self::LDAZeroPage => Cycles::Normal(3u8),
            Self::LDAImmediate => Cycles::Normal(2u8),
            Self::LDXAbsoluteY => Cycles::Page(4u8),
            Self::LDXAbsolute => Cycles::Normal(4u8),
            Self::LDXZeroPageY => Cycles::Normal(4u8),
            Self::LDXZeroPage => Cycles::Normal(3u8),
            Self::LDXImmediate => Cycles::Normal(2u8),
            Self::LDYAbsoluteX => Cycles::Page(4u8),
            Self::LDYAbsolute => Cycles::Normal(4u8),
            Self::LDYZeroPageX => Cycles::Normal(4u8),
            Self::LDYZeroPage => Cycles::Normal(3u8),
            Self::LDYImmediate => Cycles::Normal(2u8),
            Self::LSRAbsoluteX => Cycles::Normal(7u8),
            Self::LSRAbsolute => Cycles::Normal(6u8),
            Self::LSRAccumulator => Cycles::Normal(2u8),
            Self::LSRZeroPageX => Cycles::Normal(6u8),
            Self::LSRZeroPage => Cycles::Normal(5u8),
            Self::NOP => Cycles::Normal(2u8),
            Self::ORAIndirectY => Cycles::Page(5u8),
            Self::ORAIndirectX => Cycles::Normal(6u8),
            Self::ORAAbsoluteY => Cycles::Page(4u8),
            Self::ORAAbsoluteX => Cycles::Page(4u8),
            Self::ORAAbsolute => Cycles::Normal(4u8),
            Self::ORAZeroPageX => Cycles::Normal(4u8),
            Self::ORAZeroPage => Cycles::Normal(3u8),
            Self::ORAImmediate => Cycles::Normal(2u8),
            Self::PHA => Cycles::Normal(3u8),
            Self::PHP => Cycles::Normal(3u8),
            Self::PLA => Cycles::Normal(4u8),
            Self::PLP => Cycles::Normal(4u8),
            Self::ROLAbsoluteX => Cycles::Normal(7u8),
            Self::ROLAbsolute => Cycles::Normal(6u8),
            Self::ROLAccumulator => Cycles::Normal(2u8),
            Self::ROLZeroPageX => Cycles::Normal(6u8),
            Self::ROLZeroPage => Cycles::Normal(5u8),
            Self::RORAbsoluteX => Cycles::Normal(7u8),
            Self::RORAbsolute => Cycles::Normal(6u8),
            Self::RORAccumulator => Cycles::Normal(2u8),
            Self::RORZeroPageX => Cycles::Normal(6u8),
            Self::RORZeroPage => Cycles::Normal(5u8),
            Self::RTI => Cycles::Normal(6u8),
            Self::RTS => Cycles::Normal(6u8),
            Self::SBCIndirectY => Cycles::Page(5u8),
            Self::SBCIndirectX => Cycles::Normal(6u8),
            Self::SBCAbsoluteY => Cycles::Page(4u8),
            Self::SBCAbsoluteX => Cycles::Page(4u8),
            Self::SBCAbsolute => Cycles::Normal(4u8),
            Self::SBCZeroPageX => Cycles::Normal(4u8),
            Self::SBCZeroPage => Cycles::Normal(3u8),
            Self::SBCImmediate => Cycles::Normal(2u8),
            Self::SEC => Cycles::Normal(2u8),
            Self::SED => Cycles::Normal(2u8),
            Self::SEI => Cycles::Normal(2u8),
            Self::STAIndirectY => Cycles::Normal(6u8),
            Self::STAIndirectX => Cycles::Normal(6u8),
            Self::STAAbsoluteY => Cycles::Normal(5u8),
            Self::STAAbsoluteX => Cycles::Normal(5u8),
            Self::STAAbsolute => Cycles::Normal(4u8),
            Self::STAZeroPageX => Cycles::Normal(4u8),
            Self::STAZeroPage => Cycles::Normal(3u8),
            Self::STXAbsolute => Cycles::Normal(4u8),
            Self::STXZeroPageY => Cycles::Normal(4u8),
            Self::STXZeroPage => Cycles::Normal(3u8),
            Self::STYAbsolute => Cycles::Normal(4u8),
            Self::STYZeroPageX => Cycles::Normal(4u8),
            Self::STYZeroPage => Cycles::Normal(3u8),
            Self::TAX => Cycles::Normal(2u8),
            Self::TAY => Cycles::Normal(2u8),
            Self::TSX => Cycles::Normal(2u8),
            Self::TXA => Cycles::Normal(2u8),
            Self::TXS => Cycles::Normal(2u8),
            Self::TYA => Cycles::Normal(2u8),
        }
    }
    pub fn bytes(&self) -> u8 {
        match *self {
            Self::ADCIndirectY => 2u8,
            Self::ADCIndirectX => 2u8,
            Self::ADCAbsoluteY => 3u8,
            Self::ADCAbsoluteX => 3u8,
            Self::ADCAbsolute => 3u8,
            Self::ADCZeroPageX => 2u8,
            Self::ADCZeroPage => 2u8,
            Self::ADCImmediate => 2u8,
            Self::ANDIndirectY => 2u8,
            Self::ANDIndirectX => 2u8,
            Self::ANDAbsoluteY => 3u8,
            Self::ANDAbsoluteX => 3u8,
            Self::ANDAbsolute => 3u8,
            Self::ANDZeroPageX => 2u8,
            Self::ANDZeroPage => 2u8,
            Self::ANDImmediate => 2u8,
            Self::ASLAbsoluteX => 3u8,
            Self::ASLAbsolute => 3u8,
            Self::ASLAccumulator => 1u8,
            Self::ASLZeroPageX => 2u8,
            Self::ASLZeroPage => 2u8,
            Self::BCCRelative => 2u8,
            Self::BCSRelative => 2u8,
            Self::BEQRelative => 2u8,
            Self::BITAbsolute => 3u8,
            Self::BITZeroPage => 2u8,
            Self::BMIRelative => 2u8,
            Self::BNERelative => 2u8,
            Self::BPLRelative => 2u8,
            Self::BRK => 2u8,
            Self::BVCRelative => 2u8,
            Self::BVSRelative => 2u8,
            Self::CLC => 1u8,
            Self::CLD => 1u8,
            Self::CLI => 1u8,
            Self::CLV => 1u8,
            Self::CMPIndirectY => 2u8,
            Self::CMPIndirectX => 2u8,
            Self::CMPAbsoluteY => 3u8,
            Self::CMPAbsoluteX => 3u8,
            Self::CMPAbsolute => 3u8,
            Self::CMPZeroPageX => 2u8,
            Self::CMPZeroPage => 2u8,
            Self::CMPImmediate => 2u8,
            Self::CPXAbsolute => 3u8,
            Self::CPXZeroPage => 2u8,
            Self::CPXImmediate => 2u8,
            Self::CPYAbsolute => 3u8,
            Self::CPYZeroPage => 2u8,
            Self::CPYImmediate => 2u8,
            Self::DECAbsoluteX => 3u8,
            Self::DECAbsolute => 3u8,
            Self::DECZeroPageX => 2u8,
            Self::DECZeroPage => 2u8,
            Self::DEX => 1u8,
            Self::DEY => 1u8,
            Self::EORIndirectY => 2u8,
            Self::EORIndirectX => 2u8,
            Self::EORAbsoluteY => 3u8,
            Self::EORAbsoluteX => 3u8,
            Self::EORAbsolute => 3u8,
            Self::EORZeroPageX => 2u8,
            Self::EORZeroPage => 2u8,
            Self::EORImmediate => 2u8,
            Self::INCAbsoluteX => 3u8,
            Self::INCAbsolute => 3u8,
            Self::INCZeroPageX => 2u8,
            Self::INCZeroPage => 2u8,
            Self::INX => 1u8,
            Self::INY => 1u8,
            Self::JMPIndirect => 3u8,
            Self::JMPAbsolute => 3u8,
            Self::JSR => 3u8,
            Self::LDAIndirectY => 2u8,
            Self::LDAIndirectX => 2u8,
            Self::LDAAbsoluteY => 3u8,
            Self::LDAAbsoluteX => 3u8,
            Self::LDAAbsolute => 3u8,
            Self::LDAZeroPageX => 2u8,
            Self::LDAZeroPage => 2u8,
            Self::LDAImmediate => 2u8,
            Self::LDXAbsoluteY => 3u8,
            Self::LDXAbsolute => 3u8,
            Self::LDXZeroPageY => 2u8,
            Self::LDXZeroPage => 2u8,
            Self::LDXImmediate => 2u8,
            Self::LDYAbsoluteX => 3u8,
            Self::LDYAbsolute => 3u8,
            Self::LDYZeroPageX => 2u8,
            Self::LDYZeroPage => 2u8,
            Self::LDYImmediate => 2u8,
            Self::LSRAbsoluteX => 3u8,
            Self::LSRAbsolute => 3u8,
            Self::LSRAccumulator => 1u8,
            Self::LSRZeroPageX => 2u8,
            Self::LSRZeroPage => 2u8,
            Self::NOP => 1u8,
            Self::ORAIndirectY => 2u8,
            Self::ORAIndirectX => 2u8,
            Self::ORAAbsoluteY => 3u8,
            Self::ORAAbsoluteX => 3u8,
            Self::ORAAbsolute => 3u8,
            Self::ORAZeroPageX => 2u8,
            Self::ORAZeroPage => 2u8,
            Self::ORAImmediate => 2u8,
            Self::PHA => 1u8,
            Self::PHP => 1u8,
            Self::PLA => 1u8,
            Self::PLP => 1u8,
            Self::ROLAbsoluteX => 3u8,
            Self::ROLAbsolute => 3u8,
            Self::ROLAccumulator => 1u8,
            Self::ROLZeroPageX => 2u8,
            Self::ROLZeroPage => 2u8,
            Self::RORAbsoluteX => 3u8,
            Self::RORAbsolute => 3u8,
            Self::RORAccumulator => 1u8,
            Self::RORZeroPageX => 2u8,
            Self::RORZeroPage => 2u8,
            Self::RTI => 1u8,
            Self::RTS => 1u8,
            Self::SBCIndirectY => 2u8,
            Self::SBCIndirectX => 2u8,
            Self::SBCAbsoluteY => 3u8,
            Self::SBCAbsoluteX => 3u8,
            Self::SBCAbsolute => 3u8,
            Self::SBCZeroPageX => 2u8,
            Self::SBCZeroPage => 2u8,
            Self::SBCImmediate => 2u8,
            Self::SEC => 1u8,
            Self::SED => 1u8,
            Self::SEI => 1u8,
            Self::STAIndirectY => 2u8,
            Self::STAIndirectX => 2u8,
            Self::STAAbsoluteY => 3u8,
            Self::STAAbsoluteX => 3u8,
            Self::STAAbsolute => 3u8,
            Self::STAZeroPageX => 2u8,
            Self::STAZeroPage => 2u8,
            Self::STXAbsolute => 3u8,
            Self::STXZeroPageY => 2u8,
            Self::STXZeroPage => 2u8,
            Self::STYAbsolute => 3u8,
            Self::STYZeroPageX => 2u8,
            Self::STYZeroPage => 2u8,
            Self::TAX => 1u8,
            Self::TAY => 1u8,
            Self::TSX => 1u8,
            Self::TXA => 1u8,
            Self::TXS => 1u8,
            Self::TYA => 1u8,
        }
    }
    pub fn asm_name(&self) -> &'static str {
        match *self {
            Self::ADCIndirectY => "ADC",
            Self::ADCIndirectX => "ADC",
            Self::ADCAbsoluteY => "ADC",
            Self::ADCAbsoluteX => "ADC",
            Self::ADCAbsolute => "ADC",
            Self::ADCZeroPageX => "ADC",
            Self::ADCZeroPage => "ADC",
            Self::ADCImmediate => "ADC",
            Self::ANDIndirectY => "AND",
            Self::ANDIndirectX => "AND",
            Self::ANDAbsoluteY => "AND",
            Self::ANDAbsoluteX => "AND",
            Self::ANDAbsolute => "AND",
            Self::ANDZeroPageX => "AND",
            Self::ANDZeroPage => "AND",
            Self::ANDImmediate => "AND",
            Self::ASLAbsoluteX => "ASL",
            Self::ASLAbsolute => "ASL",
            Self::ASLAccumulator => "ASL",
            Self::ASLZeroPageX => "ASL",
            Self::ASLZeroPage => "ASL",
            Self::BCCRelative => "BCC",
            Self::BCSRelative => "BCS",
            Self::BEQRelative => "BEQ",
            Self::BITAbsolute => "BIT",
            Self::BITZeroPage => "BIT",
            Self::BMIRelative => "BMI",
            Self::BNERelative => "BNE",
            Self::BPLRelative => "BPL",
            Self::BRK => "BRK",
            Self::BVCRelative => "BVC",
            Self::BVSRelative => "BVS",
            Self::CLC => "CLC",
            Self::CLD => "CLD",
            Self::CLI => "CLI",
            Self::CLV => "CLV",
            Self::CMPIndirectY => "CMP",
            Self::CMPIndirectX => "CMP",
            Self::CMPAbsoluteY => "CMP",
            Self::CMPAbsoluteX => "CMP",
            Self::CMPAbsolute => "CMP",
            Self::CMPZeroPageX => "CMP",
            Self::CMPZeroPage => "CMP",
            Self::CMPImmediate => "CMP",
            Self::CPXAbsolute => "CPX",
            Self::CPXZeroPage => "CPX",
            Self::CPXImmediate => "CPX",
            Self::CPYAbsolute => "CPY",
            Self::CPYZeroPage => "CPY",
            Self::CPYImmediate => "CPY",
            Self::DECAbsoluteX => "DEC",
            Self::DECAbsolute => "DEC",
            Self::DECZeroPageX => "DEC",
            Self::DECZeroPage => "DEC",
            Self::DEX => "DEX",
            Self::DEY => "DEY",
            Self::EORIndirectY => "EOR",
            Self::EORIndirectX => "EOR",
            Self::EORAbsoluteY => "EOR",
            Self::EORAbsoluteX => "EOR",
            Self::EORAbsolute => "EOR",
            Self::EORZeroPageX => "EOR",
            Self::EORZeroPage => "EOR",
            Self::EORImmediate => "EOR",
            Self::INCAbsoluteX => "INC",
            Self::INCAbsolute => "INC",
            Self::INCZeroPageX => "INC",
            Self::INCZeroPage => "INC",
            Self::INX => "INX",
            Self::INY => "INY",
            Self::JMPIndirect => "JMP",
            Self::JMPAbsolute => "JMP",
            Self::JSR => "JSR",
            Self::LDAIndirectY => "LDA",
            Self::LDAIndirectX => "LDA",
            Self::LDAAbsoluteY => "LDA",
            Self::LDAAbsoluteX => "LDA",
            Self::LDAAbsolute => "LDA",
            Self::LDAZeroPageX => "LDA",
            Self::LDAZeroPage => "LDA",
            Self::LDAImmediate => "LDA",
            Self::LDXAbsoluteY => "LDX",
            Self::LDXAbsolute => "LDX",
            Self::LDXZeroPageY => "LDX",
            Self::LDXZeroPage => "LDX",
            Self::LDXImmediate => "LDX",
            Self::LDYAbsoluteX => "LDY",
            Self::LDYAbsolute => "LDY",
            Self::LDYZeroPageX => "LDY",
            Self::LDYZeroPage => "LDY",
            Self::LDYImmediate => "LDY",
            Self::LSRAbsoluteX => "LSR",
            Self::LSRAbsolute => "LSR",
            Self::LSRAccumulator => "LSR",
            Self::LSRZeroPageX => "LSR",
            Self::LSRZeroPage => "LSR",
            Self::NOP => "NOP",
            Self::ORAIndirectY => "ORA",
            Self::ORAIndirectX => "ORA",
            Self::ORAAbsoluteY => "ORA",
            Self::ORAAbsoluteX => "ORA",
            Self::ORAAbsolute => "ORA",
            Self::ORAZeroPageX => "ORA",
            Self::ORAZeroPage => "ORA",
            Self::ORAImmediate => "ORA",
            Self::PHA => "PHA",
            Self::PHP => "PHP",
            Self::PLA => "PLA",
            Self::PLP => "PLP",
            Self::ROLAbsoluteX => "ROL",
            Self::ROLAbsolute => "ROL",
            Self::ROLAccumulator => "ROL",
            Self::ROLZeroPageX => "ROL",
            Self::ROLZeroPage => "ROL",
            Self::RORAbsoluteX => "ROR",
            Self::RORAbsolute => "ROR",
            Self::RORAccumulator => "ROR",
            Self::RORZeroPageX => "ROR",
            Self::RORZeroPage => "ROR",
            Self::RTI => "RTI",
            Self::RTS => "RTS",
            Self::SBCIndirectY => "SBC",
            Self::SBCIndirectX => "SBC",
            Self::SBCAbsoluteY => "SBC",
            Self::SBCAbsoluteX => "SBC",
            Self::SBCAbsolute => "SBC",
            Self::SBCZeroPageX => "SBC",
            Self::SBCZeroPage => "SBC",
            Self::SBCImmediate => "SBC",
            Self::SEC => "SEC",
            Self::SED => "SED",
            Self::SEI => "SEI",
            Self::STAIndirectY => "STA",
            Self::STAIndirectX => "STA",
            Self::STAAbsoluteY => "STA",
            Self::STAAbsoluteX => "STA",
            Self::STAAbsolute => "STA",
            Self::STAZeroPageX => "STA",
            Self::STAZeroPage => "STA",
            Self::STXAbsolute => "STX",
            Self::STXZeroPageY => "STX",
            Self::STXZeroPage => "STX",
            Self::STYAbsolute => "STY",
            Self::STYZeroPageX => "STY",
            Self::STYZeroPage => "STY",
            Self::TAX => "TAX",
            Self::TAY => "TAY",
            Self::TSX => "TSX",
            Self::TXA => "TXA",
            Self::TXS => "TXS",
            Self::TYA => "TYA",
        }
    }
    pub fn code(&self) -> u8 {
        *self as u8
    }
    pub fn mode(&self) -> Mode {
        match *self {
            Self::ADCIndirectY => Mode::IndirectY,
            Self::ADCIndirectX => Mode::IndirectX,
            Self::ADCAbsoluteY => Mode::AbsoluteY,
            Self::ADCAbsoluteX => Mode::AbsoluteX,
            Self::ADCAbsolute => Mode::Absolute,
            Self::ADCZeroPageX => Mode::ZeroPageX,
            Self::ADCZeroPage => Mode::ZeroPage,
            Self::ADCImmediate => Mode::Immediate,
            Self::ANDIndirectY => Mode::IndirectY,
            Self::ANDIndirectX => Mode::IndirectX,
            Self::ANDAbsoluteY => Mode::AbsoluteY,
            Self::ANDAbsoluteX => Mode::AbsoluteX,
            Self::ANDAbsolute => Mode::Absolute,
            Self::ANDZeroPageX => Mode::ZeroPageX,
            Self::ANDZeroPage => Mode::ZeroPage,
            Self::ANDImmediate => Mode::Immediate,
            Self::ASLAbsoluteX => Mode::AbsoluteX,
            Self::ASLAbsolute => Mode::Absolute,
            Self::ASLAccumulator => Mode::Accumulator,
            Self::ASLZeroPageX => Mode::ZeroPageX,
            Self::ASLZeroPage => Mode::ZeroPage,
            Self::BCCRelative => Mode::Relative,
            Self::BCSRelative => Mode::Relative,
            Self::BEQRelative => Mode::Relative,
            Self::BITAbsolute => Mode::Absolute,
            Self::BITZeroPage => Mode::ZeroPage,
            Self::BMIRelative => Mode::Relative,
            Self::BNERelative => Mode::Relative,
            Self::BPLRelative => Mode::Relative,
            Self::BRK => Mode::Implied,
            Self::BVCRelative => Mode::Relative,
            Self::BVSRelative => Mode::Relative,
            Self::CLC => Mode::Implied,
            Self::CLD => Mode::Implied,
            Self::CLI => Mode::Implied,
            Self::CLV => Mode::Implied,
            Self::CMPIndirectY => Mode::IndirectY,
            Self::CMPIndirectX => Mode::IndirectX,
            Self::CMPAbsoluteY => Mode::AbsoluteY,
            Self::CMPAbsoluteX => Mode::AbsoluteX,
            Self::CMPAbsolute => Mode::Absolute,
            Self::CMPZeroPageX => Mode::ZeroPageX,
            Self::CMPZeroPage => Mode::ZeroPage,
            Self::CMPImmediate => Mode::Immediate,
            Self::CPXAbsolute => Mode::Absolute,
            Self::CPXZeroPage => Mode::ZeroPage,
            Self::CPXImmediate => Mode::Immediate,
            Self::CPYAbsolute => Mode::Absolute,
            Self::CPYZeroPage => Mode::ZeroPage,
            Self::CPYImmediate => Mode::Immediate,
            Self::DECAbsoluteX => Mode::AbsoluteX,
            Self::DECAbsolute => Mode::Absolute,
            Self::DECZeroPageX => Mode::ZeroPageX,
            Self::DECZeroPage => Mode::ZeroPage,
            Self::DEX => Mode::Implied,
            Self::DEY => Mode::Implied,
            Self::EORIndirectY => Mode::IndirectY,
            Self::EORIndirectX => Mode::IndirectX,
            Self::EORAbsoluteY => Mode::AbsoluteY,
            Self::EORAbsoluteX => Mode::AbsoluteX,
            Self::EORAbsolute => Mode::Absolute,
            Self::EORZeroPageX => Mode::ZeroPageX,
            Self::EORZeroPage => Mode::ZeroPage,
            Self::EORImmediate => Mode::Immediate,
            Self::INCAbsoluteX => Mode::AbsoluteX,
            Self::INCAbsolute => Mode::Absolute,
            Self::INCZeroPageX => Mode::ZeroPageX,
            Self::INCZeroPage => Mode::ZeroPage,
            Self::INX => Mode::Implied,
            Self::INY => Mode::Implied,
            Self::JMPIndirect => Mode::Indirect,
            Self::JMPAbsolute => Mode::Absolute,
            Self::JSR => Mode::Implied,
            Self::LDAIndirectY => Mode::IndirectY,
            Self::LDAIndirectX => Mode::IndirectX,
            Self::LDAAbsoluteY => Mode::AbsoluteY,
            Self::LDAAbsoluteX => Mode::AbsoluteX,
            Self::LDAAbsolute => Mode::Absolute,
            Self::LDAZeroPageX => Mode::ZeroPageX,
            Self::LDAZeroPage => Mode::ZeroPage,
            Self::LDAImmediate => Mode::Immediate,
            Self::LDXAbsoluteY => Mode::AbsoluteY,
            Self::LDXAbsolute => Mode::Absolute,
            Self::LDXZeroPageY => Mode::ZeroPageY,
            Self::LDXZeroPage => Mode::ZeroPage,
            Self::LDXImmediate => Mode::Immediate,
            Self::LDYAbsoluteX => Mode::AbsoluteX,
            Self::LDYAbsolute => Mode::Absolute,
            Self::LDYZeroPageX => Mode::ZeroPageX,
            Self::LDYZeroPage => Mode::ZeroPage,
            Self::LDYImmediate => Mode::Immediate,
            Self::LSRAbsoluteX => Mode::AbsoluteX,
            Self::LSRAbsolute => Mode::Absolute,
            Self::LSRAccumulator => Mode::Accumulator,
            Self::LSRZeroPageX => Mode::ZeroPageX,
            Self::LSRZeroPage => Mode::ZeroPage,
            Self::NOP => Mode::Implied,
            Self::ORAIndirectY => Mode::IndirectY,
            Self::ORAIndirectX => Mode::IndirectX,
            Self::ORAAbsoluteY => Mode::AbsoluteY,
            Self::ORAAbsoluteX => Mode::AbsoluteX,
            Self::ORAAbsolute => Mode::Absolute,
            Self::ORAZeroPageX => Mode::ZeroPageX,
            Self::ORAZeroPage => Mode::ZeroPage,
            Self::ORAImmediate => Mode::Immediate,
            Self::PHA => Mode::Implied,
            Self::PHP => Mode::Implied,
            Self::PLA => Mode::Implied,
            Self::PLP => Mode::Implied,
            Self::ROLAbsoluteX => Mode::AbsoluteX,
            Self::ROLAbsolute => Mode::Absolute,
            Self::ROLAccumulator => Mode::Accumulator,
            Self::ROLZeroPageX => Mode::ZeroPageX,
            Self::ROLZeroPage => Mode::ZeroPage,
            Self::RORAbsoluteX => Mode::AbsoluteX,
            Self::RORAbsolute => Mode::Absolute,
            Self::RORAccumulator => Mode::Accumulator,
            Self::RORZeroPageX => Mode::ZeroPageX,
            Self::RORZeroPage => Mode::ZeroPage,
            Self::RTI => Mode::Implied,
            Self::RTS => Mode::Implied,
            Self::SBCIndirectY => Mode::IndirectY,
            Self::SBCIndirectX => Mode::IndirectX,
            Self::SBCAbsoluteY => Mode::AbsoluteY,
            Self::SBCAbsoluteX => Mode::AbsoluteX,
            Self::SBCAbsolute => Mode::Absolute,
            Self::SBCZeroPageX => Mode::ZeroPageX,
            Self::SBCZeroPage => Mode::ZeroPage,
            Self::SBCImmediate => Mode::Immediate,
            Self::SEC => Mode::Implied,
            Self::SED => Mode::Implied,
            Self::SEI => Mode::Implied,
            Self::STAIndirectY => Mode::IndirectY,
            Self::STAIndirectX => Mode::IndirectX,
            Self::STAAbsoluteY => Mode::AbsoluteY,
            Self::STAAbsoluteX => Mode::AbsoluteX,
            Self::STAAbsolute => Mode::Absolute,
            Self::STAZeroPageX => Mode::ZeroPageX,
            Self::STAZeroPage => Mode::ZeroPage,
            Self::STXAbsolute => Mode::Absolute,
            Self::STXZeroPageY => Mode::ZeroPageY,
            Self::STXZeroPage => Mode::ZeroPage,
            Self::STYAbsolute => Mode::Absolute,
            Self::STYZeroPageX => Mode::ZeroPageX,
            Self::STYZeroPage => Mode::ZeroPage,
            Self::TAX => Mode::Implied,
            Self::TAY => Mode::Implied,
            Self::TSX => Mode::Implied,
            Self::TXA => Mode::Implied,
            Self::TXS => Mode::Implied,
            Self::TYA => Mode::Implied,
        }
    }
    pub fn run(&self, input: [u8; 2], cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::CpuBus) {
        let additional_cycles = match *self {
            Self::ADCIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ADCImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::carrying_add(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::ANDIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ANDImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a &= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b01000000) == 0b01000000);
                    1
                }
            }
            Self::ASLAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) << 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ASLAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) << 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ASLAccumulator => {
                cpu.accumulator(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) << 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ASLZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) << 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ASLZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) << 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::BCCRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if !cpu.flags.contains(Flags::C) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BCSRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if cpu.flags.contains(Flags::C) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BEQRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if cpu.flags.contains(Flags::Z) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BITAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = cpu.registers.a & data;
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b10000000) == 0b10000000);
                    0
                }
            }
            Self::BITZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = cpu.registers.a & data;
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (data & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, (data & 0b10000000) == 0b10000000);
                    0
                }
            }
            Self::BMIRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if cpu.flags.contains(Flags::Z) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BNERelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if !cpu.flags.contains(Flags::Z) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BPLRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if !cpu.flags.contains(Flags::N) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BRK => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.pc += 1;
                    cpu.flags.insert(Flags::I);
                    let [high, low] = cpu.registers.pc.to_le_bytes();
                    bus.write(high, 0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer -= 1;
                    bus.write(low, 0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer -= 1;
                    cpu.flags.insert(Flags::B);
                    bus.write(
                        cpu.flags.bits(),
                        0x100 + (cpu.registers.stack_pointer as u16),
                    );
                    cpu.registers.stack_pointer -= 1;
                    cpu.flags.remove(Flags::B);
                    cpu.registers.pc = u16::from_be_bytes([bus.read(0xFFFF), bus.read(0xFFFE)]);
                    0
                }
            }
            Self::BVCRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if !cpu.flags.contains(Flags::V) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::BVSRelative => {
                cpu.relative(input, bus);
                let data = cpu.fetch(bus);
                {
                    let mut cycles = 0;
                    if cpu.flags.contains(Flags::V) {
                        cycles += 1;
                        cpu.addr = cpu.registers.pc + cpu.rel;
                        if (cpu.addr & 0xFF00) != (cpu.registers.pc & 0xFF00) {
                            cycles += 1;
                        }
                        cpu.registers.pc = cpu.addr;
                    }
                    cycles
                }
            }
            Self::CLC => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.remove(Flags::C);
                    0
                }
            }
            Self::CLD => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.remove(Flags::D);
                    0
                }
            }
            Self::CLI => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.remove(Flags::I);
                    0
                }
            }
            Self::CLV => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.remove(Flags::V);
                    0
                }
            }
            Self::CMPIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CMPImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.a.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.a >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    1
                }
            }
            Self::CPXAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.x.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.x >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::CPXZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.x.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.x >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::CPXImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.x.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.x >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::CPYAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.y.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.y >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::CPYZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.y.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.y >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::CPYImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    let sub = cpu.registers.y.wrapping_sub(data);
                    cpu.flags.set(Flags::C, cpu.registers.y >= data);
                    cpu.flags.set(Flags::Z, sub == 0);
                    cpu.flags.set(Flags::N, (sub & 0x80) == 0x80);
                    0
                }
            }
            Self::DECAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let new_data = data.wrapping_sub(1);
                    bus.write(new_data, cpu.addr);
                    cpu.flags.set(Flags::Z, new_data == 0);
                    cpu.flags.set(Flags::N, new_data & 0x80 > 0);
                    0
                }
            }
            Self::DECAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let new_data = data.wrapping_sub(1);
                    bus.write(new_data, cpu.addr);
                    cpu.flags.set(Flags::Z, new_data == 0);
                    cpu.flags.set(Flags::N, new_data & 0x80 > 0);
                    0
                }
            }
            Self::DECZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let new_data = data.wrapping_sub(1);
                    bus.write(new_data, cpu.addr);
                    cpu.flags.set(Flags::Z, new_data == 0);
                    cpu.flags.set(Flags::N, new_data & 0x80 > 0);
                    0
                }
            }
            Self::DECZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let new_data = data.wrapping_sub(1);
                    bus.write(new_data, cpu.addr);
                    cpu.flags.set(Flags::Z, new_data == 0);
                    cpu.flags.set(Flags::N, new_data & 0x80 > 0);
                    0
                }
            }
            Self::DEX => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = cpu.registers.x.wrapping_sub(1);
                    cpu.flags.set(Flags::Z, cpu.registers.x == 0);
                    cpu.flags.set(Flags::N, cpu.registers.x & 0x80 > 0);
                    0
                }
            }
            Self::DEY => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
                    cpu.flags.set(Flags::Z, cpu.registers.y == 0);
                    cpu.flags.set(Flags::N, cpu.registers.y & 0x80 > 0);
                    0
                }
            }
            Self::EORIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::EORImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a ^= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    1
                }
            }
            Self::INCAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = data.wrapping_add(1);
                    bus.write(tmp, cpu.addr);
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::INCAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = data.wrapping_add(1);
                    bus.write(tmp, cpu.addr);
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::INCZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = data.wrapping_add(1);
                    bus.write(tmp, cpu.addr);
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::INCZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = data.wrapping_add(1);
                    bus.write(tmp, cpu.addr);
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::INX => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = cpu.registers.x.wrapping_add(1);
                    cpu.registers.x = tmp;
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::INY => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    let tmp = cpu.registers.y.wrapping_add(1);
                    cpu.registers.x = tmp;
                    cpu.flags.set(Flags::Z, tmp == 0);
                    cpu.flags.set(Flags::N, (tmp & 0x80) == 0x80);
                    0
                }
            }
            Self::JMPIndirect => {
                cpu.indirect(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.pc = cpu.addr;
                    0
                }
            }
            Self::JMPAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.pc = cpu.addr;
                    0
                }
            }
            Self::JSR => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
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
            }
            Self::LDAIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDAImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.a == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDXAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.x == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDXAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.x == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDXZeroPageY => {
                cpu.zero_page_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.x == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDXZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.x == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDXImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.x == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDYAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDYAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDYZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDYZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LDYImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = data;
                    cpu.flags.set(Flags::ZERO, cpu.registers.y == 0);
                    cpu.flags
                        .set(Flags::NEGATIVE, (data & 0b10000000) == 0b10000000);
                    1
                }
            }
            Self::LSRAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) >> 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::LSRAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) >> 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::LSRAccumulator => {
                cpu.accumulator(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) >> 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::LSRZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) >> 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::LSRZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = (data as u16) >> 1;
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::NOP => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    0
                }
            }
            Self::ORAIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::ORAImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a |= data;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::PHA => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(
                        cpu.registers.a,
                        0x100 + (cpu.registers.stack_pointer as u16),
                    );
                    cpu.registers.stack_pointer -= 1;
                    0
                }
            }
            Self::PHP => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(
                        (cpu.flags | Flags::B | Flags::U).bits(),
                        0x100 + (cpu.registers.stack_pointer as u16),
                    );
                    cpu.registers.stack_pointer -= 1;
                    cpu.flags &= !(Flags::B | Flags::U);
                    0
                }
            }
            Self::PLA => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.stack_pointer += 1;
                    cpu.registers.a = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::PLP => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.stack_pointer += 1;
                    cpu.flags = Flags::from_bits_truncate(
                        bus.read(0x100 + (cpu.registers.stack_pointer as u16)),
                    ) | Flags::U;
                    0
                }
            }
            Self::ROLAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result =
                        ((data as u16) << 1) | if cpu.flags.contains(Flags::C) { 1 } else { 0 };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ROLAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result =
                        ((data as u16) << 1) | if cpu.flags.contains(Flags::C) { 1 } else { 0 };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ROLAccumulator => {
                cpu.accumulator(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result =
                        ((data as u16) << 1) | if cpu.flags.contains(Flags::C) { 1 } else { 0 };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ROLZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result =
                        ((data as u16) << 1) | if cpu.flags.contains(Flags::C) { 1 } else { 0 };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::ROLZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result =
                        ((data as u16) << 1) | if cpu.flags.contains(Flags::C) { 1 } else { 0 };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0xFF00 > 0);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RORAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = ((data as u16) >> 1)
                        | if cpu.flags.contains(Flags::C) {
                            1 << 7
                        } else {
                            0
                        };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0x01 == 0x01);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RORAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = ((data as u16) >> 1)
                        | if cpu.flags.contains(Flags::C) {
                            1 << 7
                        } else {
                            0
                        };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0x01 == 0x01);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RORAccumulator => {
                cpu.accumulator(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = ((data as u16) >> 1)
                        | if cpu.flags.contains(Flags::C) {
                            1 << 7
                        } else {
                            0
                        };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0x01 == 0x01);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RORZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = ((data as u16) >> 1)
                        | if cpu.flags.contains(Flags::C) {
                            1 << 7
                        } else {
                            0
                        };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0x01 == 0x01);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RORZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let result = ((data as u16) >> 1)
                        | if cpu.flags.contains(Flags::C) {
                            1 << 7
                        } else {
                            0
                        };
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::C, result & 0x01 == 0x01);
                    let result_u8 = (result & 0xFF).try_into().unwrap();
                    if self.mode() == Mode::Implied {
                        cpu.registers.a = result_u8;
                    } else {
                        bus.write(result_u8, cpu.addr);
                    }
                    0
                }
            }
            Self::RTI => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    let status_bits = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.flags = Flags::from_bits_truncate(status_bits) & (!Flags::B) & (!Flags::U);
                    cpu.registers.stack_pointer += 1;
                    let low = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer += 1;
                    let high = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer += 1;
                    cpu.registers.pc = u16::from_be_bytes([high, low]);
                    0
                }
            }
            Self::RTS => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    let low = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer += 1;
                    let high = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
                    cpu.registers.stack_pointer += 1;
                    cpu.registers.pc = u16::from_be_bytes([high, low]);
                    0
                }
            }
            Self::SBCIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SBCImmediate => {
                cpu.immediate(input, bus);
                let data = cpu.fetch(bus);
                {
                    let (result, carry) = crate::utils::borrowing_sub(
                        cpu.registers.a,
                        data,
                        cpu.flags.contains(Flags::CARRY),
                    );
                    cpu.flags.set(Flags::C, carry);
                    cpu.flags.set(Flags::N, (result & 0b10000000) == 0b10000000);
                    cpu.flags.set(Flags::V, carry);
                    cpu.flags.set(Flags::Z, result == 0);
                    cpu.registers.a = result;
                    1
                }
            }
            Self::SEC => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.insert(Flags::C);
                    0
                }
            }
            Self::SED => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.insert(Flags::D);
                    0
                }
            }
            Self::SEI => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.flags.insert(Flags::I);
                    0
                }
            }
            Self::STAIndirectY => {
                cpu.indirect_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAIndirectX => {
                cpu.indirect_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAAbsoluteY => {
                cpu.absolute_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAAbsoluteX => {
                cpu.absolute_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STAZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.a, cpu.addr);
                    0
                }
            }
            Self::STXAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.x, cpu.addr);
                    0
                }
            }
            Self::STXZeroPageY => {
                cpu.zero_page_y(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.x, cpu.addr);
                    0
                }
            }
            Self::STXZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.x, cpu.addr);
                    0
                }
            }
            Self::STYAbsolute => {
                cpu.absolute(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.y, cpu.addr);
                    0
                }
            }
            Self::STYZeroPageX => {
                cpu.zero_page_x(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.y, cpu.addr);
                    0
                }
            }
            Self::STYZeroPage => {
                cpu.zero_page(input, bus);
                let data = cpu.fetch(bus);
                {
                    bus.write(cpu.registers.y, cpu.addr);
                    0
                }
            }
            Self::TAX => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = cpu.registers.a;
                    cpu.flags.set(Flags::Z, cpu.registers.x == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.x & 0x80) == 0x80);
                    0
                }
            }
            Self::TAY => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.y = cpu.registers.a;
                    cpu.flags.set(Flags::Z, cpu.registers.y == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.y & 0x80) == 0x80);
                    0
                }
            }
            Self::TSX => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = cpu.registers.stack_pointer;
                    cpu.flags.set(Flags::Z, cpu.registers.x == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.x & 0x80) == 0x80);
                    0
                }
            }
            Self::TXA => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = cpu.registers.x;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
            Self::TXS => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.x = cpu.registers.stack_pointer;
                    0
                }
            }
            Self::TYA => {
                cpu.implied(input, bus);
                let data = cpu.fetch(bus);
                {
                    cpu.registers.a = cpu.registers.y;
                    cpu.flags.set(Flags::Z, cpu.registers.a == 0);
                    cpu.flags.set(Flags::N, (cpu.registers.a & 0x80) == 0x80);
                    0
                }
            }
        };
        cpu.remaining_cycles += additional_cycles;
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct UnknownOpcode;
impl std::fmt::Display for UnknownOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown Opcode")
    }
}
impl std::error::Error for UnknownOpcode {}
impl TryFrom<u8> for Instructions {
    type Error = UnknownOpcode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            113u8 => Ok(Self::ADCIndirectY),
            97u8 => Ok(Self::ADCIndirectX),
            121u8 => Ok(Self::ADCAbsoluteY),
            125u8 => Ok(Self::ADCAbsoluteX),
            109u8 => Ok(Self::ADCAbsolute),
            117u8 => Ok(Self::ADCZeroPageX),
            101u8 => Ok(Self::ADCZeroPage),
            105u8 => Ok(Self::ADCImmediate),
            49u8 => Ok(Self::ANDIndirectY),
            33u8 => Ok(Self::ANDIndirectX),
            57u8 => Ok(Self::ANDAbsoluteY),
            61u8 => Ok(Self::ANDAbsoluteX),
            45u8 => Ok(Self::ANDAbsolute),
            53u8 => Ok(Self::ANDZeroPageX),
            37u8 => Ok(Self::ANDZeroPage),
            41u8 => Ok(Self::ANDImmediate),
            30u8 => Ok(Self::ASLAbsoluteX),
            14u8 => Ok(Self::ASLAbsolute),
            10u8 => Ok(Self::ASLAccumulator),
            22u8 => Ok(Self::ASLZeroPageX),
            6u8 => Ok(Self::ASLZeroPage),
            144u8 => Ok(Self::BCCRelative),
            176u8 => Ok(Self::BCSRelative),
            240u8 => Ok(Self::BEQRelative),
            44u8 => Ok(Self::BITAbsolute),
            36u8 => Ok(Self::BITZeroPage),
            48u8 => Ok(Self::BMIRelative),
            208u8 => Ok(Self::BNERelative),
            16u8 => Ok(Self::BPLRelative),
            0u8 => Ok(Self::BRK),
            80u8 => Ok(Self::BVCRelative),
            112u8 => Ok(Self::BVSRelative),
            24u8 => Ok(Self::CLC),
            216u8 => Ok(Self::CLD),
            88u8 => Ok(Self::CLI),
            184u8 => Ok(Self::CLV),
            209u8 => Ok(Self::CMPIndirectY),
            193u8 => Ok(Self::CMPIndirectX),
            217u8 => Ok(Self::CMPAbsoluteY),
            221u8 => Ok(Self::CMPAbsoluteX),
            205u8 => Ok(Self::CMPAbsolute),
            213u8 => Ok(Self::CMPZeroPageX),
            197u8 => Ok(Self::CMPZeroPage),
            201u8 => Ok(Self::CMPImmediate),
            236u8 => Ok(Self::CPXAbsolute),
            228u8 => Ok(Self::CPXZeroPage),
            224u8 => Ok(Self::CPXImmediate),
            204u8 => Ok(Self::CPYAbsolute),
            196u8 => Ok(Self::CPYZeroPage),
            192u8 => Ok(Self::CPYImmediate),
            222u8 => Ok(Self::DECAbsoluteX),
            206u8 => Ok(Self::DECAbsolute),
            214u8 => Ok(Self::DECZeroPageX),
            198u8 => Ok(Self::DECZeroPage),
            202u8 => Ok(Self::DEX),
            136u8 => Ok(Self::DEY),
            81u8 => Ok(Self::EORIndirectY),
            65u8 => Ok(Self::EORIndirectX),
            89u8 => Ok(Self::EORAbsoluteY),
            93u8 => Ok(Self::EORAbsoluteX),
            77u8 => Ok(Self::EORAbsolute),
            85u8 => Ok(Self::EORZeroPageX),
            69u8 => Ok(Self::EORZeroPage),
            73u8 => Ok(Self::EORImmediate),
            254u8 => Ok(Self::INCAbsoluteX),
            238u8 => Ok(Self::INCAbsolute),
            246u8 => Ok(Self::INCZeroPageX),
            230u8 => Ok(Self::INCZeroPage),
            232u8 => Ok(Self::INX),
            200u8 => Ok(Self::INY),
            108u8 => Ok(Self::JMPIndirect),
            76u8 => Ok(Self::JMPAbsolute),
            32u8 => Ok(Self::JSR),
            177u8 => Ok(Self::LDAIndirectY),
            161u8 => Ok(Self::LDAIndirectX),
            185u8 => Ok(Self::LDAAbsoluteY),
            189u8 => Ok(Self::LDAAbsoluteX),
            173u8 => Ok(Self::LDAAbsolute),
            181u8 => Ok(Self::LDAZeroPageX),
            165u8 => Ok(Self::LDAZeroPage),
            169u8 => Ok(Self::LDAImmediate),
            190u8 => Ok(Self::LDXAbsoluteY),
            174u8 => Ok(Self::LDXAbsolute),
            182u8 => Ok(Self::LDXZeroPageY),
            166u8 => Ok(Self::LDXZeroPage),
            162u8 => Ok(Self::LDXImmediate),
            188u8 => Ok(Self::LDYAbsoluteX),
            172u8 => Ok(Self::LDYAbsolute),
            180u8 => Ok(Self::LDYZeroPageX),
            164u8 => Ok(Self::LDYZeroPage),
            160u8 => Ok(Self::LDYImmediate),
            94u8 => Ok(Self::LSRAbsoluteX),
            78u8 => Ok(Self::LSRAbsolute),
            74u8 => Ok(Self::LSRAccumulator),
            86u8 => Ok(Self::LSRZeroPageX),
            70u8 => Ok(Self::LSRZeroPage),
            234u8 => Ok(Self::NOP),
            17u8 => Ok(Self::ORAIndirectY),
            1u8 => Ok(Self::ORAIndirectX),
            25u8 => Ok(Self::ORAAbsoluteY),
            29u8 => Ok(Self::ORAAbsoluteX),
            13u8 => Ok(Self::ORAAbsolute),
            21u8 => Ok(Self::ORAZeroPageX),
            5u8 => Ok(Self::ORAZeroPage),
            9u8 => Ok(Self::ORAImmediate),
            72u8 => Ok(Self::PHA),
            8u8 => Ok(Self::PHP),
            104u8 => Ok(Self::PLA),
            40u8 => Ok(Self::PLP),
            62u8 => Ok(Self::ROLAbsoluteX),
            46u8 => Ok(Self::ROLAbsolute),
            42u8 => Ok(Self::ROLAccumulator),
            54u8 => Ok(Self::ROLZeroPageX),
            38u8 => Ok(Self::ROLZeroPage),
            126u8 => Ok(Self::RORAbsoluteX),
            110u8 => Ok(Self::RORAbsolute),
            106u8 => Ok(Self::RORAccumulator),
            118u8 => Ok(Self::RORZeroPageX),
            102u8 => Ok(Self::RORZeroPage),
            64u8 => Ok(Self::RTI),
            96u8 => Ok(Self::RTS),
            241u8 => Ok(Self::SBCIndirectY),
            225u8 => Ok(Self::SBCIndirectX),
            249u8 => Ok(Self::SBCAbsoluteY),
            253u8 => Ok(Self::SBCAbsoluteX),
            237u8 => Ok(Self::SBCAbsolute),
            245u8 => Ok(Self::SBCZeroPageX),
            229u8 => Ok(Self::SBCZeroPage),
            233u8 => Ok(Self::SBCImmediate),
            56u8 => Ok(Self::SEC),
            248u8 => Ok(Self::SED),
            120u8 => Ok(Self::SEI),
            145u8 => Ok(Self::STAIndirectY),
            129u8 => Ok(Self::STAIndirectX),
            153u8 => Ok(Self::STAAbsoluteY),
            157u8 => Ok(Self::STAAbsoluteX),
            141u8 => Ok(Self::STAAbsolute),
            149u8 => Ok(Self::STAZeroPageX),
            133u8 => Ok(Self::STAZeroPage),
            142u8 => Ok(Self::STXAbsolute),
            150u8 => Ok(Self::STXZeroPageY),
            134u8 => Ok(Self::STXZeroPage),
            140u8 => Ok(Self::STYAbsolute),
            148u8 => Ok(Self::STYZeroPageX),
            132u8 => Ok(Self::STYZeroPage),
            170u8 => Ok(Self::TAX),
            168u8 => Ok(Self::TAY),
            186u8 => Ok(Self::TSX),
            138u8 => Ok(Self::TXA),
            154u8 => Ok(Self::TXS),
            152u8 => Ok(Self::TYA),
            _ => Err(UnknownOpcode),
        }
    }
}

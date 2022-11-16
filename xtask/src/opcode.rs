use scraper::Selector;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OpcodeElement {
    name: String,
    cycles: Cycles,
    bytes: u8,
    flags: Flags,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpcodeElementWithCode {
    pub code: u8,
    pub name: String,
    pub cycles: Cycles,
    pub bytes: u8,
    pub flags: Flags,
    pub mode: Mode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Cycles {
    Normal(u8),
    Paged(u8),
    Branched(u8),
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u8 {
        const CARRY =               1 << 0;
        const ZERO_RESULT =         1 << 1;
        const INTERRUPT_DISABLE =   1 << 2;
        const DECIMAL =             1 << 3;
        const BREAK =               1 << 4;
        const OVERFLOW =            1 << 6;
        const NEGATIVE =            1 << 7;
    }
}

impl Flags {
    fn parse(input: &str) -> Self {
        input
            .chars()
            .map(|c| match c {
                'N' => Self::NEGATIVE,
                'Z' => Self::ZERO_RESULT,
                'C' => Self::CARRY,
                'I' => Self::INTERRUPT_DISABLE,
                'D' => Self::DECIMAL,
                'V' => Self::OVERFLOW,
                _ => Self::empty(),
            })
            .fold(Self::empty(), |a, c| a | c)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Mode {
    Implied,
    Immediate,
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

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;
        write!(
            f,
            "{}",
            match *self {
                Implied => "",
                Immediate => "Immediate",
                ZeroPage => "ZeroPage",
                ZeroPageX => "ZeroPageX",
                ZeroPageY => "ZeroPageY",
                Accumulator => "Accumulator",
                Absolute => "Absolute",
                AbsoluteX => "AbsoluteX",
                AbsoluteY => "AbsoluteY",
                Indirect => "Indirect",
                IndirectX => "IndirectX",
                IndirectY => "IndirectY",
                Relative => "Relative",
            }
        )
    }
}

impl Mode {
    fn parse(input: &str) -> Mode {
        match input {
            "Immediate" => Self::Immediate,
            "Zero Page" => Self::ZeroPage,
            "Zero Page, X" => Self::ZeroPageX,
            "Zero Page, Y" => Self::ZeroPageY,
            "Accumulator" => Self::Accumulator,
            "Absolute" => Self::Absolute,
            "Absolute, X" => Self::AbsoluteX,
            "Absolute, Y" => Self::AbsoluteY,
            "Indirect" => Self::Indirect,
            "(Indirect, X)" => Self::IndirectX,
            "(Indirect), Y" => Self::IndirectY,
            "Relative" => Self::Relative,
            "" => Self::Implied,
            unknown => {
                println!("unknown mode: `{unknown}`");
                Self::Implied
            }
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            Self::Immediate => "Immediate",
            Self::ZeroPage => "ZeroPage",
            Self::ZeroPageX => "ZeroPageX",
            Self::ZeroPageY => "ZeroPageY",
            Self::Accumulator => "Accumulator",
            Self::Absolute => "Absolute",
            Self::AbsoluteX => "AbsoluteX",
            Self::AbsoluteY => "AbsoluteY",
            Self::Indirect => "Indirect",
            Self::IndirectX => "IndirectX",
            Self::IndirectY => "IndirectY",
            Self::Relative => "Relative",
            Self::Implied => "Implied",
        }
    }
}

fn parse_size_time(input: &str) -> (u8, Cycles) {
    let mut iter = input.split_whitespace();
    let size: u8 = iter.next().unwrap().parse().unwrap();
    let cycles = {
        let raw = iter.next().unwrap();
        if raw.contains('/') {
            let mut iter = raw.split('/');
            let _branched = iter.next().unwrap();
            let normal: u8 = iter.next().unwrap().parse().unwrap();
            Cycles::Branched(normal)
        } else if raw.ends_with('+') {
            Cycles::Paged(raw.strip_suffix('+').unwrap().parse().unwrap())
        } else {
            Cycles::Normal(raw.parse().unwrap())
        }
    };
    (size, cycles)
}

pub fn run() -> Vec<(u8, OpcodeElementWithCode)> {
    let html_table = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/opcodes1.html");
    let more_data = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/opcodes2.html");
    let opcode_data = scraper::Html::parse_fragment(
        std::fs::read_to_string(html_table)
            .expect("Unable to open first file")
            .as_str(),
    );
    let opcode2_data = scraper::Html::parse_fragment(
        std::fs::read_to_string(more_data)
            .expect("Unable to open first file")
            .as_str(),
    );
    let mut opcodes_out = std::collections::BTreeMap::new();

    for (mut index, high) in (0x00..=0xF0u8).step_by(0x10).enumerate() {
        index += 2;
        let selector = Selector::parse(&format!("tbody > tr:nth-child({index}) > td")).unwrap();
        let data = opcode_data
            .select(&selector)
            .skip(1)
            .map(|t| t.text())
            .map(|t| t.collect::<arrayvec::ArrayVec<_, 3>>())
            .map(|v| (v.len() == 3).then_some(v))
            .map(|v| {
                v.map(|d| {
                    let name = d[0].split_whitespace().next().unwrap().to_string();
                    let (bytes, cycles) = parse_size_time(d[1]);
                    let flags = Flags::parse(d[2]);
                    OpcodeElement {
                        name,
                        bytes,
                        cycles,
                        flags,
                    }
                })
            })
            .enumerate()
            .map(|(low, elem)| (high | low as u8, elem))
            .map(|(code, elem)| {
                (
                    code,
                    elem.map(
                        |OpcodeElement {
                             name,
                             cycles,
                             bytes,
                             flags,
                         }| OpcodeElementWithCode {
                            name,
                            code,
                            cycles,
                            bytes,
                            flags,
                            mode: Mode::Implied,
                        },
                    ),
                )
            });
        opcodes_out.extend(data);
    }
    opcode2_data
        .select(&Selector::parse("table > tbody > tr > td > table > tbody").unwrap())
        .flat_map(|t| t.children().skip(1))
        .filter(|t| t.value().is_element())
        .map(|f| {
            f.children()
                .filter(|t| t.value().is_element())
                .collect::<arrayvec::ArrayVec<_, 10>>()
        })
        .map(|v| {
            (
                u8::from_str_radix(
                    v[0].first_child()
                        .unwrap()
                        .value()
                        .as_text()
                        .unwrap()
                        .trim(),
                    16,
                )
                .unwrap(),
                if let Some(mode) = v.get(2) {
                    Mode::parse(
                        mode.first_child()
                            .unwrap()
                            .value()
                            .as_text()
                            .unwrap()
                            .trim(),
                    )
                } else {
                    Mode::Implied
                },
            )
        })
        .for_each(|(code, mode)| {
            if let Some(Some(opcode)) = opcodes_out.get_mut(&code) {
                opcode.mode = mode;
            }
        });
    let relative_patch = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../data/opcodes3.json"
    ))
    .unwrap();
    let relative_patch_json: std::collections::HashMap<&str, Mode> =
        serde_json::from_str(&relative_patch).unwrap();

    for (opcode, mode) in relative_patch_json {
        let opcode = u8::from_str_radix(opcode, 16).unwrap();
        if let Some(c) = opcodes_out.get_mut(&opcode).and_then(Option::as_mut) {
            c.mode = mode;
        }
    }

    let json = serde_json::to_string_pretty(&opcodes_out).unwrap();
    std::fs::write("out/opcode.json", &json[..]).unwrap();
    let mut out_vec = opcodes_out
        .into_iter()
        .filter_map(|(c, v)| v.map(|v| (c, v)))
        .collect::<Vec<_>>();

    out_vec.sort_by(|lhs, rhs| {
        if lhs.1.name == rhs.1.name {
            lhs.1.mode.cmp(&rhs.1.mode).reverse()
        } else {
            lhs.1.name.cmp(&rhs.1.name)
        }
    });
    out_vec
}

use crate::opcode::{Cycles, Mode, OpcodeElementWithCode};

pub fn run(opcodes: Vec<(u8, OpcodeElementWithCode)>) {
    let asm_to_opcode = {
        let mut out = std::collections::BTreeMap::<&str, Vec<&OpcodeElementWithCode>>::new();

        for opcode in opcodes.iter().map(|(_, v)| v) {
            let entry = out.entry(&opcode.name);
            entry.or_default().push(opcode);
        }
        out
    };

    let mut unimplemented = std::collections::HashSet::new();

    let modes = asm_to_opcode
        .iter()
        .flat_map(|(name, vecs)| std::iter::repeat(*name).zip(vecs.iter()))
        .map(|(asm, &opcode)| {
            (
                syn::Ident::new(
                    &format!("{asm}{}", opcode.mode),
                    proc_macro2::Span::call_site(),
                ),
                opcode,
            )
        })
        .map(|(ident, opcode)| {
            let mode = syn::Ident::new(opcode.mode.to_str(), proc_macro2::Span::call_site());
            quote::quote! {
                Self::#ident => Mode::#mode
            }
        });

    let code_data = walkdir::WalkDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/handler/"))
        .into_iter()
        .map(|f| f.unwrap())
        .filter(|f| f.file_type().is_file())
        .map(|f| {
            (
                f.path().file_stem().unwrap().to_str().unwrap().to_string(),
                std::fs::read_to_string(f.path()).unwrap(),
            )
        })
        .map(|(opcode, code)| (opcode, syn::parse_str::<syn::Expr>(&code).unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    let real_code = asm_to_opcode
        .iter()
        .flat_map(|(name, vecs)| std::iter::repeat(*name).zip(vecs.iter()))
        .map(|(asm, &opcode)| {
            (
                syn::Ident::new(
                    &format!("{asm}{}", opcode.mode),
                    proc_macro2::Span::call_site(),
                ),
                code_data.get(asm),
                opcode,
            )
        })
        .map(|(asm, code, opcode)| {
            (
                asm,
                code.map(|c| quote::quote! {#c}).unwrap_or_else(|| {
                    unimplemented.insert(opcode.name.clone());
                    let message = format!(
                        "Opcode {asm}({code:02X}) not implemented!",
                        asm = opcode.name,
                        code = opcode.code
                    );
                    quote::quote! { {panic!(#message); 0}}
                }),
                opcode,
            )
        })
        .map(|(asm, code, opcode)| {
            let adressing_mode = match opcode.mode {
                Mode::Implied => quote::quote! {cpu.implied(input, bus);},
                Mode::Accumulator => quote::quote! {cpu.accumulator(input, bus);},
                Mode::ZeroPage => quote::quote! {cpu.zero_page(input, bus);},
                Mode::ZeroPageX => quote::quote! {cpu.zero_page_x(input, bus);},
                Mode::ZeroPageY => quote::quote! {cpu.zero_page_y(input, bus);},
                Mode::Absolute => quote::quote! {cpu.absolute(input, bus);},
                Mode::AbsoluteX => quote::quote! {cpu.absolute_x(input, bus);},
                Mode::AbsoluteY => quote::quote! {cpu.absolute_y(input, bus);},
                Mode::Immediate => quote::quote! {cpu.immediate(input, bus);},
                Mode::Indirect => quote::quote! {cpu.indirect(input, bus);},
                Mode::IndirectX => quote::quote! {cpu.indirect_x(input, bus);},
                Mode::IndirectY => quote::quote! {cpu.indirect_y(input, bus);},
                Mode::Relative => quote::quote! {cpu.relative(input, bus);},
            };
            quote::quote! {
                Self::#asm => {
                    #adressing_mode
                    let data = cpu.fetch(bus);
                    #code
                }
            }
        });

    let instruction_definition = opcodes.iter().map(|(code, data)| {
        let name = data.name.as_str();
        let mode = &data.mode;
        let ident = syn::Ident::new(&format!("{name}{mode}"), proc_macro2::Span::call_site());
        quote::quote! {
            #ident = #code,
        }
    });
    let cycles_definition = opcodes.iter().map(|(_, v)| v).map(|data| {
        let name = data.name.as_str();
        let mode = &data.mode;
        let cycles = match data.cycles {
            Cycles::Normal(c) | Cycles::Paged(c) | Cycles::Branched(c) => c,
        };
        let cycle_type = match data.cycles {
            Cycles::Normal(_) => syn::Ident::new("Normal", proc_macro2::Span::call_site()),
            Cycles::Paged(_) => syn::Ident::new("Page", proc_macro2::Span::call_site()),
            Cycles::Branched(_) => syn::Ident::new("Branch", proc_macro2::Span::call_site()),
        };
        let ident = syn::Ident::new(&format!("{name}{mode}"), proc_macro2::Span::call_site());
        quote::quote! {
            Self::#ident => Cycles::#cycle_type(#cycles)
        }
    });
    let bytes_definition = opcodes.iter().map(|(_, v)| v).map(|data| {
        let name = data.name.as_str();
        let mode = &data.mode;
        let bytes = data.bytes;
        let ident = syn::Ident::new(&format!("{name}{mode}"), proc_macro2::Span::call_site());
        quote::quote! {
            Self::#ident => #bytes
        }
    });

    let asm_name_definition = opcodes.iter().map(|(_, v)| v).map(|data| {
        let name = data.name.as_str();
        let mode = &data.mode;
        let ident = syn::Ident::new(&format!("{name}{mode}"), proc_macro2::Span::call_site());
        quote::quote! {
            Self::#ident => #name
        }
    });
    let from_u8 = opcodes.iter().map(|(code, data)| {
        let name = data.name.as_str();
        let mode = &data.mode;
        let ident = syn::Ident::new(&format!("{name}{mode}"), proc_macro2::Span::call_site());

        quote::quote! {
            #code => Ok(Self::#ident)
        }
    });

    let out_code = quote::quote! {
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        #[repr(u8)]
        pub enum Instructions {
            #(#instruction_definition)*
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub enum Cycles {
            Normal(u8),
            Page(u8),
            Branch(u8)
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
                    #(#cycles_definition),*
                }
            }
            pub fn bytes(&self) -> u8 {
                match *self {
                    #(#bytes_definition),*
                }
            }

            pub fn asm_name(&self) -> &'static str {
                match *self {
                    #(#asm_name_definition),*
                }
            }

            pub fn code(&self) -> u8 {
                *self as u8
            }

            pub fn mode(&self) -> Mode {
                match *self {
                    #(#modes),*
                }
            }


            pub fn run(&self, input: [u8;2], cpu: &mut crate::cpu::Cpu, bus: &mut crate::bus::CpuBus){
                let additional_cycles = match *self {
                    #(#real_code),*
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
                    #(#from_u8),*,
                    _ => Err(UnknownOpcode),
                }
            }
        }
    }
    .to_string();

    use std::io::Write;

    let path = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../out/instructions.rs"
    ))
    .canonicalize()
    .unwrap();

    let amount = std::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap()
        .write(out_code.as_bytes())
        .unwrap();
    println!("Unimplemented OPCODES: {:?}", {
        struct AlwaysDisplayString(String);
        impl std::fmt::Display for AlwaysDisplayString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::fmt::Debug for AlwaysDisplayString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        let mut v = unimplemented.into_iter().collect::<Vec<_>>();
        v.sort();
        v.into_iter().map(AlwaysDisplayString).collect::<Vec<_>>()
    });

    println!("Written {amount} to {}", path.display());
}

use derive_try_from_primitive::TryFromPrimitive;
use std::convert::TryFrom;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Type {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Machine {
    X86 = 0x03,
    X86_64 = 0x3e,
}
#[derive(Debug)]
pub struct File {
    // we'll add fields as we go
    pub r#type: Type,
    pub machine: Machine,
}

impl File {
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            bytes::complete::{tag, take},
            combinator::map,
            error::context,
            number::complete::le_u16,
            sequence::tuple,
        };
        let (i, _) = tuple((
            // -------
            context("Magic", tag(Self::MAGIC)),
            context("Class", tag(&[0x2])),
            context("Endianness", tag(&[0x1])),
            context("Version", tag(&[0x1])),
            context("OS ABI", nom::branch::alt((tag(&[0x0]), tag(&[0x3])))),
            // -------
            context("Padding", take(8_usize)),
        ))(i)?;

        let (i, (r#type, machine)) = tuple((
            context("Type", map(le_u16, |x| Type::try_from(x).unwrap())),
            context("Machine", map(le_u16, |x| Machine::try_from(x).unwrap())),
        ))(i)?;

        let res = Self { machine, r#type };

        Ok((i, res))
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use std::convert::TryFrom;

    #[test]
    fn try_enums() {
        assert_eq!(Machine::X86_64 as u16, 0x3E);
        assert_eq!(Machine::try_from(0x3E), Ok(Machine::X86_64));
        assert_eq!(Machine::try_from(0xFA), Err(0xFA));
    }

    #[test]
    fn type_to_u16() {
        assert_eq!(super::Type::Dyn as u16, 0x3);
    }
}

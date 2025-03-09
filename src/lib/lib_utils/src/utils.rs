use std::fmt;

pub fn truncate_hex(f: &mut fmt::Formatter<'_>, data: &[u8]) -> fmt::Result {
    if data.len() > (64 * 64) {
        writeln!(f, "[")?;
        writeln!(f, "\t < {} bytes >", data.len())?;
        write!(f, "]")
    } else {
        writeln!(f, "[")?;
        let mut padded_data = data.to_vec();
        while padded_data.len() % 64 != 0 {
            padded_data.push(0);
        }
        for chunk in padded_data.chunks(64) {
            write!(f,"\t")?;
            for (i, byte) in chunk.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:02X}", byte)?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}
pub fn format_hex(f: &mut fmt::Formatter<'_>, data: &[u8]) -> fmt::Result {
        writeln!(f, "[")?;
        let mut padded_data = data.to_vec();
        while padded_data.len() % 64 != 0 {
            padded_data.push(0);
        }
        for chunk in padded_data.chunks(64) {
            write!(f,"\t")?;
            for (i, byte) in chunk.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:02X}", byte)?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
}

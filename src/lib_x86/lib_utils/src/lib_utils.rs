#![allow(unused, dead_code)]
//todo remove global allow after initial development

use std::fmt;
use std::fmt::Write;

pub fn format_truncated_hex(f: &mut fmt::Formatter<'_>, data: &[u8]) -> fmt::Result {
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
            write!(f, "\t")?;
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

pub fn dump_truncated_hex(data: &[u8]) -> String {
    let mut f = String::new();
    if data.len() > (64 * 64) {
        writeln!(f, "[").expect("shouldnt fail writing");
        writeln!(f, "\t < {} bytes >", data.len()).expect("shouldnt fail writing");
        write!(f, "]").expect("shouldnt fail writing")
    } else {
        writeln!(f, "[").expect("shouldnt fail writing");
        let mut padded_data = data.to_vec();
        while padded_data.len() % 64 != 0 {
            padded_data.push(0);
        }
        for chunk in padded_data.chunks(64) {
            write!(f, "\t").expect("shouldnt fail writing");
            for (i, byte) in chunk.iter().enumerate() {
                if i > 0 {
                    write!(f, " ").expect("shouldnt fail writing");
                }
                write!(f, "{:02X}", byte).expect("shouldnt fail writing");
            }
            writeln!(f).expect("shouldnt fail writing");
        }
        write!(f, "]").expect("shouldnt fail writing")
    }

    f
}

pub fn format_hex(f: &mut fmt::Formatter<'_>, data: &[u8]) -> fmt::Result {
    writeln!(f, "[")?;
    let mut padded_data = data.to_vec();
    while padded_data.len() % 64 != 0 {
        padded_data.push(0);
    }
    for chunk in padded_data.chunks(64) {
        write!(f, "\t")?;
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
pub fn dump_hex(data: &[u8]) -> String {
    let mut f = String::new();
    writeln!(f, "[").expect("shouldnt fail writing");
    let mut padded_data = data.to_vec();
    while padded_data.len() % 64 != 0 {
        padded_data.push(0);
    }
    for chunk in padded_data.chunks(64) {
        write!(f, "\t").expect("shouldnt fail writing");
        for (i, byte) in chunk.iter().enumerate() {
            if i > 0 {
                write!(f, " ").expect("shouldnt fail writing");
            }
            write!(f, "{:02X}", byte).expect("shouldnt fail writing");
        }
        writeln!(f).expect("shouldnt fail writing");
    }
    write!(f, "]").expect("shouldnt fail writing");
    f
}
pub fn dump_hex_unpadded(data: &[u8]) -> String {
    let mut f = String::new();
    println!("here!");
    writeln!(f, "[").expect("shouldnt fail writing");
    for chunk in data.chunks(64) {
        write!(f, "\t").expect("shouldnt fail writing");
        for (i, byte) in chunk.iter().enumerate() {
            if i > 0 {
                write!(f, " ").expect("shouldnt fail writing");
            }
            write!(f, "{:02X}", byte).expect("shouldnt fail writing");
        }
        writeln!(f).expect("shouldnt fail writing");
    }
    write!(f, "]").expect("shouldnt fail writing");
    f
}

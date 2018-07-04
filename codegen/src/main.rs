extern crate xml;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use xml::reader::EventReader;
use xml::reader::XmlEvent;

type Attrs = HashMap<String, String>;

fn main() {
    let file = File::open("ucd.part.xml").unwrap();
    let file = BufReader::new(file);

    let mut out = File::create("out.test").unwrap();

    let parser = EventReader::new(file);
    for e in parser {
        let e = e.unwrap();
        match e {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let mut attrs = HashMap::with_capacity(attributes.len());
                for attribute in attributes {
                    attrs.insert(attribute.name.local_name, attribute.value);
                }

                start_element(name.local_name, attrs, &mut out);
            }
            _ => {}
        }
    }
}

fn start_element(name: String, attrs: Attrs, out: &mut impl Write) {
    match name.as_ref() {
        "char" if attrs.contains_key("cp") => {
            write_char(attrs, out);
        }
        _ => {}
    }
}

#[derive(Debug, PartialEq)]
enum CharOp {
    Hex,
    Str,
    Enum(&'static str),

    Unhandled,
}

fn indent(by: usize) -> String {
    " ".repeat(by * 4)
}

fn write_char(mut attrs: Attrs, out: &mut impl Write) {
    fix_na(&mut attrs);

    let iter = attrs
        .into_iter()
        .map(|(name, value)| {
            let op = get_char_op(&name);
            let name = get_char_name(&name);

            (name, value, op)
        })
        .filter(|(_, _, op)| op != &CharOp::Unhandled);

    writeln!(out, "Codepoint {{").unwrap();
    for (name, value, op) in iter {
        let value = format_op(&op, &value);

        writeln!(out, "{}{}: {}", indent(2), name, value).unwrap();
    }
    writeln!(out, "}}").unwrap();
}

fn fix_na(attrs: &mut Attrs) {
    let cp = attrs.get("cp").unwrap().clone();
    let codepoint = usize::from_str_radix(&cp, 16).unwrap();

    // https://www.unicode.org/versions/latest/ch04.pdf
    enum NamingRule {
        NR1,
        NR2(&'static str),
        NR3,
        NR4,
    }

    let nr = match codepoint {
        0x0Ac00...0x0D7A3 => NamingRule::NR1,
        0x03400...0x04DB5 => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x04E00...0x09FEA => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x20000...0x2A6D6 => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x2A700...0x2B734 => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x2B740...0x2B81D => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x2B820...0x2CEA1 => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x2CEB0...0x2EBE0 => NamingRule::NR2("CJK UNIFIED IDEOGRAPH-"),
        0x17000...0x187EC => NamingRule::NR2("TANGUT IDEOGRAPH-"),
        0x1B170...0x1B2FB => NamingRule::NR2("NUSHU CHARACTER-"),
        0x0F900...0x0FA6D => NamingRule::NR2("CJK COMPATIBILITY IDEOGRAPH-"),
        0x0FA70...0x0FAD9 => NamingRule::NR2("CJK COMPATIBILITY IDEOGRAPH-"),
        0x2F800...0x2FA1D => NamingRule::NR2("CJK COMPATIBILITY IDEOGRAPH-"),
        _ if attrs.get("na").unwrap() != "" => NamingRule::NR3,
        _ => NamingRule::NR4,
    };

    match nr {
        NamingRule::NR1 => {
            // These are actually done for us already in the xml file
        }
        NamingRule::NR2(prefix) => {
            attrs.insert("na".into(), format!("{}{}", prefix, cp));
        }
        NamingRule::NR3 => {}
        NamingRule::NR4 => {
            let na1 = attrs.get("na1").unwrap().clone();
            if &na1 != "" {
                attrs.insert("na".into(), na1);
            };
        }
    };
}

fn get_char_op(name: &str) -> CharOp {
    use CharOp::*;
    match name {
        "cp" => Hex,
        "age" | "na" => Str,
        _ => Unhandled,
    }
}

fn get_char_name(name: &str) -> String {
    match name {
        "cp" => "codepoint",
        "na" => "name",
        "blk" => "block",
        "gc" => "general_category",

        other => other,
    }.to_owned()
}

fn format_op(op: &CharOp, value: &str) -> String {
    use CharOp::*;
    match op {
        Hex => format!("0x{}", value),
        Str => format!("\"{}\"", value),
        Enum(name) => format!("{}::{}", name, get_enum_name(name, value)),
        _ => unreachable!(),
    }
}

fn get_enum_name(name: &str, value: &str) -> String {
    match name {
        "GeneralCategory" => match value {
            "Lu" => "UppercaseLetter",
            "Ll" => "LowercaseLetter",
            "Lt" => "TitlecaseLetter",
            "Lm" => "ModifierLetter",
            "Lo" => "OtherLetter",
            "Mn" => "NonspacingMark",
            "Mc" => "SpacingMark",
            "Me" => "EnclosingMark",
            "Nd" => "DecimalNumber",
            "Nl" => "LetterNumber",
            "No" => "OtherNumber",
            "Pc" => "ConnectorPunctuation",
            "Pd" => "DashPunctuation",
            "Ps" => "OpenPunctuation",
            "Pe" => "ClosePunctuation",
            "Pi" => "InitialPunctuation",
            "Pf" => "FinalPunctuation",
            "Po" => "OtherPunctuation",
            "Sm" => "MathSymbol",
            "Sc" => "CurrencySymbol",
            "Sk" => "ModifierSymbol",
            "So" => "OtherSymbol",
            "Zs" => "SpaceSeparator",
            "Zl" => "LineSeparator",
            "Zp" => "ParagraphSeparator",
            "Cc" => "Control",
            "Cf" => "Format",
            "Cs" => "Surrogate",
            "Co" => "PrivateUse",
            "Cn" => "Unassigned",
            _ => unreachable!(),
        },

        _ => unimplemented!(),
    }.to_owned()
}

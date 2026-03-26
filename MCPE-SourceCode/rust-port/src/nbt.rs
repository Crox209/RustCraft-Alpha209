use std::collections::BTreeMap;

use crate::data_io::{DataInput, DataOutput};

pub const TAG_END: i8 = 0;
pub const TAG_BYTE: i8 = 1;
pub const TAG_SHORT: i8 = 2;
pub const TAG_INT: i8 = 3;
pub const TAG_LONG: i8 = 4;
pub const TAG_FLOAT: i8 = 5;
pub const TAG_DOUBLE: i8 = 6;
pub const TAG_BYTE_ARRAY: i8 = 7;
pub const TAG_STRING: i8 = 8;
pub const TAG_LIST: i8 = 9;
pub const TAG_COMPOUND: i8 = 10;

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List { element_type: i8, items: Vec<TagValue> },
    Compound(BTreeMap<String, TagValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTag {
    pub name: String,
    pub value: TagValue,
}

impl TagValue {
    pub fn id(&self) -> i8 {
        match self {
            TagValue::End => TAG_END,
            TagValue::Byte(_) => TAG_BYTE,
            TagValue::Short(_) => TAG_SHORT,
            TagValue::Int(_) => TAG_INT,
            TagValue::Long(_) => TAG_LONG,
            TagValue::Float(_) => TAG_FLOAT,
            TagValue::Double(_) => TAG_DOUBLE,
            TagValue::ByteArray(_) => TAG_BYTE_ARRAY,
            TagValue::String(_) => TAG_STRING,
            TagValue::List { .. } => TAG_LIST,
            TagValue::Compound(_) => TAG_COMPOUND,
        }
    }
}

pub fn get_tag_name(tag_type: i8) -> &'static str {
    match tag_type {
        TAG_END => "TAG_End",
        TAG_BYTE => "TAG_Byte",
        TAG_SHORT => "TAG_Short",
        TAG_INT => "TAG_Int",
        TAG_LONG => "TAG_Long",
        TAG_FLOAT => "TAG_Float",
        TAG_DOUBLE => "TAG_Double",
        TAG_BYTE_ARRAY => "TAG_Byte_Array",
        TAG_STRING => "TAG_String",
        TAG_LIST => "TAG_List",
        TAG_COMPOUND => "TAG_Compound",
        _ => "UNKNOWN",
    }
}

fn write_payload(out: &mut dyn DataOutput, value: &TagValue) {
    match value {
        TagValue::End => {}
        TagValue::Byte(v) => out.write_byte(*v),
        TagValue::Short(v) => out.write_short(*v),
        TagValue::Int(v) => out.write_int(*v),
        TagValue::Long(v) => out.write_long_long(*v),
        TagValue::Float(v) => out.write_float(*v),
        TagValue::Double(v) => out.write_double(*v),
        TagValue::ByteArray(data) => {
            out.write_int(data.len() as i32);
            out.write_bytes(data);
        }
        TagValue::String(s) => out.write_string(s),
        TagValue::List {
            element_type,
            items,
        } => {
            out.write_byte(*element_type);
            out.write_int(items.len() as i32);
            for item in items {
                write_payload(out, item);
            }
        }
        TagValue::Compound(map) => {
            for (name, value) in map {
                write_named_tag(out, &NamedTag { name: name.clone(), value: value.clone() });
            }
            out.write_byte(TAG_END);
        }
    }
}

fn read_payload(input: &mut dyn DataInput, tag_type: i8) -> Option<TagValue> {
    match tag_type {
        TAG_END => Some(TagValue::End),
        TAG_BYTE => Some(TagValue::Byte(input.read_byte())),
        TAG_SHORT => Some(TagValue::Short(input.read_short())),
        TAG_INT => Some(TagValue::Int(input.read_int())),
        TAG_LONG => Some(TagValue::Long(input.read_long_long())),
        TAG_FLOAT => Some(TagValue::Float(input.read_float())),
        TAG_DOUBLE => Some(TagValue::Double(input.read_double())),
        TAG_BYTE_ARRAY => {
            let len = input.read_int().max(0) as usize;
            Some(TagValue::ByteArray(input.read_bytes(len)))
        }
        TAG_STRING => Some(TagValue::String(input.read_string())),
        TAG_LIST => {
            let element_type = input.read_byte();
            let len = input.read_int().max(0) as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_payload(input, element_type)?);
            }
            Some(TagValue::List { element_type, items })
        }
        TAG_COMPOUND => {
            let mut map = BTreeMap::new();
            loop {
                let tag = read_named_tag(input)?;
                if tag.value.id() == TAG_END {
                    break;
                }
                map.insert(tag.name, tag.value);
            }
            Some(TagValue::Compound(map))
        }
        _ => None,
    }
}

pub fn write_named_tag(out: &mut dyn DataOutput, tag: &NamedTag) {
    out.write_byte(tag.value.id());
    if tag.value.id() == TAG_END {
        return;
    }
    out.write_string(&tag.name);
    write_payload(out, &tag.value);
}

pub fn read_named_tag(input: &mut dyn DataInput) -> Option<NamedTag> {
    let tag_type = input.read_byte();
    if tag_type == TAG_END {
        return Some(NamedTag {
            name: String::new(),
            value: TagValue::End,
        });
    }

    let name = input.read_string();
    let value = read_payload(input, tag_type)?;
    Some(NamedTag { name, value })
}

pub fn write_root_compound(out: &mut dyn DataOutput, name: &str, data: BTreeMap<String, TagValue>) {
    write_named_tag(
        out,
        &NamedTag {
            name: name.to_string(),
            value: TagValue::Compound(data),
        },
    );
}

pub fn read_root_compound(input: &mut dyn DataInput) -> Option<(String, BTreeMap<String, TagValue>)> {
    let root = read_named_tag(input)?;
    if let TagValue::Compound(map) = root.value {
        Some((root.name, map))
    } else {
        None
    }
}


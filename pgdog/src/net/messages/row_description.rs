//! RowDescription (B) message.

use crate::net::c_string_buf;

use super::code;
use super::prelude::*;

/// Column field description.
#[derive(Clone, Debug)]
pub struct Field {
    /// Name of the field.
    pub name: String,
    /// Table OID.
    pub table_oid: i32,
    /// Column number.
    pub column: i16,
    /// Type OID.
    pub type_oid: i32,
    /// Type size.
    pub type_size: i16,
    /// Type modifier.
    pub type_modifier: i32,
    /// Format code.
    pub format: i16,
}

impl Field {
    /// Numeric field.
    pub fn numeric(name: &str) -> Self {
        Self {
            name: name.into(),
            table_oid: 0,
            column: 0,
            type_oid: 1700,
            type_size: -1,
            type_modifier: -1,
            format: 0, // We always use text format.
        }
    }

    /// Text field.
    pub fn text(name: &str) -> Self {
        Self {
            name: name.into(),
            table_oid: 0,
            column: 0,
            type_oid: 25,
            type_size: -1,
            type_modifier: -1,
            format: 0, // We always use text format.
        }
    }

    /// Boolean field.
    pub fn bool(name: &str) -> Self {
        Self {
            name: name.into(),
            table_oid: 0,
            column: 0,
            type_oid: 16,
            type_size: 1,
            type_modifier: -1,
            format: 0, // We always use text format.
        }
    }
}

/// RowDescription message.
#[derive(Debug, Clone)]
pub struct RowDescription {
    /// Fields.
    fields: Vec<Field>,
}

impl RowDescription {
    pub fn new(fields: &[Field]) -> Self {
        Self {
            fields: fields.to_vec(),
        }
    }
}

impl FromBytes for RowDescription {
    fn from_bytes(mut bytes: Bytes) -> Result<Self, Error> {
        code!(bytes, 'T');
        let _len = bytes.get_i32();

        let fields = (0..bytes.get_i16())
            .map(|_| Field {
                name: c_string_buf(&mut bytes),
                table_oid: bytes.get_i32(),
                column: bytes.get_i16(),
                type_oid: bytes.get_i32(),
                type_size: bytes.get_i16(),
                type_modifier: bytes.get_i32(),
                format: bytes.get_i16(),
            })
            .collect();

        Ok(Self { fields })
    }
}

impl ToBytes for RowDescription {
    fn to_bytes(&self) -> Result<Bytes, Error> {
        let mut payload = Payload::named(self.code());
        payload.put_i16(self.fields.len() as i16);

        for field in &self.fields {
            payload.put_string(&field.name);
            payload.put_i32(field.table_oid);
            payload.put_i16(field.column);
            payload.put_i32(field.type_oid);
            payload.put_i16(field.type_size);
            payload.put_i32(field.type_modifier);
            payload.put_i16(field.format);
        }

        Ok(payload.freeze())
    }
}

impl Protocol for RowDescription {
    fn code(&self) -> char {
        'T'
    }
}

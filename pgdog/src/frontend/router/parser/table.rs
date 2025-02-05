use pg_query::{protobuf::*, NodeEnum};

/// Table name in a query.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Table<'a> {
    /// Table name.
    pub name: &'a str,
    /// Schema name, if specified.
    pub schema: Option<&'a str>,
}

impl<'a> TryFrom<&'a Node> for Table<'a> {
    type Error = ();

    fn try_from(value: &'a Node) -> Result<Self, Self::Error> {
        if let Some(NodeEnum::RangeVar(range_var)) = &value.node {
            return Ok(range_var.into());
        }

        Err(())
    }
}

impl<'a> From<&'a RangeVar> for Table<'a> {
    fn from(range_var: &'a RangeVar) -> Self {
        let name = if let Some(ref alias) = range_var.alias {
            alias.aliasname.as_str()
        } else {
            range_var.relname.as_str()
        };
        Self {
            name,
            schema: if !range_var.schemaname.is_empty() {
                Some(range_var.schemaname.as_str())
            } else {
                None
            },
        }
    }
}

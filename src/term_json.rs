use serde::ser::{Serialize, Serializer, SerializeMap};

#[derive(Serialize)]
#[serde(untagged)]
pub enum JSONValue<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    Null,
    String(&'a str),
}

pub enum JSONKey {
    Hole,
    Id,
    Value,
    Variable,
    Wildcard,
    Word,
}

impl JSONKey {
    fn name(&self) -> &'static str {
        use self::JSONKey::*;
        match *self {
            Hole => "hole",
            Id => "id",
            Value => "value",
            Variable => "variable",
            Wildcard => "wildcard",
            Word => "word"
        }
    }
}

/// A struct that gets serde serialized into a map
/// with a key single key value pair as defined by
/// its fields.
pub struct ReprJSON<'a>(JSONKey, JSONValue<'a>);

impl <'a> ReprJSON<'a> {

    #[inline(always)]
    pub fn bool_value<'b>(b: bool) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Value, JSONValue::Bool(b))
    }

    #[inline(always)]
    pub fn float_value<'b>(f: f64) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Value, JSONValue::Float(f))
    }

    #[inline(always)]
    pub fn int_value<'b>(i: i64) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Value, JSONValue::Int(i))
    }

    #[inline(always)]
    pub fn null_value<'b>() -> ReprJSON<'b> {
        ReprJSON(JSONKey::Value, JSONValue::Null)
    }

    #[inline(always)]
    pub fn string_value<'b>(s: &'b str) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Value, JSONValue::String(s))
    }

    #[inline(always)]
    pub fn hole<'b>() -> ReprJSON<'b> {
        ReprJSON(JSONKey::Hole, JSONValue::Bool(true))
    }

    #[inline(always)]
    pub fn id<'b>(id: &'b str) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Id, JSONValue::String(id))
    }

    #[inline(always)]
    pub fn variable<'b>(v: &'b str) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Variable, JSONValue::String(v))
    }

    #[inline(always)]
    pub fn word<'b>(word: &'b str) -> ReprJSON<'b> {
        ReprJSON(JSONKey::Word, JSONValue::String(word))
    }

    #[inline(always)]
    pub fn wildcard<'b>() -> ReprJSON<'b> {
        ReprJSON(JSONKey::Wildcard, JSONValue::Bool(true))
    }

}

impl <'a> Serialize for ReprJSON<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.0.name(), &self.1)?;
        map.end()
    }
}

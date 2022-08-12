use prost_types::{
    value::Kind as ProtoKind, ListValue as ProtoList, Struct as ProtoStruct, Value as ProtoValue,
};
use serde_json::{Number as JsonNumber, Value as JsonValue};

#[doc(hidden)]
#[macro_export]
macro_rules! precondition {
    ($cond:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err(Status::failed_precondition(format!($($arg)*)));
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! precondition_fn {
    ($($arg:tt)*) => {
        || Status::failed_precondition(format!($($arg)*))
    };
}

pub fn json_to_proto(value: JsonValue) -> ProtoValue {
    ProtoValue {
        kind: Some(json_to_kind(value)),
    }
}

fn json_to_kind(value: JsonValue) -> ProtoKind {
    match value {
        JsonValue::Null => ProtoKind::NullValue(0),
        JsonValue::Bool(b) => ProtoKind::BoolValue(b),
        JsonValue::Number(n) => ProtoKind::NumberValue(
            n.as_f64()
                .or_else(|| n.as_i64().map(|i| i as _))
                .or_else(|| n.as_u64().map(|u| u as _))
                .unwrap(),
        ),
        JsonValue::String(s) => ProtoKind::StringValue(s),
        JsonValue::Array(a) => ProtoKind::ListValue(ProtoList {
            values: a.into_iter().map(json_to_proto).collect(),
        }),
        JsonValue::Object(o) => ProtoKind::StructValue(ProtoStruct {
            fields: o.into_iter().map(|(k, v)| (k, json_to_proto(v))).collect(),
        }),
    }
}

pub fn proto_to_json(value: ProtoValue) -> Option<JsonValue> {
    value.kind.map(kind_to_json)
}

fn kind_to_json(value: ProtoKind) -> JsonValue {
    match value {
        ProtoKind::NullValue(_) => JsonValue::Null,
        ProtoKind::BoolValue(b) => JsonValue::Bool(b),
        ProtoKind::NumberValue(n) => JsonValue::Number(JsonNumber::from_f64(n).unwrap()),
        ProtoKind::StringValue(s) => JsonValue::String(s),
        ProtoKind::ListValue(a) => {
            JsonValue::Array(a.values.into_iter().filter_map(proto_to_json).collect())
        }
        ProtoKind::StructValue(o) => JsonValue::Object(
            o.fields
                .into_iter()
                .filter_map(|(k, v)| proto_to_json(v).map(|v| (k, v)))
                .collect(),
        ),
    }
}

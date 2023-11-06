#[derive(Debug)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    String(JsonString),
    Number(JsonNumber),
    Array(JsonArray),
    Object(JsonObject),
}

#[derive(Debug)]
pub struct JsonString {
    pub text: String,
    pub value: String,
}

#[derive(Debug)]
pub struct JsonNumber {
    pub text: String,
    pub value: f64,
}

#[derive(Debug)]
pub struct JsonArray {
    pub elements: Vec<JsonValue>,
}

#[derive(Debug)]
pub struct JsonObject {
    pub properties: Vec<JsonProperty>,
}

#[derive(Debug)]
pub struct JsonProperty {
    pub name: String,
    pub value: JsonValue
}
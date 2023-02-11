use std::fmt::{format, Debug, Display};

const BODY: &str = "---BODY---";
const HEAD_VARIANT: &str = "variant";
const HEAD_VARIANT_TYPE_CODE: &str = "code";
const HEAD_VARIANT_TYPE_COMMAND: &str = "command";
const HEAD_VARIANT_TYPE_ANSWER: &str = "answer";
const HEAD_VARIANT_TYPE_SYSTEM: &str = "system";
const HEAD_SEPARATOR: &str = ":";

#[derive(Debug, PartialEq)]
pub enum PayloadType {
    Code,    // Multiline of command (full code)
    Command, // One line of code
    Answer,  // Answer of code/command executor in runtime
    System,  // Answer of some system info
}

pub struct Payload {
    variant: PayloadType,
    body: String,
    raw: String,
}

impl Payload {
    pub fn new(value: String) -> Payload {
        let (head, body) = value.split_once(BODY).unwrap_or(("", value.as_str()));

        Payload {
            variant: Self::get_variant(head),
            body: body.to_string(),
            raw: value,
        }
    }

    fn get_variant(head: &str) -> PayloadType {
        let keyword = format!("{}{}", HEAD_VARIANT, HEAD_SEPARATOR);
        let word: String = match head.lines().find(|val| val.contains(keyword.as_str())) {
            Some(val) => val.split(keyword.as_str()).collect(),
            None => return PayloadType::Code,
        };

        match word.trim() {
            HEAD_VARIANT_TYPE_CODE => PayloadType::Code,
            HEAD_VARIANT_TYPE_COMMAND => PayloadType::Command,
            HEAD_VARIANT_TYPE_ANSWER => PayloadType::Answer,
            HEAD_VARIANT_TYPE_SYSTEM => PayloadType::System,
            _ => PayloadType::Code,
        }
    }
}

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Payload")
            .field("variant", &self.variant)
            .field("body", &self.body)
            .finish()
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nvariant:  {:?} \nbody: {}", self.variant, self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::Payload;
    use super::PayloadType;
    #[test]
    fn empty_head() {
        let code = r#"
            echo ""
            echo "Test"
        "#;

        let ob = Payload::new(code.to_string());

        assert_eq!(ob.variant, PayloadType::Code);
        assert_eq!(ob.body, code);
        assert_eq!(ob.raw, code);
    }
}

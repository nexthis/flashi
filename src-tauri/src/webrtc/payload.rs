use std::fmt::{format, Debug, Display};

const BODY: &str = "---BODY---";
const HEAD_VARIANT: &str = "variant";
const HEAD_VARIANT_TYPE_CODE: &str = "code";
const HEAD_VARIANT_TYPE_COMMAND: &str = "command";
const HEAD_VARIANT_TYPE_ANSWER: &str = "answer";
const HEAD_VARIANT_TYPE_SYSTEM: &str = "system";
const HEAD_SEPARATOR: &str = ":";

#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn body(&self) -> String {
        return self.body.clone();
    }

    pub fn variant(&self) -> PayloadType {
        return self.variant;
    }

    pub fn raw(&self) -> String {
        return self.raw.clone();
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
    use super::HEAD_VARIANT_TYPE_ANSWER;
    use super::HEAD_VARIANT_TYPE_CODE;
    use super::HEAD_VARIANT_TYPE_COMMAND;
    use super::HEAD_VARIANT_TYPE_SYSTEM;

    #[test]
    fn basic() {
        let code = r#"
            variant: code
            ---BODY---
            echo ""
            echo "Test"
        "#;

        let ob = Payload::new(code.to_string());

        assert_eq!(ob.variant, PayloadType::Code);
        assert_eq!(ob.raw, code);
    }

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

    #[test]
    fn heads() {
        let types = [
            HEAD_VARIANT_TYPE_CODE,
            HEAD_VARIANT_TYPE_COMMAND,
            HEAD_VARIANT_TYPE_ANSWER,
            HEAD_VARIANT_TYPE_SYSTEM,
        ];

        let variant = [
            PayloadType::Code,
            PayloadType::Command,
            PayloadType::Answer,
            PayloadType::System,
        ];

        assert_eq!(
            types.len(),
            variant.len(),
            "types and variant must have the same length FIX THE TESTS"
        );

        for index in 0..types.len() {
            let head = format!("variant: {} ", types[index]);
            let code = r#"
                echo ""
                echo "Test"
            "#;

            let ob = Payload::new(format!("{} \n ---BODY--- \n {}", head, code));

            assert_eq!(ob.variant, variant[index]);
            assert_eq!(ob.body.trim(), code.trim());
        }
    }
}

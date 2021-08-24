use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ name | pascal_case }}Config {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{{ name | snake_case }}_config() {

    }
}

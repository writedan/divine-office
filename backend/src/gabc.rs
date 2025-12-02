use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GabcFile {
    headers: HashMap<String, String>,
    body: String,
}

impl GabcFile {
    /// Parse a GABC file from a string
    pub fn new(content: &str) -> Result<Self, String> {
        let mut headers = HashMap::new();
        let mut lines = content.lines();
        let mut body_lines = Vec::new();
        let mut in_body = false;

        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.is_empty() && !in_body {
                in_body = true;
                continue;
            }

            if !in_body {
                if let Some(colon_pos) = trimmed.find(':') {
                    let key = trimmed[..colon_pos].trim().to_string();
                    let value = trimmed[colon_pos + 1..].trim().to_string();
                    headers.insert(key, value);
                }
            } else {
                body_lines.push(line);
            }
        }

        let body = body_lines.join("\n");

        Ok(GabcFile { headers, body })
    }

    /// Get a header attribute by key
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    /// Set a header attribute
    pub fn set_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into(), value.into());
    }

    /// Remove a header attribute
    pub fn remove_header(&mut self, key: &str) -> Option<String> {
        self.headers.remove(key)
    }

    /// Get all headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get the raw body (notation and text combined)
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Extract raw text from the GABC notation, preserving spaces
    pub fn extract_text(&self) -> String {
        let mut result = String::new();
        let mut in_notation = false;
        let mut paren_content = String::new();

        for ch in self.body.chars() {
            match ch {
                '(' => {
                    in_notation = true;
                    paren_content.clear();
                }
                ')' => {
                    let is_clef = paren_content.chars().all(|c| {
                        c.is_ascii_digit() || matches!(c, 'c' | 'f' | 'b')
                    }) && !paren_content.is_empty();
                    
                    if !is_clef && paren_content.is_empty() {
                        result.push_str("()");
                    }
                    
                    in_notation = false;
                    paren_content.clear();
                }
                _ if in_notation => {
                    paren_content.push(ch);
                }
                _ => {
                    result.push(ch);
                }
            }
        }

        result.trim().to_string()
    }

    /// Serialize the GABC file back to a string
    pub fn to_string(&self) -> String {
        let mut output = String::new();

        let mut keys: Vec<_> = self.headers.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = self.headers.get(key) {
                output.push_str(&format!("{}: {}\n", key, value));
            }
        }

        output.push('\n');

        output.push_str(&self.body);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gabc() {
        let gabc_content = r#"name: Kyrie XVI
office-part: Kyrie
mode: 1
book: Graduale Romanum

(c4) KY(f)ri(gfg)e(h.) *() e(ixjvIH'GhvF'E)lé(ghg')i(g)son.(f.) <i>bis</i>(::)"#;

        let gabc = GabcFile::new(gabc_content).unwrap();

        assert_eq!(gabc.get_header("name"), Some("Kyrie XVI"));
        assert_eq!(gabc.get_header("mode"), Some("1"));
        assert_eq!(gabc.get_header("book"), Some("Graduale Romanum"));
    }

    #[test]
    fn test_extract_text() {
        let gabc_content = r#"name: Test

(c4) KY(f)ri(gfg)e(h.) *() e(ixjvIH'GhvF'E)lé(ghg')i(g)son.(f.) <i>bis</i>(::)"#;

        let gabc = GabcFile::new(gabc_content).unwrap();
        let text = gabc.extract_text();

        assert_eq!(text, "KYrie *() eléison. <i>bis</i>");
    }

    #[test]
    fn test_set_header() {
        let gabc_content = r#"name: Original
mode: 1

(c4) Test(f)"#;

        let mut gabc = GabcFile::new(gabc_content).unwrap();
        gabc.set_header("name", "Modified");
        gabc.set_header("annotation", "New annotation");

        assert_eq!(gabc.get_header("name"), Some("Modified"));
        assert_eq!(gabc.get_header("annotation"), Some("New annotation"));
        assert_eq!(gabc.get_header("mode"), Some("1"));
    }

    #[test]
    fn test_preserve_spaces() {
        let gabc_content = r#"name: Test

(c4) First(f)  word(g)   with(h)    spaces(i)"#;

        let gabc = GabcFile::new(gabc_content).unwrap();
        let text = gabc.extract_text();

        assert_eq!(text, "First  word   with    spaces");
    }

    #[test]
    fn test_to_string() {
        let gabc_content = r#"name: Test
mode: 1

(c4) Test(f)"#;

        let gabc = GabcFile::new(gabc_content).unwrap();
        let serialized = gabc.to_string();

        assert!(serialized.contains("mode: 1"));
        assert!(serialized.contains("name: Test"));
        assert!(serialized.contains("(c4) Test(f)"));
    }
}
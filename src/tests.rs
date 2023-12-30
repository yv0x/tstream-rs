use crate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_tokenize() {
        // Basic tokenization
        let input = "This is a test";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["This", "is", "a", "test"]);
        assert_eq!(tokens.len(), 4);

        // Multiple spaces
        let input = "    Multiple    spaces    ";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["Multiple", "spaces"]);
        assert_eq!(tokens.len(), 2);

        // Empty string
        let input = "";
        let tokens = tokenize(input);
        assert_eq!(tokens, Vec::<String>::new());
        assert_eq!(tokens.len(), 0);

        // String with only spaces
        let input = "     ";
        let tokens = tokenize(input);
        assert_eq!(tokens, Vec::<String>::new());
        assert_eq!(tokens.len(), 0);

        // Special characters
        let input = "!@#$ %^& *()";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["!@#$", "%^&", "*()"]);
        assert_eq!(tokens.len(), 3);

        // Mixed spaces and tabs
        let input = "Tabs\tand  spaces";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["Tabs", "and", "spaces"]);
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_prompts_toml_exists_and_populated() {
        let file_contents = fs::read_to_string("openai_prompts.toml").expect("Could read the file");

        let prompts: Prompts =
            toml::from_str(&file_contents).expect("Could not deserialize the prompts");

        for (i, option) in prompts.options.iter().enumerate() {
            assert!(option.model.is_some(), "No model defined for option {}", i);
            assert!(
                option.long.is_some() || option.code.is_some() || option.standard.is_some(),
                "No prompt available for option {}",
                i
            );
        }
    }

    #[test]
    fn test_mistral_prompts_toml_exists_and_populated() {
        let file_contents =
            fs::read_to_string("mistral_prompts.toml").expect("Could read the file");

        let prompts: Prompts =
            toml::from_str(&file_contents).expect("Could not deserialize the prompts");

        for (i, option) in prompts.options.iter().enumerate() {
            assert!(option.model.is_some(), "No model defined for option {}", i);
            assert!(
                option.long.is_some() || option.code.is_some() || option.standard.is_some(),
                "No prompt available for option {}",
                i
            );
        }
    }
}

pub(crate) fn quote_parser(args: &String) -> Vec<String> {
    // parse the args with quotes into a vector of strings

    // string builder pattern
    let mut output = Vec::<String>::new();

    // strip the command part of the string
    let input = args.split(" ").collect::<Vec<_>>()[1..]
        .join(" ")
        .replace("''", "")
        .replace("\"\"", "")
        .to_string();

    // consume the input string and split into snippets
    let mut snippet = String::new();
    let mut chars_iter = input.chars();

    while let Some(char) = chars_iter.next() {
        if char == '\\' {
            // push the next char and advance
            let next_char = chars_iter.next();
            if let Some(next_char) = next_char {
                snippet.push(next_char);
            }
        } else if char == '\'' {
            // advance to the next quote
            // and capture the snippet
            while let Some(c) = chars_iter.next() {
                if c == '\'' {
                    break;
                }
                snippet.push(c);
            }

            // push the snippet to the output and reset.
            if !snippet.trim().is_empty() {
                output.push(snippet.trim().to_string());
                snippet = String::new();
            }
        } else if char == '"' {
            // advance to the next quote
            // and capture the snippet
            while let Some(c) = chars_iter.next() {
                if c == '"' {
                    // peek ahead to see if next char is whitespace
                    if chars_iter
                        .clone()
                        .peekable()
                        .peek()
                        .is_some_and(|c| c.is_whitespace())
                    {
                        break;
                    } else {
                        // chars_iter.next();
                        // break;
                    }
                } else {
                    snippet.push(c);
                }
            }

            // push the snippet to the output and reset.
            if !snippet.trim().is_empty() {
                output.push(snippet.to_string());
                snippet = String::new();
            }
        } else if char.is_whitespace() {
            input.chars().next();
            if !snippet.trim().is_empty() {
                output.push(snippet.to_string());
                snippet = String::new();
            }
        } else {
            snippet.push(char);
        }
    }

    // push the last snippet
    if !snippet.trim().is_empty() {
        output.push(snippet.to_string());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_quotes() {
        let test_strings = vec![
            ("echo 'hello world'".to_string(), "hello world".to_string()),
            (
                "echo 'hello         world'".to_string(),
                "hello         world".to_string(),
            ),
            (
                "echo hello         world".to_string(),
                "hello world".to_string(),
            ),
            ("echo 'hello''world'".to_string(), "helloworld".to_string()),
            ("cat 'test file'".to_string(), "test file".to_string()),
        ];

        for (input, expected) in test_strings {
            assert_eq!(quote_parser(&input).join(" "), expected);
        }
    }

    #[test]
    fn test_double_quotes() {
        let test_strings = vec![
            (
                "echo \"hello      world\"".to_string(),
                "hello      world".to_string(),
            ),
            (
                "echo \"hello\"\"world\"".to_string(),
                "helloworld".to_string(),
            ),
            ("echo \"hello\"world".to_string(), "helloworld".to_string()),
            (
                "echo \"hello\" \"world\"".to_string(),
                "hello world".to_string(),
            ),
            (
                "echo \"Shell's test\"".to_string(),
                "Shell's test".to_string(),
            ),
        ];

        for (input, expected) in test_strings {
            assert_eq!(quote_parser(&input).join(" "), expected);
        }
    }

    #[test]
    fn test_escape_strings() {
        let test_strings = vec![
            (
                "echo three\\ \\ \\ spaces".to_string(),
                "three   spaces".to_string(),
            ),
            (
                "echo before\\  after".to_string(),
                "before  after".to_string(),
            ),
            (
                "echo test\\nexample".to_string(),
                "testnexample".to_string(),
            ),
            (
                "echo hello\\\\world".to_string(),
                "hello\\world".to_string(),
            ),
            ("echo \\'hello\\'".to_string(), "'hello'".to_string()),
        ];

        for (idx, (input, expected)) in test_strings.iter().enumerate() {
            println!(
                "{:<2} {:<23} -> [{}]\t-> {}",
                idx,
                input,
                quote_parser(&input).join(", "),
                quote_parser(&input).join(" ")
            );
            assert_eq!(quote_parser(&input).join(" "), *expected);
        }
    }
}

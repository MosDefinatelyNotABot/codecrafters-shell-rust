static SPECIAL_CHARS: &[char] = &['\"', '\\', '$', '`', '\n'];

pub(crate) fn term_tokenizer(args: &str) -> (String, Vec<String>) {
    // parse the args with quotes into a vector of strings
    // extract the command part of the string

    // strip the command part of the string
    let input = args.replace("''", "").replace("\"\"", "").to_string();

    // string builder pattern
    let mut output = Vec::<String>::new();

    // consume the input string and split into snippets
    let mut snippet = String::new();
    let mut chars_iter = input.chars();

    while let Some(char) = chars_iter.next() {
        if char == '\\' {
            // push the next char and advance
            let c = chars_iter.next();
            if let Some(c) = c {
                snippet.push(c);
            }
        } else if char == '\'' {
            // advance to the next quote
            // and capture the snippet
            for c in chars_iter.by_ref() {
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
                if c == '\\' {
                    if let Some(next_char) = chars_iter.next()
                        && SPECIAL_CHARS.contains(&next_char)
                    {
                        snippet.push(next_char);
                    }
                } else if c == '"' {
                    // peek ahead to see if next char is whitespace
                    if chars_iter
                        .clone()
                        .peekable()
                        .peek()
                        .is_some_and(|c| c.is_whitespace())
                    {
                        break;
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

    (output[0].to_string(), output[1..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> (String, String) {
        let (cmd, args) = term_tokenizer(&input);
        (cmd, args.join(" "))
    }

    #[test]
    fn test_single_quotes() {
        let cases = [
            ("echo 'hello world'", "echo", "hello world"),
            ("echo 'hello         world'", "echo", "hello         world"),
            ("echo hello         world", "echo", "hello world"),
            ("echo 'hello''world'", "echo", "helloworld"),
            ("cat 'test file'", "cat", "test file"),
        ];

        for (input, expected_cmd, expected_args) in cases {
            let (cmd, args) = parse(input);
            assert_eq!(
                (cmd.as_str(), args.as_str()),
                (expected_cmd, expected_args),
                "input: {input}"
            );
        }
    }

    #[test]
    fn test_double_quotes() {
        let cases = [
            ("echo \"hello      world\"", "echo", "hello      world"),
            ("echo \"hello\"\"world\"", "echo", "helloworld"),
            ("echo \"hello\"world", "echo", "helloworld"),
            ("echo \"hello\" \"world\"", "echo", "hello world"),
            ("echo \"Shell's test\"", "echo", "Shell's test"),
        ];

        for (input, expected_cmd, expected_args) in cases {
            let (cmd, args) = parse(input);
            assert_eq!(
                (cmd.as_str(), args.as_str()),
                (expected_cmd, expected_args),
                "input: {input}"
            );
        }
    }

    #[test]
    fn test_escape_strings() {
        let cases = [
            ("echo three\\ \\ \\ spaces", "echo", "three   spaces"),
            ("echo before\\  after", "echo", "before  after"),
            ("echo test\\nexample", "echo", "testnexample"),
            ("echo hello\\\\world", "echo", "hello\\world"),
            ("echo \\'hello\\'", "echo", "'hello'"),
        ];

        for (input, expected_cmd, expected_args) in cases {
            let (cmd, args) = parse(input);
            assert_eq!(
                (cmd.as_str(), args.as_str()),
                (expected_cmd, expected_args),
                "input: {input}"
            );
        }
    }

    #[test]
    fn test_literal_special_chars_in_double_quotes() {
        let cases = [
            (
                "echo \"A \\\\ escapes itself\"",
                "echo",
                "A \\ escapes itself",
            ),
            (
                "echo \"A \\\" inside double quotes\"",
                "echo",
                "A \" inside double quotes",
            ),
            (
                "echo \"just\'one\'\\\\n'backslash\"",
                "echo",
                "just\'one\'\\n'backslash",
            ),
            (
                "echo \"inside\\\"literal_quote.\"outside\\\"",
                "echo",
                "inside\"literal_quote.outside\"",
            ),
        ];

        for (input, expected_cmd, expected_args) in cases {
            let (cmd, args) = parse(input);
            assert_eq!(
                (cmd.as_str(), args.as_str()),
                (expected_cmd, expected_args),
                "input: {input}"
            );
        }
    }

    #[test]
    fn test_quoted_executable_names() {
        let cases = [
            ("'my program' argument1", "my program", "argument1"),
            (
                "\"exe with spaces\" file.txt",
                "exe with spaces",
                "file.txt",
            ),
        ];

        for (input, expected_cmd, expected_args) in cases {
            let (cmd, args) = parse(input);
            assert_eq!(
                (cmd.as_str(), args.as_str()),
                (expected_cmd, expected_args),
                "input: {input}"
            );
        }
    }
}

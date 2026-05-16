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
        // println!("'{}'", char);
        if char == '\'' {
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
                output.push(snippet.trim().to_string());
                snippet = String::new();
            }
        } else if char.is_whitespace() {
            input.chars().next();
            // println!("{}", snippet);
            if !snippet.trim().is_empty() {
                output.push(snippet.trim().to_string());
                snippet = String::new();
            }
        } else {
            snippet.push(char);
        }
    }

    // push the last snippet
    if !snippet.trim().is_empty() {
        output.push(snippet.trim().to_string());
    }

    output
}

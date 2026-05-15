// use regex::Regex;

pub(crate) fn parse_echo(args: &String) -> String {
    // string builder pattern
    let mut output = String::new();

    // strip the command string
    let input = args
        .strip_prefix("echo ")
        .expect("args must start with 'echo '")
        .to_string();

    // consume the input string and as snippets and append to output

    let mut idx = 0;

    while idx < input.len() {
        if input.chars().nth(idx).unwrap() == '\'' {
            // find the next quote capture the text between quotes and advance
            let mut quote_idx = idx + 1;
            while quote_idx < input.len() && input.chars().nth(quote_idx).unwrap() != '\'' {
                output.push(input.chars().nth(quote_idx).unwrap());
                quote_idx += 1;
            }
            idx = quote_idx;
        } else if input.chars().nth(idx).unwrap() == ' ' {
            // advance to the next character
            while idx < input.len() && input.chars().nth(idx).unwrap() == ' ' {
                idx += 1;
            }
            idx -= 1;
            output.push(' ');
        } else {
            output.push(input.chars().nth(idx).unwrap());
        }

        idx += 1;
    }

    output
}

/// Parses the command line input into a list of arguments.
pub fn parse_arguments(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_arg = false;
    let mut escaped = false;
    let mut dq_escaped = false;

    for ch in input.chars(){
        //For escaping :
        if escaped{
            current.push(ch);
            in_arg = true;
            escaped = false;
            continue;
        }
        //For single quote:
        if in_single_quote{
            if ch =='\'' {
                in_single_quote = false;
            }else{
                current.push(ch);
            }
        }else if in_double_quote{//For double quote:
            if dq_escaped{
                match ch {
                    '"' => current.push('"'),
                    '\\' => current.push('\\'),
                    _ => {current.push('\\');current.push(ch);}
                }
                dq_escaped = false; 
            }else if ch == '\\'{
                dq_escaped = true;
            }else if ch == '"'{
                in_double_quote = false;
            }else{
                current.push(ch);
            }
        }else{
            match ch{
                '\\' => {
                    escaped = true;
                    in_arg = true;
                }
                ' ' | '\t' => {
                    if in_arg {
                        args.push(current.clone());
                        current.clear();
                        in_arg = false;
                    }
                }
                '\'' => {
                    in_single_quote = true;
                    in_arg = true;
                }
                '"' => {
                    in_double_quote = true;
                    in_arg = true;
                }
                _ => {
                    current.push(ch);
                    in_arg = true;
                }
            }
        }
    }
    if in_arg{
        args.push(current);
    }
    args
}

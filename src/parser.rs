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

//for the pipelines:
pub fn parse_redirections(args: &[String]) -> (Vec<String>, Option<(String, bool)>, Option<(String, bool)>) {
    let mut clean = Vec::new();
    let mut stdout_file = None;
    let mut stderr_file = None;
    let mut i = 0;
    while i < args.len(){
        if (args[i] == ">" || args[i] == "1>") && i + 1 < args.len(){
            stdout_file = Some((args[i+1].clone(), false));
            i+=2;
        }else if (args[i] == ">>" || args[i] == "1>>") && i + 1 < args.len(){
            stdout_file = Some((args[i+1].clone(),true));
            i+=2;
        }else if args[i] == "2>" && i + 1 < args.len(){
            stderr_file = Some((args[i+1].clone(),false));
            i+=2;
        }else if args[i] == "2>>" && i + 1 < args.len(){
            stderr_file = Some((args[i+1].clone(),true));
            i+=2;
        }else{
            clean.push(args[i].clone());
            i+=1;
        }
    }
    (clean, stdout_file, stderr_file)
}

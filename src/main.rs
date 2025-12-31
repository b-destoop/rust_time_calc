use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The expression of time to calculate
    time_expression: Option<String>,
    #[clap(short, long, default_value_t = false)]
    /// values given are minutes, not seconds
    minutes: bool,
}
// TODO: Perform calculation from file input

fn main() {
    let args = Cli::parse();
    let expression = args.time_expression.as_deref().unwrap_or("");
    let minutes_flag = args.minutes;
    println!("expression: {}", expression);

    let mut symbols_infix = parse_symbols(expression);
    if minutes_flag {
        symbols_infix = inflate_values(symbols_infix, 60);
    }
    let symbols_postfix = to_postfix(&symbols_infix);
    let result_seconds = evaluate_postfix(&symbols_postfix);
    let result = seconds_to_time(result_seconds);

    // println!("symbols infix: {:?}", symbols_infix);
    // println!("symbols postfix: {:?}", symbols_postfix);
    // println!("Result: {}", result_seconds);
    // println!("Real result: {}", result);
    println!("Result: {}", result)
}

fn inflate_values(symbols: Vec<String>, factor: u32) -> Vec<String> {
    let mut inflated_symbols: Vec<String> = Vec::new();
    for s in symbols {
        let number = s.parse::<u32>();
        if number.is_ok() {
            let new_val = number.unwrap() * factor;
            inflated_symbols.push(new_val.to_string());
        } else {
            inflated_symbols.push(s);
        }
    }

    inflated_symbols
}

fn evaluate_postfix(symbols_postfix: &[String]) -> u32 {
    let mut result_stack: Vec<u32> = Vec::new();

    for s in symbols_postfix {
        match s {
            s if s.len() == 1 && is_operator(s.chars().next().unwrap()) => {
                let b = result_stack.pop().unwrap();
                let a = result_stack.pop().unwrap();
                let operator = s.chars().next().unwrap();
                result_stack.push(perform_operation(operator, a, b));
            }
            _ => {
                result_stack.push(s.parse::<u32>().unwrap());
            }
        }
    }

    result_stack.pop().unwrap()
}

fn perform_operation(operator: char, a: u32, b: u32) -> u32 {
    match operator {
        '+' => {
            return a + b;
        }
        '-' => {
            return a - b;
        }
        '*' => {
            return a * b;
        }
        '/' => {
            return a / b;
        }
        '^' => {
            return a.pow(b as u32);
        }
        _ => {
            println!("Cannot perform operation with operator {}", operator);
            return 0;
        }
    }
}

fn parse_symbols(input: &str) -> Vec<String> {
    let mut symbols: Vec<String> = Vec::new();

    let mut time_string_buffer = String::new();
    for c in input.chars() {
        // if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' {
        if is_operator(c) {
            if !time_string_buffer.is_empty() {
                let seconds = time_to_seconds(&time_string_buffer);
                symbols.push(seconds.to_string());
                time_string_buffer.clear();
            }
            symbols.push(c.to_string());
        } else if c.is_numeric() || c == ':' {
            time_string_buffer.push(c);
        }
    }
    if !time_string_buffer.is_empty() {
        let seconds = time_to_seconds(&time_string_buffer);
        symbols.push(seconds.to_string());
    }

    symbols
}

fn is_operator(character: char) -> bool {
    let operators = "+-*/^()";
    operators.contains(character)
}

fn time_to_seconds(time: &str) -> u32 {
    let sections: Vec<&str> = time.split(':').collect();
    let mut i = 0;
    let mut seconds = 0;
    for number in sections.iter().rev() {
        let number: u32 = number.parse().unwrap();
        match i {
            0 => {
                // seconds
                seconds += number;
            }
            1 => {
                // minutes
                seconds += number * 60;
            }
            2 => {
                // hours
                seconds += number * 60 * 60;
            }
            _ => {
                println!("Time notation is wrong...")
            }
        }
        i += 1;
    }
    seconds
}

fn seconds_to_time(input: u32) -> String {
    let hours = input / 3600;
    let minutes = if hours == 0 {
        input / 60
    } else {
        (input / 60) % 60
    };
    let seconds = input % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn to_postfix(symbols_infix: &Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut postfix: Vec<String> = Vec::new();

    for s in symbols_infix {
        match s.as_str() {
            // case 1: encounter an operator
            "+" | "-" | "/" | "*" | "^" => {
                // stack is not empty...
                while (stack.len() != 0) && (priority(s) <= priority(&stack[stack.len() - 1])) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(s.to_string());
            }
            "(" => {
                stack.push(s.to_string());
            }
            ")" => {
                // stack cannot be 0, as there has to be a ( somewhere
                while stack[stack.len() - 1] != '('.to_string() {
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            // case 2: encounter a number
            _ => {
                postfix.push(s.to_string());
            }
        }
    }
    while stack.len() != 0 {
        postfix.push(stack.pop().unwrap());
    }

    postfix
}

/*
1. priorities of operators
    ->  +,-
    ->  *,/
    ->  ^

2. if scanned operator is <= then the top of the stack in priority
then pop operator until we have low priority. Add the popped elements
to the postfix

3. if "(" push it to the stack

4. if ")" pop elements until "(" and add popped elements to postfix

5. if operand then just add to the postfix
*/

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
}

/*
00:10 + 06:12 / 2
["10", "+", "372", "/", "2"]
10 + 372 / 2

symbol      stack       postfix
10                      10
+           +           10
372         +           10, 372
/           +, /        10, 372
2           +, /        10, 372, 2

--  finis the stack
            +           10, 372, 2, /
                        10, 372, 2, /, +

--------------------------------------------------------------------------------
10 / 2 + (500 ^ 1 - 10)
symbol      stack       postfix
10                      10
/           /           10
2           /           10, 2
+           +           10, 2, /        # / has a higher priority > pop from stack and place in postfix, add + to stack
(           +, (        10, 2, /
500         +, (        10, 2, /, 500
^           +, (, ^     10, 2, /, 500
1           +, (, ^     10, 2, /, 500, 1
-           +, (, -     10, 2, /, 500, 1, ^
10          +, (, -     10, 2, /, 500, 1, ^, 10
)                       10, 2, /, 500, 1, ^, 10, -, +
*/

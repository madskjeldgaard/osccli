use nannou_osc as osc;

pub fn parse_message(message: String, parse_to_type: Option<&str>) -> osc::Type {
    match parse_to_type {
        Some(parse_to_type) => match parse_to_type {
            "float" => parse_message_as_float(message),
            "int" => parse_message_as_int(message),
            "string" => parse_message_as_string(message),
            "double" => parse_message_as_double(message),
            _ => {
                println!("Unrecognized type. Falling back to String as type.",);
                parse_message_as_string(message)
            }
        },
        None => parse_message_as_string(message),
    }
}

fn parse_message_as_int(message: String) -> osc::Type {
    let parsed = message.parse::<i32>().unwrap();
    osc::Type::Int(parsed)
}

fn parse_message_as_float(message: String) -> osc::Type {
    println!("{}", message);
    let parsed = message.parse::<f32>().unwrap();
    osc::Type::Float(parsed)
}

fn parse_message_as_double(message: String) -> osc::Type {
    let parsed = message.parse::<f64>().unwrap();
    osc::Type::Double(parsed)
}

fn parse_message_as_string(message: String) -> osc::Type {
    let parsed = message;
    osc::Type::String(parsed)
}

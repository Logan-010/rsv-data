//!Core module used for encoding and decoding RSV data
//!
//!Here's an example for input to the encoder:
//!
//!```
//!let input: Vec<Vec<Option<String>>> = vec![
//!    vec![Some("Hello user!".to_string()), None],
//!    vec![Some("\n\\\'\"".to_string()), Some("😁🔃📖".to_string())]
//!];
//!```

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub fn encode_rsv(rows: &Vec<Vec<Option<String>>>) -> Vec<u8> {
    let mut parts: Vec<&[u8]> = vec![];

    for row in rows {
        for value in row {
            match value {
                //Pushes byte for null data
                None => parts.push(b"\xFE"),
                //Pushes bytes for non null data
                Some(str_value) => {
                    if !str_value.is_empty() {
                        parts.push(str_value.as_bytes());
                    }
                }
            }

            parts.push(b"\xFF");
        }

        parts.push(b"\xFD");
    }

    //Returns bianary data for RSV file
    parts.concat()
}

pub fn decode_rsv(bytes: &Vec<u8>) -> Res<Vec<Vec<Option<String>>>> {
    //Check for valid end charecter in RSV input
    if !bytes.is_empty() && bytes[bytes.len() - 1] != 0xFD {
        return Err("Incomplete RSV document".into());
    }

    let mut result: Vec<Vec<Option<String>>> = Vec::new();
    let mut current_row: Vec<Option<String>> = Vec::new();
    let mut value_start_index = 0;

    for i in 0..bytes.len() {
        if bytes[i] == 0xFF {
            let length = i - value_start_index;

            if length == 0 {
                current_row.push(Some(String::new()));
            } else if length == 1 && bytes[value_start_index] == 0xFE {
                current_row.push(None);
            } else {
                let value_bytes = bytes[value_start_index..i].to_vec();

                if let Ok(str_value) = String::from_utf8(value_bytes) {
                    current_row.push(Some(str_value));
                } else {
                    return Err("Invalid string value".into());
                }
            }

            value_start_index = i + 1;
        } else if bytes[i] == 0xFD {
            if i > 0 && value_start_index != i {
                return Err("Incomplete RSV row".into());
            }

            result.push(current_row);
            current_row = Vec::new();
            value_start_index = i + 1;
        }
    }

    //Return table
    Ok(result)
}


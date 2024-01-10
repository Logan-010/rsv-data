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

pub const VALUE_TERMINATOR: u8 = 0xFF;
pub const ROW_TERMINATOR: u8 = 0xFD;
pub const NULL_VALUE: u8 = 0xFE;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub fn encode_rsv<T: ToString>(rows: &[Vec<Option<T>>]) -> Vec<u8> {
    rows.iter().fold(vec![], |mut result, row| {
        let mut row_bytes = row
            .iter()
            .map(|v| match v {
                Some(t_value) => t_value.to_string().into_bytes(),
                None => vec![NULL_VALUE],
            })
            .fold(vec![], |mut row_result, mut value_in_bytes| {
                row_result.append(&mut value_in_bytes);
                row_result.push(VALUE_TERMINATOR);
                row_result
            });
        result.append(&mut row_bytes);
        result.push(ROW_TERMINATOR);
        result
    })
}

pub fn decode_rsv(bytes: &[u8]) -> Res<Vec<Vec<Option<String>>>> {
    //Check for valid end charecter in RSV input
    if !bytes.is_empty() && bytes.last() != Some(&ROW_TERMINATOR) {
        return Err("Incomplete RSV document".into());
    }

    let mut result: Vec<Vec<Option<String>>> = Vec::new();
    let mut current_row: Vec<Option<String>> = Vec::new();
    let mut value_start_index = 0;

    for i in 0..bytes.len() {
        if bytes[i] == VALUE_TERMINATOR {
            let length = i - value_start_index;

            if length == 0 {
                current_row.push(Some(String::new()));
            } else if length == 1 && bytes[value_start_index] == NULL_VALUE {
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
        } else if bytes[i] == ROW_TERMINATOR {
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

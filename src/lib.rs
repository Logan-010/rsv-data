//!RSV is a minimal bianary alternative to CSV solving some of its problems, escape charecters and
//!encoding. RSV aims to solve this by representing new data and new rows with special bytes that
//!no utf8 string can contain. This allows for strings with no escape charecters.
//!This code was adapted from the code at <https://github.com/Stenway/RSV-Challenge/blob/main/Rust/src/main.rs>
//!
//!Heres some examples on how to use this:
//!
//!Open RSV file and assign some data to it
//!```
//!let mut data: Rsv = Rsv::open("myRsv.rsv").unwrap();
//!data.set_data(vec![
//!    vec![Some("Hello user".to_string()), None],
//!    vec![Some("\n\\\'\"".to_string()), Some("😁🔃📖".to_string())]
//!]);
//!
//!data.save().unwrap();
//!```
//!
//!Create RSV file and assign some data to it
//!```
//!let mut data: Rsv = Rsv::create("myRsv.rsv").unwrap();
//!data.set_data(vec![
//!    vec![Some("Hello user".to_string()), None],
//!    vec![Some("\n\\\'\"".to_string()), Some("😁🔃📖".to_string())]
//!]);
//!
//!data.save().unwrap();
//!```

use std::io::{Read, Write};

pub mod core;

///Convience/Abstraction struct for RSV, allows opening, creating, saving, and retrieving data;
pub struct Rsv {
    file: String,
    data: Vec<Vec<Option<String>>>,
}

impl Rsv {
    pub fn create(file_name: &str) -> core::Res<()> {
        let mut file = std::fs::File::create(file_name)?;
        //Adding valid end byte
        let byte = [0xFD];
        file.write_all(&byte)?;

        Ok(())
    }

    pub fn open(file_name: &str) -> core::Res<Self> {
        let mut file = std::fs::File::open(file_name)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;

        let data = core::decode_rsv(&bytes)?;

        Ok(Rsv {
            file: String::from(file_name),
            data,
        })
    }

    pub fn save(&self) -> core::Res<()> {
        let mut file = std::fs::File::open(&self.file)?;
        file.write_all(&core::encode_rsv(&self.data))?;

        Ok(())
    }

    pub fn data(&self) -> Vec<Vec<Option<String>>> {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: Vec<Vec<Option<String>>>) {
        self.data = data;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_decode() {
        let data = vec![
            vec![Some(String::from("Hello world")), None],
            vec![Some(String::from("☀️"))],
        ];

        let encoded_data = crate::core::encode_rsv(&data);

        assert_eq!(data, crate::core::decode_rsv(&encoded_data).unwrap());
    }

    #[test]
    fn test_decode_error() {
        let mut data = crate::core::encode_rsv(&vec![vec![None]]);
        data.pop();

        if crate::core::decode_rsv(&data).is_ok() {
            panic!("Expected Err, but got Ok");
        }
    }
}

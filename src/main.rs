
mod tests;
mod phone_key_encoder_decoder {

#[derive(Debug)]
struct Button {
    no: usize,
    repeat: usize
}

pub fn encode(plain_text: String) -> Result<String, String> {
    let mut output = String::new();

    let buttons = match set_button(plain_text) {
        Ok(buttons_vec) => buttons_vec,
        Err(x) => return Err(x),
    };

    let mut prev: i32 = -1;
    for button in buttons {
        if prev == button.no as i32 {
            output += ".";
        }
        for _r in 0..button.repeat {
            output += &format!("{}", button.no);
        }
        prev = button.no as i32;
    }

    return Ok(output);


}

pub fn decode(encoded_text: String) -> Result<String, String> {
    let numbers = [" ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut output = String::new();
    

    let buttons = match get_button(encoded_text) {
        Ok(buttons_vec) => buttons_vec,
        Err(e) => return Err(e),
    };

    // println!("{:?}", buttons);
    
    for button in buttons {

        let str_group = numbers[button.no];
        let chr = &str_group[button.repeat - 1..button.repeat];
        output += chr;

    } 
    return Ok(output);
}

fn get_button(s: String) -> Result<Vec<Button>, String> {
    //let button_group = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut prev_index: i32 = -1;
    let mut output: Vec<Button> = Vec::new();
    let mut char_index: usize = 0;

    let mut str_chars = s.chars();
    
    while let Some(chr) = str_chars.next() {
        
        if chr == '.' {
            output.push(Button{no: prev_index as usize, repeat: char_index + 1});
            char_index = 0;
            prev_index = -1;
            continue;
        }

        let index = match chr.to_digit(10) {
            Some(i) => i,
            None => { 
                return Err(format!("Error decoding, invalid character {}", chr));
            }
        };

        if (index < 2 && index != 0) || index > 9 {
            return Err(format!("Error decoding, invalid number {}", index));
        }

        if prev_index == -1 {
            prev_index = index as i32;
        } else if prev_index == index as i32 {
            char_index += 1;
        } else {
            output.push(Button{no: prev_index as usize, repeat: char_index + 1});
            char_index = 0;
            prev_index = index as i32;
        }
    }
    output.push(Button{no: prev_index as usize, repeat: char_index + 1});

    return Ok(output);
}

fn set_button(s: String) -> Result<Vec<Button>, String> {
    let button_group = [" ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut buttons: Vec<Button> = Vec::new();

    for c in s.chars() {
        let mut found = false;
        for (i, button_set) in button_group.iter().enumerate() {
            if button_set.contains(c) {
                buttons.push(Button{no: i, repeat: button_set.find(c).unwrap() as usize + 1});
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!("Character can not be printed by phone buttons, {}", c));
        }
    }

    return Ok(buttons);
}
}

fn main() {

    use phone_key_encoder_decoder;

    let plaintext = String::from("rust is awesome");
    println!("Plain Text: {}", plaintext);

    match phone_key_encoder_decoder::encode(plaintext) {
        Err(x) => println!("{}", x),
        Ok(s) => {
            println!("Encoded: {}", s);
            match phone_key_encoder_decoder::decode(s) {
                Err(x) => println!("{}", x),
                Ok(s) => println!("Decoded: {}", s)
            }
        }
    };
}

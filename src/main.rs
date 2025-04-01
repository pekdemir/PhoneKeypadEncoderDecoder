
// fn decode(s: String) -> Result<String, String> {
//     let numbers = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
//     let mut prev_index = 0;
//     let mut output = String::new();
//     let mut str_index: usize = 0;
    
//     for c in s.chars() {
//         let index: usize = c.to_digit(10).unwrap() as usize;
//         if index < 2 || index > 9 {
//             return Err(format!("Error decoding, invalid number {}", index));
//         }

//         if prev_index == 0 || prev_index == index {
//             str_index += 1;
//             prev_index = index;
//         }else{
//             let str_group = numbers[prev_index];
//             let chr = &str_group[str_index..str_index+1];
//             str_index = 0;
//             prev_index = index;
//             output += chr;
//         }
        
        
//         //println!("{}", index);
//     } 
//     return Ok(output);
// }
#[derive(Debug)]
struct Button {
    no: usize,
    repeat: usize
}

fn encode(plain_text: String) -> Result<String, String> {
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

fn decode(encoded_text: String) -> Result<String, String> {
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

        let index = chr.to_digit(10).unwrap();
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

fn main() {

    //let encoded = String::from("44255266073355");

    // match decode(encoded) {
    //     Err(x) => println!("{}", x),
    //     Ok(s) => println!("{}", s),
    // };

    let plaintext = String::from("hello world from rust");
    println!("Plain Text: {}", plaintext);

    match encode(plaintext) {
        Err(x) => println!("{}", x),
        Ok(s) => {
            println!("Encoded: {}", s);
            match decode(s) {
                Err(x) => println!("{}", x),
                Ok(s) => println!("Decoded: {}", s)
            }
        }
    };



}
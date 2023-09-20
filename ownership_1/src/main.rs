fn main() {
    let s = String::from("Hello");
    let (s2, len) = calculate_length(s);
    println!("Length Of {} is {}", s2, len);

    // ------- Reference
    let s3 = String::from("hello 1");
    let len = calculate_length_usize(&s3);
    println!("panjang dari kata :: {} adalah => {}", s3, len);

    let mut s4 = String::from("hai kamu ");
    change(&mut s4);
    println!("hasil dari mutable => {}", s4);

    let mut s5 = String::from("hai kamu yang ke 5");
    let v = &s5;
    let v1 = &s5;
    println!("ini adalah nilai \n v => {}, \n v1 => {}", v, v1);

    let v3 = &mut s5;
    println!("ini adalah\nnilai v3 => {}", v3);

    // -- slice
    let mut sample = String::from("Hello Musa aku adalah expert di rust ");
    let fwl = first_word_count(&sample);
    println!(" panjang dari variabel fwl => {}", fwl);
    let hello = &sample[..=fwl];
    let fwl_data_1 = first_word_string(&sample);
    println!("ini adalah kata hello => {}", hello);
    println!("ini adalah kata fwl_data_1 => {}", fwl_data_1);

    let str_literal = "ini adalah aku";
    println!("ini adalah kata pertama => {}", first_word_str(str_literal));
    sample.clear();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_usize(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
    sample kata
        "Hello Musa ini aku orang yang expert di rust"
*/
fn first_word_count(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}


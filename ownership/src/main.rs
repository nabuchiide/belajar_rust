fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let x = 5;
    let y = x; // copy Trait (data di simpan di stack)
    println!("x = {}, y={}", x, y);

    let s1 = String::from("hello");
    // let s2 = s1; // move Trait
    let s2 = s1.clone();
    println!("s1 = {} ,s2 = {}", s1, s2);

    let str1 = String::from("ini adalah aku"); // s comes into scope
    takes_ownership(str1.clone());
    println!("str1 setelah {}", str1);
    // ---------------------------------------------------- s is no longer valid

    let x1 = 10; // x comes into scope1
    makes_copy(x1); // x values is Copy
    //----------------------------------------------------- use x afterword
    println!("x setelah {}", x1);

    // ----------------------------------- give ownership
    println!("ini give ownership");
    let str01 = gives_ownerships();
    println!("ini str01 => {}", str01);

    let mut str02 = String::from("hello");
    str02.push_str(" world");
    let str03 = takes_and_give_back(str02);
    println!("ini str03 => {}", str03);

    //-------------------------------------- tuple
    let (tupleStr, len) = calculate_length(str03);
    println!("length of {} is {} ", tupleStr, len);
}

fn takes_ownership(some_string: String) {
    println!("takes ownership => {}", some_string);
}// here some_string goes out of scope, droped is called

fn makes_copy(some_integer: i32) {
    println!("makes_copy => {}", some_integer)
}

fn gives_ownerships() -> String {
    let some_str = String::from("world");
    some_str
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s, length)
}
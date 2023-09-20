use core::num;

fn main() {
    //type data
    let a: i32 = 10;
    let mut b: i32 = 20;

    const HIGEST_PRICE: i32 = 10_000_000;

    let x: &str = "ten";
    let x: i32 = 10;

    let y = 10;
    let f = 2.1;
    let valid = true;
    let invalid: bool = false;
    let c = 'Z';

    //compund: tuples and arrays
    let tup: (i16, f64, u8) = (100, 1.3, 1);
    let (x0, y0, z0) = tup;
    let fist = tup.0;
    let second = tup.1;
    let third = tup.2;

    let tup2 = ();

    let array1 = [1, 2, 3];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];
    let array3 = [3, 5]; // => let be = [3,3,3,3,3]

    print!("Hello Musa \n");
    my_function(3, 'h');
    let ress = valid_function();
    println!("ini hasil dari function return {}", ress);

    let ress1 = valid_function1();
    println!("ini hasil dari function reutrn {}", ress1);

    let ress3 = valid_function2(HIGEST_PRICE, y);
    println!(
        "ini hasil dari penjumlahan {} dan {} menjadi => {}",
        x, y, ress3
    );
    flow_control_if(y);
    flow_control_loop();
    flow_control_while_loop();
    flow_control_for_loop();
}

// void funtion
fn my_function(value: i32, label: char) {
    println!("result {} , {}", value, label);
}

// return function
fn valid_function() -> i32 {
    return 5;
}

fn valid_function1() -> i32 {
    5
}

fn valid_function2(x: i32, y: i32) -> i32 {
    x + y
}

// controll flow (percabangan dan  perulangan)

fn flow_control_if(num:i32) {
    //normal if
    if num < 5 {
        print!("condition true");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else {
        print!("condition fals");
    }

    // set boolean values
    let cond: bool = if num < 5 {
        true
    } else if num % 3 == 0 {
        true
    } else {
        false
    };

    // integer values
    let num2 = if cond { 5 } else { 6 };
    println!("value dari num2 => {}", num2);
}

fn flow_control_loop(){
    let mut counter = 0;
    let res = loop {
        counter +=1;
        print!("loop ke-{}\n", counter);
        if counter ==10 {
            break counter*3;
        }
    };

    println!("hasil dari lop nya => {}",res);
}

fn flow_control_while_loop(){
    let mut num = 3;
    while num !=0 {
        println!("sisa loop adalah {}",num);
        num -= 1;
    }
}

fn flow_control_for_loop(){
    let a = [1,2,3];
    for element in a {
        println!("the value is : {}", element);
    }

    for number in 1..=5{
        println!("ini number {}", number);
    }

    println!("thants it Bye")
}
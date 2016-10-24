use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let puid = &args[1];
        println!("puid={}", puid);
        println!("tableNum={:03}", get_table_num(puid));
    } else {
        println!("{} , input puid args ", &args[0]);

    }
}

fn get_table_num(puid: &str) -> i32 {
    let mut hash = 0 as i32;
    for c in puid.chars() {
        print!("{}", c);
        hash = (hash * 31) + c as i32;
    }
    ( hash % 256).abs()
}

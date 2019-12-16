enum EvaType {
    Number(u32),
    String(String)
}

fn eval(exp: EvaType) -> () {
    match exp {
        EvaType::Number(i) => println!("int is {}", i),
        EvaType::String(str) => println!("String is {}", str),
    }
}

fn main() {
    println!("Hello, world!");
    eval(EvaType::Number(123));
    eval(EvaType::String(String::from("hello!")));
}

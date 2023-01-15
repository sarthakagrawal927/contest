// dump of random things
// op enum
enum RSEnum {
    FooFn(fn() -> i32),
    Foo(i32),
    Foo2(Option<i32>), // Option is another enum, has None & Some(T)
    Bar(String),
    Blast(Vec<String>),
}

// type Result<V,E> {
//     Err(E),
//     Ok(V)
// }

fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(7);
    }
    return Ok(());
}

enum Option2<T> {
    None,
    Some(T),
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        };
    }
}

fn bar() -> i32 {
    return 5;
}

fn main() {
    let a = vec![];
    let mut b = a;

    b.push(1);
    // a.push(2); - gives error : a is already moved, can send the
    println!("{:?}", b); // [1]

    let foo = RSEnum::FooFn(bar);
    if let RSEnum::Foo(2) = foo {}

    match foo {
        // aggressive switch case
        RSEnum::Foo(_value) => {}
        RSEnum::Foo2(Some(_value)) => {}
        RSEnum::Foo2(None) => {}

        _ => {}
    }

    let some_example = Some(5);

    if let Some(_value) = some_example {}

    some_example.map(|_x| {});
    some_example.filter(|&x| x < 10);

    for c in "asdasdas".chars() {}
}

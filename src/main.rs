use hello::cross_line::*;
use std::borrow::Cow;
use std::fmt::{Debug, Display};


fn main() {
    let n = 10;
    let m = n;
    let cell = ball_in_box(n, m);
    let set = count_cross(n, cell.as_ref());
    println!("topology count = {}", cell.len());
    println!("cross point = {:?}, count = {}", set, set.len());
    //    let v = vec![1, 2, 3, 4, 5];
    let v = (1..=10).collect::<Vec<u64>>();
    let sum = v.iter().sum::<u64>();
    let pro = v.iter().product::<u64>();
    let f = (1..=5).fold(100, |acc, i| acc + i);
    println!("{}, {}, {}", sum, pro, f);
    println!("{}", remove_spaces("hello world!"));
    let s = remove_spaces("Herman"); // s is a Cow::Borrowed variant
                                     //    let len = s.len(); // immutable function call using Deref
    let mut owned: String = s.into_owned(); // memory is allocated for a new string
    owned.push('a');
    let p = Person {
        id: 0,
        name: "Daniel".to_string(),
    };
    println!("{}", p.get_name());
    p.get_name().to_mut().push('W');
    println!("{:?}", p);
    println!("{:?}", p.get_s());

    let readonly = [1, 2];
    let borrowed = Items::new((&readonly[..]).into());
    match borrowed {
        Items {
            values: Cow::Borrowed(b),
        } => println!("borrowed {:?}", b),
        _ => panic!("expect borrowed value"),
    }

    let mut clone_on_write = borrowed;
    // Mutates the data from slice into owned vec and pushes a new value on top
    clone_on_write.values.to_mut().push(3);
    println!("clone_on_write = {:?}", clone_on_write.values);

    // The data was mutated. Let check it out.
    match clone_on_write {
        Items {
            values: Cow::Owned(_),
        } => println!("clone_on_write contains owned data"),
        _ => panic!("expect owned data"),
    }

    let b = [0, -1, -2];
    let mut input = Cow::from(&b[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0, 1, 2]) as Cow<[i32]>);
    println!("{:?}", b);
    give_me(35);
    give_me("String");
    println!("{}", 33_i32.to_out());
    show_me("Daniel");
    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());
    println!("{}", surround_with_braces("Hello"));

    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for s in shapes {
        println!("{:?}", s);
    }
    let a = Foo;

    let closure =  || {
        let b = a.clone();
    };

    println!("{:?}", a);

    let bag = Bag { food: Food::Cake };
    match bag.food {
        Food::Cake => println!("I got cake"),
        ref a => println!("I got {:?}", a)
    }
    println!("{:?}", bag);
    let a = SomeRef { part: &43 };
    println!("{:?}", a);

    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![];
    let decoder = Decoder {schema: &a, reader: &b};


    let my_result = Ok::<_, ()>(64);

    println!("{}", type_of(&my_result));

    let my_err = Err::<(), f32>(345.3);
    println!("{}", type_of(&my_err));
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn type_of<T>(_:&T) -> &'static str {
   std::any::type_name::<T>()
}

struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R
}

impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
    where 'a: 'b {

}

#[derive(Debug)]
struct SomeRef<'a, T> {
    part: &'a T
}

#[derive(Debug)]
enum Food {
    Cake,
    Rice
}

#[derive(Debug)]
struct Bag {
    food: Food
}

#[derive(Debug, Clone)]
struct Foo;

#[derive(Debug)]
struct Square(f32);
#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{ {} }}", val)
}

fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

fn show_me<T>(val: T)
where
    T: Display,
{
    println!("{}", val);
}

fn add_thing<T>(fst: T, snd: T)
where
    T: std::ops::Add,
{
    let _ = fst + snd;
}

pub trait ATrait {
    type Out;
    fn to_out(&self) -> Self::Out;
}

impl ATrait for i32 {
    type Out = i32;

    fn to_out(&self) -> Self::Out {
        99
    }
}

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

fn give_me<T>(t: T) {
    let _ = t;
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }

    println!("value: {:?}", input);
}

fn remove_spaces(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        Cow::Owned(buf)
    } else {
        input.into()
    }
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

impl Person {
    fn get_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn get_s(&self) -> Cow<str> {
        let v = vec!['a', 'b', 'c'];
        let s = v.iter().collect::<String>();
        s.into()
    }
}

struct Items<'a, X: 'a>
where
    [X]: ToOwned<Owned = Vec<X>>,
{
    values: Cow<'a, [X]>,
}

impl<'a, X: Clone + 'a> Items<'a, X>
where
    [X]: ToOwned<Owned = Vec<X>>,
{
    fn new(v: Cow<'a, [X]>) -> Self {
        Items { values: v }
    }
}

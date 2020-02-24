use hello::cross_line::*;
use std::borrow::Cow;

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

// #![feature(error_type_id)]
// #![feature(backtrace)]

use std::{
    // ascii::AsciiExt,
    collections::HashMap,
    // error::{self, Error},
    fmt::Debug,
    hash::Hash,
    // process::Output,
    // ptr::from_raw_parts,
    // sync::atomic::AtomicI32,
    thread,
    time::Duration,
};

fn main() {
    println!("Hello, world!");

    a();
    exp(1);

    println!("Bye, world!");
}

struct Cache<T, U = u32>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash + Debug,
{
    calc: T,
    values: HashMap<U, U>,
}

impl<T, U> Cache<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash + Debug,
{
    fn new(calc: T) -> Self {
        Self {
            calc,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let res = (self.calc)(arg);

                self.values.insert(arg, res);
                res
            }
        }
    }
}

fn exp(intensity: u32) -> () {
    let mut cache = Cache::new(|n: u32| {
        thread::sleep(Duration::from_secs(n.into()));
        n
    });

    let a = cache.value(intensity + 2);
    println!("a: {}", a);
    let b = cache.value(intensity + 1);
    println!("b: {}", b);
    let c = cache.value(intensity + 2);
    println!("c: {}", c);
}

fn a() {
    let x = vec![];

    let z = vec![];

    let zz = move |u: &Vec<u8>| *u == z;

    pre(&zz, &x);

    fn pre<T: for<'r> Fn(&'r Vec<U>) -> bool, U>(cb: &T, x: &Vec<U>) {
        println!("{}", (*cb)(x));
        println!("{}", (*cb)(x));
    }

    let _b = (|| -> bool {
        println!("{}", zz(&x));
        println!("{}", zz(&x));
        true
    })();

    let mybox = Box::new(1234);
    let clj1 = |x| *mybox == x;

    println!("{:?}", mybox);

    let clj2: &dyn Fn(i32) -> bool = &clj1;

    println!("is box: {}", clj2(1234));

    fn give_me_clj(cb: &dyn Fn(i32) -> bool) -> () {
        println!("in gmc box: {}", cb(1234));
    }

    give_me_clj(clj2);

    trait Cute {
        fn are_you_cute(&self) -> bool {
            true
        }
    }

    // #[derive(Cute)]

    struct A1;
    struct A2;
    struct A3;

    impl Cute for A1 {}
    impl Cute for A2 {}
    impl Cute for A3 {}

    // (|par: dyn Cute| {
    //     println!("aaaa");
    // })();

    let avec: Vec<&dyn Cute> = vec![&A1, &A2, &A3];

    let _: Vec<_> = avec
        .into_iter()
        .map(|a: &dyn Cute| -> bool { a.are_you_cute() })
        .map(|b: bool| println!("b: {}", b))
        .collect();

    // assert_eq!(clj1, 123);

    // let

    const HI: &str = "HIII";

    let _dec = 22_22;
    let _hex = 0x443da;
    let _oct = 0o77;
    let _bin = 0b11_00000000_1111010101;
    let _byt = b'A';
    let _ch = 'c';
    let _tup: (i32, bool, &str) = (454121, true, "sddslk");
    let arr1 = [1, 2, 3, 4, 5];
    let _arr2: [i32; 5] = [454; 5];
    // let arr3: [i32] = [2332];

    let _l = loop {
        break 1234;
    };

    for (i, &el) in arr1.iter().enumerate() {
        println!("{}, {}", i, el);
    }

    let _slice: &[i32] = &arr1[..];

    // Display trait
    // enum, Option, match iflet
    // module

    let v = vec![1, 2, 3];

    let _th: &i32 = &v[2];
    // let sl: &[i32] = &arr1[2..2];
    let _th2: Option<&i32> = v.get(2);

    for i in &v {
        println!("{}", i);
    }

    for i in &v {
        println!("{}", i);
    }

    // Display 는 to_string()을 구현하며, String을 리턴
    // imref는 copy가능한가?

    let mut string = String::from("akia");

    //     let ref mut this = string;
    //     this.vec.extend_from_slice(" code".as_bytes())
    // }

    let code = String::from(" code");
    string.push_str(&code[..]);

    println!("{}", string);
    println!("{}", code);

    let mut akiacode = format!("{} is god {}", string, code);

    println!("{}", akiacode);

    // defref! &String -> &String[..]

    let sub = &mut akiacode[0..3];

    println!("{}", sub);

    for i in akiacode.chars() {
        println!("i: {}", i);
    }

    for i in akiacode.bytes() {
        println!("b: {}", i);
    }

    let vvv = vec![Box::new(5)];

    println!("box[0]: {}", &vvv[0]);
    println!("box[0]: {}", vvv.get(0).unwrap());
    // println!("box[0]: {}", vvv.get);

    let nucTest = NucCount {
        a: 14,
        c: 9,
        g: 10,
        t: 12,
    };

    println!("A: {}", nucTest.index(Nuc::A));

    // hash func for rust

    // pig();

    let text = "hello world!";

    let char_vec: Vec<char> = text.chars().collect();

    if let Some(fch) = char_vec.get(0) {
        println!("{}", fch);
    }

    if let Some(char_vec2) = text.chars().next() {
        println!("{}", char_vec2);
    }

    v[9999];
}

pub trait MyIndex<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}

enum Nuc {
    A,
    C,
    G,
    T,
}

struct NucCount {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl MyIndex<Nuc> for NucCount {
    type Output = usize;

    fn index(&self, nuc: Nuc) -> &Self::Output {
        match nuc {
            Nuc::A => &self.a,
            Nuc::C => &self.c,
            Nuc::G => &self.g,
            Nuc::T => &self.t,
        }
    }
}

fn pig_latin(str: &String) -> Result<String, bool> {
    if str.chars().any(|x| !x.is_ascii_alphabetic()) {
        return Err(false);
    }

    Ok(match str.chars().next() {
        Some(first_char) => match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => format!("{}hay", str),

            _ => match str.chars().skip(1).next() {
                Some(v) => {
                    let tup = if first_char.is_ascii_uppercase() {
                        (first_char.to_ascii_lowercase(), v.to_ascii_uppercase())
                    } else {
                        (first_char, v)
                    };

                    format!(
                        "{}{}{}ay",
                        tup.1,
                        str.chars().skip(2).collect::<String>(),
                        tup.0,
                    )
                }
                None => format!("{}ay", str),
            },
        },
        None => String::new(),
    })
}

// let s = String::from("hello");
// let bytes = s.into_bytes();

// assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);

#[test]
fn pig() {
    assert_eq!(
        pig_latin(&String::from("apple")),
        Ok(String::from("applehay"))
    );
    assert_eq!(
        pig_latin(&String::from("banana")),
        Ok(String::from("ananabay"))
    );
    assert_eq!(
        pig_latin(&String::from("Abiria")),
        Ok(String::from("Abiriahay"))
    );
    assert_eq!(pig_latin(&String::from("Rust")), Ok(String::from("Ustray")));
}

pub fn aasdf() -> bool {
    true
}

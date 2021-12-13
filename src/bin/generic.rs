use std::fmt::Display;

fn main() {
    let number_list = [1, 2, 3, 4, 5, 6];

    let lg = largest(&number_list[..3]);

    println!("largest number is {}", lg);

    gg(0349);

    gg_for_box(Box::new(128));

    slice();

    pointer();

    trait_l();

    life();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for ele in list.iter() {
        if ele > largest {
            largest = ele;
        }
    }

    largest
}

fn gg(mut a: i32) {
    a += 1000;

    println!("{}", a);
}

fn gg_for_box(mut box_a: Box<i32>) {
    *box_a += 1000;

    println!("{}", *box_a);
}

fn slice() {
    let sl: &[i32] = &[];
    println!("slice {:?}", sl);

    let sl: &[i32] = &vec![1, 2][..];
    println!("slice {:?}", sl);

    let sl: &[i32] = &vec![][..];
    println!("slice {:?}", sl);

    let sl: &[i32] = &vec![][..];
    println!("slice {:?}", sl);

    let sl = std::slice::from_ref(&1);
    println!("slice {:?}", sl);

    let sl = &[1, 2, 3, 4, 5][..];
    println!("slice {:?}", sl);

    // let sl = from_ref(&1234);
    // println!("slice {:?}", sl);
}

struct A();

impl Clone for A {
    fn clone(&self) -> Self {
        Self()
    }
}

impl Copy for A {}

// fn from_ref<T: Copy>(s: &T) -> &[T] {
//     let s = *s;

//     let v = vec![s];

//     &v[..]
// }

#[derive(Debug)]
pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<Cute, Abiria> Point<Abiria, Cute> {
    pub fn new(x: Abiria, y: Cute) -> Self {
        Self { x, y }
    }

    pub fn sum(&self) -> &Abiria {
        &self.x
    }

    pub fn gen<T: std::ops::Mul<Output = T> + Copy>(&self, t: T) -> T {
        t * t
    }

    pub fn mixup<Ilove, You>(self, other: Point<Ilove, You>) -> Point<Abiria, You> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<Abiria> Point<Abiria, i32> {
    pub fn is_cute(&self) -> i32 {
        self.y * 10
    }
}

impl<Num: Display + PartialOrd> Point<Num, Num> {
    pub fn who_is_big(&self) -> &Num {
        if (self.x) > (self.y) {
            &self.x
        } else {
            &self.y
        }
    }
}

impl<Cute> Point<i32, Cute> {
    pub fn is_cute2(&self) -> i32 {
        self.x * 10
    }
}

impl Point<i32, i32> {
    pub fn sum_(&self) -> i32 {
        self.x + self.y
    }
}

impl Point<String, String> {
    pub fn sum_(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
}

fn pointer() {
    let p1: Point<i32, i32> = Point { x: 100, y: 28 };
    let p2 = Point { x: "qw", y: "sakj" };

    println!("{:?}", p1);
    println!("{:?}", p2);

    let en: Enum<i32, String> = Enum::Value(451, String::from("own"));

    println!("{:?}", en);

    let Enum::Value(x, y) = en;
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    let en2 = Enum::new(x, y);

    println!("sum en2: {}", en2.sum());

    println!("sum p1: {}", p1.sum());

    let p_T_i32 = Point::new(String::from("abc"), 123);
    let p_i32_T = Point::new(456, String::from("def"));

    println!("{:?}", p_i32_T.is_cute2());
    println!("{:?}", p_T_i32.is_cute());

    let p_String_String = Point::new(String::from("abc"), String::from("def"));

    println!("p1 sum_: {:?}", p1.sum_());
    println!("p_String_String sum_: {:?}", p_String_String.sum_());

    println!("gen: {:?}", p1.gen(11111));
    println!("who is big: {:?}", p1.who_is_big());

    println!("mixup: {:?}", p1.mixup(p_String_String));
}

#[derive(Debug)]
enum Enum<T, U> {
    Value(T, U),
}

impl<T2, U> Enum<T2, U> {
    pub fn new(x: T2, y: U) -> Self {
        Self::Value(x, y)
    }

    pub fn sum(&self) -> &T2 {
        match self {
            Self::Value(x, _) => x,
        }
    }
}

// pub trait Sum<T> {
//     fn sum2(&self) -> T;
// }

// impl Sum<T> for Vec<T> {
//     fn sum2(&self) -> T {
//         self.iter().sum()
//     }
// }

fn trait_l() {
    let _v = vec![1, 2, 3, 4, 5];

    // println!("sum: {}", v.sum2());

    let tr2: &dyn Fn(_) -> bool = &tr1;

    println!("eq: {}", tr2(123));

    println!("eq2: {}", tr3()(123));
}

fn tr1<T: PartialEq>(a: T) -> bool {
    a == a
}

fn tr3<T: PartialEq>() -> Box<dyn Fn(T) -> bool> {
    Box::new(|x| x == x)
}

fn life() {
    let l = get_longest("abc", "def");

    // let s = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    // let l = {
    //     let s2 = String::from("abcdef");
    //     get_longest(&s2[..], &s)
    // };

    println!("life: {:?}", l);

    println!("tunneld value: {}", tunnel("tunnel", "lol"));

    println!("make ref: {}", make_ref());

    // let life = {
    //     let para = String::from("debitis necessitatibus quo");
    //     Life { part: &para[..] }
    // };

    let para = String::from("debitis necessitatibus quo");
    let life = Life { part: &para[..] };
    println!("{:?}", life);

    vec_life();

    life_err();

    life_in_method();
}

fn get_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn tunnel<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn make_ref<'a>() -> &'a str {
    // String::from("aaaaa").as_str()

    "aaaaaa"
}

#[derive(Debug)]
struct Life<'a> {
    part: &'a str,
}

fn get_first<'a, T>(v: &Vec<&'a T>) -> &'a T {
    v[0] // may panic
}

fn vec_life() {
    let str = String::from("STRING");

    let f: &String = {
        let vec = vec![&str];
        get_first(&vec)
    };

    println!("{}", f);
}

fn life_err() {
    fn life_lol<'a>(_a: & /*'a*/ str) -> &'a str {
        "aasdsds"
    }

    let res = {
        let s = String::from("STRING");
        life_lol(s.as_str())
    };

    println!("{}", res);
}

impl<'abiria> Life<'abiria> {
    fn level(&self, str: &str) -> &str {
        println!("inner call: {}", str);
        self.part
    }

    fn no_level<'cute>(&self, str: &'cute str) -> &'cute str {
        println!("my level: {}", self.part);
        str
    }
}

fn life_in_method() {
    let l = Life { part: "akjakj" };

    let part = l.level("str");

    println!("{}", part);

    let part = l.no_level("str");

    println!("{}", part);
}

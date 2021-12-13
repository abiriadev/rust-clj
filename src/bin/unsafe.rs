use core::slice;
use std::{
    fmt::{self, Display, Formatter},
    ops::Add,
};

fn main() {
    (|| {
        struct A<T>(T);

        impl A<i32> {
            fn say_hello(&self) {
                println!("Hello, i32!: {}", self.0);
            }
        }

        impl A<String> {
            fn say_hello(&self) {
                println!("Hello, String!: {}", self.0);
            }
        }

        let a = A(123);
        let b = A(String::from("asdf"));

        a.say_hello();
        b.say_hello();
    })();

    (|| {
        // trait ReturnSelf {
        //     fn return_self(self) -> Self {
        //         self
        //     }
        // }

        struct UnitStruct;

        // impl ReturnSelf for UnitStruct {};

        // type unsafe_trait_object = &dyn ReturnSelf;

        // let a: &dyn ReturnSelf = &UnitStruct;
    })();

    (|| {
        enum Three {
            One(i32),
            Two(String),
            Three(bool),
        }
        // use Three::One;

        let my_enum_value = Three::Three(true);

        {
            match my_enum_value {
                _One => 1,
                other => match other {
                    _Two => 2,
                    _other => 3,
                },
            }
        };

        (|| {
            let fav_color: Option<&str> = Some("purple");

            let _res: Result<u8, _> = "34".parse();

            if let Some(fav_color) = fav_color {
                println!("{}", fav_color);
            }

            println!("{:?}", fav_color);

            (|| {
                trait ElasticPush<T> {
                    fn push_and_return_self(&mut self, t: T) -> &mut Self;
                }

                impl<T> ElasticPush<T> for Vec<T> {
                    fn push_and_return_self(&mut self, t: T) -> &mut Self {
                        self.push(t);
                        self
                    }
                }

                let mut stack = Vec::new();

                stack
                    .push_and_return_self(11)
                    .push_and_return_self(22)
                    .push_and_return_self(33);

                while let Some(top) = stack.pop() {
                    println!("pop: {}", top);
                }
            })();

            (|| {
                let myv = vec![Some(1)];

                for /* Some( */ele/* ) */ in myv {
                    println!("{:?}", ele);
                }

                let tup = &(13, 45);

                (|&(x, y): &(i32, i32)| {
                    println!("x: {:?}, y: {:?}", x, y);
                })(tup);

                if let x = 5 {
                    println!("x: {}", x);
                }

                let x: i32 = 3;

                match x {
                    1 => println!("1"),
                    2 => println!("2"),
                    3 => println!("3"),
                    _ => println!("_"),
                };

                match tup {
                    (13, 456) | (456, 13) => println!("13, 456"),
                    (13, _) => println!("13, _"),
                    // (1...100, _) => println!("1...1000, _"),
                    _ => println!("_"),
                }

                struct Point {
                    x: i32,
                    y: i32,
                }

                let point = Point { x: 123, y: 456 };

                let Point { x, y: y_val } = point;

                println!("x: {}, y: {}", x, y_val);

                match point {
                    Point { x: 123, y } => println!("123, y: {}", y),
                    Point { x, y: 456 } => println!("x: {}, 456", x),
                    Point { x, y } => println!("x: {}, y: {}", x, y),
                }

                enum Message {
                    Quit,
                    Move { x: i32, y: i32 },
                    Write(String),
                    Color(i32, f64, bool),
                }

                let msg = Message::Color(123, 0.21398, true);

                match msg {
                    Message::Quit => println!("Quit"),
                    Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),

                    // Message::Write(String::from("LOL")) => println!("Write LOL :)"),
                    Message::Write(str) => println!("Write {}", str),
                    Message::Color(0, ..) => println!("Color 0"),
                    Message::Color(.., true) => println!("Color true"),
                    Message::Color(.., false) => println!("Color false"),
                };

                match Some(Message::Move { x: 123, y: 0 }) {
                    Some(Message::Move { x, y: 0 }) => println!("look! x is: {}", x),
                    _all => println!("..."),
                }

                (|_| 1)(1);

                let mut s1 = Some(123);

                let s2 = Some(456);

                match (s1, s2) {
                    (Some(_), _) => {
                        println!("You can't overwrite value!");
                    }
                    _ => s1 = s2,
                }

                let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];

                fn get_sec_f<'a, T>() -> Box<dyn Fn(&'a [T]) -> Option<&'a T>> {
                    Box::new(|nums: &[T]| -> Option<&T> {
                        match nums {
                            [_, sec, ..] if cfg!(debug_assertions) => Some(sec),
                            _ => None,
                        }
                    })
                }

                assert_eq!(get_sec_f()(&nums), Some(&2));
                let v: Vec<i32> = vec![];
                assert_eq!(get_sec_f()(&v), None);
            })();

            (|| {
                let x = 1;

                let m = match x {
                    lol @ 1 | lol @ 2 | lol @ 4 if lol > 1 => {
                        println!("{}", lol);
                        lol
                    }
                    _ => 9,
                };

                match 10 {
                    v @ 1..=20 => println!("{}", v),
                    vv @ _ => println!("{}", vv),
                };

                println!("{}", m);
            })();

            // 19 chap
            (|| {
                // unsafe
                (|| {
                    // raw pointer
                    (|| {
                        let mut num = 5;

                        let r1 = &num as *const i32;
                        let r2 = &mut num as *mut i32;

                        let _r3 = 0x7ffdd38fc6f8usize as *const i32;

                        unsafe {
                            // println!("{:?}", *r3);

                            *r2 += 100;
                            println!("{:?}", *r1);
                            println!("{:?}", *r2);
                        }

                        let _raw_ref = (|| {
                            let str = String::from("hello");

                            &str as *const String
                        })();

                        // unsafe {
                        //     println!("{:?}", *raw_ref);
                        // }

                        (|| unsafe {
                            type P<T> = *const T;

                            let arr: [i32; 2] = [111, 222];

                            let ptr: P<i32> = &arr[0];

                            println!("arr[1]: {:?}", *((ptr as usize + 0x4) as P<i32>));
                        })();

                        (|| {
                            unsafe fn dangerous() {}

                            unsafe {
                                dangerous();
                            }
                        })();

                        (|| {
                            let mut v = vec![1, 2, 3, 4, 5];

                            let r = &mut v[..];

                            let (a, b) = r.split_at_mut(3);

                            a[0] += 10;

                            assert_eq!(a, &[11, 2, 3]);
                            assert_eq!(b, &[4, 5]);

                            fn split_at_mut(
                                slice: &mut [i32],
                                mid: usize,
                            ) -> (&mut [i32], &mut [i32]) {
                                let len = slice.len();

                                let mptr: *mut i32 = slice.as_mut_ptr();

                                assert!(mid <= len);
                                unsafe {
                                    (
                                        slice::from_raw_parts_mut(mptr, mid),
                                        slice::from_raw_parts_mut(
                                            mptr.offset(mid as isize),
                                            len - mid,
                                        ),
                                    )
                                }
                            }

                            let r2 = &mut v;

                            let (a, b) = split_at_mut(r2, 4);

                            a[1] += 10;

                            assert_eq!(a, &[11, 12, 3, 4]);
                            assert_eq!(b, &[5]);
                        })();

                        (|| unsafe {
                            type P<T> = *const T;

                            let arr: [i32; 4] = [111, 222, 333, 444];

                            let ptr: P<i32> = &arr[0];

                            println!(
                                "last element of arr: {:?}",
                                *ptr.offset(arr.len() as isize - 1)
                            );
                        })();
                    })();

                    // extern
                    (|| {
                        (|| {
                            extern "C" {
                                fn abs(input: i32) -> i32;
                            }

                            (|| unsafe {
                                println!("abs of -123 is: {}", abs(-123));
                            })();

                            (|| {
                                #[no_mangle]
                                unsafe extern "C" fn hello_c(param: u32) -> i32 {
                                    if param == 0xCAFEBABE {
                                        0
                                    } else {
                                        -1
                                    }
                                }

                                (|| {
                                    use std::os::raw::c_int;

                                    extern "C" {
                                        fn abs(args: c_int) -> c_int;
                                    }

                                    (|| {
                                        let x = -1;

                                        println!("abs of {} is: {}", x, unsafe { abs(x) });

                                        extern "C" {
                                            #[link_name = "environ"]
                                            static libc_environ: *const *const std::os::raw::c_char;
                                        }

                                        (|| {
                                            let mut next: *const *const i8 =
                                                unsafe { libc_environ };

                                            while !next.is_null() && !unsafe { *next }.is_null() {
                                                let env: &str = match unsafe {
                                                    std::ffi::CStr::from_ptr(*next)
                                                }
                                                .to_str()
                                                {
                                                    Ok(v) => v,
                                                    Err(_) => "<INVALID>",
                                                };

                                                println!("{}", env);
                                                next = unsafe { next.offset(1) }
                                            }
                                        })();
                                    })();
                                })();
                            })();
                        })();

                        (|| {
                            #[repr(C)]
                            struct Data {
                                a: u32,
                                b: u16,
                            }

                            #[repr(C, packed)]
                            struct PackedData {
                                a: u32,
                            }
                        })();
                    })();

                    // mut static
                    (|| {
                        static mut HELLO_WORLD: &str = "hello world";

                        println!("{}", unsafe { HELLO_WORLD });

                        unsafe {
                            HELLO_WORLD = "hello world :)";
                        }
                    })();

                    // unsafe trait
                    (|| {
                        unsafe trait foo {}

                        unsafe impl foo for i32 {};
                    })();
                })();

                (|| {
                    pub trait Iterator {
                        type Item;

                        fn next(&mut self) -> Option<Self::Item>;
                    }

                    struct Counter;

                    impl Iterator for Counter {
                        type Item = u32;

                        fn next(&mut self) -> Option<Self::Item> {
                            None
                        }
                    }

                    println!("{:?}", Iterator::next(&mut Counter));

                    pub trait Iterator2<T> {
                        fn next(&mut self) -> Option<T>;
                    }

                    impl<T> Iterator2<T> for Counter {
                        fn next(&mut self) -> Option<T> {
                            None
                        }
                    }

                    println!("{:?}", Iterator2::<i32>::next(&mut Counter));
                })();

                {
                    // use std::ops::Add;

                    #[derive(Debug, PartialEq)]
                    struct Point {
                        x: i32,
                        y: i32,
                    }

                    impl Add for Point {
                        type Output = Self;

                        fn add(self, rhs: Self) -> Self::Output {
                            Point {
                                x: self.x + rhs.x,
                                y: self.y + rhs.y,
                            }
                        }
                    }

                    let p1 = Point { x: 123, y: 456 };

                    let p2 = Point {
                        x: 120000,
                        y: 340000,
                    };

                    println!("{:?}", p1 + p2);

                    trait MyAdd<RHS = Self> {
                        type Output;

                        fn add(self, rhs: RHS) -> Self::Output;
                    }

                    impl MyAdd<i32> for Point {
                        type Output = Self;

                        fn add(self, rhs: i32) -> Self::Output {
                            Point {
                                x: self.x + rhs,
                                y: self.y + rhs,
                            }
                        }
                    }

                    println!("{:?}", MyAdd::add(Point { x: 100, y: 0 }, 9000));

                    #[derive(Debug)]
                    struct Meter(f64);
                    #[derive(Debug)]
                    struct MilliMeter(f64);

                    impl Add<Meter> for MilliMeter {
                        type Output = Self;

                        fn add(self, rhs: Meter) -> Self::Output {
                            Self(self.0 + rhs.0 * 1000.0)
                        }
                    }

                    impl Add for MilliMeter {
                        type Output = Self;

                        fn add(self, rhs: Self) -> Self::Output {
                            Self(self.0 + rhs.0)
                        }
                    }

                    impl Add for Meter {
                        type Output = Self;

                        fn add(self, rhs: Self) -> Self::Output {
                            Self(self.0 + rhs.0)
                        }
                    }

                    impl Add<MilliMeter> for Meter {
                        type Output = Self;

                        fn add(self, rhs: MilliMeter) -> Self::Output {
                            Self(self.0 + rhs.0 / 1000.0)
                        }
                    }

                    let mm = MilliMeter(12001.0);
                    let m1 = Meter(123.0);

                    println!("mm + m: {:?}", mm + m1);

                    let mm = MilliMeter(1212189.0);
                    let m1 = Meter(123.0);

                    println!("m + mm: {:?}", m1 + mm);

                    struct Hello;

                    impl Hello {
                        fn asc() {
                            println!("Hello::asc");
                        }

                        fn met(&self) {
                            println!("Hello::met");
                        }
                    }

                    trait Trait {
                        fn asc() {
                            println!("Trait::asc");
                        }

                        fn met(&self) {
                            println!("Trait::met");
                        }
                    }

                    impl Trait for Hello {}

                    Hello::asc();
                    <Hello as Trait>::asc();
                    Hello::met(&Hello);
                    <Hello as Trait>::met(&Hello);
                };

                {
                    pub trait MyDisplay {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>;
                    }

                    struct Point {
                        x: i32,
                        y: i32,
                    }

                    impl MyDisplay for Point {
                        fn fmt(
                            &self,
                            f: &mut std::fmt::Formatter<'_>,
                        ) -> Result<(), std::fmt::Error> {
                            write!(f, "({}, {})", self.x, self.y)
                        }
                    }

                    let _origin = Point { x: 10, y: 30 };

                    // println!("{:?}", origin.fmt());

                    println!("fmt: {:?}", 1_i32.to_string());

                    use std::io::Write;

                    let mut w = Vec::new();

                    let _ = write!(&mut w, "lol{}", "!");

                    let coll: String = w.iter().map(|&x| x as char).collect();

                    println!("w: {:?}", w);
                    println!("coll: {:?}", coll);

                    struct Data {
                        value: i32,
                    }

                    // impl Data {
                    //     fn new
                    // }

                    impl Display for Data {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                            let tmp = self.value.abs().to_string();

                            let a = f.pad_integral(self.value >= 0, "Foo ", &tmp);

                            match a {
                                Ok(v) => Ok(v),
                                Err(err) => Err(err),
                            }
                        }
                    }

                    println!("{}", Data { value: 1 });
                    println!("{}", Data { value: 0 });
                    println!("{}", Data { value: -1 });
                    println!("{:#}", Data { value: 1 });
                    println!("{:0>#8}", Data { value: -1 });

                    println!("Hello");
                    println!("Hello, {}!", "world");
                    println!("The number is: {}", 1);
                    println!("{:?}", (3, 4));
                    println!("{value}", value = 5);
                    println!("{}, {}", 1, 2);
                    println!("{:08}, {}", 41, 2);
                    println!("{:#?}", (410, 200));
                    println!("{} {0} {1} {}", 11, 22);
                    println!("{1} {} {aa} {bb} {} {aa}", 11, 22, aa = "aa", bb = "bb");
                    println!("<{:5}> <{1:5}>", "Ab", 123);
                    println!("<{:1$}>", "Ab", 10);
                    println!("<{1:0$}>", 10, "Ab");
                    println!("<{:width$}>", "Abiria", width = 10);
                    println!("<{:<10}>", "Abiria");
                    println!("<{:+<10}>", "Abiria");
                    println!("<{:^10}>", "Abiria");
                    println!("<{:*^10}>", "Abiria");
                    println!("<{:>10}>", "Abiria");
                    println!("{:+}", 1234);
                    println!("{:#x}", 1234);
                    println!("{:#X}", 1234);
                    println!("{:#b}", 1234);
                    println!("{:07}", 1234);
                    println!("{:07}", -1234);
                    println!("{:07}", -1234);
                    println!("{:#010x}", -1234);

                    struct Foo1(String);

                    impl Display for Foo1 {
                        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                            f.pad(&format!("{}{}", self.0, self.0))
                        }
                    }

                    println!("Foo1: {}", Foo1(String::from("Abiria")));
                    println!("Foo1: {:>20}", Foo1(String::from("Abiria")));
                    println!("Foo1: {:^20}", Foo1(String::from("Abiria")));

                    struct Foo2<'a>(&'a str);

                    impl Display for Foo2<'_> {
                        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                            write!(f, "{}:{}", self.0, self.0)
                        }
                    }

                    println!("Foo2: {}", Foo2("String"));

                    struct Foo3<'a>(Vec<&'a str>);

                    impl Display for Foo3<'_> {
                        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                            f.debug_list().entries(self.0.iter()).finish()
                        }
                    }

                    println!("Foo3: {}", Foo3(vec!["aaa", "bbb"]));

                    struct Abiria<'a> {
                        name: &'a str,
                        bio: &'a str,
                        skills: Vec<&'a str>,
                        view_count: u64,
                    }

                    impl Display for Abiria<'_> {
                        fn fmt(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> Result<(), std::fmt::Error> {
                            write!(
                                formatter,
                                r##"NAME: [ {name} ]

~ {bio} ~
==================

my skills:
{skills}

 (total {vc} views)"##,
                                name = self.name,
                                bio = self.bio,
                                skills = self
                                    .skills
                                    .iter()
                                    .map(|sk| format!("    - {}", sk))
                                    .collect::<Vec<String>>()
                                    .join(",\n"),
                                vc = self.view_count
                            )
                        }
                    }

                    let abiria = Abiria {
                        name: "Abiria.dev",
                        bio: "js backend developer",
                        skills: vec![
                            "JavaScript",
                            "Rust",
                            "Svelte",
                            "Typescript",
                            "Node.js",
                            "Liunx",
                        ],
                        view_count: 1232_u64,
                    };

                    println!("{}", abiria);

                    trait OutlinePrint: Display {
                        fn outline_print(&self) {
                            let out = self.to_string();

                            let len = out.len() + 4;

                            println!("{}", "*".repeat(len));
                            println!("* {} *", out);
                            println!("{}", "*".repeat(len));
                        }
                    }

                    impl OutlinePrint for i32 {}

                    println!("{}", 123);
                    123.outline_print();

                    impl OutlinePrint for Point {};

                    impl Display for Point {
                        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                            write!(f, "({}, {})", self.x, self.y)
                        }
                    }

                    Point { x: 123, y: 456 }.outline_print();
                };

                {
                    struct NewType(Vec<String>);

                    impl Display for NewType {
                        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                            write!(
                                f,
                                "<{}>",
                                self.0
                                    .iter()
                                    .map(|e| format!("({})", e))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            )
                        }
                    }

                    println!(
                        "newtype vector: {}",
                        NewType(vec![
                            String::from("apple"),
                            String::from("banana"),
                            String::from("orange")
                        ])
                    );

                    // !TODO : deref the vector to access all methods of Vec<T>

                    type KiloMeter = i32;

                    let _y: KiloMeter = 121;

                    // let never: ! = loop {};

                    // (|| -> ! { loop {} })();

                    // fn only_slice(sl: &[char]) {
                    //     //
                    //     let str: String = sl.iter().collect();

                    //     println!("{}", str);
                    // }

                    // only_slice(&['a', 'b', 'c']);
                    // only_slice(&vec!['h', 'i']);
                    // only_slice("Abiria");

                    // let s1: Box<str> = Box::<&str>::new("Abiria");

                    let siz = std::mem::size_of::<i32>();

                    println!("{}", siz);

                    println!("{:?}", std::mem::size_of::<Box<str>>());
                    println!("{:?}", std::mem::size_of::<String>());

                    let bb: Box<str> = String::from("Abiria").into_boxed_str();

                    println!("{:?}", bb);

                    let sl2: Box<[i32]> = vec![11, 22, 33].into_boxed_slice();

                    println!("{:?}", sl2);

                    fn siz(_s: &impl ?Sized) {}

                    fn siz2<T: ?Sized>(_s: &T) {}
                };

                {
                    type f_2T<T> = fn(T) -> T;
                    type f_cust<T, U> = fn(T, U) -> U;

                    fn add_one<T: std::ops::Add<Output = T> + Copy + Into<i32>>(a: T) -> i32 {
                        a.into() + 1
                    }

                    fn do_twice<T: std::ops::Add<Output = T> + Copy>(cb: fn(T) -> T, arg: T) -> T {
                        cb(arg) + cb(arg)
                    }

                    let dt: f_cust<f_2T<i32>, i32> = do_twice;

                    println!("{}", do_twice(add_one, 100));
                    println!("{}", dt(add_one, 100));

                    fn asdf<T: std::fmt::Debug>(_x: T) {}

                    // let asdf2: fn(impl std::fmt::Debug) -> () = asdf;

                    fn ret_fn() -> fn() -> i32 {
                        // let asdf = 123;

                        fn sub() -> i32 {
                            // asdf
                            1234
                        }

                        sub
                    }

                    let inner_fn = ret_fn();

                    println!("inner_fn: {}", inner_fn());

                    let only_clj = |clj: &dyn Fn(i32) -> i32, arg| clj(arg);

                    let res = only_clj(&|x| x * x, 11111);

                    println!("res: {}", res);

                    fn blk_fn<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
                        x * x
                    }

                    let res = only_clj(&blk_fn, 1111);

                    println!("res2: {}", res);

                    let _f_ptr: &dyn Fn(i32) -> i32 = &blk_fn;

                    let list_of_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];

                    let list_of_strings: Vec<String> =
                        list_of_numbers.iter().map(ToString::to_string).collect();

                    println!("list_of_strings: {:?}", list_of_strings);

                    let list_of_options: Vec<Option<&i32>> =
                        list_of_numbers.iter().map(Some).collect();

                    println!("list_of_strings: {:?}", list_of_options);
                };
            })();
        })();
    })();
}

// PhantomData

#![feature(type_name_of_val)]

use std::{cell::RefCell, rc::Rc};

// use std::borrow::Borrow;

fn main() {
    (|| {
        let b = Box::new(5);

        println!("b: {}", b);

        let some: fn(i32) -> Option<i32> = Some;

        println!("some: {:?}", some(1));

        let c1 = |myb: Box<i32>| {
            println!("myb: {:?}", *myb);
        };

        c1(b);

        // cons
        {
            enum ConsList<T: std::fmt::Display> {
                Cons(T, Box<ConsList<T>>),
                Nil,
            }

            impl<T: std::fmt::Display> std::fmt::Display for ConsList<T> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        (|| match self {
                            ConsList::Cons(value, inside_cons) =>
                                format!("({}, {})", value, inside_cons),
                            ConsList::Nil => String::from("Nil"),
                        })()
                    )
                }
            }

            use ConsList::*;

            let list = Cons(
                1,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            );

            println!("{}", list);

            {
                let x = 123;

                let x_ref = &x;

                assert_eq!(x, 123);
                assert_eq!(x_ref, &123);
                assert_eq!(*x_ref, 123);
            };

            {
                let x = 123;

                let x_ref = Box::new(x);

                assert_eq!(x, 123);
                assert_eq!(x_ref, Box::new(123));
                assert_eq!(*x_ref, 123);
            };

            {
                struct MyBox<T>(T);

                impl<T> MyBox<T> {
                    fn new(x: T) -> MyBox<T> {
                        MyBox(x)
                    }
                }

                impl<T> std::ops::Deref for MyBox<T> {
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                let mybox = MyBox::new(123);

                use std::ops::Deref;

                // assert_eq!(x_ref, &123);
                assert_eq!(*mybox, 123);
                assert_eq!(mybox.deref(), &123);
                assert_eq!(std::ops::Deref::deref(&mybox), &123);

                struct MyCuteBox<T> {
                    value: T,
                    // value_right: T,
                }

                impl<T> std::ops::Deref for MyCuteBox<T> {
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.value
                    }
                }

                let mcb: MyCuteBox<String> = MyCuteBox {
                    value: String::from("aaaaaaaaaaaaaaaaa"),
                };

                let mcb_deref: &String = mcb.deref();
                assert_eq!(*mcb_deref, String::from("aaaaaaaaaaaaaaaaa"));

                println!("deref: {:?}", *mcb_deref);

                fn hello(str: &str) {
                    println!("Hello, {}", str);
                }

                let lol = MyBox::new(MyBox::new(String::from("lol!")));

                hello(&lol);
                // hello();

                // *(*(*(lol.deref()).deref()))

                hello(&***lol);

                // print_type(&lol);
                prty(&lol);
                prty(&*lol);
                prty(&**lol);
                prty(&***lol);
                prty(&&***lol);
                prty(&lol.deref());
                prty(&(*(lol.deref())));
                prty(&(*(lol.deref())).deref());
                prty(&(*(*(lol.deref())).deref()));
                prty(&((*(*(lol.deref())).deref()).deref()));
                prty(&"aa");
                prty(&String::from("lol"));
                prty(&String::from("lol")[..]);
                prty(&&String::from("lol")[..]);
                prty(&*String::from("lol"));
                prty(&&*String::from("lol"));
                prty(&String::from("lol").deref());

                hello(String::from("hi").deref());

                fn box_only(i: &i32) {
                    println!("i32: {}", i)
                }

                box_only(&MyBox(123));
                box_only(MyBox(123).deref());

                // fn vec_only(ii: )

                prty(&vec![1]);
                prty(&&vec![1]);
                prty(&vec![1].deref());
                prty(&*vec![1]);

                fn only_slice<T: std::fmt::Debug>(sl: &[T]) {
                    println!("sl: {:?}", sl);
                }

                only_slice(&vec![1]);
                only_slice(vec![1].deref());

                prty(&[1]);
                prty(&&[1]);
                // prty(&[1].deref());
                // prty(&*[1])

                only_slice(&[1, 2, 3]);
                // only_slice("aaa".into_slice());

                // prty(&"aa".deref());

                // hello()
                // prty(&Box::new(String::from("lol")));
                // prty(&((Box::new(String::from("lol"))).deref()));
                // prty(&(((Box::new(String::from("lol"))).deref()).deref()));
                let mvvv: &Box<String> = &Box::new(String::from("lol"));
                let mvvv2 = mvvv.deref().deref();

                prty(&mvvv2);
                prty(&mvvv);
                prty(&(*mvvv));
                prty(&(**mvvv));
                prty(&(&(**mvvv)[..]));
                prty(&(&***mvvv));
                prty(&((**mvvv).deref()));

                hello(mvvv.deref().deref());
                // hello((*mvvv));

                // println!("{}", std::any::type_name_of_val(&lol));

                use std::any::type_name_of_val;

                // let prty = |x: &T| std::any::type_name_of_val(&lol);
                fn prty<T: ?Sized>(x: &T) {
                    println!("type: {}", type_name_of_val(x));
                }

                fn prty_<T: ?Sized>(x: &T, opt: &str) {
                    println!("typeof {}: {}", opt, type_name_of_val(x));
                }

                let string = String::from("Hello");

                // *sss = *"lol zzz";

                // let a: &str = string.as_slice();
                let b = string.as_str();
                let c = &string[..];
                let e = &*string;
                let d = string.deref();

                // let a = sss.deref();

                let ss = "dkjldsajks";

                // let q: Box<str> = Box::(*ss);

                fn print_type<T>(_: &T) {
                    println!("type: {}", std::any::type_name::<T>());
                }

                fn print_type_<T>(_: &T, opt: &str) {
                    println!("typeof {}: {}", opt, std::any::type_name::<T>());
                }

                fn bound(a: &impl std::ops::Deref<Target = i32>) {
                    println!("bound");
                }

                bound(&mybox);
                bound(&Box::new(123));
                // bound(&Box::new(Box::new(187871)));
                // bound(Box::new("aaa"));

                fn b2(a: &i32) {
                    println!("a: {}", a);
                }

                b2(&1111);
                let bb = &Box::new(11122);
                b2(bb);

                struct MB<T: std::fmt::Debug>(T);

                impl<T: std::fmt::Debug> Drop for MB<T> {
                    fn drop(&mut self) {
                        println!("unregister {:?}", self.0);
                    }
                }

                let myb1 = MB(123);
                drop(myb1);
                let myb2 = MB("sdjs");
            };

            {
                mod A {
                    pub mod B {
                        pub trait MyTrait {
                            fn my_asc_fn() -> &'static str {
                                "Hello, asc function! :)"
                            }

                            fn my_method_fn(&self) -> &'static str {
                                "Hello, method function :)"
                            }
                        }
                    }
                }

                struct MyStruct;

                impl A::B::MyTrait for MyStruct {}

                // use A::B::MyTrait;

                // println!("str - asc: {}", MyStruct::my_asc_fn());
                // println!("str - hethod: {}", MyStruct.my_method_fn());

                println!("str - asc: {}", <MyStruct as A::B::MyTrait>::my_asc_fn());
                println!("str - hethod: {}", A::B::MyTrait::my_method_fn(&MyStruct));
            };

            {
                use std::ops::Deref;
                use std::rc::Rc;

                fn prty<T: ?Sized>(x: &T) {
                    println!("type: {}", std::any::type_name_of_val(x));
                }

                let nil = ConsList::<i32>::Nil;

                let ten = Cons(10, Box::new(nil));

                let A_five = Cons(5, Box::new(ten));

                let A_five: Rc<ConsList<i32>> = Rc::new(A_five);

                let B_three: Rc<ConsList<i32>> = Rc::clone(&A_five);

                let C_four = A_five.clone();

                // println!("{:?}", B_three);
                println!("{}", B_three.deref());

                (|| {
                    let s = String::from("hola!");

                    // let box1 = Box::new(s);
                    // let box2 = Box::new(s);

                    let s_rc: Rc<String> = Rc::new(s);
                    println!("strong_count: {:?}", Rc::strong_count(&s_rc));

                    let rc1: Rc<String> = s_rc.clone();
                    println!("strong_count: {:?}", Rc::strong_count(&s_rc));
                    let rc2: Rc<String> = s_rc.clone();
                    println!("strong_count: {:?}", Rc::strong_count(&rc1));

                    println!("rc1 {}", rc1);
                    println!("rc2 {}", rc2);

                    let s_rc2 = s_rc;

                    println!("s_rc2: {}", s_rc2);

                    println!("rc1 {}", rc1);
                    println!("rc2 {}", rc2);

                    {
                        let rc3 = rc2.clone();
                        println!("strong_count: {:?}", Rc::strong_count(&rc3));

                        println!("rc3 {}", rc3);
                    }
                    println!("strong_count: {:?}", Rc::strong_count(&rc2));

                    prty(&(rc1));
                    prty(&(rc1.deref()));
                    prty(&(*rc1));

                    // *rc1 = String::from("aloh!");

                    println!("rc1 {}", rc1);
                    println!("rc2 {}", rc2);
                })();

                mod my_rc {
                    pub struct MyRc<T> {
                        value: std::rc::Rc<T>,
                        count: std::rc::Rc<std::cell::RefCell<u32>>,
                    }

                    impl<T> MyRc<T> {
                        pub fn new(value: T) -> Self {
                            Self {
                                value: std::rc::Rc::new(value),
                                count: std::rc::Rc::new(std::cell::RefCell::new(1)),
                            }
                        }

                        pub fn strong_count(rc: &Self) -> u32 {
                            *rc.count.borrow()
                        }

                        pub fn clone(&self) -> Self {
                            *self.count.borrow_mut() += 1;

                            Self {
                                count: std::rc::Rc::clone(&self.count),
                                value: std::rc::Rc::clone(&self.value),
                            }
                        }
                    }

                    impl<T> std::ops::Deref for MyRc<T> {
                        type Target = T;

                        fn deref(&self) -> &Self::Target {
                            &*self.value
                        }
                    }

                    impl<T> std::ops::Drop for MyRc<T> {
                        fn drop(&mut self) {
                            *self.count.borrow_mut() -= 1;
                        }
                    }
                }

                use my_rc::MyRc;

                let myrc: MyRc<String> = MyRc::new(String::from("aaa"));
                println!("count: {}", MyRc::strong_count(&myrc));

                let myrc_clone1 = myrc.clone();
                println!("count: {}", MyRc::strong_count(&myrc_clone1));
                let myrc_clone2 = myrc.clone();
                println!("count: {}", MyRc::strong_count(&myrc_clone2));
                let myrc_clone3 = myrc_clone2.clone();
                println!("count: {}", MyRc::strong_count(&myrc_clone3));

                println!("myrc_clone1: {}", *myrc_clone1);
                println!("myrc_clone2: {}", *myrc_clone2);
                println!("myrc_clone3: {}", *myrc_clone3);

                enum RcConsList<T> {
                    Cons(T, MyRc<RcConsList<T>>),
                    Nil,
                }

                impl<T: std::fmt::Display> std::fmt::Display for RcConsList<T> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(
                            f,
                            "{}",
                            (|| match self {
                                Self::Cons(value, inside_cons) =>
                                    format!("({}, {})", value, **inside_cons),
                                Self::Nil => String::from("Nil"),
                            })()
                        )
                    }
                }

                let nil = RcConsList::<i32>::Nil;

                let ten = RcConsList::Cons(10, MyRc::new(nil));

                let A_five: RcConsList<i32> = RcConsList::Cons(5, MyRc::new(ten));

                let five_rc: MyRc<RcConsList<i32>> = MyRc::new(A_five);

                let B_three: RcConsList<i32> = RcConsList::Cons(3, five_rc.clone());

                let C_four: RcConsList<i32> = RcConsList::Cons(4, MyRc::clone(&five_rc));

                println!("A: {}", *five_rc);
                println!("B: {}", B_three);
                println!("C: {}", C_four);

                fn a() {
                    use std::rc::Rc;

                    enum RcCons<T> {
                        Cons(T, Rc<RcCons<T>>),
                        Nil,
                    }

                    use RcCons::*;

                    impl<T: std::fmt::Display> std::fmt::Display for RcCons<T> {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(
                                f,
                                "{}",
                                (|| match self {
                                    Self::Cons(value, inside_cons) =>
                                        format!("({}, {})", value, **inside_cons),
                                    Self::Nil => String::from("Nil"),
                                })()
                            )
                        }
                    }

                    let nil = Rc::new(Nil);

                    let ten = Rc::new(Cons(10, nil.clone()));

                    let five: Rc<RcCons<i32>> = Rc::new(Cons(5, ten.clone()));

                    let three: Rc<RcCons<i32>> = Rc::new(Cons(3, Rc::clone(&five)));

                    let four: Rc<RcCons<i32>> = Rc::new(Cons(4, five.clone()));

                    println!("A: {}", *five);
                    println!("B: {}", *three);
                    println!("C: {}", *four);
                }

                a();

                {
                    pub trait Messenger {
                        fn send(&self, msg: String);
                    }

                    pub struct LOL;

                    impl Messenger for LOL {
                        fn send(&self, msg: String) {
                            println!("Messenger: {}", msg)
                        }
                    }

                    pub struct LimitTracker<'a, T: 'a + Messenger> {
                        messenger: &'a T,
                        value: i32,
                        max_value: i32,
                    }

                    impl<'lol, T: Messenger> LimitTracker<'lol, T> {
                        pub fn new(messenger: &'lol T, max_value: i32) -> Self {
                            Self {
                                messenger,
                                value: 0,
                                max_value,
                            }
                        }

                        pub fn set_value(&mut self, value: i32) {
                            *self = Self { value, ..*self };

                            let perc: f64 = self.value as f64 / self.max_value as f64;

                            match perc {
                                v @ 0.75..=0.9 => {
                                    self.messenger.send(format!("warnning!! over 75%!: {}", v))
                                }

                                v @ 0.9..=1.0 => {
                                    self.messenger.send(format!("warnning!! over 90%!: {}", v))
                                }

                                v if v > 1.0 => self
                                    .messenger
                                    .send(format!("warnning!! you exceed max!: {}", v)),

                                _ => {}
                            }
                        }
                    }

                    // usestr
                    struct Mock {
                        sm: std::cell::RefCell<Vec<String>>,
                    }

                    impl Messenger for Mock {
                        fn send(&self, msg: String) {
                            self.sm.borrow_mut().push(msg);
                        }
                    }

                    // #[test]
                    // fn t() {
                    let mock = Mock {
                        sm: std::cell::RefCell::new(vec![]),
                    };
                    let mut lt: LimitTracker<'_, Mock> = LimitTracker::new(&mock, 100);

                    // lt.messenger.send("skjsjk");

                    lt.set_value(1);
                    lt.set_value(80);
                    lt.set_value(95);
                    lt.set_value(101);

                    println!("logs: {:?}", mock.sm.borrow().join("   ,   "));
                    // }
                };

                {
                    let val = 123;

                    let refcell: std::cell::RefCell<i32> = std::cell::RefCell::new(val);

                    let mut mut_ref: std::cell::RefMut<i32> = refcell.borrow_mut();

                    *mut_ref += 10000;

                    drop(mut_ref);

                    let ref1: std::cell::Ref<i32> = refcell.borrow();

                    println!("mut ref: {}", ref1);

                    prty(&ref1);
                    prty(&(ref1.deref()));
                    prty(&(*ref1));
                };
            };

            {
                use std::cell::RefCell;
                use std::rc::Rc;

                let rc_refcell: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("hello")));

                let rc2: Rc<RefCell<String>> = rc_refcell.clone();

                rc2.borrow_mut().push_str(", world!");

                println!("rc_refcell: {}", RefCell::borrow(&rc_refcell));
            };

            (|| {
                use std::{cell::RefCell, rc::Rc};

                enum RcRefCons<T: std::fmt::Display> {
                    Cons(Rc<RefCell<T>>, Rc<RefCell<RcRefCons<T>>>),
                    Nil,
                }

                impl<T: std::fmt::Display> std::fmt::Display for RcRefCons<T> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(
                            f,
                            "{}",
                            (|| match self {
                                RcRefCons::Cons(value, inside_cons) => format!(
                                    "({}, {})",
                                    *RefCell::borrow(&value),
                                    RefCell::borrow(&inside_cons)
                                ),
                                RcRefCons::Nil => String::from("Nil"),
                            })()
                        )
                    }
                }

                let nil: Rc<RefCell<RcRefCons<i32>>> = Rc::new(RefCell::new(RcRefCons::Nil));

                let ten = Rc::new(RefCell::new(RcRefCons::Cons(
                    Rc::new(RefCell::new(10)),
                    nil,
                )));

                let val: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));

                let a = Rc::new(RefCell::new(RcRefCons::Cons(val.clone(), ten)));

                let b = Rc::new(RefCell::new(RcRefCons::Cons(
                    Rc::new(RefCell::new(3)),
                    a.clone(),
                )));

                let c = Rc::new(RefCell::new(RcRefCons::Cons(
                    Rc::new(RefCell::new(4)),
                    Rc::clone(&a),
                )));

                println!("A: {}", RefCell::borrow(&a));
                println!("B: {}", RefCell::borrow(&b));
                println!("C: {}", RefCell::borrow(&c));

                *RefCell::borrow_mut(&val) += 10;

                println!("A: {}", RefCell::borrow(&a));
                println!("B: {}", RefCell::borrow(&b));
                println!("C: {}", RefCell::borrow(&c));
            })();
        };
    })();

    {
        let a = 1;
        let ref aa = a;

        let th = Some(String::from("hello world!"));

        match th {
            Some(ref val) => println!("{}", val),
            None => println!("world"),
        };

        println!("{:?}", th);

        match 3 {
            ref mut num @ 1..=4 => println!("1..4: {:?}", num),
            num => println!("..: {:?}", num),
        }
    };

    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        use std::{cell::RefCell, rc::Rc};
        use List::*;

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match *self {
                    Nil => None,
                    Cons(_, ref refcell_rc_i32) => Some(refcell_rc_i32),
                }
            }
        }

        let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("A: rc count -> {}", Rc::strong_count(&a));
        println!("A: a.tail -> {:?}", a.tail());

        let b: Rc<List> = Rc::new(Cons(4, RefCell::new(a.clone())));

        println!("A: rc count2 -> {}", Rc::strong_count(&a));
        println!("B: rc count -> {}", Rc::strong_count(&b));
        println!("B: b.tail -> {:?}", b.tail());

        if let Some(v) = a.tail() {
            // println!("{:?}", v);
            *v.borrow_mut() = b.clone();
        }

        println!("A: rc count3 -> {}", Rc::strong_count(&a));
        println!("B: rc count2 -> {}", Rc::strong_count(&b));
        // println!("A: a.tail2 -> {:?}", a.tail());
        // println!("B: b.tail2 -> {:?}", b.tail());

        fn prty<T: ?Sized>(x: &T) {
            println!("type: {}", std::any::type_name_of_val(x));
        }
    };

    {
        use std::rc::Weak;

        #[derive(Debug)]
        struct Node {
            value: i32,
            children: RefCell<Vec<Rc<Node>>>,
            parent: RefCell<Weak<Node>>,
        }

        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(
                    f,
                    "[{}]{}",
                    self.value,
                    if self.children.borrow().len() > 0 {
                        format!(
                            " /\n{}",
                            (*self.children.borrow())
                                .iter()
                                .map(|rc_n: &Rc<Node>| format!(" ├────{}\n", *rc_n))
                                .collect::<String>()
                        )
                    } else {
                        format!("")
                    }
                )
            }
        }

        let child: Rc<Node> = Rc::new(Node {
            value: 1,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        println!(
            "strong: {}, weak: {}",
            Rc::strong_count(&child),
            Rc::weak_count(&child)
        );

        let parent = Node {
            value: 2,
            children: RefCell::new(vec![Rc::clone(&child)]),
            parent: RefCell::new(Weak::new()),
        };

        println!(
            "strong: {}, weak: {}",
            Rc::strong_count(&child),
            Rc::weak_count(&child)
        );

        let p_rc = Rc::new(parent);

        *child.parent.borrow_mut() = Rc::downgrade(&p_rc);

        // let nn = Node {
        //     value: 29,
        //     children: RefCell::new(vec![child.clone()]),
        //     parent: RefCell::new(Weak::new()),
        // };

        // (*p_rc.children.borrow_mut()).push(Rc::new(Node {
        //     value: 3,
        //     children: RefCell::new(vec![]),
        //     parent: RefCell::new(Weak::new()),
        // }));

        // (*p_rc.children.borrow_mut()).push(Rc::new(nn));

        println!("{}", p_rc);

        // println!("{:?}", child.parent.borrow().upgrade().unwrap().value);

        println!(
            "strong: {}, weak: {}",
            Rc::strong_count(&child),
            Rc::weak_count(&child)
        );
    };
}

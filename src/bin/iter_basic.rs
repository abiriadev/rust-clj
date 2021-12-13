pub fn main() {
    (|| {
        //
        let mut v = vec![1, 2, 3];

        let iter = v.iter();

        for val in iter {
            println!("{} ", val);
        }

        trait MyIterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        struct TestIter(i32);

        impl MyIterator for TestIter {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    0 => None,
                    v @ _ => {
                        self.0 -= 1;

                        Some(v)
                    }
                }
            }
        }

        let mut testIter = TestIter(5);

        loop {
            match testIter.next() {
                Some(v) => println!("v: {}", v),
                None => break,
            };
        }

        for vv in v.iter_mut() {
            *vv += 100;
            println!("vv: {}", vv);
        }

        let sum: u64 = v.iter().sum();
        println!("sum: {}", sum);

        for vv in v.into_iter() {
            println!("vv: {}", vv);
        }

        let chars: Vec<u8> = vec![97, 98, 99, 100];

        let abcd: String = chars.into_iter().map(|e| e as char).collect();

        println!("abcd: {}", abcd);

        let mut shs = vec![1, 2, 3];

        shs.iter_mut().for_each(|e| *e += 100);

        println!("shs: {:?}", shs);

        let coll = shs.iter().filter(|&e| *e > 101).collect::<Vec<_>>();

        println!("coll: {:?}", coll);

        struct TestIter2(i32);

        impl Iterator for TestIter2 {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    0 => None,
                    v @ _ => {
                        self.0 -= 1;

                        Some(v)
                    }
                }
            }
        }

        let testIter2 = TestIter2(6);

        let my_v: Vec<i32> = testIter2.map(|e| e * 10).collect();

        println!("my_v: {:?}", my_v);

        let ssum: i32 = TestIter2(100)
            .zip(TestIter2(100).skip(1))
            .inspect(|(x, y)| println!("{x} * {y} = {xy}", x = x, y = y, xy = x * y))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .inspect(|n| println!("remained? {:?} | n: {n}", true, n = n))
            .sum();

        println!("ssum: {}", ssum);
    })();
}

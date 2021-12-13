#[macro_export]
macro_rules! mvec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}

macro_rules! sum {
    ( $( $x:expr ), * ) => {
        {
            // let mut temp_vec = Vec::new();
            let mut sum = 0;

            $(
                // temp_vec.push($x);
                sum += $x;
            )*

            // temp_vec
            sum
        }
    };
}

fn main() {
    println!(
        "{}",
        mvec![1, 2, 3]
            .into_iter()
            .map(|a| a.to_string())
            // .map(ToString::to_string)
            .collect::<String>()
    );

    println!("sum: {}", sum![1, 2, 3]);
}

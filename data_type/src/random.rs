use rand::Rng;

fn learn_random() {
    let mut generator = rand::rng();
    // 1..=3右侧闭区间 1..3右侧开区间
    let num: i32 = generator.random_range(1..=3);
    // let y: f64 = gen.gen(); // generates a float between 0 and 1
    println!("{}", num);
}

// tokio::spawn(async move {
//         let mut rng = StdRng::from_entropy();
// })

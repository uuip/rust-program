use std::iter::repeat_with;

use ndarray::prelude::*;

fn make_a2_from_vec() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let n_row = 10;
    let n_column = 5;

    let mut rng = fastrand::Rng::new();

    let out: Vec<f64> = repeat_with(|| rng.f64()).take(n_row * n_column).collect();
    let data = Array2::from_shape_vec((n_row, n_column), out)?;
    println!("{:?}", data.slice(s![0, ..]));
    println!("{:?}", data[[0, 1]]);
    Ok(())
}

fn make_a2_from_fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let n_row = 10;
    let n_column = 5;

    let mut rng = fastrand::Rng::new();
    let data = Array2::from_shape_simple_fn((n_row, n_column), || rng.f64());
    // let data = data.t();
    println!("{:?}", data[[0, 1]]);
    Ok(())
}

use std::iter::repeat_with;

use ndarray::prelude::*;

fn make_a2_from_vec() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let n_row = 1_0;
    let n_column = 5;

    let mut rng = fastrand::Rng::new();

    let mut out: Vec<Vec<f64>> = Vec::new();
    for _ in 0..n_row {
        let navi = Vec::from_iter(repeat_with(|| rng.f64()).take(n_column));
        out.push(navi);
    }
    let data = Array2::from_shape_vec((n_row, n_column), out.iter().flatten().collect())?;
    println!("{:?}", data.slice(s![0, ..]));
    println!("{:?}", data[[0, 1]]);
    Ok(())
}

fn make_a2_from_fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let n_row = 1_0;
    let n_column = 5;

    let mut rng = fastrand::Rng::new();
    let data = Array2::from_shape_simple_fn((n_column, n_row), || rng.f64());
    let data = data.t();
    println!("{:?}", data[[0, 1]]);
    Ok(())
}

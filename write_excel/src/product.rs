use itertools::Itertools;

fn product(n: usize) -> Vec<String> {
    let chars = ('a'..='z').chain('A'..='Z');
    chars
        .clone()
        .cartesian_product(chars)
        .map(|(a, b)| format!("{a}{b}"))
        .take(n)
        .collect()
}

use itertools::Itertools;

fn product(n: usize) -> Vec<String> {
    let chars = ('a'..='z').chain('A'..='Z');
    chars
        .cartesian_product(('a'..='z').chain('A'..='Z'))
        .take(n)
        .map(|(a, b)| {
            let mut s = String::with_capacity(2);
            s.push(a);
            s.push(b);
            s
        })
        .collect()
}

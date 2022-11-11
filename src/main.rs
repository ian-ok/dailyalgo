fn main() {
    let mut a = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let b = a.to_owned();

    for i in 0..=(a.len() - 1) {
        for j in 0..=(a.len() - 1) {
            a[i][j] = b[&a.len() - 1 - j][i];
        }
    }

    println!("{:?}", a)
}

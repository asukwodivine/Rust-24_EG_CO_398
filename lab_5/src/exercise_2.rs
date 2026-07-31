pub fn run() {
    let nums1 = [1, 2, 3];
    let nums2 = [10, 20, 30];
    let sums = zip_with(&nums1, &nums2, |x, y| x + y);
    println!("Exercise 2 sums: {:?}", sums);

    let names1 = ["apple", "mango"];
    let names2 = ["pie", "banana"];
    let joined = zip_with(&names1, &names2, |x, y| format!("{} {}", x, y));
    println!("Exercise 2 joined: {:?}", joined);
}

fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

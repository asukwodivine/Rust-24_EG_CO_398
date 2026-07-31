pub fn run() {
    fn stats(data: &[f64]) -> (f64, f64, f64) {
        let sum: f64 = data.iter().sum();
        let mean = sum / data.len() as f64;
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (mean, min, max)
    }

    let mut scores: Vec<f64> = vec![85.0, 92.0, 78.5, 95.0, 60.0, 88.0];

    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted: {:?}", scores);

    let high_scores: Vec<f64> = scores.iter().filter(|&&s| s >= 80.0).copied().collect();
    println!("High scores: {:?}", high_scores);

    let (mean, min, max) = stats(&scores);
    println!("Mean={:.2} Min={:.2} Max={:.2}", mean, min, max);

    let median = if scores.len().is_multiple_of(2){
        (scores[scores.len() / 2 - 1] + scores[scores.len() / 2]) / 2.0
    } else {
        scores[scores.len() / 2]
    };
    println!("Median: {:.2}", median);

    let variance = scores.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / scores.len() as f64;
    let std_dev = variance.sqrt();
    println!("Variance: {:.2}, Std Dev: {:.2}", variance, std_dev);
}

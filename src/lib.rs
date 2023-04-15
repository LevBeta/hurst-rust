pub fn hurst_exponent(x: Vec<f64>) -> f64 {
    let n: f64 = x.len() as f64;
    let mean: f64 = mean(&x, &n);
    let y = x.iter().map(|&v| v - mean).collect::<Vec<f64>>();
    let cums = cumulative_sum(&y);
    let r = range(&cums);
    let std = std_dev(&x, &n, &mean);
    ((r / std).log2()) / (n.log2())
}

pub fn mean(x: &Vec<f64>, n: &f64) -> f64 {
    let sum: f64 = x.iter().sum();
    sum / n
}

pub fn range(values: &[f64]) -> f64 {
    let (min, max) = values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| (min.min(*x), max.max(*x)));
    max - min
}

pub fn std_dev(x: &[f64], n: &f64, mean: &f64) -> f64 {
    let variance = x.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    variance.sqrt()
}

pub fn cumulative_sum(x: &Vec<f64>) -> Vec<f64> {
    let mut acc = 0.0;
    let mut result = Vec::with_capacity(x.len());
    for a in x.iter().copied() {
        acc += a;
        result.push(acc);
    }
    result
}

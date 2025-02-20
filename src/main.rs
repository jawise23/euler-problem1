use std::time::Instant;

/*
fn sum_of_multiples_v1(limit: i64, factors: &[i64]) -> Option<i64> {
    if limit <= 0 || factors.is_empty() {
        return None;
    }

    Some(
        (1..limit)
            .filter(|&n| factors.iter().any(|&f| f != 0 && n % f == 0))
            .sum(),
    )
}
*/
fn sum_of_multiples_v1(limit: i64, factors: &[i64]) -> Option<i64> {
    if limit <= 0 || factors.is_empty() {
        return None;
    }

    Some(
        (1..limit)
            .filter(|&n| factors.iter().any(|&f| f != 0 && n % f == 0))
            .sum(),
    )
}

// New implementation using arithmetic progression
fn sum_of_multiples_v2(limit: u128, factors: &[u128]) -> Option<u128> {
    if limit <= 0 || factors.is_empty() {
        return None;
    }

    fn sum_divisible_by(n: u128, limit: u128) -> u128 {
        let p = (limit - 1) / n;
        n * p * (p + 1) / 2
    }

    Some(sum_divisible_by(3, limit) + sum_divisible_by(5, limit) - sum_divisible_by(15, limit))
}

fn benchmark(iterations: u32, limit: u128) {
    let mut times_v2 = std::time::Duration::new(0, 0);

    println!("Testing with limit: {}", limit);

    for _ in 0..iterations {
        let start = Instant::now();
        let _result2 = sum_of_multiples_v2(limit, &[3_u128, 5_u128]);
        times_v2 += start.elapsed();
    }

    println!("\nOver {} iterations:", iterations);
    println!("V2 average time: {:?}", times_v2 / iterations);
    println!("V2 total time: {:?}", times_v2);
    println!(
        "Result: {:?}",
        sum_of_multiples_v2(limit, &[3_u128, 5_u128])
    );
}

fn main() {
    let test_cases = vec![1_000_000_000_000_u128];

    for limit in test_cases {
        benchmark(1000, limit);
        println!("----------------------------------------");
    }
}

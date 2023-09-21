fn sum_u32_collection(numbers: &[u32]) -> Option<u32> {
    let mut result = 0u32;

    for &num in numbers {
        // 使用checked_add方法来检查加法是否会溢出，如果溢出返回None
        result = result.checked_add(num)?;
    }

    Some(result)
}

fn main() {
    let numbers1 = &[1, 2, 3, 4, 5];
    let numbers2 = &[u32::MAX, 1];

    match sum_u32_collection(numbers1) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred!"),
    }

    match sum_u32_collection(numbers2) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred!"),
    }
}

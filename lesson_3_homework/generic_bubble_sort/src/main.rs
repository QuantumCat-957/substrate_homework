fn main() {
    println!("Hello, world!");
}

fn generic_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn int_bubble_sort() {
        let mut int_arr = [64, 34, 25, 12, 22, 11, 90];
        println!("Original int array: {:?}", int_arr);
        crate::generic_bubble_sort(&mut int_arr);
        println!("Sorted int array: {:?}", int_arr);
        let result = [11, 12, 22, 25, 34, 64, 90];
        assert_eq!(result, int_arr);
    }

    #[test]
    fn float_bubble_sort() {
        let mut float_arr = [64.0, 34.5, 25.1, 12.9, 22.7, 11.3, 90.6];

        println!("Original float array: {:?}", float_arr);
        crate::generic_bubble_sort(&mut float_arr);
        println!("Sorted float array: {:?}", float_arr);
        let result = [11.3, 12.9, 22.7, 25.1, 34.5, 64.0, 90.6];
        assert_eq!(result, float_arr);
    }
}

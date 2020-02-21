fn quick_sort(list: &mut Vec<usize>, p: i32, r: i32) {
    if p < r {
        let q = partition(list, p, r);
        println!("input: {:?}", p);
        quick_sort(list, p, q - 1);
        quick_sort(list, q + 1, r);
    }
}

fn partition(list: &mut Vec<usize>, p: i32, r: i32) -> i32 {
    let x = list[r as usize];
    let mut i = p - 1;
    for j in p..r {
        if list[j as usize] <= x {
            i = i + 1;
            list.swap(i as usize, j as usize);
        }
    }

    i = i + 1;
    list.swap(i as usize, r as usize);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let mut input = vec![2, 8, 7, 1, 3, 5, 6, 4];
        println!("input: {:?}", input);

        let expect = vec![2, 1, 3, 4, 7, 5, 6, 8];
        let q = partition(&mut input, 0, 7);

        assert_eq!(input, expect);
        assert_eq!(q, 3);
    }

    #[test]
    fn test_quick_sort_even() {
        let mut input = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let expect = vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16];
        quick_sort(&mut input, 0, 9);
        assert_eq!(input, expect);
    }
}

fn main() {
    let mut list = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    quick_sort(&mut list, 0, 9);
    println!("{:?}", list)
}

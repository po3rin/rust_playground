struct List {
    data : Vec<usize>,
    heap_size: usize,
}

fn left(i: usize) -> usize {
    2 * i +1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn max_heapify(list: &mut Vec<usize>, i :usize) {
    let l = left(i);
    let r = right(i);

    let mut largest = i;
    if l < list.len() && list[l] > list[i]{
        largest = l
    }
    if r < list.len() && list[r] > list[largest]{
        largest = r
    }

    if largest != i {
        list.swap(i, largest);
        max_heapify(list, largest);
    }
}

fn build_max_heap(list: &mut Vec<usize>) {
    let heap_size = list.len();
    for i in (0..heap_size/2).rev() {
        println!("{}",i);
        max_heapify(list, i)
    }
}

// fn heap_sort(list: &mut Vec<usize>) {
//     build_max_heap(list)
//     for i in (2..list.len()+1).rev() {
//         println!("{}",i);
//         max_heapify(list, 0)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_heapify() {
        let mut input = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        max_heapify(&mut input, 1);
        assert_eq!(input, expect);
    }

    #[test]
    fn test_build_max_heap() {
        let mut input = vec![4,1,3,2,16,9,10,14,8,7];
        let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        build_max_heap(&mut input);
        assert_eq!(input, expect);
    }
}

fn main() {
    let mut list = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    max_heapify(&mut list, 1);
    println!("{:?}", list)
}

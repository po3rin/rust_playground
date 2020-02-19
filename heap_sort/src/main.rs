struct List {
    data: Vec<usize>,
    heap_size: usize,
}

impl List {
    fn new(data: Vec<usize>) -> Self {
        let heap_size = data.len();
        List { data, heap_size }
    }
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn max_heapify(list: &mut List, i: usize) {
    let l = left(i);
    let r = right(i);

    let mut largest = i;
    if l < list.heap_size && list.data[l] > list.data[i] {
        largest = l
    }
    if r < list.heap_size && list.data[r] > list.data[largest] {
        largest = r
    }

    if largest != i {
        list.data.swap(i, largest);
        max_heapify(list, largest);
    }
}

fn build_max_heap(list: &mut List) {
    list.heap_size = list.data.len();
    for i in (0..list.data.len() / 2).rev() {
        max_heapify(list, i)
    }
}

fn heap_sort(list: &mut List) {
    build_max_heap(list);
    for i in (1..list.data.len()).rev() {
        list.data.swap(i, 0);
        list.heap_size = list.heap_size - 1;
        max_heapify(list, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_heapify() {
        let mut input = List::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        max_heapify(&mut input, 1);
        assert_eq!(input.data, expect);
    }

    #[test]
    fn test_build_max_heap() {
        let mut input = List::new(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
        let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        build_max_heap(&mut input);
        assert_eq!(input.data, expect);
    }

    #[test]
    fn test_heap_sort_even() {
        let mut input = List::new(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
        let expect = vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16];
        heap_sort(&mut input);
        assert_eq!(input.data, expect);
    }

    #[test]
    fn test_heap_sort_odd() {
        let mut input = List::new(vec![16, 14, 10, 8, 7, 9, 3, 2, 4]);
        let expect = vec![2, 3, 4, 7, 8, 9, 10, 14, 16];
        heap_sort(&mut input);
        assert_eq!(input.data, expect);
    }
}

fn main() {
    let mut list = List::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
    max_heapify(&mut list, 1);
    println!("{:?}", list.data)
}

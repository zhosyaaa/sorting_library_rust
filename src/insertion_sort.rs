
pub fn insertion_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j - 1, j);
            j -= 1;
        }
    }
}
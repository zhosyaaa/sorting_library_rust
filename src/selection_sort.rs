
pub fn selection_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    for i in 0..input.len() {
        let swap_val = {
            let mut min = &input[i];
            let mut index_min = i;
            
            for j in i + 1..input.len() {
                if input[j] < *min {
                    min = &input[j];
                    index_min = j;
                }
            }
            index_min
        };

        if i != swap_val {
            input.swap(i, swap_val);
        }
    }
}

pub fn selection_double_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let mut left = 0;
    let mut right = input.len() - 1;
    let mut min = left;
    let mut max = left;

    while left <= right {
        for i in left..=right {
            if input[i] > input[max] {
                max = i;
            }
            if input[i] < input[min] {
                min = i;
            }
        }
        if max == left {max = min;}
        input.swap(left, min);
        input.swap(right, max);

        left += 1;
        right -= 1;

        min = left;
        max = right;
    }
}

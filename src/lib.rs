use std::cmp::min;

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

pub fn merge_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let len = input.len();
    let mid = len / 2;
    merge_sort(&mut input[..mid]);
    merge_sort(&mut input[mid..]);

    let mut tmp = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        if input[i] < input[j] {
            tmp.push(input[i]);
            i += 1;
        } else {
            tmp.push(input[j]);
            j += 1;
        }
    }
    if i < mid {
        tmp.extend_from_slice(&input[i..mid]);
    } else if j < len {
        tmp.extend_from_slice(&input[j..len]);
    }

    input.copy_from_slice(&tmp[..]);
}

pub fn merge_bottom_up_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    let mut width = 1;

    let mut tmp = input.to_vec();
    let len = input.len();

    while width < len {
        let mut i = 0;
        while i < len {
            let start = min(i + 2 * width, len);
            let mid = min(i + width, len);

            merge(&input[i..mid], &input[mid..start], &mut tmp[i..start]);
            input[i..start].copy_from_slice(&tmp[i..start]);

            i += 2 * width;
        }
        width *= 2;
    }
}

fn merge<T: PartialOrd + Copy>(in1: &[T], in2: &[T], tmp: &mut [T]) {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while left < in1.len() && right < in2.len() {
        if in1[left] <= in2[right] {
            tmp[index] = in1[left];
            index += 1;
            left += 1;
        } else {
            tmp[index] = in2[right];
            index += 1;
            right += 1;
        }
    }

    if left < in1.len() {
        tmp[index..].copy_from_slice(&in1[left..]);
    }
    if right < in2.len() {
        tmp[index..].copy_from_slice(&in2[right..]);
    }
}


pub fn quick_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() > 1 {
        let pivot = lomuto_partition(input);
        quick_sort(&mut input[..pivot]);
        quick_sort(&mut input[pivot + 1..]);
    }
}

/// Partitions a slice according to the Lomuto partition scheme.
fn lomuto_partition<T: PartialOrd>(input: &mut [T]) -> usize {
    let pivot = input.len() - 1;
    let mut swap = 0;
    for i in 0..pivot {
        if input[i] < input[pivot] {
            if swap != i {
                input.swap(swap, i);
            }
            swap += 1;
        }
    }

    if swap != pivot {
        input.swap(swap, pivot);
    }
    swap
}

pub fn quick_dual_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    dual_pivot(input, 0, input.len() - 1);
}

fn dual_pivot<T: PartialOrd + Copy>(input: &mut [T], start: usize,
end: usize) {
    if start >= end {return;}
    if input[start] > input[end] {
        input.swap(start, end);
    }
    let lpivot = input[start];
    let rpivot = input[end];

    let mut startm = start + 1;
    let mut endm = end - 1;

    let mut point = startm;

    while point <= endm {
        if input[point] < lpivot {
            input.swap(point, startm);
            startm += 1;
        }
        else if input[point] >= rpivot {
            while input[endm] > rpivot && point < endm {
                endm -= 1;
            }
            input.swap(point, endm);

            if input[point] < lpivot {
                input.swap(point, startm);
                startm += 1;
            }
        }
        point += 1;
    }
    startm -= 1;
    endm += 1;
    input.swap(start, startm);
    input.swap(end, endm);

    dual_pivot(input, start, startm);
    dual_pivot(input, startm + 1, endm);
    dual_pivot(input, endm, end);
}


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

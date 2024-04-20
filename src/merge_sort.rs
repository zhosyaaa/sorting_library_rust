use std::cmp::min;

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
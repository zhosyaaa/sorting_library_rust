
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
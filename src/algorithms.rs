pub fn nils_quicksort(a: &mut [i32]) {
    let mut sorted = false;
    for pair in a.windows(2) {
        if pair[0] > pair[1] {
            sorted = true;
            break;
        } else {
            sorted = false;
        }
    }
    if sorted {
        let pivot = if let Some(&pivot) = a.last() {
            pivot
        } else {
            return;
        };
        let mut index = 0;
        for current in 0..a.len() - 1 {
            if a[current] < pivot {
                a.swap(index, current);
                index += 1;
            }
        }
        let last = a.len() - 1;
        a.swap(index, last);
        let (lower, upper) = a.split_at_mut(index);
        nils_quicksort(lower);
        nils_quicksort(&mut upper[1..]); // ignoring pivot element
    }
}

pub fn anton_quicksort(a: &mut [i32]) {
    if a.is_empty() {
        return;
    }
    let pivot = a[a.len() / 2];

    let mut i = 0;
    let mut j = a.len() - 1;
    loop {
        while a[i] < pivot.clone() {
            i += 1;
        }

        while a[j] > pivot.clone() {
            j -= 1;
        }

        if i >= j {
            break;
        }

        a.swap(i, j);
    }

    anton_quicksort(&mut a[..i]);
    anton_quicksort(&mut a[j + 1..]);
}

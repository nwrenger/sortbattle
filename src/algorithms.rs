pub fn nils_quicksort(vec: &mut [i32]) {
    let mut sorted = false;
    for pair in vec.windows(2) {
        if pair[0] > pair[1] {
            sorted = true;
            break;
        } else {
            sorted = false;
        }
    }
    if sorted {
        let pivot = if let Some(&pivot) = vec.last() {
            pivot
        } else {
            return;
        };
        let mut index = 0;
        for current in 0..vec.len() - 1 {
            if vec[current] < pivot {
                vec.swap(index, current);
                index += 1;
            }
        }
        let last = vec.len() - 1;
        vec.swap(index, last);
        let (lower, upper) = vec.split_at_mut(index);
        nils_quicksort(lower);
        nils_quicksort(&mut upper[1..]); // ignoring pivot element
    }
}

pub fn anton_quicksort(a: &mut [i32]) {
    if a.is_empty() {
        return;
    }
    let pivot = a[a.len() / 2];

    let mut i: i32 = -1; //ignore possible overflow
    let mut j = a.len();
    loop {
        i += 1;
        while a[i as usize] < pivot {
            i += 1;
        }

        j -= 1;
        while a[j] > pivot {
            j -= 1;
        }

        if i as usize >= j {
            break;
        }

        a.swap(i as usize, j);
    }

    anton_quicksort(&mut a[..i as usize]);
    anton_quicksort(&mut a[j + 1..]); //ignore pivot element
}

pub fn rosetta_quicksort<T, F>(v: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v, f);
        rosetta_quicksort(&mut v[0..pivot_index], f);
        rosetta_quicksort(&mut v[pivot_index + 1..len], f);
    }
}

fn partition<T, F>(v: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if f(&v[i], &v[last_index]) {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

pub fn aws_quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let p = aws_partition(arr);
    aws_quicksort(&mut arr[..p]);
    aws_quicksort(&mut arr[p + 1..]);
}

fn aws_partition(arr: &mut [i32]) -> usize {
    let pivot = arr[0];
    let mut i = 1;
    let mut j = arr.len() - 1;

    loop {
        while i <= j && arr[i] < pivot {
            i += 1;
        }

        while i <= j && arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
    }

    arr.swap(0, j);
    j
}

pub fn bubblesort(vec: &mut [i32]) {
    let n = vec.len();
    let mut newn = 1;
    while newn >= 1 {
        newn = 0;
        for i in 0..n - 1 {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                newn = i;
            }
        }
    }
}

pub fn cocktailshakersort(vec: &mut [i32]) {
    let mut end = vec.len() - 1;
    let mut start = 0;
    let mut swapped = true;

    while start < end && swapped {
        swapped = false;
        for i in start..end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        end -= 1;
        swapped = false;
        for i in (start..end).rev() {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
        start += 1;
    }
}

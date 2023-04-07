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

    let mut i = 0;
    let mut j = a.len() - 1;
    loop {
        
        while a[i] < pivot {
            i += 1;
        }
        
        while a[j] > pivot {
            j -= 1;
        }
        
        
        if i >= j {
            break;
        }
        
        if a[i] == pivot && a[j] == pivot {
            i += 1;
        }

        a.swap(i, j);
    }

    anton_quicksort(&mut a[..i]);
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

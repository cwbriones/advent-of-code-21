pub fn search_permutations<T, F>(items: &mut [T], mut f: F) -> bool
where
    F: FnMut(&[T]) -> bool,
{
    let mut counts = vec![0; items.len()];
    while permute(&mut counts, items) {
        if f(items) {
            return true;
        }
    }
    false
}

fn permute<T>(counts: &mut [usize], slice: &mut [T]) -> bool {
    if counts[0] == slice.len() {
        return false;
    }
    for i in (0..slice.len() - 1).rev() {
        let j = counts[i];
        if j + i < slice.len() - 1 {
            if j > 0 {
                slice.swap(i, i + j);
            }
            slice.swap(i, i + j + 1);
            counts[i] += 1;
            return true;
        } else if i == 0 {
            counts[i] += 1;
            slice.swap(0, slice.len() - 1);
            return true;
        }
        slice.swap(i, i + j);
        counts[i] = 0;
    }
    unreachable!();
}

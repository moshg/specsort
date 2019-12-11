#![feature(is_sorted)]

use std::cmp::Ordering;

pub trait SpecSort: Ord + Sized {
    fn sort(s: &mut [Self]);
    fn sort_by<F>(s: &mut [Self], compare: F)
    where
        F: FnMut(&Self, &Self) -> Ordering;
    fn sort_by_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self;
    fn sort_by_cached_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self;

    fn sort_unstable(s: &mut [Self]);
    fn sort_unstable_by<F>(s: &mut [Self], compare: F)
    where
        F: FnMut(&Self, &Self) -> Ordering;
    fn sort_unstable_by_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self;
}

impl SpecSort for bool {
    #[inline]
    fn sort(s: &mut [Self]) {
        <bool as SpecSort>::sort_unstable(s)
    }

    #[inline]
    fn sort_by<F>(s: &mut [Self], compare: F)
    where
        F: FnMut(&Self, &Self) -> Ordering,
    {
        <bool as SpecSort>::sort_unstable_by(s, compare);
    }

    #[inline]
    fn sort_by_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        bucket_sort_by_bool(s, f);
    }

    #[inline]
    fn sort_by_cached_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        bucket_sort_by_bool(s, f);
    }

    #[inline]
    fn sort_unstable(s: &mut [Self]) {
        sort_bool(s, false);
    }

    #[inline]
    fn sort_unstable_by<F>(s: &mut [Self], mut compare: F)
    where
        F: FnMut(&Self, &Self) -> Ordering,
    {
        match compare(&false, &true) {
            Ordering::Less => sort_bool(s, false),
            Ordering::Equal => (),
            Ordering::Greater => sort_bool(s, true),
        }
    }

    #[inline]
    fn sort_unstable_by_me<T, F>(s: &mut [T], mut f: F)
    where
        F: FnMut(&T) -> Self,
    {
        s.sort_unstable_by_key(f)
    }
}

fn sort_bool(s: &mut [bool], reverse: bool) {
    let mut true_count = 0;
    for b in s.iter_mut() {
        true_count += *b as usize;
    }

    let (head_value, head_count) = if reverse {
        (true, true_count)
    } else {
        (false, s.len() - true_count)
    };
    let (head, tail) = s.split_at_mut(head_count);
    for b in head {
        *b = head_value;
    }
    for b in tail {
        *b = !head_value;
    }
}

fn bucket_sort_by_bool<T, F>(s: &mut [T], mut f: F)
where
    F: FnMut(&T) -> bool,
{
    use std::alloc;
    use std::mem;
    use std::ptr;

    unsafe {
        if mem::size_of::<T>() == 0 || s.is_empty() {
            return;
        }

        let layout =
            alloc::Layout::from_size_align(mem::size_of::<T>() * s.len(), mem::align_of::<T>())
                .unwrap();
        let true_ptr = alloc::alloc(layout) as *mut T;
        let mut true_len = 0;
        let false_ptr = alloc::alloc(layout) as *mut T;
        let mut false_len = 0;

        for x in s.iter_mut() {
            if f(x) {
                ptr::copy_nonoverlapping(x, true_ptr.add(true_len), 1);
                true_len += 1;
            } else {
                ptr::copy_nonoverlapping(x, false_ptr.add(false_len), 1);
                false_len += 1;
            }
        }

        ptr::copy_nonoverlapping(false_ptr, s.as_mut_ptr(), false_len);
        ptr::copy_nonoverlapping(true_ptr, s.as_mut_ptr().add(false_len), true_len);

        alloc::dealloc(false_ptr as *mut u8, layout);
        alloc::dealloc(true_ptr as *mut u8, layout);
    }
}

#[cfg(test)]
mod tests {
    use crate::SpecSort;
    use rand::random;

    fn random_vec(len: usize) -> Vec<bool> {
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(rand::random());
        }
        v
    }

    #[test]
    fn test_bool() {
        let mut v = random_vec(10000);
        bool::sort_unstable(&mut v);
        assert!(v.is_sorted());

        let mut v = random_vec(10000);
        bool::sort_unstable_by_me(&mut v, |b| *b);
        assert!(v.is_sorted());
    }
}

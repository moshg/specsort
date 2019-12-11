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
    fn sort_unstable_by_cached_me<T, F>(s: &mut [T], f: F)
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

    fn sort_by_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        unimplemented!()
    }

    fn sort_by_cached_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        unimplemented!()
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

    fn sort_unstable_by_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        if s.is_empty() {
            return;
        }

        let mut start = 0;
        let mut end = s.len() - 1;
        let mut i = 0;
        while i <= end {
            unimplemented!()
        }
    }

    fn sort_unstable_by_cached_me<T, F>(s: &mut [T], f: F)
    where
        F: FnMut(&T) -> Self,
    {
        unimplemented!()
    }
}

pub fn sort_bool(s: &mut [bool], reverse: bool) {
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

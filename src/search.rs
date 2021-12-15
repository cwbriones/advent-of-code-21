// CmpByKey is a wrapper to avoid implementing comparisons for a struct
// when you only need to compare on a single field.
pub struct CmpByKey<T, F> {
    pub t: T,
    f: F,
}

pub fn cmp_by_key<T, U, F>(t: T, f: F) -> CmpByKey<T, F>
where
    F: Fn(&T) -> U,
    U: PartialEq,
{
    CmpByKey { t, f }
}

impl<T, U, F> PartialEq for CmpByKey<T, F>
where
    F: Fn(&T) -> U,
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let f = &self.f;
        f(&self.t).eq(&f(&other.t))
    }
}

impl<T, U, F> Eq for CmpByKey<T, F>
where
    F: Fn(&T) -> U,
    U: Eq,
{
}

impl<T, U, F> PartialOrd for CmpByKey<T, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let f = &self.f;
        f(&self.t).partial_cmp(&f(&other.t))
    }
}

impl<T, U, F> Ord for CmpByKey<T, F>
where
    F: Fn(&T) -> U,
    U: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let f = &self.f;
        f(&self.t).cmp(&f(&other.t))
    }
}

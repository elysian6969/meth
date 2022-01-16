use super::Vec;

pub struct Iter<'a, T, const N: usize> {
    vec: &'a Vec<T, N>,
    forward: usize,
    backward: isize,
}

impl<'a, T, const N: usize> Iter<'a, T, N> {
    #[inline]
    pub(crate) const fn new(vec: &'a Vec<T, N>) -> Self {
        Self {
            vec,
            forward: 0,
            backward: vec.len() as isize,
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.vec.len().saturating_sub(self.forward)
    }

    #[inline]
    pub const fn count(self) -> usize {
        self.len()
    }

    #[inline]
    pub const fn next(&mut self) -> Option<&T> {
        self.nth(0)
    }

    #[inline]
    pub const fn nth(&mut self, n: usize) -> Option<&T> {
        let i = self.forward.saturating_add(n);

        self.forward = i.saturating_add(1);
        self.vec.get(i)
    }

    #[inline]
    pub const fn next_back(&mut self) -> Option<&T> {
        self.nth_back(0)
    }

    #[inline]
    pub const fn nth_back(&mut self, n: usize) -> Option<&T> {
        let i = self.backward.saturating_sub(n as isize);

        self.backward = i.saturating_sub(1);

        if i < 0 {
            return None;
        }

        self.vec.get(i as usize)
    }
}

use super::Vec;

pub struct IterMut<'a, T, const N: usize> {
    vec: &'a mut Vec<T, N>,
    forward: usize,
    backward: isize,
}

impl<'a, T, const N: usize> IterMut<'a, T, N> {
    #[inline]
    pub(crate) const fn new(vec: &'a mut Vec<T, N>) -> Self {
        let backward = vec.len() as isize;

        Self {
            vec,
            forward: 0,
            backward,
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
    pub const fn next(&mut self) -> Option<&mut T> {
        self.nth(0)
    }

    #[inline]
    pub const fn nth(&mut self, n: usize) -> Option<&mut T> {
        let i = self.forward.saturating_add(n);

        self.forward = i.saturating_add(1);
        self.vec.get_mut(i)
    }

    #[inline]
    pub const fn next_back(&mut self) -> Option<&mut T> {
        self.nth_back(0)
    }

    #[inline]
    pub const fn nth_back(&mut self, n: usize) -> Option<&mut T> {
        let i = self.backward.saturating_sub(n as isize);

        self.backward = i.saturating_sub(1);

        if i < 0 {
            return None;
        }

        self.vec.get_mut(i as usize)
    }
}

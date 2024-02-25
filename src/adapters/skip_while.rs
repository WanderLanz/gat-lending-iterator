use crate::LendingIterator;
use core::fmt;

/// A lending iterator that skips items based on a predicate.
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SkipWhile<I, P> {
    iter: I,
    predicate: P,
    flag: bool,
}

impl<I, P> SkipWhile<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self {
            iter,
            predicate,
            flag: false,
        }
    }
}

impl<I: fmt::Debug, P> fmt::Debug for SkipWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

impl<I, P> LendingIterator for SkipWhile<I, P>
where
    I: LendingIterator,
    P: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a> where I: 'a, P: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            match self_.iter.next() {
                Some(item) => {
                    if self_.flag || !(self_.predicate)(&item) {
                        self_.flag = true;
                        return Some(item)
                    }
                }
                None => return None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

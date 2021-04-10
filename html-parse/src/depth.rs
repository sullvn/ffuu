pub trait DepthChange {
    fn depth_change(&self) -> isize;
}

pub struct DepthIterator<T: DepthChange, I: Iterator<Item = T>> {
    iter: I,
    depth: isize,
}

impl<T: DepthChange, I: Iterator<Item = T>> From<I> for DepthIterator<T, I> {
    fn from(iter: I) -> Self {
        DepthIterator { iter, depth: 0 }
    }
}

impl<T: DepthChange, I: Iterator<Item = T>> Iterator for DepthIterator<T, I> {
    type Item = (T, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| {
            self.depth += x.depth_change();
            (x, self.depth)
        })
    }
}

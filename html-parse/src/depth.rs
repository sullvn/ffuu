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
            let dc = x.depth_change();
            let (depth_old, depth_new) = (self.depth, self.depth + dc);
            let depth_reported = if 0 < dc { depth_old } else { depth_new };

            self.depth = depth_new;
            (x, depth_reported)
        })
    }
}

pub trait WithDepthIterator<T: DepthChange>: Iterator<Item = T> + Sized {
    fn with_depth(self) -> DepthIterator<T, Self>;
}

impl<T, I> WithDepthIterator<T> for I
where
    T: DepthChange,
    I: Iterator<Item = T>,
{
    fn with_depth(self) -> DepthIterator<T, Self> {
        self.into()
    }
}

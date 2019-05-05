use std::marker::PhantomData;
// use std::rc::Rc;

#[allow(dead_code)]
struct FwdCycl2<'a, T>(PhantomData<&'a T>);
#[allow(dead_code)]
struct FwdCycl4<'a, T>(PhantomData<&'a T>);
#[allow(dead_code)]
struct BkdCycl2<'a, T>(PhantomData<&'a T>);
// type WB2C<'a, T> = (&'a T, &'a T);
// type WC3C<'a, T> = (&'a T, &'a T, &'a T);
// type WF4C<'a, T> = (&'a T, &'a T, &'a T, &'a T);
// type WB4C<'a, T> = (&'a T, &'a T, &'a T, &'a T);
// type WC5C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T);
// type WF6C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T, &'a T);
// type WB6C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T, &'a T);

#[derive(Clone, Copy)]
enum BorderAction<T>
where
    T: Copy,
{
    Constant(T),
    Cycle,
    Mirror,
}

trait GetOr<'a, T> {
    fn get_or_cycle(&'a self, index: i64) -> Option<&'a T>;
    fn get_or_mirror(&'a self, index: i64) -> Option<&'a T>;
    fn get_or_constant(&'a self, index: i64, constant: &'a T) -> Option<&'a T>;
}

impl<'a, T> GetOr<'a, T> for Vec<T> {
    fn get_or_cycle(&'a self, index: i64) -> Option<&'a T> {
        let len = self.len() as i64;
        let mut i = index;
        while i < 0 {
            i += len;
        }
        while i >= len {
            i -= len;
        }
        self.get(i as usize)
    }

    fn get_or_mirror(&'a self, index: i64) -> Option<&'a T> {
        let len = self.len() as i64;
        let mut i = index;
        if i < 0 {
            i *= -1;
        }
        if i >= len {
            i = (len - 1) - (i % len);
        }
        self.get(i as usize)
    }

    fn get_or_constant(&'a self, index: i64, constant: &'a T) -> Option<&'a T> {
        let len = self.len() as i64;
        if index < 0 || index >= len {
            return Some(constant);
        };
        self.get(index as usize)
    }
}

trait Windowed<'a, T>
where
    T: Copy,
{
    type Item;
    type WIter;
    fn into_window(vec: &'a Vec<T>, index: usize, border: BorderAction<T>) -> Option<Self::Item>;
    fn into_witer(vec: &'a Vec<T>, border: BorderAction<T>) -> Self::WIter;
}

impl<'a, T> Windowed<'a, T> for FwdCycl2<'a, T>
where
    T: Copy,
    std::vec::Vec<T>: GetOr<'a, T>,
{
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: BorderAction<T>,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }

        match border {
            BorderAction::Constant(_c) => None,
            BorderAction::Cycle => Some((
                vector
                    .get_or_cycle(index as i64)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_cycle(index as i64 + 1)
                    .expect("Failed to get item from Vec"),
            )),
            BorderAction::Mirror => Some((
                vector
                    .get_or_mirror(index as i64)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_mirror(index as i64 + 1)
                    .expect("Failed to get item from Vec"),
            )),
        }
    }

    fn into_witer(vec: &'a Vec<T>, border: BorderAction<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border: border,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Windowed<'a, T> for FwdCycl4<'a, T>
where
    T: Copy,
    std::vec::Vec<T>: GetOr<'a, T>,
{
    type Item = (&'a T, &'a T, &'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: BorderAction<T>,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }

        match border {
            BorderAction::Constant(_c) => None,
            BorderAction::Cycle => Some((
                vector
                    .get_or_cycle(index as i64 - 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_cycle(index as i64)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_cycle(index as i64 + 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_cycle(index as i64 + 2)
                    .expect("Failed to get item from Vec"),
            )),
            BorderAction::Mirror => Some((
                vector
                    .get_or_mirror(index as i64 - 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_mirror(index as i64)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_mirror(index as i64 + 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_mirror(index as i64 + 2)
                    .expect("Failed to get item from Vec"),
            )),
        }
        // let len = vector.len();
        // if index >= len {
        //     return None;
        // }
        // let result = (
        //     vector
        //         .get_or_cycle(index as i64 - 1)
        //         .expect("Failed to get item from Vec"),
        //     vector
        //         .get_or_cycle(index as i64)
        //         .expect("Failed to get item from Vec"),
        //     vector
        //         .get_or_cycle(index as i64 + 1)
        //         .expect("Failed to get item from Vec"),
        //     vector
        //         .get_or_cycle(index as i64 + 2)
        //         .expect("Failed to get item from Vec"),
        // );
        // Some(result)
    }

    fn into_witer(vec: &'a Vec<T>, border: BorderAction<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border: border,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Windowed<'a, T> for BkdCycl2<'a, T>
where
    T: Copy,
    std::vec::Vec<T>: GetOr<'a, T>,
{
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: BorderAction<T>,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }

        match border {
            BorderAction::Constant(_c) => None,
            BorderAction::Cycle => Some((
                vector
                    .get_or_cycle(index as i64 - 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_cycle(index as i64)
                    .expect("Failed to get item from Vec"),
            )),
            BorderAction::Mirror => Some((
                vector
                    .get_or_mirror(index as i64 - 1)
                    .expect("Failed to get item from Vec"),
                vector
                    .get_or_mirror(index as i64)
                    .expect("Failed to get item from Vec"),
            )),
        }
    }

    fn into_witer(vec: &'a Vec<T>, border: BorderAction<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border: border,
            phantom: PhantomData,
        }
    }
}

trait IntoWindowedIterator<'a, T>
where
    T: Copy,
{
    fn into_witer<W>(self, border: BorderAction<T>) -> W::WIter
    where
        W: Windowed<'a, T>;
}

impl<'a, T> IntoWindowedIterator<'a, T> for &'a Vec<T>
where
    T: Copy,
{
    fn into_witer<W>(self, border: BorderAction<T>) -> W::WIter
    where
        W: Windowed<'a, T>,
    {
        W::into_witer(self, border)
    }
}

struct VecWindowedIterator<'a, T: 'a, W>
where
    T: Copy,
{
    vector: &'a Vec<T>,
    index: usize,
    border: BorderAction<T>,
    phantom: PhantomData<&'a W>,
}

impl<'a, T, W> Iterator for VecWindowedIterator<'a, T, W>
where
    T: Copy,
    W: Windowed<'a, T>,
{
    type Item = W::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match W::into_window(self.vector, self.index, self.border) {
            Some(w) => Some(w),
            _ => None,
        };
        self.index += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_window() {
        use self::BorderAction::Cycle;
        let v: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let mut it = v.into_witer::<FwdCycl2<_>>(Cycle);
        let tp = it.next().unwrap();
        assert_eq!(tp, (&1., &2.));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&2., &3.));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&3., &4.));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&4., &5.));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&5., &1.));
        let tp = it.next();
        assert_eq!(tp, None);
    }

    #[test]
    fn backward_window() {
        use self::BorderAction::Cycle;
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut it = v.into_witer::<BkdCycl2<_>>(Cycle);
        let tp = it.next().unwrap();
        assert_eq!(tp, (&5, &1));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&1, &2));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&2, &3));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&3, &4));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&4, &5));
        let tp = it.next();
        assert_eq!(tp, None);
    }

    #[test]
    fn for_forward_iterator2() {
        use self::BorderAction::Cycle;
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut v1: u8 = 1;
        let mut v2: u8 = 2;
        for (i, ip1) in v.into_witer::<FwdCycl2<_>>(Cycle) {
            assert_eq!(*i, v1);
            assert_eq!(*ip1, v2);
            v1 += 1;
            v2 += 1;
            if v2 > 5 {
                v2 -= 5;
            }
        }
    }

    #[test]
    fn for_forward_iterator4() {
        use self::BorderAction::Cycle;
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut v1: u8 = 5;
        let mut v2: u8 = 1;
        let mut v3: u8 = 2;
        let mut v4: u8 = 3;
        for (im1, i, ip1, ip2) in v.into_witer::<FwdCycl4<_>>(Cycle) {
            assert_eq!(*im1, v1);
            assert_eq!(*i, v2);
            assert_eq!(*ip1, v3);
            assert_eq!(*ip2, v4);
            v1 += 1;
            v2 += 1;
            v3 += 1;
            v4 += 1;
            if v1 > 5 {
                v1 -= 5;
            }
            if v3 > 5 {
                v3 -= 5;
            }
            if v4 > 5 {
                v4 -= 5;
            }
        }
    }
}

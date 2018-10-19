use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
struct FwdCycl2<'a, T>(PhantomData<&'a T>);
#[derive(Debug, PartialEq)]
struct FwdCycl4<'a, T>(PhantomData<&'a T>);
#[derive(Debug, PartialEq)]
struct BkdCycl2<'a, T>(PhantomData<&'a T>);
// type WB2C<'a, T> = (&'a T, &'a T);
// type WC3C<'a, T> = (&'a T, &'a T, &'a T);
// type WF4C<'a, T> = (&'a T, &'a T, &'a T, &'a T);
// type WB4C<'a, T> = (&'a T, &'a T, &'a T, &'a T);
// type WC5C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T);
// type WF6C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T, &'a T);
// type WB6C<'a, T> = (&'a T, &'a T, &'a T, &'a T, &'a T, &'a T);

trait Windowed<'a, T> {
    type Item;
    type WIter;
    fn into_window(vec: &'a Vec<T>, index: usize) -> Option<Self::Item>;
    fn into_witer(vec: &'a Vec<T>) -> Self::WIter;
}

impl<'a, T> Windowed<'a, T> for FwdCycl2<'a, T> {
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(vector: &'a Vec<T>, index: usize) -> Option<Self::Item> {
        let len = vector.len();
        if index >= len || len < 2 {
            return None;
        }
        let result = {
            let i: &T;
            let ip1: &T;
            i = vector.get(index).unwrap();

            if index == len - 1 {
                ip1 = vector.get(0).unwrap();
            } else {
                ip1 = vector.get(index + 1).unwrap();
            }
            (i, ip1)
        };
        Some(result)
    }
    fn into_witer(vec: &'a Vec<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Windowed<'a, T> for FwdCycl4<'a, T> {
    type Item = (&'a T, &'a T, &'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(vector: &'a Vec<T>, index: usize) -> Option<Self::Item> {
        let len = vector.len();
        if index >= len || len < 4 {
            return None;
        }
        let result = {
            let im1: &T;
            let i: &T;
            let ip1: &T;
            let ip2: &T;
            i = vector.get(index).unwrap();

            if index == 0 {
                im1 = vector.get(len - 1).unwrap();
            } else {
                im1 = vector.get(index - 1).unwrap();
            }

            if index == len - 1 {
                ip1 = vector.get(0).unwrap();
                ip2 = vector.get(1).unwrap();
            } else if index == len - 2 {
                ip1 = vector.get(index + 1).unwrap();
                ip2 = vector.get(0).unwrap();
            } else {
                ip1 = vector.get(index + 1).unwrap();
                ip2 = vector.get(index + 2).unwrap();
            }
            (im1, i, ip1, ip2)
        };
        Some(result)
    }
    fn into_witer(vec: &'a Vec<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Windowed<'a, T> for BkdCycl2<'a, T> {
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, Self>;
    fn into_window(vector: &'a Vec<T>, index: usize) -> Option<Self::Item> {
        let len = vector.len();
        if index >= len || len < 2 {
            return None;
        }
        let result = {
            let i: &T;
            let im1: &T;
            i = vector.get(index).unwrap();

            if index == 0 {
                im1 = vector.get(len - 1).unwrap();
            } else {
                im1 = vector.get(index - 1).unwrap();
            }
            (im1, i)
        };
        Some(result)
    }
    fn into_witer(vec: &'a Vec<T>) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            phantom: PhantomData,
        }
    }
}

trait IntoWindowedIterator<'a, T> {
    fn into_witer<W>(self) -> W::WIter
    where
        W: Windowed<'a, T>;
}

impl<'a, T> IntoWindowedIterator<'a, T> for &'a Vec<T> {
    fn into_witer<W>(self) -> W::WIter
    where
        W: Windowed<'a, T>,
    {
        W::into_witer(self)
    }
}

struct VecWindowedIterator<'a, T: 'a, W> {
    vector: &'a Vec<T>,
    index: usize,
    phantom: PhantomData<&'a W>,
}

impl<'a, T, W> Iterator for VecWindowedIterator<'a, T, W>
where
    W: Windowed<'a, T>,
{
    type Item = W::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match W::into_window(self.vector, self.index) {
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
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut it = v.into_witer::<FwdCycl2<u8>>();
        let tp = it.next().unwrap();
        assert_eq!(tp, (&1, &2));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&2, &3));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&3, &4));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&4, &5));
        let tp = it.next().unwrap();
        assert_eq!(tp, (&5, &1));
        let tp = it.next();
        assert_eq!(tp, None);
    }

    #[test]
    fn backward_window() {
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut it = v.into_witer::<BkdCycl2<u8>>();
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
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut v1: u8 = 1;
        let mut v2: u8 = 2;
        for (i, ip1) in v.into_witer::<FwdCycl2<u8>>() {
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
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut v1: u8 = 5;
        let mut v2: u8 = 1;
        let mut v3: u8 = 2;
        let mut v4: u8 = 3;
        for (im1, i, ip1, ip2) in v.into_witer::<FwdCycl4<u8>>() {
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

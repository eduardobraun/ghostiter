use std::marker::PhantomData;

#[allow(dead_code)]
struct Fwd2<T>(PhantomData<T>);
#[allow(dead_code)]
struct Fwd4<T>(PhantomData<T>);
#[allow(dead_code)]
struct Bkd2<T>(PhantomData<T>);

struct Cycle<T> ( PhantomData<T> );
struct Mirror<T> ( PhantomData<T> );
struct Constant<'a, T> ( &'a T );

trait BorderAction<'a, T> {
    fn get_border(&self, vector: &'a Vec<T>, index: i64) -> Option<&'a T>;
}

impl<'a, T> BorderAction<'a,T> for Cycle<T>
where
    T:Copy,
{
    fn get_border(&self, vector: &'a Vec<T>, index: i64) -> Option<&'a T> {
        let len = vector.len() as i64;
        let mut i = index;
        while i < 0 {
            i += len;
        }
        while i >= len {
            i -= len;
        }
        vector.get(i as usize)
    }
}

impl<'a, T> BorderAction<'a,T> for Mirror<T>
where
    T:Copy,
{
    fn get_border(&self, vector: &'a Vec<T>, index: i64) -> Option<&'a T> {
        let len = vector.len() as i64;
        let mut i = index;
        if i < 0 {
            i *= -1;
        }
        if i >= len {
            i = (len - 1) - (i % len);
        }
        vector.get(i as usize)
    }
}

impl<'a, T> BorderAction<'a,T> for Constant<'a, T>
where
    T:Copy,
{
    fn get_border(&self, _vector: &'a Vec<T>, _index: i64) -> Option<&'a T> {
        Some(self.0)
    }
}

trait GetOr<'a, T, A> 
where
    T: Copy,
    A: 'a+BorderAction<'a,T>,
{
    fn get_or_border(&'a self, index: i64, border: &'a A) -> Option<&'a T>;
}

impl<'a, T, A> GetOr<'a, T, A> for Vec<T>
where
    T: Copy,
    A: 'a+BorderAction<'a,T>,
{
    fn get_or_border(&'a self, index: i64, border: &'a A) -> Option<&'a T> {
        let len = self.len() as i64;
        if index < 0 || index >= len {
            return border.get_border(self, index);
        };
        self.get(index as usize)
    }
}

trait Windowed<'a, T, A>
where
    T: Copy,
    A: 'a+BorderAction<'a,T>,
{
    type Item;
    type WIter;
    fn into_window(vec: &'a Vec<T>, index: usize, border: &'a A) -> Option<Self::Item>;
    fn into_witer(vec: &'a Vec<T>, border: &'a A) -> Self::WIter;
}

impl<'a, T, A> Windowed<'a, T, A> for Fwd2<T>
where
    T: 'a+Copy,
    std::vec::Vec<T>: GetOr<'a, T, A>,
    A: 'a+BorderAction<'a, T>,
{
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, A, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: &'a A,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }

        Some((vector.get_or_border(index as i64, &border)
                    .expect("Failed to get item from Vec"),
              vector.get_or_border(index as i64+1, &border)
                    .expect("Failed to get item from Vec")
                    ))
    }

    fn into_witer(vec: &'a Vec<T>, border: &'a A) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, A> Windowed<'a, T, A> for Fwd4<T>
where
    T: 'a+Copy,
    A: 'a+BorderAction<'a, T>,
    std::vec::Vec<T>: GetOr<'a, T, A>,
{
    type Item = (&'a T, &'a T, &'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, A, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: &'a A,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }
        Some(
            (vector.get_or_border(index as i64-1, &border)
                    .expect("Failed to get item from Vec"),
             vector.get_or_border(index as i64, &border)
                    .expect("Failed to get item from Vec"),
              vector.get_or_border(index as i64+1, &border)
                        .expect("Failed to get item from Vec"),
              vector.get_or_border(index as i64+2, &border)
                    .expect("Failed to get item from Vec")
                    ))
    }

    fn into_witer(vec: &'a Vec<T>, border: &'a A) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border: border,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, A> Windowed<'a, T, A> for Bkd2<T>
where
    T: 'a+Copy,
    A: 'a+BorderAction<'a, T>,
    std::vec::Vec<T>: GetOr<'a, T, A>,
{
    type Item = (&'a T, &'a T);
    type WIter = VecWindowedIterator<'a, T, A, Self>;
    fn into_window(
        vector: &'a Vec<T>,
        index: usize,
        border: &'a A,
    ) -> Option<Self::Item> {
        if index >= vector.len() {
            return None;
        }

        Some(
            (vector.get_or_border(index as i64-1, &border)
                    .expect("Failed to get item from Vec"),
             vector.get_or_border(index as i64, &border)
                    .expect("Failed to get item from Vec"))
            )
    }

    fn into_witer(vec: &'a Vec<T>, border: &'a A) -> Self::WIter {
        VecWindowedIterator {
            vector: vec,
            index: 0,
            border: border,
            phantom: PhantomData,
        }
    }
}

trait IntoWindowedIterator<'a, T, A>
where
    T: Copy,
    A: BorderAction<'a,T>,
{
    fn into_witer<W>(self, border: &'a A) -> W::WIter
    where
        W: Windowed<'a, T, A>;
}

impl<'a, T, A> IntoWindowedIterator<'a, T, A> for &'a Vec<T>
where
    T: Copy,
    A: 'a+BorderAction<'a,T>,
{
    fn into_witer<W>(self, border: &'a A) -> W::WIter
    where
        W: Windowed<'a, T, A>,
    {
        W::into_witer(self, border)
    }
}

struct VecWindowedIterator<'a, T: 'a, A: 'a, W>
where
    T: Copy,
    A: BorderAction<'a,T>,
{
    vector: &'a Vec<T>,
    index: usize,
    border:  &'a A,
    phantom: PhantomData<&'a W>,
}

impl<'a, T, A, W> Iterator for VecWindowedIterator<'a, T, A, W>
where
    T: Copy,
    A: BorderAction<'a,T>,
    W: Windowed<'a, T, A>,
{
    type Item = W::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match W::into_window(self.vector, self.index, &self.border) {
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
        let v = vec![1., 2., 3., 4., 5.];
        let mut it = v.into_witer::<Fwd2<_>>(&Cycle(PhantomData));
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
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let mut it = v.into_witer::<Bkd2<_>>(&Cycle(PhantomData));
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
        for (i, ip1) in v.into_witer::<Fwd2<_>>(&Cycle(PhantomData)) {
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
        for (im1, i, ip1, ip2) in v.into_witer::<Fwd4<_>>(&Cycle(PhantomData)) {
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

//! # Comprehension
//!
//!
//! Use `co!` to include a collect call.
//! It's just: co!(...) <=> c!(...).collect()

#[macro_export]
macro_rules! comp {

    // ------------------------------------------------------------------------------------
    //          Arrays / Sets
    // ------------------------------------------------------------------------------------


    // Basic
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr] => {
        $iter.into_iter().map(|$i| $ex)
    };
    [$ex:expr, for $i:pat in $iter:expr, if $cond:expr] => {
        $iter.into_iter().filter(|$i| $cond).map(|$i| $ex)
    };


    // Handle nested arrays
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$i| $crate::c_internal![@inn $ex $(, for $i2 in $iter2)*])

    };
    // ------------------------------------------------------------------------------------

    // Handle nested array ifs.
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr]=>{
        $iter.into_iter().flat_map(|$i| $crate::c_internal!(@innif $ex, if $cond $(, for $i2 in $iter2)*))
    };
    // ------------------------------------------------------------------------------------



    // ------------------------------------------------------------------------------------
    //          HashMaps
    // ------------------------------------------------------------------------------------
    // NOTE: These are essentially the same as arrays. Its up to you to collect as a map.
    // Use co to automatically collect as a hashmap.

    // Basic
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter]
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter]
    };
    {$key:expr => $ex:expr, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex), for $i in $iter, if $cond]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex),  for $i in $iter, if $cond]
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex),  for $i in $iter, if $cond]
    };


    // Handle nested maps
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    // ------------------------------------------------------------------------------------

    // Handle nested map ifs.
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond:expr }=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond]
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond]
    };
    // ------------------------------------------------------------------------------------

}

#[macro_export]
macro_rules! compc {

    // ------------------------------------------------------------------------------------
    //          Arrays / Sets
    // ------------------------------------------------------------------------------------


    // Basic
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr] => {
        $crate::comp![$ex, for $i in $iter].collect()
    };
    [$ex:expr, for $i:pat in $iter:expr, if $cond:expr] => {
        $crate::comp![$ex, for $i in $iter, if $cond].collect()

    };


    // Handle nested arrays
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*]=>{
        $crate::comp![$ex, for $i in $iter $(, for $i2 in $iter2)*].collect()

    };
    // ------------------------------------------------------------------------------------

    // Handle nested array ifs.
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr]=>{
        $crate::comp![$ex, for $i in $iter $(, for $i2 in $iter2)* if $cond].collect()
    };
    // ------------------------------------------------------------------------------------



    // ------------------------------------------------------------------------------------
    //          HashMaps
    // ------------------------------------------------------------------------------------
    // NOTE: These are essentially the same as arrays. Its up to you to collect as a map.
    // Use co to automatically collect as a hashmap.

    // Basic
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter].collect()
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr} => {
        $crate::comp![($key, $ex), for $i in $iter].collect()
    };
    {$key:expr => $ex:expr, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex), for $i in $iter, if $cond].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex),  for $i in $iter, if $cond].collect()
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr, if $cond:expr} => {
        $crate::comp![($key,$ex),  for $i in $iter, if $cond].collect()
    };


    // Handle nested maps
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*].collect()
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*].collect()
    };
    // ------------------------------------------------------------------------------------

    // Handle nested map ifs.
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond:expr }=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond].collect()
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*, if $cond: expr}=>{
        $crate::comp![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*, if $cond].collect()
    };
    // ------------------------------------------------------------------------------------

}
#[macro_export]
macro_rules! c_internal {

    [@inn $ex:expr, for $i:pat in $iter:expr] => {
        $iter.into_iter().map(move |$i| $ex)
    };
    [@inn $ex:expr, for $i:pat in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$i| $crate::c_internal![@inn $ex $(, for $i2 in $iter2)*])
    };
    [@innif $ex:expr, if $cond:expr, for $i:pat in $iter:expr] => {
        $iter.into_iter().filter(move |$i| $cond).map(move |$i| $ex)
    };
    [@innif $ex:expr, if $cond: expr, for $i:pat in $iter:expr $(,for $i2:pat in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$i| $crate::c_internal![@innif $ex, if $cond $(, for $i2 in $iter2)*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct NonCopyNumber(i32);
    impl std::ops::Add for NonCopyNumber {
        type Output = NonCopyNumber;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }
    impl PartialEq<i32> for NonCopyNumber {
        fn eq(&self, other: &i32) -> bool {
            self.0 == *other
        }
    }
    impl std::ops::Add<i32> for NonCopyNumber {
        type Output = NonCopyNumber;

        fn add(self, rhs: i32) -> Self::Output {
            Self(self.0 + rhs)
        }
    }

    impl std::ops::Mul<i32> for NonCopyNumber {
        type Output = NonCopyNumber;

        fn mul(self, rhs: i32) -> Self::Output {
            Self(self.0 * rhs)
        }
    }
    impl std::ops::Mul<NonCopyNumber> for i32 {
        type Output = NonCopyNumber;

        fn mul(self, rhs: NonCopyNumber) -> Self::Output {
            NonCopyNumber(rhs.0 * self)
        }
    }
    impl std::ops::Mul for NonCopyNumber {
        type Output = NonCopyNumber;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }
    impl std::convert::From<i32> for NonCopyNumber {
        fn from(other: i32) -> Self {
            Self(other)
        }
    }
    const NONCOPYABLENESTED: [[[NonCopyNumber; 2]; 2]; 2] = [
        [
            [NonCopyNumber(1), NonCopyNumber(2)],
            [NonCopyNumber(3), NonCopyNumber(4)],
        ],
        [
            [NonCopyNumber(5), NonCopyNumber(6)],
            [NonCopyNumber(7), NonCopyNumber(8)],
        ],
    ];

    #[test]
    fn basic() {
        let v: Vec<NonCopyNumber> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .into_iter()
            .map(|x| x.into())
            .collect();

        let y: Vec<NonCopyNumber> = comp![x, for x in v].collect();

        assert_eq!(
            y,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<NonCopyNumber>>()
        );
    }
    #[test]
    fn complex_collection() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = comp![(ind*x), for (ind,x) in v.into_iter().enumerate()].collect();

        assert_eq!(y, vec![0, 2, 6, 12, 20, 30, 42, 56, 72, 90]);
    }
    #[test]
    fn basic_expr() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = comp![2*x, for x in v.iter()].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    }
    #[test]
    fn basic_expr2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = comp![if x > 5 {x*2} else{x}, for x in v].collect();

        assert_eq!(y, vec![1, 2, 3, 4, 5, 12, 14, 16, 18, 20]);
    }
    #[test]
    fn filter() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = comp![x, for x in v, if x %2 == 0].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10]);
    }
    #[test]
    fn filter2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Pass all
        let y: Vec<_> = comp![x, for x in v, if x %2 == 1 || x%2 ==0].collect();

        assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn nested_basic() {
        // let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let v = NONCOPYABLENESTED.clone();

        let y: Vec<_> = comp![2i32*p, for x in v, for y in x, for p in y].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16]);
    }
    #[test]
    fn nested_ranges() {
        let y: Vec<(i32, i32)> =
            comp![(i,j), for i in 0..3, for j in 0..3, if (i+j) % 3 == 0].collect();
        assert_eq!(y, vec![(0, 0), (1, 2), (2, 1)]);
    }
    #[test]
    fn nested_multiarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let v2 = v.clone();

        // This is an edge case. The `y` inner variable is used 2 times. We need to clone it.
        // Since the expression is evaluated at the end, we need to clone in 'for' loop.
        // There is no way to automatically clone the collection, without cloning in other
        // (unnecessary) cases.
        // Thankfully, the Rust LSP will tell you to clone it.
        // This edge case will also not work if the type does not impl Copy.
        let y: Vec<_> = comp![p*y[0], for x in v, for y in x, for p in y.clone()].collect();

        let v = v2;
        assert_eq!(
            y,
            vec![
                1 * v[0][0][0],
                2 * v[0][0][0],
                3 * v[0][1][0],
                4 * v[0][1][0],
                5 * v[1][0][0],
                6 * v[1][0][0],
                7 * v[1][1][0],
                8 * v[1][1][0],
            ]
        );
    }
}

#[cfg(test)]
mod map_tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn basic() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, i32> = comp! {x=>x, for x in v}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (1, 1),
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 6),
                (7, 7),
                (8, 8),
                (9, 9),
                (10, 10)
            ])
        );
    }
    #[test]
    fn basic_expr() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, i32> = comp! {2*x=>x, for x in v}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (2, 1),
                (4, 2),
                (6, 3),
                (8, 4),
                (10, 5),
                (12, 6),
                (14, 7),
                (16, 8),
                (18, 9),
                (20, 10)
            ])
        );
    }
    #[test]
    fn basic_expr2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, bool> = comp! {x=> x>5, for x in v}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (1, false),
                (2, false),
                (3, false),
                (4, false),
                (5, false),
                (6, true),
                (7, true),
                (8, true),
                (9, true),
                (10, true)
            ])
        );
    }
    #[test]
    fn filter() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, i32> = comp! {x=>x%3, for x in v, if x %2 == 0}.collect();

        assert_eq!(y, HashMap::from([(2, 2), (4, 1), (6, 0), (8, 2), (10, 1)]));
    }
    #[test]
    fn filter2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Pass all
        let y: HashMap<i32, i32> = comp! {x=>x%3, for x in v, if x %2 == 1 || x%2 ==0}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (1, 1),
                (2, 2),
                (3, 0),
                (4, 1),
                (5, 2),
                (6, 0),
                (7, 1),
                (8, 2),
                (9, 0),
                (10, 1)
            ])
        );
    }
    #[test]
    fn nested_basic() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        let y: HashMap<i32, i32> = comp! {2*p=>p%3, for x in v, for y in x, for p in y}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (2, 1),
                (4, 2),
                (6, 0),
                (8, 1),
                (10, 2),
                (12, 0),
                (14, 1),
                (16, 2)
            ])
        );
    }
    #[test]
    fn nested_basic2() {
        let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];

        let y: HashMap<_, _> =
            comp! {z=>ind, for x in v, for (ind,z) in comp![2*z, for z in x].enumerate()}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (2, 0),
                (4, 1),
                (6, 0),
                (8, 1),
                (10, 0),
                (12, 1),
                (14, 0),
                (16, 1)
            ])
        );
    }
    #[test]
    fn nested_multiarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        let y: HashMap<_, _> =
            comp! {2*p=>y[0], for x in v, for y in x, for p in y.clone()}.collect();

        assert_eq!(
            y,
            HashMap::from([
                (2, 1),
                (4, 1),
                (6, 3),
                (8, 3),
                (10, 5),
                (12, 5),
                (14, 7),
                (16, 7)
            ])
        );
    }
}

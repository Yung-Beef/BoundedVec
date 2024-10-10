//type BoundedVec<u8, ConstU32<100>>

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct BoundedVec<T, U> {
    data: Vec<T>,
    limit: U,
}

impl<T, U> BoundedVec<T, U>
where
    U: Into<usize> + Clone + std::cmp::PartialOrd<i32>,
{
    fn new(limit: U) -> Result<BoundedVec<T, U>, &'static str> {
        if limit < 1 {
            Err("Cannot create a zero length vector")
        } else {
            Ok(BoundedVec {
                data: Vec::new(),
                limit,
            })
        }
    }

    fn try_from_vec<V: Clone>(v: &Vec<V>, limit: U) -> Result<BoundedVec<V, U>, &'static str> {
        let length: usize = limit.clone().into();
        if v.len() > length {
            Err("Vector too long")
        } else {
            Ok(BoundedVec {
                data: v.to_vec(),
                limit,
            })
        }
    }
}

fn main() {}

#[test]
fn test_try_from_vec() {
    let v: Vec<i32> = vec![1, 2, 3];
    let test = BoundedVec::<i32, u16>::try_from_vec(&v, 4u16);
    println!("{:?}", test);
    assert_eq!(test, Ok(BoundedVec { data: v, limit: 4 }));
}
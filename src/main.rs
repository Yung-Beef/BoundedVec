//type BoundedVec<u8, ConstU32<100>>

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct BoundedVec<T> {
    data: Vec<T>,
    limit: i32,
}

impl<T> BoundedVec<T>
{
    fn new(limit: i32) -> Result<BoundedVec<T>, &'static str> {
        if limit < 0 {
            Err("Cannot create a negative length vector")
        } else {
            Ok(BoundedVec {
                data: Vec::new(),
                limit,
            })
        }
    }

    fn try_from_vec<V: Clone>(v: &Vec<V>, limit: i32) -> Result<BoundedVec<V>, &'static str> {
        let length: usize = limit.clone().try_into().expect("i32 should always convert to usize");
        if v.len() > length {
            Err("Vector too long")
        } else {
            Ok(BoundedVec {
                data: v.to_vec(),
                limit,
            })
        }
    }

    fn push(&mut self, data: T) -> Result<(), &'static str> {
        let limit: usize = self.limit.clone().try_into().expect("i32 should always convert to usize");
        if self.data.len() >= limit {
            Err("Vector full")
        } else {
            self.data.push(data);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

fn main() {}

#[test]
fn test_new() {
    let l = -5;
    assert_eq!(Err("Cannot create a negative length vector"), BoundedVec::<i8>::new(l));
    let l = 5;
    assert_eq!(Ok(BoundedVec { data: Vec::new(), limit: 5 }), BoundedVec::<i8>::new(l));
}

#[test]
fn test_try_from_vec() {
    let v: Vec<i32> = vec![1, 2, 3];
    let test = BoundedVec::<i32>::try_from_vec(&v, 4);
    println!("{:?}", test);
    assert_eq!(test, Ok(BoundedVec { data: v, limit: 4 }));
}

#[test]
fn test_push() {
    let mut vec = BoundedVec::<i8>::new(5).unwrap();
    for _ in 0..5 {
        let _ = vec.push(1);
    }
    assert_eq!(vec, BoundedVec { data: vec![1, 1, 1, 1, 1], limit: 5 });
    assert_eq!(vec.push(1), Err("Vector full"));
}

#[test]
fn test_pop() {
    let mut vec = BoundedVec::<i8>::new(5).unwrap();
    let _ = vec.push(5);
    assert_eq!(vec.pop(), Some(5));
}
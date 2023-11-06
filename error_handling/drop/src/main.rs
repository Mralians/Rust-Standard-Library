use std::fmt::Debug;

struct CustomSmartPointer<D>
where
    D: Debug,
{
    data: D,
}
impl<D> CustomSmartPointer<D>
where
    D: Debug,
{
    fn new(data: D) -> Self {
        Self { data }
    }
}
impl<D> Drop for CustomSmartPointer<D>
where
    D: Debug,
{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {:?}", self.data);
    }
}
fn main() {
    // dropped in a LIFO way
    let a = CustomSmartPointer::new("A");
    let b = CustomSmartPointer::new("B");
    let c = CustomSmartPointer::new("C");
    let d = CustomSmartPointer::new("D");
    std::mem::drop(c);
}

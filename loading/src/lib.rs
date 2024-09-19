pub trait Load<T> {
    fn load(self) -> T;
}

pub trait FromData<'a>: Sized {
    type Error;
    type Owned: Borrow<Self::Borrowed>;
    type Borrowed: ?Sized;
    fn transform(request: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>>;
    fn from_data(request: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error>;
}

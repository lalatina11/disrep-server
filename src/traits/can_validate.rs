pub trait CanValidate {
    fn validate(self) -> Result<Self, String>
    where
        Self: Sized;
}

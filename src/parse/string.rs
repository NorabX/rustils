pub trait ToStr {

    fn to_str(self)
        -> &'static str;
}

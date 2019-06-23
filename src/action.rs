pub trait Action {
    fn perform(&self, selection: &str);
}

#[derive(Debug, Serialize, Clone)]
pub struct Job {
    pub(crate) url: String,
    pub(crate) dest: String,
}

impl Job {
    pub fn new<S>(url: S, dest: S) -> Self
    where
        S: Into<String>,
    {
        Job {
            url: url.into(),
            dest: dest.into(),
        }
    }
}

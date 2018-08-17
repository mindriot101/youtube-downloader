#[derive(Debug, Serialize)]
pub struct Job {
    url: String,
    dest: String,
}

impl Job {
    pub fn new<S>(url: S, dest: S) -> Self
        where S: Into<String> {
            Job {
                url: url.into(),
                dest: dest.into(),
            }
        }
}




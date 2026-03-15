

pub trait StringLifeHacks {
    ///
    /// Peek a value from a string. If such value exists, the value is removed,
    /// and then, the original string will be trimmed.
    ///
    /// Return true if such value exist, false otherwise.
    ///
    fn trimpeek(&mut self, prefix: impl AsRef<str>) -> bool;

    ///
    /// Split a string with java behaviour. Where T is the max length
    ///
    fn jsplit<const T: usize, D: AsRef<str>>(&self, delim: D) -> [Option<&str>; T];
}

impl StringLifeHacks for String {
    fn trimpeek(&mut self, prefix: impl AsRef<str>) -> bool {
        if let Some(stripped) = self.strip_prefix(prefix.as_ref()) {
            *self = stripped.trim().to_string();

            return true;
        }
        return false;
    }

    fn jsplit<const T: usize, D: AsRef<str>>(&self, delim: D) -> [Option<&str>; T] {
        let mut iter = self.split(delim.as_ref());

        let mut arr: [Option<&str>; T] = [0; T].map(|_| None);
        
        for i in 0..T {
            arr[i] = iter.next();
        }

        return arr;
    }
}

impl StringLifeHacks for &str {
    fn trimpeek(&mut self, prefix: impl AsRef<str>) -> bool {
        if let Some(stripped) = self.strip_prefix(prefix.as_ref()) {
            *self = stripped.trim();

            return true;
        }
        return false;
    }

    fn jsplit<const T: usize, D: AsRef<str>>(&self, delim: D) -> [Option<&str>; T] {
        let mut iter = self.split(delim.as_ref());

        let mut arr: [Option<&str>; T] = [0; T].map(|_| None);
        
        for i in 0..T {
            arr[i] = iter.next();
        }

        return arr;
    }
}

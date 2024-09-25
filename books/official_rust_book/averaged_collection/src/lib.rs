
pub struct AveragedCollection {
    list: Vec<u64>,
    average: f64,
}


impl AveragedCollection {
    /// Create new, empty, collection.
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    /// Push a new value into our collection.
    pub fn add(&mut self, value: u64) {
        self.list.push(value);
        self.update_average();
    }

    /// Return current value of running mean.
    pub fn average(&self) -> f64 {
        self.average
    }

    /// Remove most-recently added value and return it.
    pub fn remove(&mut self) -> Option<u64> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let length = self.list.len();
        self.average = match length {
            0 => 0.0,
            length => {
                let total: u64 = self.list.iter().sum();
                total as f64 / length as f64
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = AveragedCollection::new();
        assert_eq!(c.list, Vec::new());
        assert_eq!(c.average, 0.0);
    }

    #[test]
    fn test_add() {
        let mut c = AveragedCollection::new();
        c.add(10);
        assert_eq!(c.average, 10.0);
        c.add(20);
        assert_eq!(c.average, 15.0);
        c.add(30);
        assert_eq!(c.average, 20.0);

        assert_eq!(c.list, vec![10, 20, 30]);
    }

    #[test]
    fn test_averge() {
        let mut c = AveragedCollection::new();
        c.add(10);
        assert_eq!(c.average(), c.average);
    }

    #[test]
    fn test_remove() {
        let mut c = AveragedCollection::new();
        c.add(10);
        c.add(20);
        c.add(30);

        assert_eq!(c.remove(), Some(30));
        assert_eq!(c.average, 15.0);

        assert_eq!(c.remove(), Some(20));
        assert_eq!(c.average, 10.0);

        assert_eq!(c.remove(), Some(10));
        assert_eq!(c.average, 0.0);

        assert_eq!(c.remove(), None);
        assert_eq!(c.average, 0.0);
    }
}

use super::Properties;

impl std::fmt::Display for Properties<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.pairs {
            writeln!(f, "{}={}", k, v)?;
        }

        Ok(())
    }
}

use {crate::Term, std::fmt};

#[derive(Debug)]
pub struct Dnf(pub Vec<Term>);

impl Dnf {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl fmt::Display for Dnf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = self.0.iter();
        if let Some(term) = terms.next() {
            write!(formatter, "(")?;
            write!(formatter, "{}", term)?;
            for term in terms {
                write!(formatter, " & {}", term)?;
            }
            write!(formatter, ")")
        } else {
            write!(formatter, "0")
        }
    }
}

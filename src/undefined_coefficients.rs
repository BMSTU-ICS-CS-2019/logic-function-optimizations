use {
    crate::{Dnf, Term},
    std::{collections::HashSet, fmt},
};

pub fn undefined_coefficients_method(
    terms: &Vec<Term>,
    length: usize,
    mut monitor: impl UndefinedCoefficientsMonitor,
) -> Dnf {
    let n_equations = 1 << length;
    let mut zeroes = HashSet::with_capacity(n_equations);
    for i in 0..n_equations {
        zeroes.insert(i as u128);
    }
    for term in terms {
        zeroes.remove(&term.0);
    }
    zeroes.shrink_to_fit();

    println!("Zeroes: {:?}", zeroes);

    let zero_coefficients: HashSet<Coefficient> = zeroes
        .into_iter()
        .flat_map(|zero| Coefficient::permute_coefficients(zero, length))
        .collect();

    for term in terms {
        monitor.on_term_equation(
            term,
            Coefficient::permute_coefficients(term.0, length)
                .into_iter()
                .filter(|coefficient| !zero_coefficients.contains(coefficient))
                .collect(),
        );
    }

    todo!()
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coefficient(Vec<Option<bool>>);

impl Coefficient {
    fn permute_coefficients(value: u128, length: usize) -> Vec<Coefficient> {
        let n_coefficients = 1 << length;

        // zero coefficient is skipped
        let mut coefficients = Vec::with_capacity(n_coefficients - 1);
        for mask in 1..n_coefficients {
            // 1s in mask represent the used variables
            let mut coefficient = Vec::with_capacity(length);
            for offset in (0..length).rev() {
                coefficient.push(if (mask >> offset) & 0b1 == 1 {
                    // this bit should be included
                    Some(((value >> offset) & 0b1) != 0)
                } else {
                    None
                });
            }
            coefficients.push(Coefficient(coefficient));
        }

        coefficients
    }

    pub fn to_latex_string(&self) -> String {
        let mut result = String::new();
        result += "_{";

        {
            let mut i = self.0.len();
            let mut non_first = false;
            for sign in &self.0 {
                i -= 1;
                if sign.is_some() {
                    if non_first {
                        result += " ";
                    }
                    result += &i.to_string();
                    non_first = true;
                }
            }
        }
        result += "}^{";
        {
            let mut i = self.0.len();
            let mut non_first = false;
            for sign in &self.0 {
                i -= 1;
                if let Some(sign) = sign {
                    if non_first {
                        result += " ";
                    }
                    if *sign {
                        result += "1";
                    } else {
                        result += "0";
                    }
                    non_first = true;
                }
            }
        }
        result + "}"
    }
    //fn express(zeroes: &Vec<Coefficient>) -> Vec<Coefficient> {}
}

impl fmt::Display for Coefficient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{{ ")?;

        let mut i = self.0.len();
        for sign in &self.0 {
            i -= 1;
            if let Some(sign) = sign {
                write!(formatter, "{}/", i)?;
                if *sign {
                    write!(formatter, "1")?
                } else {
                    write!(formatter, "0")?
                }
                write!(formatter, " ")?;
            }
        }

        write!(formatter, "}}")
    }
}

pub trait UndefinedCoefficientsMonitor {
    fn on_term_equation(&mut self, term: &Term, non_zero_coefficients: Vec<Coefficient>) {}
}

#[cfg(test)]
mod tests {
    use crate::undefined_coefficients::Coefficient;

    #[test]
    fn test_permute_coefficients() {
        assert_eq!(
            Coefficient::permute_coefficients(0b110, 3),
            vec![
                Coefficient(vec![None, None, Some(false)]),
                Coefficient(vec![None, Some(true), None]),
                Coefficient(vec![None, Some(true), Some(false)]),
                Coefficient(vec![Some(true), None, None]),
                Coefficient(vec![Some(true), None, Some(false)]),
                Coefficient(vec![Some(true), Some(true), None]),
                Coefficient(vec![Some(true), Some(true), Some(false)]),
            ]
        );
    }
}

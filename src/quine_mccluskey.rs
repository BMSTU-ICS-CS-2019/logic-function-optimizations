use {
    crate::Term,
    std::{
        collections::{HashMap, HashSet},
        fmt,
        mem::replace,
    },
};

pub fn quine_mccluskey_method(
    minterms: &Vec<Term>,
    length: usize,
    mut monitor: impl QuineMccluskeyMethodMonitor,
) {
    if minterms.is_empty() {
        return; // Dnf::empty();
    }

    let mut groups = HashMap::new();

    for term in minterms {
        let ones = term.0.count_ones();
        groups
            .entry(ones)
            .or_insert_with(HashSet::new)
            .insert(Implicant::from_term(term, length));
    }

    let mut primary_implicants = HashSet::new();

    while !groups.is_empty() {
        let mut new_groups = HashMap::with_capacity(groups.len() - 1);

        // FIXME: `merged` can be localized in the following loop
        let mut merged = HashSet::new();
        for ones in groups.keys() {
            // compare all pairs of implicants which differ in ones only by one :kekw:
            if let (Some(left_implicants), Some(right_implicants)) =
                (groups.get(&ones), groups.get(&(ones + 1)))
            {
                for left in left_implicants {
                    'comparison: for right in right_implicants {
                        let mut common = Vec::with_capacity(length);
                        let mut left_iter = left.0.iter();
                        let mut right_iter = right.0.iter();

                        let mut found_different = false;
                        while let Some(left) = left_iter.next() {
                            let right = right_iter
                                .next()
                                .expect("left and right should be of similar length");

                            // ternary logic is valid in the following context
                            common.push(if left == right {
                                // the values are just similar
                                *left
                            } else {
                                if found_different {
                                    // more than one position differs
                                    continue 'comparison;
                                } else {
                                    found_different = true;
                                    // this is the first (possibly: the only) differing position
                                    None
                                }
                            });
                        }

                        let common = Implicant(common);

                        new_groups
                            .entry(*ones)
                            .or_insert_with(HashSet::new)
                            .insert(common);
                        merged.insert(left.clone());
                        merged.insert(right.clone());
                    }
                }
            }
        }

        for group in replace(&mut groups, new_groups).into_values() {
            for implicant in group {
                if !merged.contains(&implicant) {
                    primary_implicants.insert(implicant);
                }
            }
        }
    }

    monitor.on_primary_implicants_found(&primary_implicants);

    for implicant in &primary_implicants {
        'term_check: for term in minterms {
            let mut offset = length;

            for bit in implicant.0.iter() {
                offset -= 1;
                if let Some(bit) = bit {
                    if *bit != ((term.0 >> offset) & 0b1 != 0) {
                        continue 'term_check;
                    }
                }
            }

            monitor.on_primary_implicant_match(implicant, term);
        }
    }

    // todo!()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Implicant(Vec<Option<bool>>);

impl Implicant {
    fn from_term(term: &Term, length: usize) -> Self {
        if length == 0 {
            return Self(vec![]);
        }

        let mut value = Vec::with_capacity(length);
        for offset in (0..length).rev() {
            value.push(Some(((term.0 >> offset) & 0b1) != 0));
        }

        Self(value)
    }
}

impl fmt::Display for Implicant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in &self.0 {
            match bit {
                Some(false) => write!(formatter, "0")?,
                Some(true) => write!(formatter, "1")?,
                None => write!(formatter, "~")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicant_from_term() {
        assert_eq!(Implicant::from_term(&Term(0), 0), Implicant(vec![]));

        assert_eq!(
            Implicant::from_term(&Term(0b110), 3),
            Implicant(vec![Some(true), Some(true), Some(false)]),
        );

        assert_eq!(
            Implicant::from_term(&Term(0b0100), 4),
            Implicant(vec![Some(false), Some(true), Some(false), Some(false)]),
        );
    }
}

pub trait QuineMccluskeyMethodMonitor {
    fn on_primary_implicants_found<'a>(
        &mut self,
        _primary_implicants: impl IntoIterator<Item = &'a Implicant>,
    ) {
    }

    fn on_primary_implicant_match(&mut self, _primary_implicant: &Implicant, _term: &Term) {}
}

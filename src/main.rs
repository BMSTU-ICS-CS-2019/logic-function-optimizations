mod dnf;
mod quine_mccluskey;
mod term;
mod undefined_coefficients;

use std::num::ParseIntError;
pub use {dnf::*, quine_mccluskey::*, term::*, undefined_coefficients::*};

fn main() -> Result<(), ParseIntError> {
    let arguments = std::env::args();

    let mut terms = Vec::with_capacity(arguments.len());

    let mut length = 0;
    for argument in arguments.skip(1) {
        let bits = u128::from_str_radix(&argument, 2)?;
        terms.push(Term(bits));
        length = length.max(u128::BITS - bits.leading_zeros());
    }
    let length = length as usize;

    println!("# Quine McCluskey Method\n");
    quine_mccluskey_method_main(&terms, length);
    println!();
    println!("# Undefined Coefficients Method\n");
    undefined_coefficients_method_main(&terms, length);

    Ok(())
}

fn quine_mccluskey_method_main(terms: &Vec<Term>, length: usize) {
    struct Monitor {
        last_implicant: Option<Implicant>,
    }
    impl QuineMccluskeyMethodMonitor for Monitor {
        fn on_primary_implicants_found<'a>(
            &mut self,
            primary_implicants: impl IntoIterator<Item = &'a Implicant>,
        ) {
            println!("Found primary implicants:\n");
            for implicant in primary_implicants {
                println!("- `{implicant}`");
            }
            println!("\nMatches:\n");
        }

        fn on_primary_implicant_match(&mut self, primary_implicant: &Implicant, term: &Term) {
            if match &self.last_implicant {
                None => true,
                Some(last_implicant) => last_implicant != primary_implicant,
            } {
                println!("`{primary_implicant}`:\n");
                self.last_implicant = Some(primary_implicant.clone());
            }

            let term = term.0;
            println!("  - `{term:0b}` (`{term}`)");
        }
    }

    quine_mccluskey_method(
        &terms,
        length,
        Monitor {
            last_implicant: None,
        },
    );
}

fn undefined_coefficients_method_main(terms: &Vec<Term>, length: usize) {
    struct Monitor;
    impl UndefinedCoefficientsMonitor for Monitor {
        fn on_term_equation(&mut self, _term: &Term, non_zero_coefficients: Vec<Coefficient>) {
            let mut non_first = false;
            for coefficient in non_zero_coefficients {
                if non_first {
                    print!(" + ")
                }
                print!("K{}", coefficient.to_latex_string());
                non_first = true;
            }
            println!(" = 1\\newline");
        }
    }

    println!("$$");
    undefined_coefficients_method(terms, length, Monitor);
    println!("$$");
}

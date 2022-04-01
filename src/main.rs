mod dnf;
mod quine_mccluskey;
mod term;
mod undefined_coefficients;

pub use {dnf::*, quine_mccluskey::*, term::*, undefined_coefficients::*};

fn main() {
    let terms: Vec<Term> = [
        0b000000, 0b000011, 0b000010, 0b000110, // 1
        0b000111, 0b000101, 0b000100, 0b001000, // 2
        0b011010, 0b011110, 0b011101, 0b010000, // 3
        0b010010, 0b010110, 0b010100, 0b110000, // 4
        0b110010, 0b110110, 0b110100, 0b111010, // 5
        0b111101, 0b111110, 0b101000, 0b101001, // 6
        0b101101, 0b101100, 0b100000, 0b100001, // 7
        0b100010, 0b100110, 0b100101, 0b100100, // 8
    ]
    .into_iter()
    .map(|bits| {
        println!("-> {bits}");
        Term(bits)
    })
    .collect();

    // quine_mccluskey_method_main(&terms);
    undefined_coefficients_method_main(&terms);
}

fn quine_mccluskey_method_main(terms: &Vec<Term>) {
    struct Monitor {
        last_implicant: Option<Implicant>,
    }
    impl QuineMccluskeyMethodMonitor for Monitor {
        fn on_primary_implicants_found<'a>(
            &mut self,
            primary_implicants: impl IntoIterator<Item = &'a Implicant>,
        ) {
            println!("===== < Found primary implicants >=====");
            for implicant in primary_implicants {
                println!(":: {implicant}");
            }
        }

        fn on_primary_implicant_match(&mut self, primary_implicant: &Implicant, term: &Term) {
            if match &self.last_implicant {
                None => true,
                Some(last_implicant) => last_implicant != primary_implicant,
            } {
                println!("<===== {primary_implicant} =====>");
                self.last_implicant = Some(primary_implicant.clone());
            }

            let term = term.0;
            println!("{primary_implicant}: {term:06b} ({term})");
        }
    }

    quine_mccluskey_method(
        &terms,
        6,
        Monitor {
            last_implicant: None,
        },
    );
}

fn undefined_coefficients_method_main(terms: &Vec<Term>) {
    struct Monitor;
    impl UndefinedCoefficientsMonitor for Monitor {
        fn on_term_equation(
            &mut self,
            term: &Term,
            non_zero_coefficients: Vec<crate::Coefficient>,
        ) {
            let mut non_first = false;
            for coefficient in non_zero_coefficients {
                if non_first {
                    print!(" + ")
                }
                print!("K{}", coefficient.to_latex_string());
                non_first = true;
            }
            println!(" = 1\\\\");
        }
    }

    undefined_coefficients_method(&terms, 6, Monitor);
}

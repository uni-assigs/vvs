// export function calculaIR (salario: number): number {
//     let ret: number = 0.0
//     if (salario > 1000.0 && salario <= 2000.0) {
//       ret += (salario - 1000.0) * (0.15)
//     }
//     if (salario > 2000.0) {
//       ret += 150 + (salario - 2000.0) * (0.20)
//     }
//     if (salario > 3000.0) {
//       ret += (salario - 3000.0) * (0.05)
//     }
//     return ret
//   }

// O IR (Imposto de Renda) é calculado da seguinte forma:
//   se salário-base > 2000, IR = (150) + (salário-base - 2000) * 20%
//   se 1000 < salário-base <= 2000, IR = (salário-base - 1000) * 15%
//   se salário-base <= 1000, IR = 0

fn calculate_income_tax(revenue: f64) -> f64 {
    match revenue {
        rev if rev <= 1000.0 => 0.0,
        rev if rev <= 2000.0 => (rev - 1000.0) * 0.15,
        rev => 150.0 + (rev - 2000.0) * 0.20,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_income_tax;

    #[test]
    fn revenue_under_or_eq_to_one_thousand() {
        // All revenues under or equal to a thousand are taxed by zero

        let revenues = vec![250.0, 500.0, 750.0, 900.0, 1000.0];

        let income_taxes: Vec<_> = revenues.into_iter().map(calculate_income_tax).collect();

        // Make sure the calculated taxes are all zero
        assert_eq!(income_taxes, vec![0.0; 5])
    }

    #[test]
    fn revenue_under_or_eq_to_two_thousands() {
        // All revenues (r) under or equal to two thousand (and over a thousand) are taxed by (r - 1000.0) * 15%
        let revenues = vec![1100.0, 1250.0, 1500.0, 1750.0, 2000.0];

        let tax = |rev| (rev - 1000.0) * 0.15;

        let expected_taxes: Vec<_> = revenues.iter().map(tax).collect();

        let income_taxes: Vec<_> = revenues.into_iter().map(calculate_income_tax).collect();

        assert_eq!(income_taxes, expected_taxes);
    }

    #[test]
    fn revenue_over_two_thousands() {
        // All revenues (r) under or equal to three thousands (and over two thousands) are taxed by 150 + (r - 2000.0) * 20%
        let revenues = vec![2150.0, 2250.0, 2500.0, 2750.0, 3000.0];

        let tax = |rev| {
            150.0 + (rev - 2000.0) * 0.20
        };

        let expected_taxes:Vec<_> = revenues.iter().map(tax).collect();

        let income_taxes: Vec<_> = revenues.into_iter().map(calculate_income_tax).collect();

        assert_eq!(expected_taxes, income_taxes)
    }
}

fn main() {
    println!("Income tax of R${} is R${}", 2299.0, calculate_income_tax(2299.0));
}

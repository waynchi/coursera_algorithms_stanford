fn ks_multiply(expr_1: String, expr_2: String) -> String {

    println!("ks multiply. Expr_1: {}. Expr_2: {}.", expr_1, expr_2);
    // Special Case
    if expr_1.len() == 0 || expr_2.len() == 0 {
        return String::from("0");
    }

    // Base case
    if expr_1.len() <= 1 && expr_2.len() <= 1 {
        let expr_1_int: u32 = match expr_1.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse expression 1 into u32 int");
                return String::from("Error");
            }
        };

        let expr_2_int: u32 = match expr_2.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse expression 2 into u32 int");
                return String::from("Error");
            }
        };

        println!("Return base case: {}", (expr_1_int * expr_2_int).to_string());
        return (expr_1_int * expr_2_int).to_string();
    }

    // Let's count the split from the right to the left
    let split_mark = std::cmp::max(expr_1.len() / 2, expr_2.len() / 2);

    // Prevent edge cases
    let (expr_a, expr_b) = match expr_1.len() > split_mark {
        true => expr_1.split_at(expr_1.len() - split_mark),
        false => expr_1.split_at(0)
    };

    let (expr_c, expr_d) = match expr_2.len() > split_mark {
        true => expr_2.split_at(expr_2.len() - split_mark),
        false => expr_2.split_at(0)
    };

    // a b (expr 1)
    // c d (expr 2)

    // ac
    let num_zeroes: usize = std::cmp::max(expr_b.len(), expr_d.len());
    let expr_ac_zeroes: String = match String::from_utf8(vec![b'0'; num_zeroes * 2]) {
        Ok(zero_string) => zero_string,
        Err(_) => {
            println!("Could not create string of zeroes");
            return String::from("Error");
        }
    };

    let expr_ac: String = ks_multiply(expr_a.to_string(), expr_c.to_string());
    let expr_ac: String = format!("{}{}", expr_ac, expr_ac_zeroes);

    // bd
    let expr_bd: String = ks_multiply(expr_b.to_string(), expr_d.to_string());

    // (a + b)(c + d)
    // (ad + bc)

    let expr_ad_bc_zeroes: String =
        match String::from_utf8(vec![b'0'; num_zeroes]) {
            Ok(zero_string) => zero_string,
            Err(_) => {
                println!("Could not create string of zeroes");
                return String::from("Error");
            }
        };

    let expr_ad: String = ks_multiply(expr_a.to_string(), expr_d.to_string());
    let expr_bc: String = ks_multiply(expr_b.to_string(), expr_c.to_string());

    let expr_ad_bc: String = ks_add(&expr_ad, &expr_bc, "0".to_string());

    let expr_ad_bc: String = format!("{}{}", expr_ad_bc, expr_ad_bc_zeroes);

    println!("Expr_1: {}. Expr_2: {}.", expr_1, expr_2);
    println!("a: {} | b: {} | c: {} | d: {}", expr_a, expr_b, expr_c, expr_d);
    println!("ac: {} | bd: {} | ad: {} | bc: {}", expr_ac, expr_bd, expr_ad, expr_bc);

    let expr_final: String = ks_add(&expr_ac, &expr_bd, "0".to_string());
    let expr_final: String = ks_add(&expr_final, &expr_ad_bc, "0".to_string());

    println!("Expr_Final: {}", expr_final);
    return expr_final;
}

// fn ks_subtract() {}

fn ks_add(expr_1: &String, expr_2: &String, carryover: String) -> String {
    // Special Function to add Strings that are integers together.

    // Base case
    if expr_1.len() == 0 {
        println!("expr_2: {} | carryover: {}", expr_2, carryover);
        return ks_add(&expr_2, &carryover, "0".to_string());
    }

    if expr_2.len() == 0 {
        println!("expr_1: {} | carryover: {}", expr_1, carryover);
        return ks_add(&expr_1, &carryover, "0".to_string());
    }

    if expr_1.len() <= 1 && expr_2.len() <= 1 {
        let expr_1_int: u32 = match expr_1.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse expression 1 into u32 int");
                return String::from("Error");
            }
        };

        let expr_2_int: u32 = match expr_2.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse expression 2 into u32 int");
                return String::from("Error");
            }
        };

        let carryover_int: u32 = match carryover.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parser carryover into u32 int");
                return String::from("Error");
            }
        };

        return (expr_1_int + expr_2_int + carryover_int).to_string();
    }

    // a b (expr 1)
    // c d (expr 2)

    let (expr_a, expr_b) = expr_1.split_at(expr_1.len() - 1);
    let (expr_c, expr_d) = expr_2.split_at(expr_2.len() - 1);

    let expr_b_d = ks_add(&expr_b.to_string(), &expr_d.to_string(), carryover);

    let (internal_carryover, expr_b_d) = expr_b_d.split_at(expr_b_d.len() - 1);

    let expr_b_d = expr_b_d.to_string();
    let mut internal_carryover = internal_carryover.to_string();
    if internal_carryover.len() == 0 {
        internal_carryover = "0".to_string();
    }

    println!("b_d: {} | int_carry: {} ", expr_b_d, internal_carryover);
    println!("a: {} | b: {} | c: {} | d: {}", expr_a, expr_b, expr_c, expr_d);
    let expr_final = format!("{}{}", ks_add(&expr_a.to_string(), &expr_c.to_string(), internal_carryover), expr_b_d);

    return expr_final.to_string();

}

fn main() {
    let expr_1 : String = "3141592653589793238462643383279502884197169399375105820974944592".to_string();
    let expr_2 : String = "2718281828459045235360287471352662497757247093699959574966967627".to_string();

    // let ks_add_1 = String::from("11");
    // let ks_add_2 = String::from("9");
    // let ks_add_carryover = String::from("0");

    // let ks_add_test: String = ks_add(&ks_add_1, &ks_add_2, ks_add_carryover);
    // println!("ks add test value: {}", ks_add_test);

    // let expr_1: String = "12345".to_string();
    // let expr_2: String = "12345".to_string();

    let product: String = ks_multiply(expr_1, expr_2);

    println!("The product is {}", product);
}

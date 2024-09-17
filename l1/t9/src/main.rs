fn set_bit(num: &i64, set_to_one: bool, i: u8) -> i64 {
    // паника если i > 65
    if i > 64 {
        panic!("i больше 64");
    }

    // маска для перестановки i-го справа бита
    let mask: i64 = 1 << (i - 1);

    if set_to_one {
        // перестановка на 1 через bitwise or
        num | mask
    } else {
        // новая маска через bitwise xor
        let mask_secondary: i64 = mask ^ i64::MAX;
        // перестановка на 0 через bitwise and
        num & mask_secondary
    }
}

fn main() {
    // тест
    let num: i64 = 100;

    println!("Число до изменения байта               {:64b}", &num);
    println!(
        "Число после изменения 2-го байта на 1: {:64b}",
        &set_bit(&num, true, 2)
    );
    println!(
        "Число после изменения 3-го байта на 0: {:64b}",
        &set_bit(&num, false, 3)
    );
}

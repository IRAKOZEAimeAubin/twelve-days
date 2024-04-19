fn main() {
    println!("Twelve Days of Christmas");

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            ordinal_numbers(day)
        );

        for i in (0..day).rev() {
            if i == 0 {
                println!("{}", gifts[i as usize]);
            } else {
                println!("{},", gifts[i as usize]);
            }
        }

        println!();
    }
}

fn ordinal_numbers(n: u32) -> String {
    let last_digit = n % 10;
    let last_two_digits = n % 100;

    if last_two_digits >= 11 && last_two_digits <= 13 {
        return format!("{}th", n);
    }

    match last_digit {
        1 => format!("{}st", n),
        2 => format!("{}nd", n),
        3 => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

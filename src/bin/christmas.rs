fn main() {
    let gifts = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[i]
        );

        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }

        println!(); // Adds a blank line between verses.
    }
}

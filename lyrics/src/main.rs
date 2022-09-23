fn main() {
    const ORDINAL_NUMS: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    const PATTERNS: [&str; 12] = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three french hens,",
        "four calling birds,",
        "five gold rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for n in 0..ORDINAL_NUMS.len() {
        println!("\nOn the {} day of Christmas my true love sent to me,", ORDINAL_NUMS[n]);

        for m in (0..n + 1).rev() {
            if m == 0 && n > 0 {
                println!("and {}", PATTERNS[m]);
            } else {
                println!("{}", PATTERNS[m]);
            }
        }
    }
}

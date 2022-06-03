fn main() {
    println!("THE TWELVE DAYS OF CHRISTMAS");

    for day in 0..12 {
        day_intro(day);

        if day == 0 {
            println!("A partridge in a pear tree");
            continue;
        }

        for gift_day in (0..(day + 1)).rev() {
            if gift_day == 0 {
                gifts(gift_day, "And ");
                continue;
            }
            gifts(gift_day, "");
        }
    }
}

fn day_intro(day_num: usize) {
    let days_nums = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!(
        "\nOn the {} day of Christmas, my true love sent to me",
        days_nums[day_num]
    );
}

fn gifts(gift_num: usize, prefix: &str) {
    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    println!("{}{}", prefix, gifts[gift_num]);
}

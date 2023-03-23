fn add_lyric (day: String, line: String, lyric: String) -> String{
    let mut new_lyric :String;
    let empty: String = String::from("");

    println!("On the {day} day of Christmas\nmy true love sent to me");
    
    // if the lyrics and line are ""
    if lyric.eq(&line) {
        // do not print any aditional line
        new_lyric = String::from("");
    } else {
        // if lyric is empty
        if lyric.eq(&empty) {
            // we will not concatenate the lyrics
            new_lyric = line;
        } else {
            new_lyric = line + "\n" + &lyric;
        }
        println!("{new_lyric}");
    }

    println!("A partridge in a pear tree.\n");
    new_lyric
}

fn main() {
    let mut day :String = String::from("first");
    let mut line :String = String::from("");
    let mut lyric :String = String::from("");
    
    lyric = add_lyric(day, line, lyric);

    day = String::from("second");
    line = String::from("Two turtle doves,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("third");
    line = String::from("Three French hens,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("fourth");
    line = String::from("Four calling birds,");
    lyric = add_lyric(day, line, lyric);


    day = String::from("fifth");
    line = String::from("Five golden rings,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("sixth");
    line = String::from("Six geese a-laying,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("seventh");
    line = String::from("Seven swans a-swimming,");
    lyric = add_lyric(day, line, lyric);
 
    day = String::from("eighth");
    line = String::from("Eight maids a-milking,");
    lyric = add_lyric(day, line, lyric);
 
    day = String::from("ninth");
    line = String::from("Nine ladies dancing,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("tenth");
    line = String::from("Ten lords a-leaping,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("eleventh");
    line = String::from("Eleven pipers piping,");
    lyric = add_lyric(day, line, lyric);

    day = String::from("twelfth");
    line = String::from("Twelve drummers drumming,");
    lyric = add_lyric(day, line, lyric);
  
}

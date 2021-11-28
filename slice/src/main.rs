fn main() {
    let s = String::from("Hello, world! This is a pen!");
    let (word, ptr) = &first_word(&s);
    let word2 = &second_word(&s, *ptr);

    println!("first word is: {}", word);
    println!("second word is: {}", word2);
    println!("original string is: {}", s);
}

fn first_word(s: &String) -> (&str, usize) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i], i);
        }
    }

    (&s[..], 0)
}
fn second_word(s: &String, ptr: usize) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count = count + 1;
            if count == 2 {
                return &s[(ptr + 1)..i];
            }
        }
    }

    &s[..]
}

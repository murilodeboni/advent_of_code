use regex::Regex;

fn main() {
    let input = [
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"
    ];

    let first_re = Regex::new(r"^[a-zA-Z]*(\d).*$").unwrap();
    let last_re = Regex::new(r"^.*(\d)[a-zA-Z]*$").unwrap();

    let mut ans = 0;

    for i in input {
        let Some(first_digit) = first_re.captures(i) else { return };
        let Some(last_digit) = last_re.captures(i) else { return };
        ans = &ans + [&first_digit[1],&last_digit[1]].join("").parse::<i32>().unwrap();
        println!("{}", &ans);
    }

}
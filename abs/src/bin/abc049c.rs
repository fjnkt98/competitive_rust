use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(s: String) -> Result<()> {
    let mut c = s.chars().rev().collect::<String>();
    loop {
        if c.starts_with("maerd") {
            c = c[5..].to_string();
        } else if c.starts_with("remaerd") {
            c = c[7..].to_string();
        } else if c.starts_with("esare") {
            c = c[5..].to_string();
        } else if c.starts_with("resare") {
            c = c[6..].to_string();
        } else {
            break;
        }
    }

    if c.chars().count() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }

    return Ok(());
}

fn main() {
    input! {
        s: String,
    };

    solve(s).unwrap();
}

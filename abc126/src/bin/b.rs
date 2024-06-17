use proconio::input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn solve(s: String) -> Result<()> {
    let l = s[0..2].parse::<i64>().unwrap();
    let r = s[2..4].parse::<i64>().unwrap();

    if 1 <= l && l <= 12 {
        if 1 <= r && r <= 12 {
            println!("AMBIGUOUS");
        } else {
            println!("MMYY")
        }
    } else {
        if 1 <= r && r <= 12 {
            println!("YYMM")
        } else {
            println!("NA")
        }
    }

    return Ok(());
}

fn main() {
    input! {
        s: String,
    };

    solve(s).unwrap();
}

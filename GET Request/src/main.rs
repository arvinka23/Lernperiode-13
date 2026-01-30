use anyhow::Result;

fn main() -> Result<()> {
    let res = reqwest::blocking::get("http://httpbin.org/get")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", res.text()?);

    Ok(())
}

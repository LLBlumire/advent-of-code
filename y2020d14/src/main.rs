use y2020d14::*;

fn main() -> Result<()> {
    let input = Args::from_args().input()?;
    let parsed_input = ParsedInput::try_from(input.as_str())?;
    let output = Output::try_from(parsed_input)?;
    println!("{:#?}", output);
    Ok(())
}

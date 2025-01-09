use self::ipso_ascii::draw_ipso_ascii;

pub mod ipso_ascii;

pub fn run() -> std::io::Result<()> {
    draw_ipso_ascii();
    Ok(())
}

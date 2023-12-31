mod tilts;
mod smashes;
mod other;
mod aerials;

pub fn install() {
    tilts::install();
    smashes::install();
    other::install();
    aerials::install();
}
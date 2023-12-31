mod tilts;
mod smashes;
mod other;
mod aerials;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    other::install(agent);
    aerials::install(agent);
}
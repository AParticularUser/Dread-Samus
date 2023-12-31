mod escape;
pub mod speedbooster;
pub mod shinespark;
mod jump;
mod grab;
mod entry;

pub fn install(agent: &mut smashline::Agent) {
    escape::install(agent);
    speedbooster::install(agent);
    shinespark::install(agent);
    jump::install(agent);
    grab::install(agent);
    entry::install(agent);
}
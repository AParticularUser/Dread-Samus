mod special_hi;
pub mod special_s;
mod special_lw;
mod special_n;

pub fn install(agent: &mut smashline::Agent) {
    special_hi::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
}
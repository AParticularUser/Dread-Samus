mod escape;
pub mod speedbooster;
pub mod shinespark;
mod jump;
mod grab;

pub fn install() {
    escape::install();
    speedbooster::install();
    shinespark::install();
    jump::install();
    grab::install();
}
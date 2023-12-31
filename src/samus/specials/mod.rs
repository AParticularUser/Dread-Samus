mod special_n;
mod special_hi;
mod special_s;
mod special_lw;

pub fn install() {
  special_n::install();
  special_hi::install();
  special_s::install();
  special_lw::install();
}
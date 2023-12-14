mod agent;
mod acmd;
mod status;
mod frame;

#[smashline::installer]
pub fn installer() {
    install();
}

pub fn install() {
    acmd::install();
    status::install();
    //agent::install();
    //frame::install();
}
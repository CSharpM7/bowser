mod agent;
mod acmd;
mod status;
mod frame;

#[cfg(feature = "dev")]
#[smashline::installer]
pub fn install_dev() {
    acmd::install();
}

pub fn install() {
    agent::install();
    frame::install();
    status::install();
    
    #[cfg(not(feature = "dev"))]
    acmd::install();
    
    #[cfg(feature = "dev")]
    install_dev();
}

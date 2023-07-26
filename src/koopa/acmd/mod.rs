mod dashattack;
mod nair;
mod specialn;
mod specialnmax;
mod breath;

pub fn install() {
    dashattack::install();
    nair::install();
    specialn::install();
    specialnmax::install();
    breath::install();
}
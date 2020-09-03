use lds_plotter::{create_plot, create_pseudo_random_points, create_quasi_random_points};
fn main() {
    create_plot("rd_plot.svg", 1000, create_quasi_random_points);
    create_plot("pseudo_plot.svg", 1000, create_pseudo_random_points);
}

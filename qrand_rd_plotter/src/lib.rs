use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use qrand_std::{create_sequence, LowDiscrepancySequence};
use randomize::{formulas, PCG64};

pub fn create_plot<F>(path_to_svg: &str, number_of_points: usize, create_random_points: F)
where
    F: Fn(usize) -> Vec<(f64, f64)>,
{
    let points = create_random_points(number_of_points);
    let plot = Plot::new(points).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .colour("blue")
            .size(2.0),
    );
    let view = ContinuousView::new()
        .add(plot)
        .x_range(0.0, 1.0)
        .y_range(0.0, 1.0);
    Page::single(&view).save(path_to_svg).unwrap();
}

pub fn create_quasi_random_points(number_of_points: usize) -> Vec<(f64, f64)> {
    let sequence = create_sequence(2);
    let mut points = Vec::with_capacity(number_of_points);
    for i in 0..number_of_points {
        let (x, y) = (
            sequence.element(i, 0).unwrap_or(0.0),
            sequence.element(i, 1).unwrap_or(0.0),
        );
        points.push((x, y))
    }
    points
}

pub fn create_pseudo_random_points(number_of_points: usize) -> Vec<(f64, f64)> {
    let mut random = PCG64::default();
    let mut points = Vec::with_capacity(number_of_points);
    for _ in 0..number_of_points {
        let (x, y) = (
            formulas::f64_half_open_right(random.next_u64()),
            formulas::f64_half_open_right(random.next_u64()),
        );
        points.push((x, y))
    }
    points
}

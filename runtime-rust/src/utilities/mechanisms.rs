use ndarray::prelude::*;

use crate::utilities::noise;

pub fn laplace_mechanism(data: &ArrayD<f64>, epsilon: &f64, sensitivity: &f64) -> ArrayD<f64> {
    let scale: f64 = sensitivity / epsilon;
    let mut data_vec: Vec<f64> = data.clone().into_dimensionality::<Ix1>().unwrap().to_vec();
    data_vec = data_vec.iter().map(|x| x + noise::sample_laplace(0., scale)).collect();
    return arr1(&data_vec).into_dyn();
}

pub fn gaussian_mechanism(data: &ArrayD<f64>, epsilon: &f64, delta: &f64, sensitivity: &f64) -> ArrayD<f64> {
    let scale: f64 = sensitivity * (2. * (1.25 / delta).ln()).sqrt() / epsilon;
    let mut data_vec: Vec<f64> = data.clone().into_dimensionality::<Ix1>().unwrap().to_vec();
    data_vec = data_vec.iter().map(|x| x + noise::sample_gaussian(0., scale)).collect();
    return arr1(&data_vec).into_dyn();
}

use crate::Matrix;

pub fn gaussianJordanElimination(left_matrix: &Matrix, right_matrix: &Vec<f64>) -> Vec<f64> {
    let combined = combineMatrices(left_matrix, right_matrix);
    let fwdIntegration = forwardElimination(&combined);
    //NOW, FINAL STEP IS BACKWARD SUBSTITUTION WHICH RETURNS THE TERMS NECESSARY FOR POLYNOMIAL REGRESSION
    backwardSubstitution(
        &fwdIntegration,
        &Vec::new(),
        fwdIntegration.len() - 1,
        fwdIntegration[0].len() - 2,
    )
}

fn combineMatrices(left: &Matrix, right: &Matrix)->Matrix {
    Vec::new()
}

fn forwardElimination(any_matrix: &Matrix)->Matrix {
    Vec::new()
}

fn backwardSubstitution(anyMatrix: &Matrix, arr:, row, col)->Vec<f64> {
    Vec::new()
}
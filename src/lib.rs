mod matrix;

pub type Matrix = Vec<Vec<f64>>;
#[derive(Debug)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug)]
pub struct PolynomialRegression {
    pub data: Vec<DataPoint>,
    pub degree: usize,
    pub matrix: Matrix,
    pub left_matrix: Matrix,
    pub right_matrix: Vec<f64>,
}


impl PolynomialRegression {
    pub fn new(data_points: Vec<DataPoint>, degree: usize) -> Self {
        PolynomialRegression {
            data: data_points,
            degree,
            matrix: Vec::new(),
            left_matrix: Vec::new(),
            right_matrix: Vec::new(),
        }
    }

    /**
     * Sums up all x coordinates raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */

    fn sumX(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.x.powi(power)).sum()
    }

    /**
     * sums up all x * y where x is raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sumXTimesY(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.x.powi(power) * d.y).sum()
    }

    /**
     * Sums up all Y's raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sumY(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.y.powi(power)).sum()
    }

    /**
     * generate the left matrix
     */
    fn generateLeftMatrix(&mut self) {
       

        for i in 0..=self.degree {
            self.left_matrix.push(Vec::new());
            for j in 0..=self.degree {
                if i == 0 && j == 0 {
                    self.left_matrix[i].push( self.data.len() as f64);
                } else {
                    let v = self.sumX((i + j) as i32);
                    self.left_matrix[i].push(v);
                }
            }
        }
    }
    /**
     * generates the right hand matrix
     */
    fn generateRightMatrix(&mut self) {
        /*
        for (let i = 0; i <= this.degree; i++) {
            if (i === 0) {
                this.rightMatrix[i] = this.sumY(this.data, 1);
            } else {
                this.rightMatrix[i] = this.sumXTimesY(this.data, i);
            }
        }
        */
        for i in 0..=self.degree {
             
                if i == 0   {
                    self.right_matrix.push(self.sumY(1) as f64);
                } else {
                    self.right_matrix.push(self.sumXTimesY(i as i32) as f64);

                }
            
        }
    }
    /**
     * gets the terms for a polynomial
     * @returns {*}
     */
    pub fn getTerms(&mut self) -> Vec<f64> {
        self.generateLeftMatrix();
        self.generateRightMatrix();
        matrix::gaussianJordanElimination(&self.left_matrix, &self.right_matrix)
    }
    /**
     * Predicts the Y value of a data set based on polynomial coefficients and the value of an independent variable
     * @param terms
     * @param x
     * @returns {number}
     */
    fn predictY(terms: Vec<f64>, x: f64) -> f64 {
        /*
        let result = 0;
        for (let i = terms.length - 1; i >= 0; i--) {
            if (i === 0) {
                result += terms[i];
            } else {
                result += terms[i] * Math.pow(x, i);
            }
        }
        return result;
        */
        0.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

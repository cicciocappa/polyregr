type Matrix = Vec<Vec<f64>>;

struct DataPoint {
    x: f64,
    y: f64,
}

struct PolynomialRegression {
    data: Vec<DataPoint>,
    degree: u32,
    matrix: Matrix,
    left_matrix: Matrix,
    right_matrix: Matrix,
}

impl PolynomialRegression {
    fn new(data_points: Vec<DataPoint>, degree: u32) -> Self {
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

    fn sumX(&self, power: u32) -> f64 {
        let sum = 0.0;
        sum
    }

    /**
     * sums up all x * y where x is raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sumXTimesY(&self, power: u32) -> f64 {
        let sum = 0.0;
        sum
    }

     /**
     * Sums up all Y's raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sumY (&self, power: u32)->f64 {
        let sum = 0.0;
        return sum;
    }

    /**
     * generate the left matrix
     */
    fn generateLeftMatrix(&mut self){
        /*
        for (let i = 0; i <= this.degree; i++) {
            this.leftMatrix.push([]);
            for (let j = 0; j <= this.degree; j++) {
                if (i === 0 && j === 0) {
                    this.leftMatrix[i][j] = this.data.length;
                } else {
                    this.leftMatrix[i][j] = this.sumX(this.data, (i + j));
                }
            }
        }
        */
    }
    
    /**
     * generates the right hand matrix
     */
    fn generateRightMatrix(&mut self){
        /*
        for (let i = 0; i <= this.degree; i++) {
            if (i === 0) {
                this.rightMatrix[i] = this.sumY(this.data, 1);
            } else {
                this.rightMatrix[i] = this.sumXTimesY(this.data, i);
            }
        }
        */
    }
    
    
    /**
     * gets the terms for a polynomial
     * @returns {*}
     */
    fn getTerms(){
        //return this.matrix.gaussianJordanElimination(this.leftMatrix, this.rightMatrix);
    }
    
    /**
     * Predicts the Y value of a data set based on polynomial coefficients and the value of an independent variable
     * @param terms
     * @param x
     * @returns {number}
     */
    fn predictY(terms:Vec<f64>, x:f64)->f64{
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

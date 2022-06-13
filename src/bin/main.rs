use polyregr::{DataPoint, PolynomialRegression};

fn main(){
    let mut data = Vec::new();
    data.push(DataPoint {x:0.0, y:1.0});
    data.push(DataPoint {x:1.0, y:3.0});
    data.push(DataPoint {x:2.0, y:6.0});
    data.push(DataPoint {x:3.0, y:9.0});
    data.push(DataPoint {x:4.0, y:12.0});
    data.push(DataPoint {x:5.0, y:15.0});
    data.push(DataPoint {x:6.0, y:18.0});
    
    let mut poly = PolynomialRegression::new(data,3);
    let terms = poly.getTerms();

    println!("{:#?}",poly);
    println!("{:#?}",terms);
}

/*
 let someData = [];
  someData.push(new DataPoint(0.0, 1.0));
  someData.push(new DataPoint(1.0, 3.0));
  someData.push(new DataPoint(2.0, 6.0));
  someData.push(new DataPoint(3.0, 9.0));
  someData.push(new DataPoint(4.0, 12.0));
  someData.push(new DataPoint(5.0, 15.0));
  someData.push(new DataPoint(6.0, 18.0));
 
  let poly = new PolynomialRegression(someData, 3);
  console.log(poly);
  let terms = poly.getTerms();
 
  for(let i = 0; i < terms.length; i++){
     console.log("term " + i, terms[i]);
  }
  console.log(poly.predictY(terms, 5.0));
  */
use ndarray::arr2;

fn main() {
   let mat1 = arr2(&[[1,2,3],
      [4,5,6],
      [7,8,9]]);

   let mat2 = arr2(&[[1,1,1],
      [2,3,8],
      [7,5,2]]);
   println!("Matrix 1");      
   println!("{}", mat1);
   println!("Matrix 2");      
   println!("{}", mat2);

   let sum = &mat1 + &mat2;

   println!("Sum of above two metrices");      
   println!("{}", sum);
   
   let prod = mat1.dot(&mat2);
   println!("Product of above two metrices");      
   println!("{}", prod);
   

   println!("Substraction of above two metrices");      
   println!("{}", &mat1-&mat2);

}
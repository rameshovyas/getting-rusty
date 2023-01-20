fn main() {
    let mat1 = [
          [10,34,56],
          [65,12,34],
          [109,43,23],
    ];
 
    let mat2 = [
          [4,4,6],
          [5,2,4],
          [1,3,2],
    ];
 
    let mut result : [[i32;3];3]; 
    println!("First Matrix ");
    print_matrix(&mat1);
 
    println!("Second Matrix ");
    print_matrix(&mat2);
 
    println!("Sum of above metrices");
    result = add_matrix(&mat1, &mat2);
    print_matrix(&result);
   
    println!("Transpose of sum of two metrices");
    result = transpose(&result);
    print_matrix(&result);
   
 }
 
 
 //Print matrix 
 fn print_matrix(matrix: &[[i32;3];3]) {
     /* for i in 0..3 {
       for j in 0..3 {
          print!("{:5}", matrix[i][j]);
       }
       println!();
    }*/
    for row in matrix {
       println!("{row:?}");
    }
 }
 
 // Add two metrices
 fn add_matrix(mat1 : &[[i32;3];3], mat2: &[[i32;3];3]) -> [[i32;3];3] {
    let mut result : [[i32; 3] ; 3] = [[0;3];3];
    for i in 0..3 {
       for j in 0..3 {
          result[i][j] = mat1[i][j] + mat2[i][j];
       }
    }
       result
 }
 
 fn transpose(matrix: &[[i32; 3] ; 3]) -> [[i32 ; 3] ; 3] {
    let mut result = [[0; 3]; 3];
     for i in 0..3 {
         for j in 0..3 {
             result[j][i] = matrix[i][j];
         }
     }
     result
 }
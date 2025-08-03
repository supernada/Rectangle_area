


// WARNING: Do not modify this function
 fn print_values(width: u32, height: u32, area: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
    println!("The area is: {}", area);
}


 fn calculate_area() {

    let width1: u32 = 10;
    let height1: u32 = 20;
    let area1: u32 = height1 * width1;

    print_values(width1, height1, area1);

}


fn main() {

    calculate_area();

}

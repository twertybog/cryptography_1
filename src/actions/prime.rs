pub fn eu_alg(mut num_1: i128, mut num_2: i128) -> bool{
    loop{
        if num_1 <= 0 || num_2 <= 0{
            break;
        }
        else if num_1 >= num_2{
            num_1 = num_1 % num_2;
        }
        else if num_2 > num_1 {
            num_2 = num_2 % num_1;
        }
    }
    match i128::max(num_1, num_2){
        1 => true,
        _ => false
    }
}
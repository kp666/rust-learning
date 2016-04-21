pub fn is_leap_year(number:i16) -> bool {
    let divisible_by = |divisor:i16|  number%divisor == 0 ;
   divisible_by(4) ^  divisible_by(100) ^ divisible_by(400)       
}
pub fn is_leap_year(x:i16) -> bool {
   check_divisibility(x,4) && (
       !check_divisibility(x,100) || check_divisibility(x,400)
       )
}

fn check_divisibility(number:i16,divisor:i16) ->bool{
  number%divisor == 0
}
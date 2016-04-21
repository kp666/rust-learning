pub fn is_leap_year(x:i16) -> bool {
   let y = handle_centuries(x);
    
    let reminder = y % 4;
    if reminder==0{
        
     return   true;
    } else
    {
        println!("old");
        return false;
    }
}

fn handle_centuries(x:i16) ->i16{
    if x%100 == 0{
        x/100
    }else{
        x
    }
}
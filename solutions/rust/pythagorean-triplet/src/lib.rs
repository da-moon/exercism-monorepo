
pub fn find() -> Option<u32> {
    // let mut result : Option<u32> = None;
    let mut a : u32 =0;
    let mut b : u32 =0;
    let mut c : u32 =0;
    'outer: for i in 1..1000 {
        for j in 1..1000-a{
            for k in 1..1000-b{
                    if (i*i)+(j*j)==(k*k) && i+j+k == 1000 {
                        a = i;
                        b = j;
                        c = k ;
                        break 'outer;
                    }
            }
        }
    }
    if a*b*c!=0 {
            Some(a*b*c)
    }
    else{
        None
    }
}

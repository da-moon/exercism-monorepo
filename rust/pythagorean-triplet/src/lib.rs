use std::collections::HashSet;

/// Find all Pythagorean triplets (a,b,c) such that a + b + c == sum.
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut out = HashSet::new();

    // a < b < c and a + b + c = sum. Upper bound for a is sum/3.
    for a in 1..sum / 3 {
        for b in (a + 1)..(sum - a) {
            let c = sum.saturating_sub(a + b);
            if c <= b {
                continue;
            }
            if a * a + b * b == c * c {
                out.insert([a, b, c]);
            }
        }
    }

    out
}

// pub fn find() -> Option<u32> {
//     let mut a : u32 =0;
//     let mut b : u32 =0;
//     let mut c : u32 =0;
//     let additionResult : u32= 1000;
//     'outer: for i in 1..additionResult {
//       for j in 1..additionResult-a{
//           if i+j <= additionResult{
//               let mut k : Option<u32> = additionResult.checked_sub(i+j);
//               if k.is_none(){
//                   panic!("K is None")
//               }
//               let temp = k.unwrap();
//               if (i*i)+(j*j)==( temp* temp) && i+j+ temp == additionResult {
//                   a = i;
//                   b = j;
//                   c =  temp ;
//                   println!("a = {:?}\nb = {:?}\nc = {:?}\n",a,b,c );
//                   break 'outer;
//               }
//           }

//       }
//     }
//     if a*b*c!=0 {
//           Some(a*b*c)
//     }
//     else{
//       None
//     }
// }

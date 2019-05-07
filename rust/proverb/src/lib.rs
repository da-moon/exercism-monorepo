pub fn build_proverb(list: Vec<&str>) -> String {
    let mut resultVec: Vec<String> = Vec::new();
    let mut result : String =String::new();
      if list.len()>0{
           let mut counter = 0;
           while counter < list.len() {
               if counter< list.len() -1{
                   resultVec.push(format!("For want of a {} the {} was lost.", list[counter], list[counter+1]));
               }
           counter += 1;
           }
           resultVec.push(format!("And all for the want of a {}.", list[0]));
           result = resultVec.join("\n");
       }
    result
}

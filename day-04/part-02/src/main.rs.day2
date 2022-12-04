fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
   
    let mut current_sum=0;
    for line in lines {
        let mut chars = line.split_ascii_whitespace().rev();
        // the first Character in the reversed line should be our throw
        // which we get points for regaurdless of outcome
        let our_throw =chars.next().unwrap();
        // match our_throw {
        //     // Match a single value
        //     "X" => current_sum= current_sum+1,
        //     "Y" => current_sum= current_sum+2,
        //     "Z" => current_sum= current_sum+3,
        //     _ => println!("Somethings Wrong added 0 for '{0}'",our_throw),
        //     // TODO ^ Try commenting out this catch-all arm
        // }

        // this should be their throw which determines victorypoints
        let their_throw = chars.next().unwrap();
        match their_throw {
            // Match a single value
            "A" =>{ 
                if our_throw=="X" {
                    // Loss
                    current_sum = current_sum+3;
                }else if our_throw=="Y" {
                    // Tie
                    current_sum = current_sum+(3+1);
                }else if our_throw=="Z" {
                    // Win
                    current_sum = current_sum+(6+2);
                }
            }
            "B" => { 
                if our_throw=="X" {
                    // Loss
                    current_sum = current_sum+1;
                }else if our_throw=="Y" {
                    // Tie
                    current_sum = current_sum+(3+2);
                }else if our_throw=="Z" {
                    // Win
                    current_sum = current_sum+(6+3);
                }
            }
            "C" => { 
                if our_throw=="X" {
                    // Loss
                    current_sum = current_sum+2;
                }else if our_throw=="Y" {
                    // Tie
                    current_sum = current_sum+(3+3);
                }else if our_throw=="Z" {
                    // Win
                    current_sum = current_sum+(6+1);
                }
            },
            _ => println!("Somethings Wrong added 0 for '{0}'",our_throw),
            // TODO ^ Try commenting out this catch-all arm
        }


    }
    println!("Total Score: {0}",current_sum);
    Ok(())
}
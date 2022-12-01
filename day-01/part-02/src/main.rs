fn main() -> anyhow::Result<()> {
    let lines = include_str!("C:\\Enlistments\\advent-of-code-2022\\day-01\\input").lines();
    let mut elf_total_calories: Vec<i32> = Vec::new();
    
    let mut current_calories:i32=0;
        
    for line in lines {
        if(line.is_empty()){
            elf_total_calories.push(current_calories);
            current_calories=0;
        }else{
            let current_bar:i32=line.parse::<i32>().unwrap();
            current_calories = current_calories+current_bar;
        }
    }

    elf_total_calories.sort();
    elf_total_calories.reverse();
    
    let mut sum:i32=0;
    for i in [0,1,2]{
        let c:i32 = elf_total_calories[i];
        sum=sum+c;
    }
 
    print!("The total calories carried by the top three elfs is {0}",sum);
    Ok(())
}
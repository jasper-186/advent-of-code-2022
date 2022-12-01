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

    let max_value = elf_total_calories.iter().max();
    match max_value {
        Some(max) => println!("Max calorie count for any Elf: {0}", max),
        None      => println!( "No Elf has the max Calories" ),
    }

 
    Ok(())
}
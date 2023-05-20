
#[derive(PartialEq, Eq)]
enum Operator{
    Unknown =-1,
    None = 0,
    Add = 1,
    Multiple = 2,
}

struct Monkey {
    items:Vec<i32>,
    operation_operator:Operator,
    operation_amount:i32,
    test_divisor:i32,
    test_if_true:i32,
    test_if_false:i32,
    inspected_items_count:i32    
}

impl Monkey {
    fn inspect_items(&mut self) -> Vec<(i32,i32)> {

        let mut result: Vec<(i32,i32)> = Vec::new();

        for item in self.items.drain(..) {
            let mut new_item = item;
            
            // Apply operation to item
            if self.operation_operator == Operator::Add{
                new_item += self.operation_amount;
            }else if self.operation_operator == Operator::Multiple {
                if self.operation_amount == 0 {
                    new_item = new_item * new_item;
                }else{
                    new_item = new_item * self.operation_amount
                }
            }

            // Monkey inspects item
            self.inspected_items_count += 1;

            // divide by 3
            new_item = new_item/3;

            if new_item%self.test_divisor ==0{
                result.push((self.test_if_true,new_item));
            }else{
                result .push((self.test_if_false,new_item));
            }
        }
        return result;
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("../../input").lines();
    let mut read_file=true;
    let mut monkeys:Vec<Monkey> = Vec::new();

    while read_file{
        // Line should be the "Monkey {}:"
        let monkey_name = lines.next();      
        // Check if its empty, then ignore it
        if monkey_name.is_none(){
            read_file=false;
            continue;
        } 

        // line should be Starting items: 76, 92, 53, 93, 79, 86, 81
        let starting_items_line = lines.next();
        let starting_items_str:&str = starting_items_line.unwrap();
        let starting_items_str_split:Vec<&str> =  starting_items_str.rsplit(':').collect(); 
        let starting_items:Vec<i32> = starting_items_str_split[0].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // line should be Operation: new = old + 4
        let operation_line = lines.next();
        let operation_line_parts:Vec<&str> =  operation_line.unwrap().rsplit(' ').collect();
        let operation_value_string = operation_line_parts[0];
        let mut operation_value:i32 = 0;
        if operation_value_string != "old" {
            operation_value = operation_value_string.parse::<i32>().unwrap();
        }

        let mut operation_op = Operator::None;
        let mut operation_op_string = operation_line_parts[1];

         if operation_op_string == "+" {
            operation_op = Operator::Add;
         }else if operation_op_string == "*" {
            operation_op = Operator::Multiple;
         }

        // line should be Test: divisible by 2
        let test_line = lines.next();
        let test_line_parts:Vec<&str> =  test_line.unwrap().rsplit(' ').collect();
        let test_divisor = test_line_parts[0].parse::<i32>().unwrap();

        // line should be  If true: throw to monkey 0
        let true_line = lines.next();
        let true_line_parts:Vec<&str> =  true_line.unwrap().rsplit(' ').collect();
        let true_value = true_line_parts[0].parse::<i32>().unwrap();

        // line should be If false: throw to monkey 2
        let false_line = lines.next();
        let false_line_parts:Vec<&str> =  false_line.unwrap().rsplit(' ').collect();
        let false_value = false_line_parts[0].parse::<i32>().unwrap();


        let mut current_monkey = Monkey {
            items: starting_items,
            operation_operator: operation_op,
            operation_amount: operation_value,
            test_divisor: test_divisor,
            test_if_true:true_value,
            test_if_false: false_value,
            inspected_items_count: 0
        };

        monkeys.push(current_monkey);

        // Blank line seperating Monkeys
        lines.next();
    }

    // K, we should have all the monkeys loaded in

    let mut rounds = 20;
    while rounds > 0 {
        for i in 0..monkeys.len() {
        //for mut monkey in monkeys{
           let thrown_items = monkeys[i].inspect_items();
           for item in thrown_items  {
               monkeys[item.0 as usize].items.push(item.1);
           }
        }
        rounds-=1;


        // Debug Round results
        for i in 0..monkeys.len() {
        //for mut monkey in monkeys{
            println!("Monkey {}: {:?}", i, monkeys[i].items)
        }
    }

    let mut inspected_item_counts:Vec<i32>=monkeys.into_iter().map(|m| m.inspected_items_count).collect();
    inspected_item_counts.sort_by(|a, b| b.cmp(a));

    //println!("Monkey Business: {:?}", inspected_item_counts);
    let monkey_business = inspected_item_counts[0]*inspected_item_counts[1];
	// Correct Answer is 76728
    println!("Monkey Business: {}", monkey_business);
    Ok(())
}


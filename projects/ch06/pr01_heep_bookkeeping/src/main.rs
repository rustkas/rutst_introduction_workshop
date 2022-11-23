fn main() {
    println!("{}", get_final_orders());
}

fn get_final_orders() -> u32 {
    let orders = vec![1,2,3,4];
    let mut total_orders: u32 = 0;

    for order in orders.iter() {
        total_orders += order; 
    }

    let final_orders = finish(total_orders as u32);
    return final_orders;
}

fn finish(total_order: u32) -> u32{
    total_order
     
}
RUST lang. 
Towers of Hanoi without recursion. 

The result is output using an iterator:
    let hanoy=Hanoy::new(10);    
    hanoy.iter().enumerate().for_each(|(i,x)|{println!("Step {}",i);x.print()});

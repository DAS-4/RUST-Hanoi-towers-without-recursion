use std::time::Instant;

#[derive(Clone)]
struct Hanoy{
    from: Vec<u16>,
    to: Vec<u16>,
    addit: Vec<u16>,
}
impl Hanoy{
    fn new(cap: u16)->Self{
        Self{
            from: (1..cap+1).rev().collect(),
            to: Vec::new(),
            addit: Vec::new(),
        }
    }
    fn iter(&self)->HanoyIter{
        HanoyIter::new(self.clone())
    }
    fn exch(v1: &mut Vec<u16>, v2: &mut Vec<u16>)->bool{
        let v1_last;
        let v2_last;
        if let Some(x)=v1.last(){
            v1_last=x;
        }else{ v1_last=&(u16::MAX);}
        if let Some(x)=v2.last(){
            v2_last=x;
        }else{ v2_last=&(u16::MAX);}
        
        if v1_last<v2_last{
            v2.push(v1.pop().unwrap());
            return true;
        }else if v1_last>v2_last{
            v1.push(v2.pop().unwrap());
            return true;
        }
        false
    }
    fn print(&self){
        println!("From: {:?}",self.from);
        println!("Addit: {:?}",self.addit);
        println!("To: {:?}",self.to);
        println!();
    }
}


struct HanoyIter{
    hanoy:Hanoy,
    step:u8,
}
impl HanoyIter{
    fn new(h:Hanoy)->Self{
        Self{
            hanoy: h,
            step: 0,
        }
    }
    fn next_step(&mut self, cap: usize){
        if(cap&0x01)==1{
            const NXT_STP:[u8;4]=[1,2,3,1];
            self.step=NXT_STP[self.step as usize];
        }else{
            const NXT_STP:[u8;4]=[2,3,1,2];
            self.step=NXT_STP[self.step as usize];
        }
    }
}
impl Iterator for HanoyIter{
    type Item=Hanoy;
    fn next(&mut self) -> Option<Self::Item>{
        let cap=self.hanoy.from.len()+self.hanoy.to.len()+self.hanoy.addit.len();
        if self.step==0{// inicial state
        }else if self.step==1{
            if !Hanoy::exch(&mut self.hanoy.from, &mut self.hanoy.to){
                return None;   
            }
        }else if self.step==2{
            if !Hanoy::exch(&mut self.hanoy.from, &mut self.hanoy.addit){
                return None;   
            }
        }else{
            if !Hanoy::exch(&mut self.hanoy.addit, &mut self.hanoy.to){
                return None;   
            }
        }
        self.next_step(cap);
        Some(self.hanoy.clone())
    }
}

fn main(){
    let start=Instant::now();
    
    let hanoy=Hanoy::new(13);
    
    hanoy.iter().enumerate().for_each(|(i,x)|{println!("Step {}",i);x.print()});
    
    println!("Duration: {} ms",(Instant::now()-start).as_millis());
}

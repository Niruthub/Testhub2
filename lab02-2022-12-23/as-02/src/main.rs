fn main() {
    // ค่าอินพุต
    let num: u32 = 5;
    // x วนลูปแถว counter วนลูปหลัก
    let mut x = 0;
    let mut counter;
    
    loop{
        counter = 0;
        x+=1;
        while counter<(num+x-1){
            counter+=1;
            if(num-x+1) <= counter{
                print!("* ");
            }
            else{
                print!("  ");
            }      
        }
        println!("");
        
        if x == num {
            break;
        }
    }
}

fn main() {    
    let a: u32 = 3;
    let b: u32 = 4;
    let c: u32 = 5;
    
    if a.pow(2) + b.pow(2) == c.pow(2) {
        println!("Пифагорова тройка найдена: {}^2 + {}^2 = {}^2", a, b, c);
    } else {
        println!("Пифагорова тройка не найдена");
    }
}
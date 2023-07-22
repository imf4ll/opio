pub fn banner() {
    println!("\x1b[1;34m
                          _________
            _____________ ______  /
            __  ___/  __ `/  __  / 
            _  /   / /_/ // /_/ /  
            /_/    \\__,_/ \\__,_/
    
       \x1b[1;31mAUR helper & package downgrader\x1b[m\n");
}

pub fn about() -> String {
    "\x1b[1;31m
                          _________
            _____________ ______  /
            __  ___/  __ `/  __  / 
            _  /   / /_/ // /_/ /  
            /_/    \\__,_/ \\__,_/
    
       \x1b[1;31mAUR helper & package downgrader\x1b[m".to_string()
}

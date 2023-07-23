pub fn banner() {
    println!("\x1b[1;34m
                      _____       
        _________________(_)_____ 
        _  __ \\__  __ \\_  /_  __ \\
        / /_/ /_  /_/ /  / / /_/ /
        \\____/_  .___//_/  \\____/ 
              /_/

     \x1b[1;31mAUR helper & package downgrader\x1b[m\n");
}

pub fn about() -> String {
    "\x1b[1;34m
                      _____       
        _________________(_)_____ 
        _  __ \\__  __ \\_  /_  __ \\
        / /_/ /_  /_/ /  / / /_/ /
        \\____/_  .___//_/  \\____/ 
              /_/

     \x1b[1;31mAUR helper & package downgrader\x1b[m".to_string()
}

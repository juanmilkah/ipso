pub fn draw_ipso_ascii() {
    let ascii = r"
 .----------------.  .----------------.  .----------------.  .----------------. 
| .--------------. || .--------------. || .--------------. || .--------------. |
| |     _____    | || |   ______     | || |    _______   | || |     ____     | |
| |    |_   _|   | || |  |_   __ \   | || |   /  ___  |  | || |   .'    `.   | |
| |      | |     | || |    | |__) |  | || |  |  (__ \_|  | || |  /  .--.  \  | |
| |      | |     | || |    |  ___/   | || |   '.___`-.   | || |  | |    | |  | |
| |     _| |_    | || |   _| |_      | || |  |`\____) |  | || |  \  `--'  /  | |
| |    |_____|   | || |  |_____|     | || |  |_______.'  | || |   `.____.'   | |
| |              | || |              | || |              | || |              | |
| '--------------' || '--------------' || '--------------' || '--------------' |
 '----------------'  '----------------'  '----------------'  '----------------' 
"
    .to_string();

    println!("{}", ascii);
}

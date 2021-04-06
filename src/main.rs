use recursive_cmd::recursive;

#[path = "utils/args.rs"] mod args;
#[path = "commands/normal.rs"] mod cmd_normal;
#[path ="commands/recursive.rs"] mod recursive_cmd;

fn main() {
    let mut args = args::fetch_args();
    if args.len() <= 1{
        args.push(String::from("."))
    }
  
    if args.len() <= 2 && args.len() > 1{
        args.push(String::from("nothing"))
    }
    let action = args::fetch_action(&String::clone(&args[2]).as_str());

    if args::fetch_action(&String::clone(&args[1]).as_str()) == "r" && args.len() == 2 {
        args.pop();
        args.push(String::from("."));
        args.push(String::from("/r"))
    }
    if action == String::from("n") {
        cmd_normal::show_files(String::clone(&args[1]));
    } else {
        recursive(String::clone(&args[1]));
    }

  
}
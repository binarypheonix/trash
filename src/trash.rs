use std::process::{Command, Stdio, Child};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::env;

//TODO: Add support for: {redirection, custom prompts, config, shell scripting...and more...hopefully :) }

fn main(){
    println!("Welcome to Trash, TRuly Awesome SHell!");
    println!("Type \'help\' for Instructions\n");
    //the main shell REPL loop:
    loop{
        print!("$ ");
        let flu = stdout().flush();
        match flu{
            Ok(f)=>{},
            Err(e)=>{eprintln!("{}", e);},
        }
        //taking command and arguments as std input;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_cmd = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;

            match cmd{
                //cd command match (for builtin support)
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}", e);
                    }
                    prev_cmd = None;
                },
                //exit command match:
                "exit" => return,
                //sup command match (for fun) and help cmd
                "sup" => println!("Hello Trash-Talker!"), 
                "help" => {
                    println!("This is Trash, TRuly Awesome SHell\n");
                    println!("To execute commands, type <cmd-name> <args>");
                    println!("E.g. cd <dir-name>\n");
                    println!("Use | with multiple commands for piping\n");
                }
                //handling other commands, with piping
                cmd => {
                    //defining the std input stream for piped commands
                    let stdin = prev_cmd
                        .map_or(
                            Stdio::inherit(), 
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    //defining the std output stream for piped commands;
                    let stdout = if commands.peek().is_some() {
                        //if there are still piped cmds, use the piped stream;
                        Stdio::piped()
                    }else{
                        //else use the std output stream;
                        Stdio::inherit()
                    };

                    //executing the command using the stdin and stdout defined above;
                    let output = Command::new(cmd)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    //matching the output of the piped cmds to take care of errors;
                    match output {
                        Ok(output) => {prev_cmd = Some(output)},
                        Err(e) => {
                            prev_cmd = None;
                            eprintln!("{}", e);
                        },
                    };

                }

            }

        }
        
        if let Some(mut final_cmd) = prev_cmd {
            //blocking until final command has finished;
            let waiter = final_cmd.wait();
            match waiter{
                Ok(w)=>{},
                Err(e)=>{eprintln!("{}", e);},
            }
        }
        
    }
}
use clap::Parser;
use std::fs;


pub mod ui;
use ui::start;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    // Specify if you want to `remove` or `list` all the todos
    #[clap(subcommand)]
    pub action: Actions

}

#[derive(clap::Subcommand, Debug)]
enum Actions{
   Remove,
   List{
         // if action list than specify the file to list using the -f flag
         #[clap(short, long, default_value = "", global = true)]
         file: String,
   },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Todo{
    conent: String,
    priority: i32,
}

fn check_for_todo(line: &str, todos: &mut Vec<Todo>){
    if line.contains("~ TODO"){
        let words: Vec<&str> = line.split("").collect();
        let mut start: usize = 0;
        let mut end: usize= words.len();

        let mut t_start: usize = 0;
        let mut t_end: usize = words.len();

        for i in 0..words.len(){
            if words[i] == "~" && start == 0{
                start = i+2
            }
            if words[i] == "~" && i > start{
                end = i-1
            }

            if words[i] == "T" && t_start == 0{
                t_start = i
            }
            if words[i] == ":" && i > t_start && i != words.len(){
                t_end = i
            }
        }

        let priority = &words[t_start..t_end].join("").matches("O").count()-1;
        let todo: Todo = Todo { conent: words[start..end].join(""), priority: priority as i32 };
        let _ = &todos.push(todo);
    }
}
    
// ~ TODOOOOOOOO: Don't eat shit ~ //
// ~ TODOOO: Don't eat shit ~ //

fn quick_sort(seq: &mut Vec<Todo>) -> Vec<Todo> {

    if seq.len() <= 1 {
        return seq.to_vec();
    }

    let pivot = seq.pop();

    if let Some(i) = pivot {


        let mut it_g: Vec<Todo> = Vec::new();
        let mut it_s: Vec<Todo> = Vec::new();

        for td in seq{
            if td.priority < i.priority{
                it_g.push(td.clone());
            }
            else {
                it_s.push(td.clone());
            }
        }

        let mut out: Vec<Todo> = Vec::new();

        out.append(&mut quick_sort(&mut it_s));
        out.append(&mut vec![i]);
        out.append(&mut quick_sort(&mut it_g));


        return out;

    }
    else {
        return seq.to_vec();
    }
}


fn sort_by_priority(todos: &mut Vec<Todo>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();

    let seq = quick_sort(todos);
    for i in seq{
        new_vec.push(i.conent.to_string())
    }

    return new_vec;

}

fn main() {
    let args = CliArgs::parse();
    let mut todos: Vec<Todo> = Vec::new();

    match args.action{
        Actions::List { file } => {
            // read the file in the args.file and print an array of all the lines

            if file.is_empty(){
                println!("Please specify a file to list");
                panic!("Usage: todo list -f <file>");
            }
            let file_content = fs::read_to_string(&file).expect("Unable to read file");
            let lines = file_content.lines();
            for line in lines{
                check_for_todo(line, &mut todos);
            }
            let sorted_todos: Vec<String> = sort_by_priority(&mut todos);

            start(sorted_todos, file.to_string());
        
        },
        Actions::Remove => {
            println!("Removing all todos");
        }
    }

}
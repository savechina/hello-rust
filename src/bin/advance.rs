use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author="ren", version, about="about advance utils", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hello say Name
    Hello{name: String}
    ,
    /// Calc Two number
    Calc{
        #[command(subcommand)]
        operation: CalcCommands
    }
}

#[derive(Args)]
struct Mul{
     /// The first number
    a: i32,
    /// The second number 
    b: i32
}

#[derive(Args)]
struct Div{
     /// The first number
    a: i32,
    /// The second number 
    b: i32
}

#[derive(Subcommand)]
enum CalcCommands {
    /// Adds two numbers
    Add {
        /// The first number
        a: i32,
        /// The second number
        b: i32,
    },
    /// Subtracts two numbers
    Sub {
        /// The first number
        a: i32,
        /// The second number
        b: i32,
    },
    /// Multipy two numbers
    Mul(Mul),

    /// Divided two numbers
    Div(Div)
}

///advance command main entry
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello { name }=> {
            println!("hello to {}",name)
        },
        Commands::Calc{operation} => {
            excute_calc_command(operation);
        }
    }
}

///执行 clac command run
fn excute_calc_command(operation: &CalcCommands) {
    match operation {  
            CalcCommands::Add { a, b } => {
                println!("{} + {} = {}", a, b, a + b);
            }
            CalcCommands::Sub { a, b } => {
                println!("{} - {} = {}", a, b, a - b);
            },
            CalcCommands::Mul(Mul { a, b })=>{
                println!("{} * {} = {}",a,b,a*b);
            },
            CalcCommands::Div(s)=>{
                println!("{} / {} = {}",s.a,s.b,s.a/s.b);
            }
        }
}

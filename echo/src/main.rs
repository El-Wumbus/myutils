use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    match RunningAs::decide(opt)
    {
        RunningAs::Empty(_) => println!(),
        RunningAs::Normal(x) => println!("{}", x),
        RunningAs::NoNewline(x) => print!("{}", x),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "echo")]
struct Opt
{
    /// Print string
    #[structopt(name = "string")]
    string: Vec<String>,

    /// Don't print trailing newline
    #[structopt(short, long)]
    no_newline: bool,

}
impl Opt
{
    fn join(&self) -> String
    {
        if !self.string.is_empty()
        {
            self.string.join(" ")
        }
        else 
        {
            String::from("")
        }
    }
}
enum RunningAs
{
    Empty(bool),
    Normal(String),
    NoNewline(String),
}

impl RunningAs
{
    fn decide(opt: Opt) -> RunningAs
    {
        if opt.no_newline
        {
            return RunningAs::NoNewline(opt.join());
        }
        if opt.join() != String::from("")
        {
            return RunningAs::Normal(opt.join());
        }
        RunningAs::Empty(true)
    }
}
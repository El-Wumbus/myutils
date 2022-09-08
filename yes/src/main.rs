use structopt::StructOpt;

fn main()
{
    let opt = Opt::from_args();
    match RunningAs::decide(opt)
    {
        RunningAs::Normal(_) => loop{println!("y");},
        RunningAs::String(x) => loop{println!("{}", x);},
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "yes")]
struct Opt
{
    /// Print string instead of 'y'
    #[structopt(name = "string")]
    string: Vec<String>,
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
    Normal(bool),
    String(String),
}

impl RunningAs
{
    fn decide(opt: Opt) -> RunningAs
    {
        
        if opt.join() != String::from("")
        {
            return RunningAs::String(opt.join())
        }
        return RunningAs::Normal(true)
    }
}

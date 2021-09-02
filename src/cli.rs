use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "Todo", about = "Command-line todo application")]
pub enum Cli {
    /// Create new task
    Add {
        /// Task title
        #[structopt(short, long)]
        title: String,
    },
    /// Modify existing task
    Update {
        /// Task index from task list
        #[structopt(short, long)]
        index: usize,
        /// Task title
        #[structopt(short, long)]
        title: String,
    },
    /// Delete task
    Delete {
        /// Task index
        #[structopt(short, long, required_unless_all = &["all"])]
        index: usize,
    },
    /// Get task list
    List,
    /// Delete all tasks
    DeleteAll
}

impl Cli {
    pub fn get_arguments() -> Self {
        Cli::from_args()
    }
}
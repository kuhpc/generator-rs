use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
#[structopt(name = "generator", author)]
pub struct GeneratorConfig {
    #[structopt(long)]
    pub excel_path: Option<PathBuf>,

    #[structopt(long, default_value = "export.sh")]
    pub destination_path: String,

    #[structopt(long)]
    pub accounting: Option<String>,

    #[structopt(long)]
    pub max_jobs: Option<String>,

    #[structopt(long)]
    pub max_cpu: Option<String>,
}

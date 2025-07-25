use clap::{Parser, Subcommand};
use anyhow::{Result};

pub mod commands;
use commands::{verify, claim};


#[derive(Parser, Debug)]
#[clap(version)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Verify {
        #[arg(short, long, required=true)]
        verification_key: String,
        #[arg(short, long, required=true)]
        proof: String,
        #[arg(short, long, required=true)]
        inp: String,
    },
    Claim {
        #[arg(short, long, required=true)]
        verification_key: String,
        #[arg(short, long, required=true)]
        proof: String,
        #[arg(short, long, required=true)]
        inp: String,
        #[arg(short, long, required=true)]
        dkim_pk: String,
        #[arg(short, long, required=true)]
        address: String,
        #[arg(short, long, required=true)]
        resolver: String,
        #[arg(required=true)]
        email: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Verify {verification_key, proof, inp} => verify::exec(&verification_key, &proof, &inp),
        Commands::Claim {verification_key, proof, inp, dkim_pk, address, resolver, email} =>
            claim::exec(&verification_key, &proof, &inp, &dkim_pk, &address, &resolver, &email)
    }
}

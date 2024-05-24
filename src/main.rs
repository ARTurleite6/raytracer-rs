use clap::Parser;
use raytracer_lib::raytracer::{Configuration, RayTracer};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "configuration.json")]
    configuration: String,
}

fn load_configuration(configuration_path: &str) -> Configuration {
    let file = std::fs::File::open(configuration_path).expect("Configuration file does not exist");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader::<std::io::BufReader<std::fs::File>, Configuration>(reader)
        .expect("Error loading Configuration file")
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let configuration = load_configuration(&args.configuration);
    let output_file = &configuration.output_file.clone();
    RayTracer::with_configuration(configuration)?.render(output_file);

    Ok(())
}

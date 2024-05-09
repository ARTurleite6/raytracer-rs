use clap::Parser;
use raytracer_lib::raytracer::{Configuration, RayTracer};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "configuration.json")]
    configuration: String,
    //#[arg(short, long, default_value = "models/cornell_box_VI.obj")]
    //model: String,
    //#[arg(short, long, default_value = "camera.json")]
    //camera: String,
    //#[arg(short, long, default_value = "10")]
    //samples_per_pixel: usize,
}

fn load_configuration(configuration_path: &str) -> Configuration {
    let file = std::fs::File::open(configuration_path).expect("Configuration file does not exist");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader::<std::io::BufReader<std::fs::File>, Configuration>(reader)
        .expect("Error loading Configuration file")
        .into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let configuration = load_configuration(&args.configuration);
    let output_file = &configuration.output_file.clone();
    // RayTracer::new(&args.model, &args.camera, args.samples_per_pixel)?.render();
    RayTracer::with_configuration(configuration).render(output_file);

    Ok(())
}

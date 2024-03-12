use clap::Parser;
use raytracer_lib::raytracer::RayTracer;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "models/cornell_box.obj")]
    model: String,
    #[arg(short, long, default_value = "camera.json")]
    camera: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    RayTracer::new(&args.model, &args.camera)?.render();

    Ok(())
}

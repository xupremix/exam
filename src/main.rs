use ai_robot::prelude::MlRobot;
use clap::{Parser, Subcommand};
use oxag_ai_j::WrapperTrashinatorRobot;
use graphics_lib::client::app::App;
use graphics_lib::winit::event_loop::EventLoopBuilder;

#[derive(Subcommand)]
enum Mode {
    Giulio,
    Filippo,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();
    match args.mode {
        Mode::Giulio => {
            let mut gl_robot = WrapperTrashinatorRobot::new(10);
            let eventloop = EventLoopBuilder::new().build().unwrap();
            let mut app = App::new(&eventloop, (38, 48), "Trashinator").unwrap();
            app.start(eventloop, move || {
                let ris = gl_robot.ai_process_tick();
                eprintln!("{}", ris.0);
                ris
            }).unwrap()
        }
        Mode::Filippo => {
            let mut lol_robot = MlRobot::new()
                .set_log(false)
                .load_model("src/exam_model.pt")
                .set_map("src/exam_map.bin", 14, 10)
                .build();
            let eventloop = EventLoopBuilder::new().build().unwrap();
            let mut app = App::new(&eventloop, (48, 23), "Mlrobot").unwrap();
            app.start(eventloop, move || {
                let ris = lol_robot.step();
                eprintln!("{}", ris.0);
                ris
            }).unwrap()
        }
    }
}

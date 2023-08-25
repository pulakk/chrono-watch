mod plotters;
mod listeners;

fn main() {
    let mut plotter = plotters::StreamPlotter::new();
    listeners::listen_stdin_for_floats(|x| plotter.add(x));
}

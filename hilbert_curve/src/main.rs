use argh::FromArgs;
use plotters::prelude::*;
use std::process::Command;

#[derive(FromArgs)]
#[argh(description="Draw a space filling Hilbert curve")]
struct Args {
  #[argh(option, short='o', description="order of the Hilbert curve to draw")]
  order: u32,

  #[argh(option, short='w', default="1024", description="width of the picture")]
  width: u32,

  #[argh(option, short='h', default="1024", description="height of the picture")]
  height: u32,

  #[argh(switch, short='v', description="verbose output")]
  verbose: bool,
}

fn flip1(points: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    points.iter()
        .map(|(x, y)| (*y, *x))
        .collect::<Vec<_>>()
}

fn flip2(points: Vec<(u32, u32)>, n: u32) -> Vec<(u32, u32)> {
    points.iter()
        .map(|(x, y)| (n-*y, n-*x))
        .collect::<Vec<_>>()
}

fn shift(points: Vec<(u32, u32)>, (dx, dy): (u32, u32)) -> Vec<(u32, u32)> {
    points.iter()
        .map(|(x,y)| (x+dx, y+dy))
        .collect::<Vec<_>>()
}

fn hilbert_points(order: u32) -> Vec<(u32, u32)> {
    if order == 0 {
        vec![(0, 0)]
    } else {
        let mut points = flip1(hilbert_points(order-1));
        points.extend(shift(hilbert_points(order-1), (0, 2u32.pow(order-1))));
        points.extend(shift(hilbert_points(order-1), (2u32.pow(order-1), 2u32.pow(order-1))));
        points.extend(shift(flip2(hilbert_points(order-1), 2u32.pow(order-1)-1), (2u32.pow(order-1), 0)));
        points
    }
}

fn show_hilbert_curve(order: u32, width: u32, height: u32, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Order: {order}");
    }

    let points = hilbert_points(order);
    if verbose {
        println!("Hilbert points: {:?}", points);
    }

    let filename = format!("/tmp/hilbert_order_{order}.png");

    let root = BitMapBackend::new(filename.as_str(), (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let title = format!("Hilbert curver order {order}");

    let mut chart = ChartBuilder::on(&root)
        .caption(&title, ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..(2u32.pow(order)-1) as f32, 0f32..(2u32.pow(order)-1) as f32)?;

    chart
        .draw_series(LineSeries::new(
                    points.iter().map(|(x, y)| (*x as f32, *y as f32)),
                    &RED,
                    ))?
        .label(title)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    root.present()?;
    if verbose {
        println!("Wrote {filename}");
    }

    Command::new("qiv")
        .arg(filename.clone())
        .output()
        .expect("failed to execute qiv");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Args = argh::from_env();
    if args.verbose {
        println!("Computing curves for orders from 0 to {}", args.order);
    }

    for order in 0..=args.order {
        show_hilbert_curve(order, args.width, args.height, args.verbose)?;
    }

/*
    // This was an attempt to use plotters crate with a GTK display but I didn't manage to make it
    // work.
    let application = gtk::Application::new(
        Some("Hilbert space-filling curve"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = window::Window::new(app);
        win.show();
    });

    application.run();
*/
    Ok(())
}

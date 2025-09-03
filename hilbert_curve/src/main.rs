use argh::FromArgs;
use derive_more::{Display,Error};
use plotters::prelude::*;
use std::process::Command;

#[derive(FromArgs)]
#[argh(description="Draw a space filling Hilbert curve")]
struct Args {
#[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Hilbert(DrawCurve),
    Garden(DrawGarden),
}

impl Subcommand {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Subcommand::Hilbert(x) => draw_hilbert_curves(x),
            Subcommand::Garden(x) => draw_garden(x),
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name="hilbert", description="Draw a space filling Hilbert curve")]
struct DrawCurve {
  #[argh(option, short='o', description="order of the Hilbert curve to draw")]
  order: u32,

  #[argh(option, short='w', default="1024", description="width of the picture")]
  width: u32,

  #[argh(option, short='h', default="1024", description="height of the picture")]
  height: u32,

  #[argh(switch, short='v', description="verbose output")]
  verbose: bool,
}

#[derive(Debug,Display)]
#[display("{:?}", plants)]
struct Plants {
    plants: Vec<(char, u32)>,
}

#[derive(Debug, Display, PartialEq, Eq, Error)]
struct ParsePlantError {
    message: String
}

#[derive(Debug, Display, PartialEq, Eq, Error)]
struct NotEnoughPlantsError;

impl ParsePlantError {
    fn new(message: String) -> Self {
        ParsePlantError { message }
    }
}

impl std::str::FromStr for Plants {
    type Err = ParsePlantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let plants = value.split(',')
            .map(|s| {
                let (kind_str, nb_str) = s.split_once(':').ok_or(ParsePlantError::new(format!("Invalid plant '{s}'")))?;
                let nb = nb_str.parse::<u32>().map_err(|_| ParsePlantError::new(format!("Invalid number '{nb_str}'")))?;
                if kind_str.len() != 1 {
                    return Err(ParsePlantError::new(format!("Invalid kind '{kind_str}'")));
                }
                let kind = kind_str.chars().next().ok_or(ParsePlantError::new(format!("Invalid kind '{kind_str}'")))?;
                Ok((kind, nb))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Plants { plants })
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name="garden", description="Draw a garden")]
struct DrawGarden {
  #[argh(option, short='W', default="1024", description="width of the picture in pixels")]
  pic_width: u32,

  #[argh(option, short='H', default="1024", description="height of the picture in pixels")]
  pic_height: u32,

  #[argh(option, short='w', default="10", description="width of the garden")]
  width: u32,

  #[argh(option, short='h', default="10", description="height of the garden")]
  height: u32,

  #[argh(option, short='p', description="plants")]
  plants: Plants,

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

fn draw_hilbert_curves(args: DrawCurve) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("Computing curves for orders from 0 to {}", args.order);
    }

    for order in 0..=args.order {
        show_hilbert_curve(order, args.width, args.height, args.verbose)?;
    }

    Ok(())
}

fn draw_garden(args: DrawGarden) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("Garden size: {}x{}", args.width, args.height);
        println!("Plants: {}", args.plants);
    }
    let order = (((std::cmp::max(args.width, args.height)-1) as f32).log2() as u32) + 1;
    let mut total_count = 0;
    let mut plant_count = 0;
    let mut plant_idx = 0;
    let filename = format!("/tmp/garden.png");
    let mut root = BitMapBackend::new(filename.as_str(), (args.pic_width, args.pic_height));
    let (w, h) = ((args.pic_width / args.width) as i32, (args.pic_height / args.height) as i32);

    let colors = [
        RGBColor(255, 0, 0), RGBColor(0, 255, 0), RGBColor(0, 0, 255),
        RGBColor(255, 255, 0), RGBColor(255, 0, 255), RGBColor(0, 255, 255),
        RGBColor(127, 0, 0), RGBColor(0, 127, 0), RGBColor(0, 0, 127),
        RGBColor(127, 127, 0), RGBColor(127, 0, 127), RGBColor(0, 127, 127),
        RGBColor(255, 127, 0), RGBColor(255, 0, 127),
        RGBColor(127, 255, 0), RGBColor(127, 0, 255),
        RGBColor(0, 255, 127), RGBColor(0, 127, 255),
    ];

    let margin = 1;
    let mut prev: Option<(i32, i32)> = None;
    for p in hilbert_points(order) {
        let (x, y) = (p.0 as i32 * w, p.1 as i32 * h);
        if p.0 >= args.width || p.1 >= args.height {
            if let Some(p1) = prev {
                root.draw_line((p1.0 + w/2, p1.1 + h/2), (x + w/2, y + h/2), &WHITE)?;
            }
            prev = Some((x, y));
            continue
        }
        if plant_idx >= args.plants.plants.len() {
            return Err(Box::new(NotEnoughPlantsError));
        }
        if plant_count >= args.plants.plants[plant_idx].1 {
            plant_count = 1;
            plant_idx += 1;
            if plant_idx >= args.plants.plants.len() {
                return Err(Box::new(NotEnoughPlantsError));
            }
        } else {
            plant_count += 1;
        }
        total_count += 1;
        if args.verbose {
            println!("{total_count}: {plant_count}: {:?} -> {}", p, args.plants.plants[plant_idx].0);
        }
        root.draw_rect((x+margin, y+margin), (x+w-margin, y+h-margin), &colors[plant_idx%colors.len()], true)?;
        if let Some(p1) = prev {
            root.draw_line((p1.0 + w/2, p1.1 + h/2), (x + w/2, y + h/2), &WHITE)?;
        }
        prev = Some((x, y));
    }

    root.present()?;

    Command::new("qiv")
        .arg(filename.clone())
        .output()
        .expect("failed to execute qiv");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Args = argh::from_env();
    args.subcommand.run()
}

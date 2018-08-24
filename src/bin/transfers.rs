// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate navitia_model;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use navitia_model::Result;

#[derive(Debug, StructOpt)]
#[structopt(name = "transfers", about = "Generate transfers.")]
struct Opt {
    /// input directory.
    #[structopt(short = "i", long = "input", parse(from_os_str), default_value = ".")]
    input: PathBuf,

    /// output directory
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    // /// config file
    // #[structopt(short = "c", long = "config", parse(from_os_str))]
    // config_path: Option<PathBuf>,
    #[structopt(
        long = "max-distance",
        short = "d",
        default_value = "500",
        help = "The max distance in meters to compute the tranfer"
    )]
    max_distance: f64,

    #[structopt(
        long = "walking-speed",
        short = "s",
        default_value = "0.785",
        help = "The walking speed in meters per second. \
                You may want to divide your initial speed by \
                sqrt(2) to simulate Manhattan distances"
    )]
    walking_speed: f64,
}

fn run() -> Result<()> {
    info!("Launching transfers...");

    let opt = Opt::from_args();

    let model = navitia_model::ntfs::read(opt.input)?;
    let mut collections = model.into_collections();

    info!("Generating transfers...");
    navitia_model::transfers::generates_transfers(
        &mut collections.transfers,
        &collections.stop_points,
        opt.max_distance,
        opt.walking_speed,
    );

    let model = navitia_model::Model::new(collections)?;
    navitia_model::ntfs::write(&model, opt.output)?;
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(err) = run() {
        for cause in err.iter_chain() {
            eprintln!("{}", cause);
        }
        std::process::exit(1);
    }
}

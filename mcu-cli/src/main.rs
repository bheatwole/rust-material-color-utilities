use std::{
    fs::{metadata, File},
    io::Write,
    path::PathBuf,
};

use clap::{command, Arg, Command};
use material_color_utilities::{
    palettes::*,
    scheme::Scheme,
    utils::string_utils::{argb_from_hex, hex_from_argb},
};

fn main() {
    // Create the command line application
    let matches = command!()
        .version("1.0")
        .author("Your Name")
        .about("A simple color application")
        .subcommand(
            Command::new("generate-css")
                .about("Generate CSS with color settings")
                .arg(
                    Arg::new("primary")
                        .short('p')
                        .long("primary")
                        .required(true)
                        .value_name("#001122")
                        .help("Sets the primary color"),
                )
                .arg(
                    Arg::new("secondary")
                        .short('s')
                        .long("secondary")
                        .value_name("#001122")
                        .help("Sets the secondary color"),
                )
                .arg(
                    Arg::new("tertiary")
                        .short('t')
                        .long("tertiary")
                        .value_name("#001122")
                        .help("Sets the tertiary color"),
                )
                .arg(
                    Arg::new("error")
                        .short('e')
                        .long("error")
                        .value_name("#001122")
                        .help("Sets the error color"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .required(true)
                        .value_name("DIR")
                        .help("Sets the output directory"),
                ),
        )
        .get_matches();

    // Check if the 'generate-css' command was used
    if let Some(matches) = matches.subcommand_matches("generate-css") {
        // Create the core palette based off the colors that were passed in
        let colors = CorePaletteColors {
            primary: argb_from_hex(matches.get_one::<String>("primary").unwrap()),
            secondary: matches
                .get_one::<String>("secondary")
                .map(|hex| argb_from_hex(hex)),
            tertiary: matches
                .get_one::<String>("tertiary")
                .map(|hex| argb_from_hex(hex)),
            neutral: None,
            neutral_variant: None,
            error: matches
                .get_one::<String>("error")
                .map(|hex| argb_from_hex(hex)),
        };
        let core = CorePalette::from_colors(colors);

        // Create a light and dark scheme from the core palette
        let light = Scheme::light_from_core_palette(&core);
        let dark = Scheme::dark_from_core_palette(&core);

        // Verify we can write to the output directory
        let out_dir = matches.get_one::<String>("output").unwrap();
        if !is_directory_writable(&out_dir) {
            println!("Cannot write to '{}', quitting", out_dir);
            return;
        }

        let mut path = PathBuf::from(out_dir);
        path.push("tokens.css");
        let mut file = File::create(&path).unwrap();
        writeln!(file, ":root {{").unwrap();
        write_scheme_to_file(&light, &mut file, "md-sys-color", "light").unwrap();
        write_scheme_to_file(&dark, &mut file, "md-sys-color", "dark").unwrap();
        writeln!(file, "}}").unwrap();
    }
}

fn is_directory_writable(directory: &str) -> bool {
    let metadata = metadata(directory);
    if let Ok(metadata) = metadata {
        metadata.is_dir() && !metadata.permissions().readonly()
    } else {
        false
    }
}

fn write_scheme_to_file(
    scheme: &Scheme,
    file: &mut File,
    prefix: &str,
    suffix: &str,
) -> std::io::Result<()> {
    // Iterate over the fields of the Scheme struct
    for field in &[
        ("primary", scheme.primary),
        ("on_primary", scheme.on_primary),
        ("primary_container", scheme.primary_container),
        ("on_primary_container", scheme.on_primary_container),
        ("secondary", scheme.secondary),
        ("on_secondary", scheme.on_secondary),
        ("secondary_container", scheme.secondary_container),
        ("on_secondary_container", scheme.on_secondary_container),
        ("tertiary", scheme.tertiary),
        ("on_tertiary", scheme.on_tertiary),
        ("tertiary_container", scheme.tertiary_container),
        ("on_tertiary_container", scheme.on_tertiary_container),
        ("error", scheme.error),
        ("on_error", scheme.on_error),
        ("error_container", scheme.error_container),
        ("on_error_container", scheme.on_error_container),
        ("surface_dim", scheme.surface_dim),
        ("surface", scheme.surface),
        ("surface_bright", scheme.surface_bright),
        ("surface_container_lowest", scheme.surface_container_lowest),
        ("surface_container_low", scheme.surface_container_low),
        ("surface_container", scheme.surface_container),
        ("surface_container_high", scheme.surface_container_high),
        (
            "surface_container_highest",
            scheme.surface_container_highest,
        ),
        ("on_surface", scheme.on_surface),
        ("on_surface_variant", scheme.on_surface_variant),
        ("outline", scheme.outline),
        ("outline_variant", scheme.outline_variant),
        ("inverse_surface", scheme.inverse_surface),
        ("inverse_on_surface", scheme.inverse_on_surface),
        ("inverse_primary", scheme.inverse_primary),
        ("scrim", scheme.scrim),
        ("shadow", scheme.shadow),
    ] {
        writeln!(
            file,
            "  --{}-{}-{}: \"{}\";",
            prefix,
            field.0.replace("_", "-"),
            suffix,
            hex_from_argb(field.1)
        )?;
    }

    Ok(())
}

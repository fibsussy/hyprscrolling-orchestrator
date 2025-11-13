use anyhow::Result;
use clap::{Parser, Subcommand};
use hyprscrolling_orchestrator::hypr;
use hyprscrolling_orchestrator::{
    columnize_active_workspace, parse_abs_row, parse_col_row,
    focus_col_row, focus_abs_cycle, focus_abs_row, moveto_abs, moveto_abs_row,
};

#[derive(Parser, Debug)]
#[command(
    name="hyprscrolling-orchestrator",
    author, version,
    about="ABS navigation for hyprscrolling (focus cycles; moveto uses swapcol only)"
)]
struct Cli {
    /// Dry-run: print intended hyprctl dispatches, one per line
    #[arg(long, global = true)]
    dry: bool,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print current columns/rows
    Print,
    /// Focus by abs; accepts `N` or `N.R`
    Focus { target: String },
    /// Move active window using abs; accepts `N` or `N.R` (1-based).
    Moveto { target: String },
    /// (Debug) Focus by column/row: `C` or `C.R` (0-based)
    FocusCol { target: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    hypr::set_dry(cli.dry);

    match cli.cmd {
        Some(Cmd::Focus { target }) => {
            if target.contains('.') {
                let (abs, row) = parse_abs_row(&target)?;
                focus_abs_row(abs, row)?;
            } else {
                let abs: i32 = target.parse()?;
                focus_abs_cycle(abs)?;
            }
            return Ok(());
        }
        Some(Cmd::Moveto { target }) => {
            let (abs, row) = parse_abs_row(&target)?;
            if target.contains('.') {
                // Row ignored by design for simplified moveto
                moveto_abs_row(abs, Some(row))?;
            } else {
                moveto_abs(abs)?;
            }
            return Ok(());
        }
        Some(Cmd::FocusCol { target }) => {
            let (c, r) = parse_col_row(&target)?;
            focus_col_row(c, r)?;
            return Ok(());
        }
        Some(Cmd::Print) | None => {}
    }

    let ws_id = hyprscrolling_orchestrator::hypr::active_workspace_id()?;
    println!("Active workspace id: {ws_id}");

    let cols = columnize_active_workspace(None)?;
    println!(
        "{:>4} {:>4} {:>7} {:>7}  {:18}  {:<16}  title",
        "col", "row", "x", "y", "address", "class"
    );
    for c in &cols {
        for r in &c.rows {
            println!(
                "{:>4} {:>4} {:>7} {:>7}  {:18} {:<17} {}",
                c.col, r.row, c.x, r.y, r.client.address, r.client.class, r.client.title
            );
        }
    }

    println!("\nEnv knobs:");
    println!("  HYPR_COL_EPS=<px>    # column-grouping tolerance (default 40)");
    println!("Use --dry to print dispatches, one per line.");

    Ok(())
}

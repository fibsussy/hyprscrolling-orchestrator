#![allow(
    clippy::missing_errors_doc,
    clippy::doc_markdown,
    clippy::too_long_first_doc_paragraph,
    clippy::needless_return
)]

pub mod model;
pub mod hypr;

use anyhow::{bail, Context, Result};
use model::Client;
use std::env;

/// Tuple-type used everywhere for coordinates/sizes.
pub type Cords = (i32, i32);

#[derive(Debug, Clone)]
pub struct ColumnRow {
    pub row: usize,
    pub y: i32,
    pub client: Client,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub col: usize,
    pub x: i32,
    pub rows: Vec<ColumnRow>,
}

/* ---------- discovery ---------- */

fn column_epsilon_from_env() -> i32 {
    env::var("HYPR_COL_EPS").ok().and_then(|v| v.parse::<i32>().ok()).unwrap_or(40)
}

pub fn clients_on_current_workspace() -> Result<Vec<Client>> {
    let cur = hypr::active_workspace_id()?;
    let mut v = hypr::list_clients()?;
    v.retain(|c| c.mapped && !c.hidden && c.workspace.id == cur);
    Ok(v)
}

pub fn columnize_active_workspace(eps: Option<i32>) -> Result<Vec<Column>> {
    let mut clients = clients_on_current_workspace()?;
    clients.sort_by_key(|c| (c.at.0, c.at.1));

    let eps = eps.unwrap_or_else(column_epsilon_from_env);
    let mut cols: Vec<Column> = Vec::new();

    for c in clients {
        if let Some(last) = cols.last_mut() {
            if (c.at.0 - last.x).abs() <= eps {
                last.rows.push(ColumnRow { row: 0, y: c.at.1, client: c });
                continue;
            }
        }
        cols.push(Column { col: cols.len(), x: c.at.0, rows: vec![ColumnRow { row: 0, y: c.at.1, client: c }] });
    }

    for col in &mut cols {
        col.rows.sort_by_key(|r| r.y);
        for (i, r) in col.rows.iter_mut().enumerate() { r.row = i; }
    }
    Ok(cols)
}

/* ---------- helpers ---------- */

pub fn parse_abs_row(s: &str) -> Result<(i32, usize)> {
    if let Some((a, r)) = s.split_once('.') {
        Ok((a.trim().parse::<i32>()?, r.trim().parse::<usize>()?))
    } else {
        Ok((s.trim().parse::<i32>()?, 0))
    }
}
pub fn parse_col_row(s: &str) -> Result<(usize, usize)> {
    if let Some((c, r)) = s.split_once('.') {
        Ok((c.trim().parse::<usize>()?, r.trim().parse::<usize>()?))
    } else {
        Ok((s.trim().parse::<usize>()?, 0))
    }
}
fn find_col_row_by_addr(cols: &[Column], addr: &str) -> Option<(usize, usize)> {
    for c in cols { for r in &c.rows { if r.client.address == addr { return Some((c.col, r.row)); } } }
    None
}

fn clamp_abs_to_col_index(abs: i32, ncols: usize) -> usize {
    if ncols == 0 { return 0; }
    // Clamp to >=1 in i32 space, then losslessly convert.
    let base_i32 = (abs - 1).max(0);
    let base = usize::try_from(base_i32).unwrap_or(0);
    base.min(ncols - 1)
}

/* ---------- focus ---------- */

pub fn focus_col_row(col: usize, row: usize) -> Result<()> {
    let cols = columnize_active_workspace(None)?;
    let c = cols.get(col).with_context(|| format!("no such column {col}"))?;
    let r = c.rows.get(row).with_context(|| format!("no such row {row} in col {col}"))?;
    hypr::focus_address(&r.client.address)
}

/// Focus by 1-based column ABS, cycling if already in that column on bare `N`.
pub fn focus_abs_cycle(abs: i32) -> Result<()> {
    let cols = columnize_active_workspace(None)?;
    if cols.is_empty() { bail!("no columns"); }
    let target_col = clamp_abs_to_col_index(abs, cols.len());
    let dest = cols.get(target_col).context("target column missing")?;

    let aw = hypr::active_window()?;
    let cur = find_col_row_by_addr(&cols, &aw.address);

    let mut row = 0usize;
    if let Some((cc, cr)) = cur {
        if cc == target_col && !dest.rows.is_empty() {
            let n = dest.rows.len();
            row = (cr + 1) % n; // cycle next
        }
    }
    focus_col_row(target_col, row)
}

/// Focus explicit ABS.row without cycling.
pub fn focus_abs_row(abs: i32, row: usize) -> Result<()> {
    let cols = columnize_active_workspace(None)?;
    if cols.is_empty() { bail!("no columns"); }
    let target_col = clamp_abs_to_col_index(abs, cols.len());
    focus_col_row(target_col, row)
}

/* ---------- moveto using only swapcol ---------- */

pub fn moveto_abs_row(abs: i32, _maybe_row: Option<usize>) -> Result<()> {
    // Snapshot + locate active
    let cols = columnize_active_workspace(None)?;
    if cols.is_empty() { bail!("no columns on current workspace"); }

    let aw = hypr::active_window()?;
    let (src_col, _src_row) = find_col_row_by_addr(&cols, &aw.address).unwrap_or((0, 0));

    // Target by 1-based index (clamped to existing columns)
    let tgt_col = clamp_abs_to_col_index(abs, cols.len());

    // Horizontal swaps — compute in i32
    let tc = i32::try_from(tgt_col).unwrap_or(i32::MAX);
    let sc = i32::try_from(src_col).unwrap_or(i32::MAX);
    let delta = tc - sc;

    hypr::swapcol_delta(delta)
}

pub fn moveto_abs(abs: i32) -> Result<()> {
    moveto_abs_row(abs, None)
}

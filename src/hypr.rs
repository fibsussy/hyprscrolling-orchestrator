#![allow(
    clippy::missing_errors_doc,
    clippy::doc_markdown,
    clippy::too_long_first_doc_paragraph,
    clippy::redundant_closure_for_method_calls
)]

use anyhow::{Context, Result};
use regex::Regex;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::model::{ActiveWorkspace, Client, Workspace};

static DRY: AtomicBool = AtomicBool::new(false);

pub fn set_dry(b: bool) { DRY.store(b, Ordering::Relaxed); }
pub fn is_dry() -> bool { DRY.load(Ordering::Relaxed) }

fn run(args: &[&str]) -> Result<std::process::Output> {
    // Reads always run, not dry.
    let out = Command::new("hyprctl")
        .args(args)
        .output()
        .with_context(|| format!("failed to run hyprctl {args:?}"))?;
    if !out.status.success() {
        return Err(anyhow::anyhow!(
            "hyprctl {args:?} exited with {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(out)
}

/// In --dry: print each command per line. In real: batch them.
pub fn run_batch_dispatch(cmds: &[&str]) -> Result<()> {
    if is_dry() {
        for c in cmds { println!("hyprctl {c}"); }
        return Ok(());
    }
    let joined = cmds.join("; ");
    let out = Command::new("hyprctl")
        .args(["--batch", &joined])
        .output()
        .context("failed to run hyprctl --batch")?;
    if !out.status.success() {
        return Err(anyhow::anyhow!(
            "hyprctl --batch exited with {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

fn run_json<T: serde::de::DeserializeOwned>(args: &[&str]) -> Result<T> {
    let out = run(args)?;
    let v = serde_json::from_slice::<T>(&out.stdout)
        .with_context(|| format!("invalid JSON from hyprctl {args:?}"))?;
    Ok(v)
}

/* ---- queries ---- */

pub fn active_workspace_id() -> Result<i64> {
    let aw: ActiveWorkspace = run_json(&["activeworkspace", "-j"])?;
    Ok(aw.id)
}
pub fn list_clients() -> Result<Vec<Client>> { run_json(&["clients", "-j"]) }
pub fn active_window() -> Result<Client> { run_json(&["activewindow", "-j"]) }
pub fn list_workspaces() -> Result<Vec<Workspace>> {
    run_json::<serde_json::Value>(&["workspaces", "-j"]).map_or_else(
        |_| list_workspaces_text(),
        |v| {
            let mut out = Vec::new();
            if let Some(arr) = v.as_array() {
                for w in arr {
                    let id = w.get("id").and_then(serde_json::Value::as_i64).unwrap_or_default();
                    let name = w.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    let monitor = w.get("monitor").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    let monitor_id = w.get("monitorID").and_then(serde_json::Value::as_i64).unwrap_or_default();
                    let windows = w.get("windows").and_then(serde_json::Value::as_i64).unwrap_or_default();
                    let has_fullscreen = w.get("hasfullscreen").and_then(serde_json::Value::as_i64).unwrap_or(0) != 0;
                    let last_window = w.get("lastwindow").and_then(|x| x.as_str()).map(std::string::ToString::to_string);
                    let last_window_title = w.get("lastwindowtitle").and_then(|x| x.as_str()).map(std::string::ToString::to_string);
                    let is_persistent = w.get("ispersistent").and_then(serde_json::Value::as_i64).unwrap_or(0) != 0;
                    let extra = w.clone();
                    out.push(Workspace {
                        id, name, monitor, monitor_id, windows, has_fullscreen,
                        last_window, last_window_title, is_persistent, extra,
                    });
                }
            }
            Ok(out)
        },
    )
}

fn list_workspaces_text() -> Result<Vec<Workspace>> {
    let out = run(&["workspaces"])?;
    let s = String::from_utf8_lossy(&out.stdout);
    let header_re =
        Regex::new(r"^workspace ID (?P<id>\d+)\s+\((?P<name>[^)]+)\)\s+on monitor (?P<mon>[^:]+):")
            .unwrap();
    let kv_re = Regex::new(r"^\s*(?P<k>\w+):\s*(?P<v>.+)$").unwrap();

    let mut res = Vec::new();
    let mut cur: Option<Workspace> = None;
    for line in s.lines() {
        if let Some(caps) = header_re.captures(line) {
            if let Some(w) = cur.take() { res.push(w); }
            let id = caps["id"].parse::<i64>().unwrap_or_default();
            let name = caps["name"].to_string();
            let monitor = caps["mon"].to_string();
            cur = Some(Workspace {
                id, name, monitor,
                monitor_id: 0, windows: 0, has_fullscreen: false,
                last_window: None, last_window_title: None, is_persistent: false,
                extra: serde_json::json!({}),
            });
            continue;
        }
        if let Some(caps) = kv_re.captures(line) {
            if let Some(w) = cur.as_mut() {
                match &caps["k"] {
                    "monitorID" => w.monitor_id = caps["v"].trim().parse().unwrap_or_default(),
                    "windows" => w.windows = caps["v"].trim().parse().unwrap_or_default(),
                    "hasfullscreen" => {
                        w.has_fullscreen = caps["v"].trim().parse::<i64>().unwrap_or(0) != 0;
                    }
                    "lastwindow" => {
                        let v = caps["v"].trim();
                        if v != "0x0" && !v.is_empty() { w.last_window = Some(v.to_string()); }
                    }
                    "lastwindowtitle" => {
                        let v = caps["v"].trim();
                        if !v.is_empty() { w.last_window_title = Some(v.to_string()); }
                    }
                    "ispersistent" => {
                        w.is_persistent = caps["v"].trim().parse::<i64>().unwrap_or(0) != 0;
                    }
                    _ => {}
                }
            }
        }
    }
    if let Some(w) = cur.take() { res.push(w); }
    Ok(res)
}

/* ---- dispatch helpers ---- */

pub fn focus_address(addr: &str) -> Result<()> {
    run_batch_dispatch(&[&format!("dispatch focuswindow address:{addr}")])
}

/// Swap current column left/right by N steps (hyprscrolling).
pub fn swapcol_delta(delta: i32) -> Result<()> {
    if delta == 0 { return Ok(()); }
    let dir = if delta > 0 { "r" } else { "l" };
    let n = delta.unsigned_abs() as usize;
    let mut cmds: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        cmds.push(format!("dispatch layoutmsg swapcol {dir}"));
    }
    let refs: Vec<&str> = cmds.iter().map(std::string::String::as_str).collect();
    run_batch_dispatch(&refs)
}

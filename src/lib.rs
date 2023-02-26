use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, TimeZone, Utc};
use timeago::Formatter;

use git2::Repository;
use mlua::lua_State;
use nvim_utils::module;
use nvim_utils::prelude::*;

struct BlameInfo {
    name: Option<std::string::String>,
    email: Option<std::string::String>,
    timestamp: i64,
    message: std::string::String,
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() > n {
        s[..n].to_owned() + "…"
    } else {
        s.to_owned()
    }
}

fn was_user_committer<'a>(
    committer_name: Option<&'a str>,
    committer_email: Option<&'a str>,
) -> LuaResult<bool> {
    let config = git2::Config::open_default().map_err(|e| LuaError::ExternalError(Arc::new(e)))?;
    let name = config.get_string("user.name").ok();
    let email = config.get_string("user.email").ok();

    let is_user_name = committer_name == name.as_deref();
    let is_user_email = committer_email == email.as_deref();
    Ok(is_user_email && is_user_name)
}

fn format_timestamp<'a>(timestamp: i64) -> String {
    let timestamp: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    let now: DateTime<Utc> = Utc::now();

    let mut fmt = Formatter::new();
    fmt.num_items(1)
        .min_unit(timeago::TimeUnit::Minutes)
        .too_low("Just now");
    fmt.convert_chrono(timestamp, now)
}

fn get_blame_info<'a>(cwd: &'a Path, file: &'a Path, line: usize) -> LuaResult<BlameInfo> {
    let repo = Repository::discover(cwd).map_err(|e| LuaError::ExternalError(Arc::new(e)))?;
    let blame: git2::Blame = repo
        .blame_file(file, None)
        .map_err(|e| LuaError::ExternalError(Arc::new(e)))?;

    let hunk = blame
        .get_line(line)
        .ok_or_else(|| LuaError::RuntimeError("Could not get blame".to_owned()))?;
    let committer = hunk.final_signature();
    let name = committer.name().map(|s| s.to_owned());
    let email = committer.email().map(|s| s.to_owned());

    let oid = hunk.final_commit_id();
    let commit = repo
        .find_commit(oid)
        .map_err(|e| LuaError::ExternalError(Arc::new(e)))?;

    // Format the blame
    let timestamp = commit.time().seconds();
    let message = commit.summary().map(|s| s.to_owned()).unwrap_or_default();
    Ok(BlameInfo {
        name,
        email,
        timestamp,
        message,
    })
}

fn get_line_blame<'a>(lua: &'a Lua, (file, line): (String, usize)) -> LuaResult<String> {
    let file = Path::new(&file);
    let Some(cwd) = vim::func::getcwd(lua, None, None).ok() else {
        // Silently output an empty string on error
        return Ok(String::new())
    };

    // Get the blame hunk committer and commit
    let Some(info) = get_blame_info(cwd.as_path(), file, line).ok() else {
        // Silently output an empty string on error
        return Ok(String::new())
    };
    let timestamp = format_timestamp(info.timestamp);
    let message = truncate(&info.message, 30);

    if let Some(true) = was_user_committer(info.name.as_deref(), info.email.as_deref()).ok() {
        Ok(format!("You, {} • {}", timestamp, message))
    } else {
        Ok(format!(
            "{}, {} • {}",
            info.name.as_deref().unwrap_or("Unknown"),
            timestamp,
            message
        ))
    }
}

#[module(blam::core)]
fn core<'a>(lua: &'static Lua) -> LuaResult<LuaTable<'a>> {
    ModuleBuilder::new(lua)
        .with_fn("get_line_blame", get_line_blame)?
        .build()
}

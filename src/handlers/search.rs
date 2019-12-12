extern crate unicode_width;

use super::super::app::{App, ActiveBlock, RouteId};
use termion::event::Key;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

// Handle events when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('u') => {
            app.input = String::new();
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            app.input_cursor_position = UnicodeWidthStr::width(app.input.as_str()) as u16;
        }
        Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.chars().nth(app.input_idx - 1).unwrap();
                app.input_idx -= 1;
                let width: u16 = UnicodeWidthChar::width(last_c).unwrap() as u16;
                app.input_cursor_position -= width;
            }
        }
        Key::Right => {
            if app.input_cursor_position < app.input.len() as u16 {
                let next_c = app.input.chars().nth(app.input_idx).unwrap();
                app.input_idx += 1;
                let width: u16 = UnicodeWidthChar::width(next_c).unwrap() as u16;
                app.input_cursor_position += width;
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Search));
        }
        Key::Backspace => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let (remove_idx, last_c) = app.input.char_indices().nth(app.input_idx - 1).unwrap();
                app.input_idx -= 1;
                app.input.remove(remove_idx);
                let width: u16 = UnicodeWidthChar::width(last_c).unwrap() as u16;
                app.input_cursor_position -= width;
            }
        }
        Key::Delete => {
            if !app.input.is_empty() && app.input_idx < app.input.chars().count() {
                let (remove_idx, _last_c) = app.input.char_indices().nth(app.input_idx).unwrap();
                app.input.remove(remove_idx);
            }
        }
        Key::Char('\n') => {
            let limit = (app.block_height - 5) as i32;
            // no input no search
            if app.input.len() > 0 {
                // search tracks
                match app.cloud_music.as_ref().unwrap().search_track(
                    &app.input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.tracks = Some(result.songs.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_playlist(
                    &app.input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.playlists = Some(result.playlists.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_artist(
                    &app.input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.artists = Some(result.artists.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_album(
                    &app.input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.albums = Some(result.albums.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                app.selected_playlist_index = None;
                app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResult);
            }
        }
        // search input
        Key::Char(c) => {
            let (insert_idx, _) = app
                .input
                .char_indices()
                .nth(app.input_idx)
                .unwrap_or((app.input.len(), ' '));
            app.input.insert(insert_idx, c);
            app.input_idx += 1;
            let width: u16 = UnicodeWidthChar::width(c).unwrap() as u16;
            app.input_cursor_position += width;
        }
        _ => {}
    }
}

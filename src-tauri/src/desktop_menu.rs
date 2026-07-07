//! Desktop app menus with Codec2 called out in the native About dialog.

use ratspeak_tauri::codec2_build;
use tauri::menu::{
    AboutMetadata, Menu, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID, WINDOW_SUBMENU_ID,
};
use tauri::{AppHandle, Runtime};

fn about_metadata<R: Runtime>(app_handle: &AppHandle<R>) -> AboutMetadata<'_> {
    let pkg_info = app_handle.package_info();
    let config = app_handle.config();
    AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(codec2_build::about_version_line(&pkg_info.version.to_string())),
        comments: Some(codec2_build::BUILD_LABEL.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    }
}

pub fn default_with_codec2_about<R: Runtime>(app_handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let pkg_info = app_handle.package_info();

    let window_menu = Submenu::with_id_and_items(
        app_handle,
        WINDOW_SUBMENU_ID,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app_handle, None)?,
            &PredefinedMenuItem::maximize(app_handle, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::close_window(app_handle, None)?,
        ],
    )?;

    #[cfg(target_os = "macos")]
    let app_menu = {
        let about_metadata = about_metadata(app_handle);
        Submenu::with_items(
            app_handle,
            pkg_info.name.clone(),
            true,
            &[
                &PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?,
                &PredefinedMenuItem::separator(app_handle)?,
                &PredefinedMenuItem::services(app_handle, None)?,
                &PredefinedMenuItem::separator(app_handle)?,
                &PredefinedMenuItem::hide(app_handle, None)?,
                &PredefinedMenuItem::hide_others(app_handle, None)?,
                &PredefinedMenuItem::separator(app_handle)?,
                &PredefinedMenuItem::quit(app_handle, None)?,
            ],
        )?
    };

    #[cfg(not(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    let file_menu = Submenu::with_items(
        app_handle,
        "File",
        true,
        &[
            &PredefinedMenuItem::close_window(app_handle, None)?,
            #[cfg(not(target_os = "macos"))]
            &PredefinedMenuItem::quit(app_handle, None)?,
        ],
    )?;

    let edit_menu = Submenu::with_items(
        app_handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app_handle, None)?,
            &PredefinedMenuItem::redo(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::cut(app_handle, None)?,
            &PredefinedMenuItem::copy(app_handle, None)?,
            &PredefinedMenuItem::paste(app_handle, None)?,
            &PredefinedMenuItem::select_all(app_handle, None)?,
        ],
    )?;

    #[cfg(target_os = "macos")]
    let view_menu = Submenu::with_items(
        app_handle,
        "View",
        true,
        &[&PredefinedMenuItem::fullscreen(app_handle, None)?],
    )?;

    #[cfg(target_os = "macos")]
    let help_menu = Submenu::with_id_and_items(app_handle, HELP_SUBMENU_ID, "Help", true, &[])?;

    #[cfg(not(target_os = "macos"))]
    let help_menu = {
        let about_metadata = about_metadata(app_handle);
        Submenu::with_id_and_items(
            app_handle,
            HELP_SUBMENU_ID,
            "Help",
            true,
            &[&PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?],
        )?
    };

    #[cfg(target_os = "macos")]
    return Menu::with_items(
        app_handle,
        &[
            &app_menu,
            &file_menu,
            &edit_menu,
            &view_menu,
            &window_menu,
            &help_menu,
        ],
    );

    #[cfg(all(not(target_os = "macos"), not(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))))]
    return Menu::with_items(
        app_handle,
        &[&file_menu, &edit_menu, &window_menu, &help_menu],
    );

    #[cfg(all(not(target_os = "macos"), any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    Menu::with_items(app_handle, &[&edit_menu, &window_menu, &help_menu])
}

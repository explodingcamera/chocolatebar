#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use gdkx11::glib::Cast;
use gtk::{traits::WidgetExt, Inhibit};
use tauri::{App, LogicalPosition};

use xcb::x;
use xcb_wm::ewmh;

enum Position {
    Top,
    Bottom,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window =
        tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
            .always_on_top(true)
            .resizable(false)
            .decorations(false)
            .transparent(false)
            .skip_taskbar(true)
            .inner_size(500.0, 40.0)
            .build()?;

    window.set_position(tauri::Position::Logical(LogicalPosition::new(0.0, 0.0)))?;

    let w = window.gtk_window()?;

    // w.connect_event(|w, b| {
    //     // println!("event {b:?}");
    //     Inhibit(false)
    // });

    w.connect_configure_event(|w, b| {
        println!("event {b:?}");
        println!("event {:?}", b.event_type());
        true
    });

    // w.connect_button_press_event(|w, b| {
    //     println!("button pressed");
    //     Inhibit(true)
    // });

    // why did I have to search for hours to find this? :(
    let x11window = w.window().unwrap().downcast::<gdkx11::X11Window>().unwrap();
    let window_id = x11window.xid();

    let id: xcb::x::Window = unsafe { xcb::XidNew::new(window_id as u32) };

    println!("{window_id:?}");
    println!("{id:?}");

    let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    let conn = ewmh::Connection::connect(&xcb_con);

    // let monitor_rect = w.display().monitor(0).unwrap().geometry();
    let height = 400;
    let position = Position::Top;

    let (top, bottom) = match position {
        Position::Top => (0, height),
        Position::Bottom => (height, 0),
    };

    let strut_list: Vec<u8> = vec![
        0u32,   // left
        0,      // right
        top,    // top
        bottom, // bottom
        0,      // left_start_y
        0,      // left_end_y
        0,      // right_start_y
        0,      // right_end_y
        10,     // top_start_x
        200,    // top_end_x
        0,      // bottom_start_x
        0,      // bottom_end_x
    ]
    .iter()
    .flat_map(|x| x.to_le_bytes().to_vec())
    .collect();

    let _ = xcb_con.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: id,
        property: conn.atoms._NET_WM_WINDOW_TYPE,
        r#type: x::ATOM_ATOM,
        data: &[conn.atoms._NET_WM_WINDOW_TYPE_DOCK],
    });

    let _ = xcb_con.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: id,
        property: conn.atoms._NET_WM_STRUT_PARTIAL,
        r#type: x::ATOM_CARDINAL,
        data: &strut_list,
    });

    let _ = xcb_con.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: id,
        property: conn.atoms._NET_WM_STRUT,
        r#type: x::ATOM_CARDINAL,
        data: &strut_list[0..16],
    });

    // conn.send_request(&conn.atoms._NET_WM_STRUT_PARTIAL);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

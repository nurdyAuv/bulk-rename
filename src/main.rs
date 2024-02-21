#![windows_subsystem = "windows"]

use eframe::egui;
use egui_extras;
use std::fs;
use std::io;
use std::path::Path;
use faccess;
use faccess::AccessMode;
use faccess::PathExt;

fn get_folder(path: String, ignore_hidden: bool) -> io::Result<Folder> {
    //Get Folder, catagorize files and folders and make lists, return Folder
    let dirres = fs::read_dir(&path)?;
    let iter = dirres.into_iter().enumerate();
    let mut list_of_files: Vec<FolderItem> = vec![];
    let mut list_of_dirs: Vec<FolderItem> = vec![];
    let mut list_of_all: Vec<FolderItem> = vec![];
    for (_index, res) in iter {
        let item: fs::DirEntry = res?;
        let metadata = item.metadata()?;
        let name = item.file_name().to_str().unwrap().to_string();
        //println!("{:?} - {:?}", metadata.permissions().readonly(), name);
        let fpath_name = format!("{}{}", &path, &name);
        let can_read: bool;
        let fpath = Path::new(&fpath_name);
        if fpath.access(AccessMode::READ).is_ok() {
            can_read = true;
        } else { can_read = false };
        if metadata.is_dir() {
            if ignore_hidden {
                if !name.starts_with(".") {
                    let i = FolderItem { 
                        is_dir: true, 
                        name: name, 
                        read_into: can_read,
                        read_only: metadata.permissions().readonly(),
                        full_path: fpath_name 
                    };
                    list_of_dirs.push(i.clone());
                    list_of_all.push(i);
                } 
            } else {
                let i = FolderItem { 
                    is_dir: true, 
                    name: name,
                    read_into: can_read,
                    read_only: metadata.permissions().readonly(),
                    full_path: fpath_name  
                };
                list_of_dirs.push(i.clone());
                list_of_all.push(i);
            }
        } else {
            if ignore_hidden {
                if !name.starts_with(".") {
                    let i = FolderItem { 
                        is_dir: false, 
                        name: name,
                        read_into: can_read,
                        read_only: metadata.permissions().readonly(),
                        full_path: fpath_name 
                    };
                    list_of_files.push(i.clone());
                    list_of_all.push(i)
                };
            } else {
                let i = FolderItem { 
                    is_dir: false, 
                    name: name, 
                    read_into: can_read,
                    read_only: metadata.permissions().readonly(),
                    full_path: fpath_name  
                };
                list_of_files.push(i.clone());
                list_of_all.push(i)
            }
        }
    }
    list_of_all.sort_by_key(|a| a.name.to_owned().to_lowercase());
    list_of_dirs.sort_by_key(|a| a.name.to_owned().to_lowercase());
    list_of_files.sort_by_key(|a| a.name.to_owned().to_lowercase());

    return Ok(Folder { 
        path_full: path, 
        list_files: list_of_files, 
        list_folders: list_of_dirs, 
        list_all_items: list_of_all 
    });
}

fn rename_file(original_path: String, renamed_path: String) -> Result<(), io::Error> {
    fs::rename(original_path, renamed_path)
}

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::Vec2 { x: 1280.0, y: 800.0 }),
        min_window_size: Some(egui::Vec2 { x: 1280.0, y: 800.0 }),
        //icon_data: Some(),  <- Load Icon Data Somehow!!
        ..Default::default()
    };
    eframe::run_native(
        "Bulk Rename Utility",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<BulkGui>::default()
        }),
    )
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
struct Folder {
    path_full: String,
    list_files: Vec<FolderItem>,
    list_folders: Vec<FolderItem>,
    list_all_items: Vec<FolderItem>
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
struct FolderItem {
    is_dir: bool,
    name: String,
    read_into: bool,
    read_only: bool,
    full_path: String
}

struct EdittedItem {
    name_original: String,
    name_edited: String,
    path_original: String,
    path_edited: String
}

struct Edit {
    tag: String,
    items: Vec<EdittedItem>,
    edits: u32,
}

impl Default for FolderItem{
    fn default() -> Self {
        Self { 
            is_dir: false,
            name: "Default".to_string(),
            read_into: false,
            read_only: true,
            full_path: "".to_string()
        }
    }
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            path_full: "".to_string(),
            list_files: vec![],
            list_folders: vec![],
            list_all_items: vec![]
        }
    }
}

#[allow(dead_code)]
struct BulkGui {
    os_string: &'static str,
    version: &'static str,
    program_name: &'static str,
    first_cycle: bool,
    update_all: bool,
    show_all_items: bool,

    edits: Vec<Edit>,
    errors: Vec<String>,

    dialog_open_save: bool,
    dialog_save_enabled: bool,
    dialog_open_saving: bool,
    dialog_saving_enabled: bool,
    dialog_open_preview: bool,
    dialog_preview_enabled: bool,
    dialog_open_error: bool,
    dialog_error_enabled: bool,

    files_selected: Vec<String>,
    modifications_total: u16,
    window_min_size: Vec<f32>,
    window_size: egui::Vec2,
    default_path: String,

    section_browser_enabled: bool,
    section_files_enabled: bool,
    section_options_enabled: bool,

    section_files_hovered: bool,

    browser_folder: Folder,
    browser_list_folders: Vec<String>,
    browser_path_current: String,
    browser_path_last: String,
    browser_path_line: String,

    table_files: Vec<String>,
    table_files_selected: Vec<bool>,
    table_files_selected_vec: Vec<usize>,
    table_files_renamed: Vec<String>,
    table_files_item_clicked: bool,
    table_files_last_selected: u32,
    table_files_selected_total: u32,
    
    section_browser_size_x: f32,
    section_browser_percentage_min: u8,
    section_browser_percentage_max: u8,
    section_browser_percentage_current: u8,
    section_browser_size_y: f32,

    section_files_size_x: f32,
    section_files_percentage_min: u8,
    section_files_percentage_max: u8,
    section_files_percentage_current: u8,
    section_files_size_y: f32,

    section_options_size_x: f32,
    section_options_percentage_min: u8,
    section_options_percentage_current: u8,
    section_options_size_y: f32,

    section_options_name_enabled: bool,
    section_options_name_textbox_enabled: bool,
    section_options_combobox_name: String,
    section_options_combobox_value: NamingMode,
    section_options_combobox_default_value: NamingMode,
    section_options_name_value: String,

    section_options_case_enabled: bool,
    section_options_case_widgets_enabled: bool,
    section_options_case_combobox_name: String,
    section_options_case_combobox_value: CaseMode,
    section_options_case_combobox_default_value: CaseMode,
    section_options_case_combobox_label: String,
    section_options_case_from: u32,
    section_options_case_to: u32,

    section_options_replace_enabled: bool,
    section_options_replace_match_with: String,
    section_options_replace_replace_with: String,
    section_options_replace_match_first: bool,

    section_options_remove_enabled: bool,
    section_options_remove_first: u32,
    section_options_remove_last: u32,
    section_options_remove_to: u32,
    section_options_remove_from: u32,

    section_options_add_enabled: bool,
    section_options_add_prefix: String,
    section_options_add_insert: String,
    section_options_add_suffix: String,
    section_options_add_at: u32,

    section_options_numbering_enabled: bool,
    section_options_numbering_widgets_enabled: bool,
    section_options_numbering_insert_enabled: bool,
    section_options_numbering_combobox_name: String,
    section_options_numbering_combobox_value: NumberingMode,
    section_options_numbering_combobox_default_value: NumberingMode,
    section_options_numbering_combobox_label: String,
    section_options_numbering_at: u32,
    section_options_numbering_start: u32,
    section_options_numbering_pad: u32,

    button_defaults_enabled: bool,
    button_undo_enabled: bool,
    button_redo_enabled: bool,
    button_browser_up_enabled: bool,
    button_browser_directory_enabled: bool,

    checkbox_lock_section_resizing: bool,
    double_click_deselect_enabled: bool,

    saving_progress: f32,
    window_error_index: usize,
    window_error_text_box: String
}

enum NamingMode {
    Keep,
    Remove,
    Reverse
}

enum NumberingMode {
    None,
    Prefix,
    Suffix,
    PrefixAndSuffix,
    Insert
}

enum CaseMode {
    Same,
    Upper,
    Lower
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Default for BulkGui {
    fn default() -> Self {
        Self {
            os_string: "",
            version: "0.9.9.8",
            program_name: "Bulk Rename",
            first_cycle: true,
            update_all: true,
            show_all_items: false,

            errors: vec![],
            edits: vec![],

            dialog_open_save: false,
            dialog_save_enabled: true,
            dialog_open_saving: false,
            dialog_saving_enabled: true,
            dialog_open_preview: false,
            dialog_preview_enabled: true,
            dialog_open_error: false,
            dialog_error_enabled: true,

            files_selected: vec![],
            modifications_total: 0,
            window_min_size: vec![1366.0, 768.0],
            window_size: egui::vec2(0.0, 0.0),
            default_path: "".to_string(),

            section_browser_enabled: true,
            section_files_enabled: true,
            section_options_enabled: true,

            section_files_hovered: false,

            browser_folder: Folder::default(),
            browser_list_folders: vec![],
            browser_path_current: "".to_string(),
            browser_path_last: "".to_string(),
            browser_path_line: "".to_string(),

            table_files: vec![],
            table_files_selected: vec![],
            table_files_selected_vec: vec![],
            table_files_renamed: vec![],
            table_files_item_clicked: false,
            table_files_last_selected: 0,
            table_files_selected_total: 0,

            section_browser_size_x: 0.00,
            section_browser_percentage_min: 15,
            section_browser_percentage_max: 50,
            section_browser_percentage_current: 15,
            section_browser_size_y: 0.00,

            section_files_size_x: 0.00,
            section_files_percentage_min: 15,
            section_files_percentage_max: 50,
            section_files_percentage_current: 50,
            section_files_size_y: 0.00,

            section_options_size_x: 0.00,
            section_options_percentage_min: 35,
            section_options_percentage_current: 35,
            section_options_size_y: 0.00,

            section_options_name_enabled: false,
            section_options_name_textbox_enabled: false,
            section_options_combobox_name: "Keep".to_string(),
            section_options_combobox_value: NamingMode::Keep,
            section_options_combobox_default_value: NamingMode::Keep,
            section_options_name_value: "".to_string(),

            section_options_case_enabled: false,
            section_options_case_widgets_enabled: false,
            section_options_case_combobox_name: "Same".to_string(),
            section_options_case_combobox_value: CaseMode::Same,
            section_options_case_combobox_default_value: CaseMode::Same,
            section_options_case_combobox_label: "".to_string(),
            section_options_case_from: 0,
            section_options_case_to: 0,

            section_options_replace_enabled: false,
            section_options_replace_match_with: "".to_string(),
            section_options_replace_replace_with: "".to_string(),
            section_options_replace_match_first: true,

            section_options_remove_enabled: false,
            section_options_remove_first: 0,
            section_options_remove_last: 0,
            section_options_remove_to: 0,
            section_options_remove_from: 0,

            section_options_add_enabled: false,
            section_options_add_prefix: "".to_string(),
            section_options_add_insert: "".to_string(),
            section_options_add_suffix: "".to_string(),
            section_options_add_at: 0,

            section_options_numbering_enabled: false,
            section_options_numbering_widgets_enabled: false,
            section_options_numbering_insert_enabled: false,
            section_options_numbering_combobox_name: "None".to_string(),
            section_options_numbering_combobox_value: NumberingMode::None,
            section_options_numbering_combobox_default_value: NumberingMode::None,
            section_options_numbering_combobox_label: "".to_string(),
            section_options_numbering_at: 0,
            section_options_numbering_start: 1,
            section_options_numbering_pad: 0,

            button_defaults_enabled: false,
            button_undo_enabled: false,
            button_redo_enabled: true,
            button_browser_up_enabled: false,
            button_browser_directory_enabled: true,

            checkbox_lock_section_resizing: false,
            double_click_deselect_enabled: true,

            saving_progress: 0.0,
            window_error_index: 0,
            window_error_text_box: "".to_string()
        }
    }
}

impl BulkGui {
    fn window_save_open(&mut self) {
        self.dialog_open_save = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    fn window_save_close(&mut self) {
        self.dialog_open_save = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    fn window_saving_open(&mut self) {
        self.dialog_open_saving = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    fn window_saving_close(&mut self) {
        self.dialog_open_saving = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    fn window_preview_open(&mut self) {
        self.dialog_open_preview = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    fn window_preview_close(&mut self) {
        self.dialog_open_preview = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    fn window_error_open(&mut self) {
        self.dialog_open_error = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    fn window_error_close(&mut self) {
        self.dialog_open_error = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    fn save(&mut self) {
        self.window_saving_open();

        // Make Edit Struct and fill it.
        let mut edits = Edit {
            tag: "".to_string(),
            items: vec![],
            edits: 0
        };
        for (index, name_renamed) in self.table_files_renamed.iter().enumerate() {
            if self.table_files_selected[index]{
                let name_original = self.table_files[index].to_owned();
                let file_path_renamed = format!("{}{}", self.browser_path_current, name_renamed);
                let file_path_original = format!("{}{}", self.browser_path_current, name_original);
                let item = EdittedItem {
                    name_original: name_original.to_owned(),
                    name_edited: name_renamed.to_owned(),
                    path_original: file_path_original.to_owned(),
                    path_edited: file_path_renamed.to_owned()
                };
                edits.items.push(item);
                edits.edits += 1;
            }
        }
        let progress_slice: f32 = (1.0 / edits.edits as f32) * 100.0;
        // Commit Changes
        for item in &edits.items {
            match rename_file(item.path_original.to_owned(), item.path_edited.to_owned()) {
                Ok(_) => {
                    self.saving_progress += progress_slice;
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        self.window_saving_close();
        self.update_all = true;
        self.edits.push(edits);
    }

}

#[allow(dead_code)]
impl eframe::App for BulkGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Save Window
        if self.dialog_open_save {
            egui::Window::new("Save")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 105.0));
                ui.set_max_size(egui::Vec2::new(300.0, 105.0));
                ui.add_enabled_ui(self.dialog_save_enabled, |ui| {
                    ui.vertical( |ui| {
                        ui.add_space(7.0);
                        ui.group(|ui| {
                            ui.set_min_size(egui::Vec2::new(290.0, 90.0));
                            ui.set_max_size(egui::Vec2::new(290.0, 90.0));
                            ui.label(format!("You are about to write these changes to disk. \n\nYou have made {} changes to {} files \n\n\n Are you sure you want to save the changes?", self.modifications_total, self.table_files_selected_total));
                            ui.separator();
                            ui.horizontal( |ui| {
                                if ui.button("Cancel").clicked() {
                                    // Cancel
                                    self.window_save_close();
                                };
                                ui.separator();
                                ui.add_space(180.0);
                                ui.separator();
                                if ui.button("Save").clicked() {
                                    // Do some saving
                                    self.window_save_close();
                                    self.save();
                                };
                            })
                        });
                    });
                })
            });
        }
        
        // Saving Window
        if self.dialog_open_saving {
            egui::Window::new("Writing Changes...")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 40.0));
                ui.set_max_size(egui::Vec2::new(300.0, 40.0));
                ui.vertical(|ui| {
                    ui.separator();
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::ProgressBar::new(self.saving_progress / 100.0));
                            ui.separator();
                            ui.add(egui::Label::new((self.saving_progress as u32).to_string()));
                        })
                    })
                })
            });
        }

        // Preview Window
        if self.dialog_open_preview {
            egui::Window::new("Preview Changes")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                if ui.button("Close").clicked() {
                    self.window_preview_close();
                }
            });
        }

        // Error Window
        if self.dialog_open_error {
            egui::Window::new("Errors")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 135.0));
                ui.set_max_size(egui::Vec2::new(300.0, 135.0));
                ui.add_space(7.0);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(295.0, 100.0));
                    ui.set_max_size(egui::Vec2::new(295.0, 100.0));
                    egui::ScrollArea::vertical()
                    .id_source("error_scroll")
                    .show(ui, |ui| {
                        ui.set_width(295.0);
                        ui.wrap_text();
                        ui.label(self.errors[self.window_error_index].to_owned());
                    });
                });
                ui.separator();
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(110.0);
                    if ui.button("Close").clicked() {
                        self.window_error_close();
                    }
                    if ui.button("<").clicked() {
                        if self.window_error_index > 0 as usize {
                            self.window_error_index -= 1;
                        }
                    }
                    if ui.button(">").clicked() {
                        if self.window_error_index < self.errors.len() - 1 {
                            self.window_error_index += 1;
                        }
                    }
                    ui.add_space(100.0);
                })
            });
        }

        if self.first_cycle {
            if cfg!(windows) {
                self.os_string = "Windows";
                self.default_path = "C:/".to_string();
                self.browser_path_current = "C:/".to_string();
                self.browser_path_line = "C:/".to_string();
            }else if cfg!(unix) {
                self.os_string = "Unix";
                self.default_path = "/".to_string();
                self.browser_path_current = "/".to_string();
                self.browser_path_line = "/".to_string();
            };
            self.first_cycle = !self.first_cycle;
        }

        if self.update_all {
            self.table_files.clear();
            self.table_files_selected.clear();
            self.table_files_renamed.clear();
            self.browser_list_folders.clear();
            match get_folder(self.browser_path_current.to_owned(), true) {
                Ok(folder) => {
                    self.browser_folder = folder.clone();
                    for (_index, f) in folder.list_files.iter().enumerate() {
                        if f.read_into {
                            self.table_files.push(f.name.to_owned());
                            self.table_files_selected.push(false);
                            self.table_files_renamed.push(f.name.to_owned());
                        }
                    };
                    for (_index, f) in folder.list_folders.iter().enumerate() {
                        if self.show_all_items {
                            self.browser_list_folders.push(f.name.to_owned());
                        } else if f.read_into {
                            self.browser_list_folders.push(f.name.to_owned());
                        }
                    };
                },
                Err(error) => {
                    //Display Error
                    println!("{:?}", error);
                    self.errors.push(error.to_string().to_owned());
                    self.window_error_open();
                }
            }
            self.update_all = false;
            self.browser_path_line = self.browser_path_current.to_owned();
            if !self.first_cycle && ((self.browser_path_current.len() * 6) > (self.section_browser_size_x - 58.0) as usize) {
                //Deobfuscate Text

            }
        }

        // Main GUI Group Sizing
        self.section_browser_size_x = (frame.info().window_info.size.x * (self.section_browser_percentage_current as f32 / 100.0)).floor();
        self.section_browser_size_y = (frame.info().window_info.size.y - 95.0).floor();

        self.section_files_size_x = (frame.info().window_info.size.x * (self.section_files_percentage_current as f32 / 100.0)).floor();
        self.section_files_size_y = (frame.info().window_info.size.y - 95.0).floor();

        self.section_options_size_x = (frame.info().window_info.size.x * (self.section_options_percentage_current as f32 / 100.0) - 80.0).floor();
        self.section_options_size_y = (frame.info().window_info.size.y - 95.0).floor();

        // Instantiate Main Window 
        egui::CentralPanel::default().show(ctx, |ui| {
            //ctx.set_visuals(egui::style::Visuals::dark());

            //Set title bar name. (Technically Redundant, but for reference)
            frame.set_window_title(self.program_name);

            // Top Bar
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🗁 Open Folder").clicked() {
                        
                    };
                });
                ui.menu_button("Edit", |ui| {
                    /*
                        ///////////////////////////////////////////////////////
                        MAKE ENABLABLE
                        ///////////////////////////////////////////////////////
                     */
                    if ui.button("↪ Undo Last Action").clicked(){
                        
                    };
                    if ui.button("↩ Redo Last Action").clicked(){
                        
                    };
                });
                ui.menu_button("Options", |ui| {
                    if ui.checkbox(&mut self.checkbox_lock_section_resizing, "Lock Section Resizing").clicked() {
                        // Do something? 
                    }
                    if ui.checkbox(&mut self.double_click_deselect_enabled, "Double-Click Deselect").clicked() {
                        // Do something? 
                    }
                    if ui.checkbox(&mut self.show_all_items, "Show All Items").clicked() {
                        // Do something?
                        self.update_all = true;
                    }
                });
                ui.menu_button("Extras", |ui| {
                    ui.menu_button("Resize Panels", |ui| {
                        let browser_slider = ui.add_enabled(!self.checkbox_lock_section_resizing, egui::Slider::new(&mut self.section_browser_percentage_current, 
                            self.section_browser_percentage_min..=self.section_browser_percentage_max)
                            .logarithmic(true)
                            .text("Browser Width %")
                        );
                        if browser_slider.changed() {
                            //Resize this section.
                            let offset: f32 = 100.00 - (self.section_browser_percentage_current as f32 + self.section_options_percentage_current as f32);
                            self.section_files_percentage_current = offset as u8;
    
                        };
                    });
                    ui.menu_button("Theme", |ui| {
                        if ui.button("Dark").clicked() {
                            ctx.set_visuals(egui::style::Visuals::dark());
                        };
                        if ui.button("Light").clicked() {
                            ctx.set_visuals(egui::style::Visuals::light());
                        };
                    });
                });
                ui.menu_button("About", |ui| {
                    if ui.button("ℹ egui").clicked() {

                    }
                    if ui.button(format!("ℹ {}", self.program_name)).clicked() {

                    }
                });
                ui.label(format!("\t\tWidth: {}", frame.info().window_info.size.x.to_string()));
                ui.label(format!("Height: {}", frame.info().window_info.size.y.to_string()));
                let pointer = ctx.pointer_latest_pos();
                match pointer{
                    None => {
                        ui.label("\t\tPointer X: 0, Pointer Y: 0");
                    },
                    Some(pointer) => {
                        ui.label(format!("\t\tPointer X: {:?}, Pointer Y: {:?}", pointer.x.floor(), pointer.y.floor()));
                    }
                };
                //ui.label(format!("{}", self.section_files_hovered.to_string()));
            });
            
            ui.separator();

            // Main GUI Group
            ui.group(|ui| {
                ui.set_min_size(egui::vec2(frame.info().window_info.size.x - 28.0, frame.info().window_info.size.y - 86.0));
                ui.horizontal_top(|ui| {
                    //Browser Section
                    ui.add_enabled_ui(self.section_browser_enabled, |ui|{
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_browser_size_x, self.section_browser_size_y));
                            ui.vertical(|ui| {
                                ui.set_max_size(egui::vec2(self.section_browser_size_x - 5.0, self.section_browser_size_y - 5.0));

                                //Top Bar
                                ui.horizontal(|ui| {
                                    ui.set_width(self.section_browser_size_x - 5.0);
                                    if self.browser_path_current == self.default_path { self.button_browser_up_enabled = false }
                                    else { self.button_browser_up_enabled = true };
                                    let dir_up = ui.add_enabled(self.button_browser_up_enabled, egui::Button::new("⬆"));
                                    if dir_up.on_hover_text_at_pointer("Up in the Directory Stucture").clicked() {
                                        if cfg!(windows) {
                                            let mut sliced_path: Vec<String> = vec![];
                                            let path: String;
                                            for (_index, slice) in self.browser_path_current.split("/").enumerate() {
                                                if slice != "" {
                                                    sliced_path.push(slice.to_string());
                                                }
                                            }
                                            if sliced_path.len() > 0 {
                                                sliced_path.pop();
                                            }
                                            let joined: String = sliced_path.join("/");
                                            if joined.len() > 1 {
                                                path = format!("{}/", joined);
                                            } else { path = "C:/".to_string() }
                                            self.browser_path_last = self.browser_path_current.to_owned();
                                            self.browser_path_current = path.to_owned();
                                            self.update_all = true;
                                        } else if cfg!(unix) {
                                            let mut sliced_path: Vec<String> = vec![];
                                            let path: String;
                                            for (_index, slice) in self.browser_path_current.split("/").enumerate() {
                                                if slice != "" {
                                                    sliced_path.push(slice.to_string());
                                                }
                                            }
                                            if sliced_path.len() > 0 {
                                                sliced_path.pop();
                                            }
                                            let joined: String = sliced_path.join("/");
                                            if joined.len() > 0 {
                                                path = format!("/{}/", joined);
                                            } else { path = "/".to_string() }
                                            self.browser_path_last = self.browser_path_current.to_owned();
                                            self.browser_path_current = path.to_owned();
                                            self.update_all = true;
                                        }
                                    }
                                    let text_field = ui.add_enabled(true,  egui::TextEdit::singleline(&mut self.browser_path_line)
                                        .desired_width(self.section_browser_size_x - 60.0)
                                    );
                                    if text_field.lost_focus() && (self.browser_path_line != self.browser_path_current) {
                                        if !self.browser_path_line.ends_with("/") {
                                            self.browser_path_line += "/";
                                        }
                                        self.browser_path_last = self.browser_path_current.to_owned();
                                        self.browser_path_current = self.browser_path_line.to_owned();
                                        self.update_all = true;
                                        //println!("Changed AND Lost Focus");
                                    }
                                    text_field.on_hover_text_at_pointer(&self.browser_path_line);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        let dir_reset = ui.add_enabled(self.button_browser_directory_enabled, egui::Button::new("⟲"));
                                        if dir_reset.on_hover_text_at_pointer("Reset back to default.").clicked() {
                                            self.browser_path_current = self.default_path.to_owned();
                                            self.update_all = true;
                                        }
                                    });
                                });
                                ui.separator();
                                
                                //File Browser
                                egui::ScrollArea::vertical()
                                .id_source("1")
                                .show(ui, |ui| {
                                    ui.set_width(self.section_browser_size_x - 15.0);
                                    for folder in &self.browser_folder.list_folders {
                                        if folder.read_into {
                                            if ui.selectable_label(false, format!("🗀 {}", folder.name.to_owned())).clicked() {
                                                let path = format!("{}{}/", self.browser_path_current, folder.name.to_owned());
                                                self.browser_path_last = self.browser_path_current.to_owned();
                                                self.browser_path_current = path;
                                                self.update_all = true;
                                            };
                                        };
                                    };
                                });

                            });
                        });
                    });

                    //Files Section
                    ui.add_enabled_ui(self.section_files_enabled, |ui|{
                        let file_group = ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_files_size_x, self.section_files_size_y));
                            ui.vertical(|ui| {
                                ui.set_max_size(egui::vec2(self.section_files_size_x - 5.0, self.section_files_size_y - 5.0));
                                ui.horizontal(|ui| {
                                    ui.strong("Files");
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        let unselect = ui.add_enabled(true, egui::Button::new("⟲".to_string()));
                                        if unselect.clicked() {
                                            for (index, _item) in self.table_files.iter().enumerate() {
                                                self.table_files_selected[index] = false;
                                            };
                                        };
                                        unselect.on_hover_text_at_pointer("De-select all.")
                                    });
                                });
                                ui.separator();
                                egui_extras::TableBuilder::new(ui)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
                                .column(egui_extras::Column::auto().at_most(self.section_files_size_x - 5.0).clip(true))
                                .striped(true)
                                .body(|mut body| {
                                    for (index, item) in self.table_files.clone().iter().enumerate() {
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| {
                                                ui.set_width(self.section_files_size_x - 5.0);
                                                let label = ui.toggle_value(&mut self.table_files_selected[index], item);
                                                if label.clicked() {
                                                    self.table_files_item_clicked = true 
                                                } else { 
                                                    self.table_files_item_clicked = false 
                                                };
                                                ctx.input(|keys| {
                                                    if self.table_files_item_clicked && keys.modifiers.shift && !keys.modifiers.ctrl {
                                                        let last_selected = self.table_files_last_selected.clone() as usize;
                                                        if index > last_selected{
                                                            let select = index - last_selected;
                                                            for i in 0..=select {
                                                                self.table_files_selected[last_selected+i] = true;
                                                            };
                                                        }
                                                        else if index < last_selected {
                                                            for i in index..=last_selected {
                                                                self.table_files_selected[i] = true;
                                                            };
                                                        };
                                                    };
                                                });
                                                if self.table_files_item_clicked {
                                                    self.table_files_last_selected = index as u32;
                                                };
                                                ui.end_row();
                                            });
                                        });
                                        if self.table_files_selected[index] {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    if ui.label(format!("---> {}", self.table_files_renamed[index].to_string())).secondary_clicked() {
                                                        
                                                    }
                                                    ui.end_row();
                                                });
                                            });
                                        };
                                    };
                                });
                            });
                        });

                        //Hovering
                        if file_group.response.hovered() {
                            self.section_files_hovered = true;
                            ctx.input(|keys| {
                                //Ctrl+A to select all.
                                if keys.modifiers.ctrl && keys.key_pressed(egui::Key::A) {
                                    for (index, _item) in self.table_files.iter().enumerate() {
                                        self.table_files_selected[index] = true;
                                    };
                                };

                                //If double clicking in the panel. De-select all items (Reset view)
                                if self.double_click_deselect_enabled &&keys.pointer.button_double_clicked(egui::PointerButton::Primary) 
                                && (!keys.modifiers.ctrl 
                                && !keys.modifiers.shift) 
                                && !self.table_files_item_clicked {
                                    for (index, _item) in self.table_files.iter().enumerate() {
                                        self.table_files_selected[index] = false;
                                    };
                                };
                            });
                        } else { self.section_files_hovered = false };
                    });

                    //Options Section
                    ui.add_enabled_ui(self.section_options_enabled, |ui|{
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_options_size_x, self.section_options_size_y));
                            ui.set_max_size(egui::vec2(self.section_options_size_x, self.section_options_size_y));
                            ui.vertical(|ui| {
                                egui::ScrollArea::vertical()
                                .id_source("3")
                                .show(ui, |ui| {
                                    ui.set_max_size(egui::vec2(self.section_options_size_x - 5.0, self.section_options_size_y - 5.0));
                                    ui.horizontal(|ui| {
                                        ui.strong("Options");
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                            let reset = ui.add_enabled(true, egui::Button::new("⟲".to_string()));
                                            if reset.clicked() {
                                                
                                            };
                                            reset.on_hover_text_at_pointer("Reset back to defaults.")
                                        });
                                    });
                                    ui.separator();
    
                                    //Renaming Options
                                    ui.vertical(|ui| {
    
                                        //Name Options Priority #01
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 60.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 60.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Name");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            if ui.checkbox(&mut self.section_options_name_enabled, "").changed() {

                                                            };
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_name_enabled, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Mode");
                                                            egui::ComboBox::new("CB1", "")
                                                            .selected_text(self.section_options_combobox_name.to_owned())
                                                            .show_ui(ui, |ui| {
                                                                if ui.selectable_label(false, "Keep").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Keep;
                                                                    self.section_options_combobox_name = "Keep".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Remove").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Remove;
                                                                    self.section_options_combobox_name = "Remove".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Reverse").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Reverse;
                                                                    self.section_options_combobox_name = "Reverse".to_string();
                                                                };
            
                                                            });
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label("  ");
                                                            ui.add_enabled(self.section_options_name_textbox_enabled, 
                                                                egui::TextEdit::singleline(&mut self.section_options_name_value)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                    });
                                                });
                                            });
                                            match self.section_options_combobox_value {
                                                NamingMode::Keep => {self.section_options_name_textbox_enabled = false},
                                                NamingMode::Remove => {self.section_options_name_textbox_enabled = true},
                                                NamingMode::Reverse => {self.section_options_name_textbox_enabled = false}
                                            }
                                        });

                                        //Case Priority #02
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Case");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_case_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_case_enabled, |ui| {
                                                        //Mode
                                                        ui.horizontal(|ui| {
                                                            ui.label("Mode");
                                                            egui::ComboBox::new("2", "")
                                                            .selected_text(self.section_options_case_combobox_name.to_owned())
                                                            .show_ui(ui, |ui| {
                                                                if ui.selectable_label(false, "Same").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Same;
                                                                    self.section_options_case_combobox_name = "Same".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Upper").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Upper;
                                                                    self.section_options_case_combobox_name = "Upper".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Lower").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Lower;
                                                                    self.section_options_case_combobox_name = "Lower".to_string();
                                                                };
                                                            });
                                                            //Match and Disable Widgets
                                                            match self.section_options_case_combobox_value {
                                                                CaseMode::Same => {
                                                                    self.section_options_case_widgets_enabled = false;
                                                                },
                                                                _ => {
                                                                    self.section_options_case_widgets_enabled = true;
                                                                }
                                                            }
                                                        });

                                                        //Except
                                                        ui.horizontal(|ui| {
                                                            ui.add_enabled_ui(self.section_options_case_widgets_enabled, |ui| {
                                                                ui.label("Except from");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_case_from)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_case_from += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_case_from >= 1 {
                                                                        self.section_options_case_from -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_case_from >= 1 {
                                                                        self.section_options_case_from -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_case_from += 1;
                                                                };
    
                                                                ui.label("to");
    
                                                                let second_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_case_to)
                                                                    .clamp_range(self.section_options_case_from..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if second_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_case_to += 1;
                                                                } else if second_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_case_to >= 1 {
                                                                        self.section_options_case_to -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_case_to >= 1 {
                                                                        self.section_options_case_to -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_case_to += 1;
                                                                };
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Replace Options Priority #03
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Replace");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_replace_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
    
                                                    ui.add_enabled_ui(self.section_options_replace_enabled, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Replace ");
                                                            ui.add_enabled(true, 
                                                                egui::TextEdit::singleline(&mut self.section_options_replace_match_with)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label("With ");
                                                            ui.add_enabled(true, 
                                                                egui::TextEdit::singleline(&mut self.section_options_replace_replace_with)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                        ui.separator();
                                                        ui.horizontal(|ui| {
                                                            ui.checkbox(&mut self.section_options_replace_match_first, "First");
                                                        });
                                                    });
                                                });
                                            });
                                        });
    
                                        //Remove Options Priority #04
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Remove");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_remove_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_remove_enabled, |ui| {

                                                        // First / Last
                                                        ui.horizontal(|ui| {
    
                                                            //First
                                                            ui.group(|ui| {
                                                                ui.label("First");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_first)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_first += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_first >= 1 {
                                                                        self.section_options_remove_first -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_first >= 1 {
                                                                        self.section_options_remove_first -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_first += 1;
                                                                };
                                                            });
    
                                                                ui.separator();
    
                                                            //Last
                                                            ui.group(|ui| {
                                                                ui.label("Last");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_last)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_last += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_last >= 1 {
                                                                        self.section_options_remove_last -= 1;
                                                                    }
                                                                };
    
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_last >= 1 {
                                                                        self.section_options_remove_last -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_last += 1;
                                                                };
                                                            });
                                                        });

                                                        // To / From
                                                        ui.horizontal(|ui| {
    
                                                            //From
                                                            ui.group(|ui| {
                                                                ui.label("From ");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_from)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_from += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_from >= 1 {
                                                                        self.section_options_remove_from -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_from >= 1 {
                                                                        self.section_options_remove_from -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_from += 1;
                                                                };
                                                            });
    
    
                                                            ui.separator();
    
                                                            //To
                                                            ui.group(|ui| {
                                                                ui.label("To");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_to)
                                                                    .clamp_range(self.section_options_remove_from..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_to += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_to >= 1 {
                                                                        self.section_options_remove_to -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_to >= 1 {
                                                                        self.section_options_remove_to -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_to += 1;
                                                                };
                                                            });
                                                        });
                                                    })
                                                });
                                            });
    
                                        });
    
                                        //Add Options Priority #05
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 85.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 85.0));
                                                    ui.horizontal(|ui| {
                                                        ui.label("Add");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_add_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_add_enabled, |ui| {

                                                        //Prefix
                                                        ui.horizontal(|ui| {
                                                            ui.label("Prefix");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_prefix)
                                                            .desired_width(self.section_options_size_x - 20.0));
                                                        });

                                                        //Insert
                                                        ui.horizontal(|ui| {
                                                            ui.label("Insert");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_insert)
                                                            .desired_width(self.section_options_size_x - 200.0));
                                                            ui.label("at");
                                                            let first_value = ui.add_enabled(true, 
                                                                egui::DragValue::new(&mut self.section_options_add_at)
                                                                .clamp_range(0..=25565)
                                                                .speed(0.05)
                                                            );
                                                            if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                self.section_options_add_at += 1;
                                                            } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                if self.section_options_add_at >= 1 {
                                                                    self.section_options_add_at -= 1;
                                                                }
                                                            };
                                                            if ui.small_button("➖").clicked() {
                                                                if self.section_options_add_at >= 1 {
                                                                    self.section_options_add_at -= 1;
                                                                }
                                                            };

                                                            ui.separator();

                                                            if ui.small_button("➕").clicked() {
                                                                self.section_options_add_at += 1;
                                                            };

                                                        });

                                                        //Suffix
                                                        ui.horizontal(|ui| {
                                                            ui.label("Suffix");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_suffix)
                                                            .desired_width(self.section_options_size_x - 20.0));
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Numbering Options Priority #06
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 65.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 65.0));
                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Numbering");
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                                ui.checkbox(&mut self.section_options_numbering_enabled, "");
                                                            });
                                                        });
                                                        ui.separator();
                                                        ui.add_enabled_ui(self.section_options_numbering_enabled, |ui| {
                                                            //Mode and At
                                                            ui.horizontal(|ui| {
                                                                ui.label("Mode");
                                                                egui::ComboBox::new("6", "")
                                                                .selected_text(self.section_options_numbering_combobox_name.to_owned())
                                                                .show_ui(ui, |ui| {
                                                                    if ui.selectable_label(false, "None").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::None;
                                                                        self.section_options_numbering_combobox_name = "None".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Prefix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Prefix;
                                                                        self.section_options_numbering_combobox_name = "Prefix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Suffix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Suffix;
                                                                        self.section_options_numbering_combobox_name = "Suffix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Prefix&Suffix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::PrefixAndSuffix;
                                                                        self.section_options_numbering_combobox_name = "Pre+Suffix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Insert").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Insert;
                                                                        self.section_options_numbering_combobox_name = "Insert".to_string();
                                                                    };
                                                                });

                                                                //Match and Disable Widgets
                                                                match self.section_options_numbering_combobox_value {
                                                                    NumberingMode::None => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = false;
                                                                    },
                                                                    NumberingMode::Insert => {
                                                                        self.section_options_numbering_insert_enabled = true;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::Prefix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::Suffix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::PrefixAndSuffix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    }
                                                                }

                                                                //at
                                                                ui.add_enabled_ui(self.section_options_numbering_widgets_enabled, |ui| {
                                                                    ui.label("at");
                                                                    let first_value = ui.add_enabled(self.section_options_numbering_insert_enabled, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_at)
                                                                        .clamp_range(0..=25565)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_at += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_at >= 1 {
                                                                            self.section_options_numbering_at -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_at >= 1 {
                                                                            self.section_options_numbering_at -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_at += 1;
                                                                    };
                                                                });

                                                            });
                                                            
                                                            //Start and Pad
                                                            ui.horizontal(|ui| {
                                                                ui.add_enabled_ui(self.section_options_numbering_widgets_enabled, |ui| {
                                                                    ui.label("Start");
                                                                    let first_value = ui.add_enabled(true, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_start)
                                                                        .clamp_range(0..=999999)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_start += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_start >= 1 {
                                                                            self.section_options_numbering_start -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_start >= 1 {
                                                                            self.section_options_numbering_start -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_start += 1;
                                                                    };

                                                                    ui.label("Pad");
                                                                    let first_value = ui.add_enabled(true, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_pad)
                                                                        .clamp_range(0..=16)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_pad += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_pad >= 1 {
                                                                            self.section_options_numbering_pad -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_pad >= 1 {
                                                                            self.section_options_numbering_pad -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_pad += 1;
                                                                    };
                                                                });
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Save Box
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 70.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 70.0));
                                                    ui.vertical(|ui| {
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            let button_save = ui.add_sized(egui::vec2(64.0, 70.0), egui::Button::new("Save"));
                                                            if button_save.clicked() && self.dialog_save_enabled {
                                                                //Summon Dialog to confirm save
                                                                self.window_save_open();
                                                            };
                                                            ui.vertical(|ui| {
                                                                let button_preview = ui.button("Preview");
                                                                if button_preview.clicked() && self.dialog_preview_enabled {
                                                                    //Summon Dialog to preview changes
                                                                    self.window_preview_open();
                                                                }
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
            });

            ui.separator();

            // Bottom Bar
            ui.horizontal(|ui| {
                ui.label(format!("OS: {}, \tVersion: {}, \tSelected Files: {}, \tModifications: {}", 
                    self.os_string.to_string(), 
                    self.version, 
                    self.table_files_selected_vec.len().to_string(), 
                    self.modifications_total)
                );
            });

            let mut elements: Vec<usize> = vec![];
            //Create Vector of all selected items
            for index in 0..self.table_files_selected.len() {
                if self.table_files_selected[index] {
                    elements.push(index);
                };
            };

            // Update Vector if something is changed
            if elements != self.table_files_selected_vec {
                //println!("Cleared table_files buffer");

                self.table_files_selected_vec.clear();
                for ele in elements {
                    self.table_files_selected_vec.push(ele);
                };
            };
        });

        // Fill selected_names & Make edits
        let mut selected_names:Vec<(String, String)> = vec![];
        let mut selected_total: u32 = 0;
        for (index, filename) in self.table_files.iter().enumerate() {
            let extension = Path::new(&filename).extension().unwrap_or_default().to_str().unwrap_or_default();
            let file: &str;
            let ext: &str;
            if extension.len() >= 1 {
                let (f, e) = filename.split_at(filename.len() - extension.len() - 1);
                file = f; ext = e;
            } else { 
                let (f, e) = filename.split_at(filename.len() - extension.len());
                file = f; ext = e;
            };
            let mut file: String = file.to_string();
            let ext: String = ext.to_string();
            // Modify
            if self.table_files_selected[index] {
                selected_total += 1;

                // #01 Name
                if self.section_options_name_enabled {
                    match self.section_options_combobox_value {
                        NamingMode::Remove => {
                            let removed: String;
                            removed = self.section_options_name_value.to_owned();
                            file = removed;
                        },
                        NamingMode::Reverse => {
                            let mut reversedvec: Vec<(usize, String)> = vec![];
                            for (usize, char) in file.to_owned().chars().enumerate() {
                                reversedvec.push((usize, char.to_string()));
                            }
                            reversedvec.reverse();
                            
                            let mut reversed: String = "".to_string();
                            for (_usize, mut char) in reversedvec {
                                match char.as_str() {
                                    ">" => {char = "<".to_string()},
                                    "<" => {char = ">".to_string()},
                                    "(" => {char = ")".to_string()},
                                    ")" => {char = "(".to_string()},
                                    "{" => {char = "}".to_string()},
                                    "}" => {char = "{".to_string()},
                                    "[" => {char = "]".to_string()},
                                    "]" => {char = "[".to_string()},
                                    "/" => {char = "\\".to_string()},
                                    "\\" => {char = "/".to_string()},
                                    _ => {}
                                }
                                reversed = reversed + &char;
                            };
                            file = reversed;
                        }
                        NamingMode::Keep => {
                            // Don't do anything
                        }
                    }
                }
    
                // #02 Case
                if self.section_options_case_enabled {
                    let clamped_from = self.section_options_case_from.clamp(0, file.len() as u32);
                    let clamped_to = self.section_options_case_to.clamp(clamped_from, file.len() as u32);
                    match self.section_options_case_combobox_value {
                        CaseMode::Lower => {
                            if self.section_options_case_to >= 1 {
                                //Determine the kept section.
                                let keep: String;
                                if clamped_from == 0 && clamped_to == 0 {
                                    keep = "".to_string();
                                } else {
                                    keep = file[clamped_from as usize..=clamped_to as usize - 1].to_string();
                                };
                                if clamped_from == 0 {
                                    file = format!(
                                        "{}{}",
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_lowercase()
                                    )
                                } else {
                                    file = format!(
                                        "{}{}{}",
                                        file[0..=clamped_from as usize - 1].to_string().to_lowercase(),
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_lowercase()
                                    );
                                };
                            } else {
                                file = file.to_lowercase();
                            };
                        },
    
                        CaseMode::Upper => {
                            if self.section_options_case_to >= 1 {
                                //Determine the kept section.
                                let keep: String;
                                if clamped_from == 0 && clamped_to == 0 {
                                    keep = "".to_string();
                                } else {
                                    keep = file[clamped_from as usize..=clamped_to as usize - 1].to_string();
                                };
                                if clamped_from == 0 {
                                    file = format!(
                                        "{}{}",
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_uppercase()
                                    );
                                } else {
                                    file = format!(
                                        "{}{}{}",
                                        file[0..=clamped_from as usize - 1].to_string().to_uppercase(),
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_uppercase()
                                    );
                                };
                            } else {
                                file = file.to_uppercase();
                            };
                        },
    
                        CaseMode::Same => {
                            // Do nothing..
                        }
                    }
                }
    
                // #03 Replace
                if self.section_options_replace_enabled {
                    if self.section_options_replace_match_first == true {
                    let find = file.find(&self.section_options_replace_match_with);
                    match find {
                        Some(stringindex) => {
                            let (start, _mid) = file.split_at(stringindex);
                            let (_mid, end) = file.split_at(stringindex + self.section_options_replace_match_with.len());
                            file = format!("{}{}{}", start, self.section_options_replace_replace_with, end);
                        },
    
                        None => {
                            
                        }
                    }} else {
                        file = file.as_str().replace(self.section_options_replace_match_with.as_str(), self.section_options_replace_replace_with.as_str());
                    };
                }
    
                // #04 Remove
                if self.section_options_remove_enabled {
                    let clamped_first = self.section_options_remove_first.clamp(0, file.len() as u32);
                        
                    // Remove some chars from the beginning
                    if file.len() >= 1{
                        //file = file[clamped_first as usize..file.len()].to_string();
                        file = file.chars().skip(clamped_first as usize).collect();

                    }
                    
                    // Remove some chars from the end
                    let clamped_last = self.section_options_remove_last.clamp(0, file.len() as u32);
                    if file.len() >= 1 {
                        //file = file[0..file.len() - clamped_last as usize].to_string();
                        file = file.chars().take(file.char_indices().count() - clamped_last as usize).collect();
                    }
    
                    // Remove a section
                    let clamped_from = self.section_options_remove_from.clamp(0, file.len() as u32);
                    let clamped_to = self.section_options_remove_to.clamp(clamped_from, file.len() as u32);
                    if clamped_to >= 1 && file.len() >= 1{
                        let mut file_vec: Vec<char> = vec![];
                        let mut file_chars: String = "".to_string();
                        for character in file.chars() {
                            file_vec.push(character);
                        }
                        for character in &file_vec[0..clamped_from as usize] {
                            file_chars.push(character.to_owned());
                        }
                        for character in &file_vec[clamped_to as usize..file_vec.len()] {
                            file_chars.push(character.to_owned());
                        }
                        file = file_chars;
                        
                    }
                }
    
                // #05 Add
                if self.section_options_add_enabled {
                    if self.section_options_add_prefix.len() >= 1 {
                        file = format!("{}{}", self.section_options_add_prefix, file);
                    }
                    if self.section_options_add_insert.len() >= 1 {
                        let add_at: u32 = self.section_options_add_at.clamp(0, file.len() as u32);
                        let (start, end) = file.split_at(add_at as usize);
                        file = format!("{}{}{}", start, self.section_options_add_insert, end);
                    }
                    if self.section_options_add_suffix.len() >= 1 {
                        file = format!("{}{}", file, self.section_options_add_suffix);
                    }
                }
    
                // #06 Numbering
                if self.section_options_numbering_enabled {
                    //Padding
                    let number_index: u32 = self.section_options_numbering_start + selected_total - 1;
                    let number_length: u32 =  number_index.to_string().len() as u32;
                    let mut number: String = number_index.to_string();
                    if number_length < self.section_options_numbering_pad {
                        let padding: u32 = self.section_options_numbering_pad - number_length;
                        for _ in 0..padding {
                            number.insert(0, '0');
                        }
                    }
    
                    //Match and insert
                    match self.section_options_numbering_combobox_value {
                        NumberingMode::Prefix => {
                            file = format!("{}{}", number, file);
                        },
                        NumberingMode::Suffix => {
                            file = format!("{}{}", file, number);
                        },
                        NumberingMode::Insert => {
                            let number_at: u32 = self.section_options_numbering_at.clamp(0, file.len() as u32);
                            let (start, end) = file.split_at(number_at as usize);
                            file = format!("{}{}{}", start, number, end);
                        },
                        NumberingMode::PrefixAndSuffix => {
                            file = format!("{}{}{}", number, file, number);
                        },
                        _ => {}
                    }
                }
                
            }
            selected_names.push((file, ext));
        };
        self.table_files_selected_total = selected_total;

        // End Sections, re add names to table if something is changed.
        if selected_names.len() >= 1 {
            self.table_files_renamed.clear();
            for (name, ext) in selected_names {
                self.table_files_renamed.push(format!("{}{}", name, ext));
            }
        };

        // Get modifications total
        {
            let mut total:u16 = 0;
            if self.section_options_name_enabled { total += 1 };
            if self.section_options_case_enabled { total += 1 };
            if self.section_options_replace_enabled { total += 1};
            if self.section_options_remove_enabled { total += 1 };
            if self.section_options_add_enabled { total += 1};
            if self.section_options_numbering_enabled { total += 1};
            self.modifications_total = total;
            if total == 0 {
                self.dialog_preview_enabled = false;
                self.dialog_save_enabled = false;
            } else {
                self.dialog_preview_enabled = true;
                self.dialog_save_enabled = true;
            };
        };
        
    }
}
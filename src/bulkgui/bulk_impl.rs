use crate::bulkgui::bulk_gui::*;
use crate::bulkgui::bulk_enum::*;

use std::fs;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use faccess;
use faccess::AccessMode;
use faccess::PathExt;
use base64ct::{Base64, Encoding};
use sha2::*;
use sha1::*;
use md5::*;


pub fn get_folder(path: String, ignore_hidden: bool) -> io::Result<Folder> {
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

pub fn rename_file(original_path: String, renamed_path: String) -> Result<(), io::Error> {
    fs::rename(original_path, renamed_path)
}

pub fn hash_file(path: String, hash_mode: HashType) -> String {
    match hash_mode {
        HashType::MD5 => {
            let file = fs::File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut hasher = Md5::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{:?}", Base64::encode_string(&hasher.finalize())).to_string();
        },
        HashType::Sha1 => {
            let file = fs::File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut hasher = Sha1::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{:?}", Base64::encode_string(&hasher.finalize())).to_string();
        },
        HashType::Sha256 => {
            let file = fs::File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut hasher = Sha256::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{:?}", Base64::encode_string(&hasher.finalize())).to_string();
        },
        HashType::Sha512 => {
            let file = fs::File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut hasher = Sha512::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{:?}", Base64::encode_string(&hasher.finalize())).to_string();
        }
    };
}

impl Default for BulkGui {
    fn default() -> Self {
        Self {
            os_string: "",
            version: "0.9.11.2",
            program_name: "Bulk Rename",
            first_cycle: true,
            update_all: true,
            show_all_items: false,

            errors: vec![],
            edits_redo: vec![],
            edits_undo: vec![],

            dialog_open_save: false,
            dialog_save_enabled: true,
            dialog_open_saving: false,
            dialog_saving_enabled: true,
            dialog_open_preview: false,
            dialog_preview_enabled: true,
            dialog_open_error: false,
            dialog_error_enabled: true,
            dialog_open_redo: false,
            dialog_redo_enabled: true,
            dialog_open_undo: false,
            dialog_undo_enabled: true,

            files_selected: vec![],
            modifications_total: 0,
            window_min_size: vec![1366.0, 768.0],
            window_size: egui::vec2(0.0, 0.0),
            default_path: "".to_string(),

            section_browser_enabled: true,
            section_files_enabled: true,
            section_options_enabled: true,
            section_options_save_enabled: true,
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
            button_undo_enabled: true,
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

impl BulkGui {
    pub fn window_save_open(&mut self) {
        self.dialog_open_save = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    pub fn window_save_close(&mut self) {
        self.dialog_open_save = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn window_saving_open(&mut self) {
        self.dialog_open_saving = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    pub fn window_saving_close(&mut self) {
        self.dialog_open_saving = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn window_preview_open(&mut self) {
        self.dialog_open_preview = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    pub fn window_preview_close(&mut self) {
        self.dialog_open_preview = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn window_error_open(&mut self) {
        self.dialog_open_error = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    pub fn window_error_close(&mut self) {
        self.dialog_open_error = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn window_undo_open(&mut self) {
        self.dialog_open_undo = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }

    pub fn window_undo_close(&mut self) {
        self.dialog_open_undo = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn window_redo_open(&mut self) {
        self.dialog_open_redo = true;
        self.section_browser_enabled = false;
        self.section_files_enabled = false;
        self.section_options_enabled = false;
    }
    
    pub fn window_redo_close(&mut self) {
        self.dialog_open_redo = false;
        self.section_browser_enabled = true;
        self.section_files_enabled = true;
        self.section_options_enabled = true;
    }

    pub fn save(&mut self) {
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
                    self.errors.push(error.to_string());
                    self.dialog_open_error = true;
                    println!("{}", error);
                }
            }
        }
        self.window_saving_close();
        self.update_all = true;
        self.edits_undo.clear();
        self.edits_redo.clear();
        self.edits_undo.push(edits);
    }

    pub fn undo(&mut self) {
        self.window_saving_open();

        // Make Edit Struct and fill it.
        let mut edits = Edit {
            tag: "".to_string(),
            items: vec![],
            edits: 0
        };
        for (_index, edditeditem) in self.edits_undo.pop().unwrap().items.iter().enumerate() {
            let item = EdittedItem {
                name_original: edditeditem.name_edited.to_owned(),
                name_edited: edditeditem.name_original.to_owned(),
                path_original: edditeditem.path_edited.to_owned(),
                path_edited: edditeditem.path_original.to_owned()
            };
            edits.items.push(item);
            edits.edits += 1;
        }
        let progress_slice: f32 = (1.0 / edits.edits as f32) * 100.0;
        // Commit Changes
        for item in &edits.items {
            match rename_file(item.path_original.to_owned(), item.path_edited.to_owned()) {
                Ok(_) => {
                    self.saving_progress += progress_slice;
                },
                Err(error) => {
                    self.errors.push(error.to_string());
                    self.dialog_open_error = true;
                    println!("{}", error);
                }
            }
        }
        self.window_saving_close();
        self.update_all = true;
        self.edits_redo.push(edits);
    }

    pub fn redo(&mut self) {
        self.window_saving_open();

        // Make Edit Struct and fill it.
        let mut edits = Edit {
            tag: "".to_string(),
            items: vec![],
            edits: 0
        };
        for (_index, edditeditem) in self.edits_redo.pop().unwrap().items.iter().enumerate() {
            let item = EdittedItem {
                name_original: edditeditem.name_edited.to_owned(),
                name_edited: edditeditem.name_original.to_owned(),
                path_original: edditeditem.path_edited.to_owned(),
                path_edited: edditeditem.path_original.to_owned()
            };
            edits.items.push(item);
            edits.edits += 1;
        }
        let progress_slice: f32 = (1.0 / edits.edits as f32) * 100.0;
        // Commit Changes
        for item in &edits.items {
            match rename_file(item.path_original.to_owned(), item.path_edited.to_owned()) {
                Ok(_) => {
                    self.saving_progress += progress_slice;
                },
                Err(error) => {
                    self.errors.push(error.to_string());
                    self.dialog_open_error = true;
                    println!("{}", error);
                }
            }
        }
        self.window_saving_close();
        self.update_all = true;
        self.edits_undo.push(edits);
    }
}

use std::collections::HashMap;
use std::fs;
use std::env;
extern crate dirs;
#[allow(bad_style)]

//added miscategorization error, and incorrect category name error

fn identify() -> HashMap<String, Vec<String>> { 
    //this is our fancy custom box, you can see what it looks like with stuff in it
    //by running hackme and looking just after 'this is what the identify function returns'
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    //this is a box where we put the download path name ("/home/user_name/Downloads/")
    //we do this instead of just typing it out so that it works on other people's (different user_name) computers
    let downloadPATH = dirs::download_dir().expect("no path found");
                                                                                                    //^^this is where we explain what went wrong when we get a Err

    //this is the box where we put the list of files that are at the download path
    let filesInDownloads = fs::read_dir(&downloadPATH).expect("failed to read contents of download directory");
                                                                                                                            //^^this is where we explain what went wrong when we get a Err

    for file in filesInDownloads {
        //fileBOX and fileNAME are boxes where we put the actual file and the name of the file only
        let fileBOX = file.expect("DirEntry returned nothing");
        let fileNAME: String = fileBOX.file_name()
                                //this converts the OSstr into a string slice
                                .into_string()
                                .expect("the file_name could not be converted to a string")
                                //this converts the string slice into an owned string
                                .to_owned().clone();

        //the following code is our matching net, if a fileNAME matches what we are looking for, we put it in the map

        //the first one is a twofer cos it matches tiff and tif
        //following the || contains pattern, you can have all the image filetypes
        //use the same map insert code, instead of the bad example after this using else if
        
        if fileNAME.contains(".tif") ||
            fileNAME.contains(".gif") ||
            fileNAME.contains(".jpeg") ||
            fileNAME.contains(".jpg") ||
            fileNAME.contains(".jif") ||
            fileNAME.contains(".jfif") ||
            fileNAME.contains(".jp2") ||
            fileNAME.contains(".jpx") ||
            fileNAME.contains(".j2k") ||
            fileNAME.contains(".j2c") ||
            fileNAME.contains(".fpx") ||
            fileNAME.contains(".pcd") ||
            fileNAME.contains(".png") ||
            fileNAME.contains(".pdf")
        {
            if !map.contains_key("Images") {
                            //this is the key,  this is the value
                map.insert("Images".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Images").expect("failed to get mutable reference to Image list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
       } else if
            fileNAME.contains(".tif") ||
            fileNAME.contains(".exe") ||
            fileNAME.contains(".dmg") ||
            fileNAME.contains(".AppImage") ||
            !fileNAME.contains(".") && !fileBOX.path().is_dir() ||
            fileNAME.contains(".deb") ||
            fileNAME.contains(".rpm") ||
            fileNAME.contains(".pkg") ||
            fileNAME.contains(".msi") 
        {
            if !map.contains_key("Executables") {
                            //this is the key,  this is the value
                map.insert("Executables".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Executables").expect("failed to get mutable reference to Executables list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
        } else if 
            fileNAME.contains(".asf") ||
            fileNAME.contains(".wma") ||
            fileNAME.contains(".wmv") ||
            fileNAME.contains(".mp4") ||
            fileNAME.contains(".m4a") ||
            fileNAME.contains(".m4v") ||
            fileNAME.contains(".f4v") ||
            fileNAME.contains(".f4a") ||
            fileNAME.contains(".m4b") ||
            fileNAME.contains(".mov") ||
            fileNAME.contains(".3gp") ||
            fileNAME.contains(".3g2") ||
            fileNAME.contains(".ogg") ||
            fileNAME.contains(".oga") ||
            fileNAME.contains(".ogv") ||
            fileNAME.contains(".ogx") ||
            fileNAME.contains(".webm") ||
            fileNAME.contains(".flv") ||
            fileNAME.contains(".avi") 
        {
            if !map.contains_key("Videos") {
                            //this is the key,  this is the value
                map.insert("Videos".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Videos").expect("failed to get mutable reference to Videos list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
        } else if 
            fileNAME.contains(".iso") ||
            fileNAME.contains(".bin") ||
            fileNAME.contains(".img") ||
            fileNAME.contains(".vdi") ||
            fileNAME.contains(".rom") 
        {
            if !map.contains_key("Roms") {
                            //this is the key,  this is the value
                map.insert("Roms".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Roms").expect("failed to get mutable reference to Roms list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
        } else if 
            fileNAME.contains(".7z") ||
            fileNAME.contains(".bz") ||
            fileNAME.contains(".rar") ||
            fileNAME.contains(".tar") ||
            fileNAME.contains(".zip") 
        {
            if !map.contains_key("Compressed") {
                            //this is the key,  this is the value
                map.insert("Compressed".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Compressed").expect("failed to get mutable reference to Compressed list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
         } else if 
            fileNAME.contains(".torrent") 
        {
            if !map.contains_key("Torrents") {
                            //this is the key,  this is the value
                map.insert("Torrents".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Torrents").expect("failed to get mutable reference to Torrents list");
                //this is the value
                fileLIST.push(fileNAME); 
            }
        } else {
            //this branch is hit when the file found in downloads doesnt match any of the types so far
            //when you use continue, it skips this file and moves on to the next one
            continue
        }
    }
    return map;
}

fn create_dirs(map: &HashMap<String, Vec<String>>){
    //this poops out a PathBuf
    let pathbox = dirs::download_dir().expect("failed to find download folder");
    //this converts that PathBuf to a string
    env::set_current_dir(&pathbox).expect("Failed to set current 'dir'");

    for key in map.keys(){
        fs::create_dir_all(key).expect("Failed to create folder");
    }

}

fn move_files(map: &HashMap<String, Vec<String>>){

//copy each file into its category folder
//delete each file from downloads
    println!("{:?}", map);
    /* this is our map
        {   "KEY"       :   ["VALUE", "VALUE"]
            "Executables": ["brgl.exe", "grgl.msi"], 
            "Torrents": ["brgl.torrent"],  
            "Videos": ["brgl.asf", "trgl.mp4"], 
            "Compressed": ["brgl.7z", "grgl.zip"], 
            "Images": ["jrgl.jif", "grgl.jpeg"], 
            "Roms": ["brgl.iso", "zrgl.vdi"]
        }
    */
    
    let pathbox = dirs::download_dir().expect("failed to find download folder");
    //this converts that PathBuf to a string
    env::set_current_dir(&pathbox).expect("Failed to set current 'dir'");

             //from(downloads)      //to(The Folder it belong in)
    
            //value         key/value

    //The algorithm will say:
        //✔️for each line in the map
        //✔️give me the key and the list of values on that line
        //✔️then for each value in the list of values
        //copy that value to the folder with the name of the key
        //next value please
        //next line please
    
    for (key, values) in map.iter(){
        for file in values {
            //        ↓this is the name of the file brgle.mp4
            fs::copy(file, format!("{}/{}", key, file)).expect("failed to copy file");
            //                      ↑Video/brgl.mp4
        }
    }
    for (key, values) in map.iter(){
        for file in values {
            //        ↓this is the name of the file brgle.mp4
            fs::remove_file(&file).expect("failed to delete file");
            //                      ↑Video/brgl.mp4
        } 
    }
}

fn main() {
    let map = identify();
    create_dirs(&map);
    move_files(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    //the following are the used more than once inside tests, they are helpers
    fn create_files(file_list: &Vec<&str>){
        for file in file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }
    }

    fn missed_all(file_list: &Vec<&str>) -> Result<(),()> {
        if identify().len() == 0 {
            for file in file_list{
                fs::remove_file(&file).expect("failed to remove file");
                let mut in_box = file.clone().to_owned();
                let out_box = match in_box.chars().position(|letter| letter == '.'){
                    Some(i) => in_box[i..].to_owned(),
                    None => format!("unix executable ({})", in_box).to_owned()
                };
                println!("the filetype missed was {}", &out_box);
            }
            return Err(())
        }
        return Ok(())
    }

    fn wrong_category(file_list: &Vec<&str>, category: &String) -> Result<(), ()> {
        if identify().keys().len() > 1 {
            for key in identify().keys() {
                if key.contains(&category[..]) {
                    continue
                } else {
                    println!("you miscategorized {} as {}\n", &category, &key);
                    for file in file_list{
                        fs::remove_file(&file).expect("failed to remove file without match");
                    }
                    return Err(())
                }
            }
        }
        return Ok(())
    }

    fn missed_what(file_list: &Vec<&str>, mut err_count: i64) -> i64 {
        for value in identify().values(){
            for file in file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let mut in_box = file.clone().to_owned();
                    let out_box = match in_box.chars().position(|letter| letter == '.'){
                        Some(i) => in_box[i..].to_owned(),
                        None => in_box[..].to_owned()
                    };
                    println!("the filetype missed was {}", &out_box);
                    err_count += 1;
                }
            }
        }
        err_count
    }

    fn category_name(category: &String, mut err_count: i64) -> i64 {
        for key in identify().keys(){
            if key.contains(&category[..]){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }
        err_count
    }

    //the following are tests run together as groups in the functions marked [test]
    //they all use the helper functions above over and over again to test the code
    fn identify_returns_map_of_image_files() -> Result<(), ()> {
        let file_list = vec!["brgl.tif",
                            "drgl.tiff",
                            "frgl.gif",
                            "grgl.jpeg",
                            "hrgl.jpg",
                            "jrgl.jif",
                            "krgl.jfif",
                            "lrgl.jp2",
                            "mrgl.jpx",
                            "nrgl.j2k",
                            "prgl.j2c",
                            "srgl.fpx",
                            "trgl.pcd",
                            "urgl.png",
                            "vrgl.pdf"];

        let category = "Images".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given image file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_executable_files() -> Result<(), ()> {
        let file_list = vec!["brgl.exe",
                            "drgl.dmg",
                            "frgl.AppImage",
                            "grgl",
                            "hrgl.deb",
                            "jrgl.rpm",
                            "krgl.pkg",
                            "lrgl.msi"];

        let category = "Executables".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given executable file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_video_files() -> Result<(), ()> {
        let file_list = vec!["brgl.asf",
                            "drgl.wma",
                            "frgl.wmv",
                            "grgl.mp4",
                            "hrgl.m4a",
                            "jrgl.m4v",
                            "krgl.f4v",
                            "lrgl.f4a",
                            "mrgl.m4b",
                            "nrgl.mov",
                            "prgl.3gp",
                            "srgl.3gp2",
                            "trgl.3g2",
                            "urgl.3gpp",
                            "vrgl.3gpp2",
                            "brgl.ogg",
                            "drgl.oga",
                            "frgl.ogv",
                            "grgl.ogx",
                            "hrgl.webm",
                            "jrgl.flv",
                            "krgl.avi"];

        let category = "Videos".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given video file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_rom_files() -> Result<(), ()> {
        let file_list = vec!["brgl.iso",
                            "drgl.bin",
                            "frgl.img",
                            "grgl.vdi",
                            "hrgl.rom"];

        let category = "Roms".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given rom file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_compressed_files() -> Result<(),()> {
        let file_list = vec!["brgl.7z",
                            "drgl.bz",
                            "frgl.rar",
                            "grgl.tar",
                            "hrgl.zip"];

        let category = "Compressed".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given compressed file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_torrent_files() -> Result<(), ()> {
        let file_list = vec!["brgl.torrent"];

        let category = "Torrents".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given torrent file types: \n{:?}\n", identify());

        let mut err_count = 0;

        match missed_all(&file_list) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        match wrong_category(&file_list, &category) {
            Ok(_) => (),
            Err(_) => return Err(())
        }

        err_count = missed_what(&file_list, err_count);

        err_count = category_name(&category, err_count);

        if err_count > 0 {
            return Err(())
        } else {
            return Ok(())
        }
    }

    fn identify_returns_map_of_multiple_types() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in file_map.values() {
            let created_file = fs::File::create(&file[0]).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given multiple file types: \n{:?}\n", identify());

        let identify_result = identify();

        if file_map == identify_result {
            for file in file_map.values(){
                fs::remove_file(&file[0]).expect("failed to remove file without match");
            }
            return Ok(())
        } else {
            for file in file_map.values(){
                fs::remove_file(&file[0]).expect("failed to remove file without match");
            }
            for key in file_map.keys(){
                let error_key = format!("the '{}' category was not set correctly", key);
                let mut key_list = vec![];
                match identify_result.get(key) {
                    Some(_) => (),
                    None => key_list.push(&error_key)
                }
                
                for result_key in key_list {
                    println!("{}", result_key);
                }
                
            }
            println!("");
            return Err(())
        }
    }

    fn identify_ignores_file_names_containing_extensions() -> Result<(),()> {
        let file_list = vec!["torrent.bad",
                            "zippy.bad",
                            "isolated.bad",
                            "movies.bad",
                            "debian.bad",
                            "jiffylube.bad"];

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to downloads");

        create_files(&file_list);

        println!("\nthis is what the identify function returns when given bad filenames: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                    println!("should not have identified {}", &file);
                    err_count += 1;
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                }
            }
        }
        if err_count > 0 {
            return Err(())
        } else {
            return Ok(())
        }
    }

    #[test]
    fn identify_returns_correct_maps(){
        identify_returns_map_of_image_files().expect("the output of identify did not match the image files in the downloads folder");
        identify_returns_map_of_executable_files().expect("the output of identify did not match the executable files in the downloads folder");
        identify_returns_map_of_video_files().expect("the output of identify did not match the video files in the downloads folder");
        identify_returns_map_of_rom_files().expect("the output of identify did not match the rom files in the downloads folder");
        identify_returns_map_of_compressed_files().expect("the output of identify did not match the compressed files in the downloads folder");
        identify_returns_map_of_torrent_files().expect("the output of identify did not match the compressed files in the downloads folder");

        identify_ignores_file_names_containing_extensions().expect("the identify function was fooled by file_names containing extensions");
        identify_returns_map_of_multiple_types().expect("the output of identify did not match the multiple types in the downloads folder");
    }

    #[test]
    fn creates_dirs_from_map_keys(){
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let mut missed_list: Vec<String> = vec![];

        let download_path = dirs::download_dir().expect("failed to unwrap path");

        create_dirs(&file_map);

        let mut ok_count = 0;
        for file_box in fs::read_dir(&download_path).expect("failed to read directory contents") {
            let file_box = file_box.expect("could not get a dir_name from the DirEntry");
            let dir_name = &file_box.file_name()
                                .into_string()
                                .expect("could not convert dir_name to string")
                                .to_owned();

            for key in file_map.keys() {
                if dir_name.contains(&key[..]) {
                    fs::remove_dir_all(&key).expect("failed to remove directory");
                    ok_count += 1;
                } else {
                    if !missed_list.contains(&key){
                        missed_list.push(key.to_owned());
                    }
                    continue;
                }
            }
        }
        if ok_count == file_map.keys().len() {
            assert!(true);
        } else {
            for key in missed_list{
                println!("the {} directory was not created by create_dirs()", &key);
            }
            panic!("only {} out of {} directories were successfully created", ok_count, file_map.keys().len());
        }
    }

    fn files_from_map_in_correct_dirs() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for directory in file_map.keys() {
            fs::create_dir_all(&directory).expect("failed to create folder");
        }

        for file in file_map.values() {
            let created_file = fs::File::create(&file[0]).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        move_files(&file_map);

        let mut ok_count = 0;
        'dir: for file_box in fs::read_dir(&download_path).expect("failed to get file list from directory") {
            let file_box = file_box.expect("failed to unwrap file_box");
            let entry_name = &file_box.file_name().into_string().expect("failed to convert dir file_box to string");
            for key in file_map.keys() {
                if key.contains(&entry_name[..]) {
                    env::set_current_dir(&file_box.path()).expect("failed to set map key dir");
                    'file: for file in fs::read_dir(&file_box.path()).expect("failed to get file list from directory") {
                        let file = file.expect("failed to unwrap file_box");
                        let file_name = &file.file_name().into_string().expect("failed to convert dir file_box to string");

                        if file_name.contains(&file_map[key][0][..]){
                            ok_count += 1;
                        } else {
                            continue 'dir
                        }
                    }
                }
            }
        }

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for directory in file_map.keys() {
            fs::remove_dir_all(&directory).expect("failed to remove folder");
        }

        if ok_count != 6 {
            for file in file_map.values() {
                match fs::remove_file(&file[0]){
                    Ok(_) => continue,
                    Err(_) => continue
                }
            }
        }

        if ok_count == 6 {
            return Ok(());
        } else {
            return Err(());
        }
    }

    fn files_moved_no_longer_in_downloads() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");

        let mut err_count = 0;
        for file in fs::read_dir(&download_path).expect("failed to read contents of downloads"){
            let file = file.expect("failed to unwrap dir file_box");
            let file_name = &file.file_name().into_string().expect("failed to convert dir file_box to string");
            for value in file_map.values(){
                if value.contains(file_name) {
                    println!("move_files() did not remove the {} file after copying", file_name);
                    fs::remove_file(file.path()).expect("failed to remove file");
                    err_count += 1;
                } else {
                    continue;
                }
            }
        }
        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    #[test]
    fn copies_files_and_cleans_up() {
        files_from_map_in_correct_dirs().expect("the files did not end up in the correct dirs after move");
        files_moved_no_longer_in_downloads().expect("the files were still in the download folder after move");
    }
}

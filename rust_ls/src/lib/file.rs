
pub mod files{
    use std::{fs, os::unix::fs::PermissionsExt};
    pub fn list_files(src: &str) -> Result<(),std::io::Error> {
        
        // get directory entry
        let path = match fs::read_dir(src) {
            Ok(dirs) => dirs,
            Err(e) => {
                return Err(e);
            }
        };

        println!("{:<12} {:<10} {:<10}","Permission","Size","Name");
        for dir in path {

            // if error just ignore, might be something wrong with that particular file
            // we move onto the next, do not want to panic here
            if dir.is_err(){
                println!("error listing directory {:?}",dir.err());
                continue;
            }
            let entry = dir?; // since error canse is already handeled above
            
            let path = entry.path();
            let output = prepare_print_stmt(entry);
            if let Ok(output) = output {
                println!("{}",output);
            } else {
                println!("error while trying to get info for {:?} {:?}",path,output.err());
            }
        }

        Ok(())
    }

    fn prepare_print_stmt(dir: fs::DirEntry) -> Result<String,std::io::Error> {
        let mut output  = String::new();
        let metadata = dir.metadata()?;

        if metadata.is_dir() {
            output = output + "d";
        } else if metadata.is_symlink() {
            output = output + "s";
        } else {
            output = output + "f";
        }

        //let creation_time = metadata.created()?;

        /*
        In Unix-based systems, file permissions are represented as a set of bits. Each permission (read, write, execute) is represented by a specific bit. These permissions are divided into three categories:

        User (owner): The person who owns the file.
        Group: Other users in the same group as the file's owner.
        Others: Everyone else.

        Each category has three permission bits:

        Read (r): Allowed to read the file.
        Write (w): Allowed to write (modify) the file.
        Execute (x): Allowed to execute the file (run it, if it's a program).

        rwxr-xr--  ->  111 101 100
        rwx, bits set means read permission is set
        0o040 =        100 000 000  -> user, group, world
        And operation. means the bit is set, means r is set

        */
        let mode = metadata.permissions().mode(); // get the user permission in octal  ex: 0o755 Binary representation  is 1 1110 1101
        let user_perm = format!(
            "{}{}{}",
            if mode & 0o400 != 0 { 'r' } else {'-'},
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' }
        );
        let group_perms = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' }
        );
        let other_perms = format!(
            "{}{}{}",
            if mode & 0o004 != 0 { 'r' } else { '-' },
            if mode & 0o002 != 0 { 'w' } else { '-' },
            if mode & 0o001 != 0 { 'x' } else { '-' }
        );

        let permission = format!("{}{}{}{}",output,user_perm,group_perms,other_perms);
        let binding = dir.path();
        let path = match binding.to_str() {
            Some(v) => v,
            None => "",
        };
        output  = format!("{:<12} {:<10} {:<10}",permission,metadata.len(),path);

        return Ok(output);
    }
}
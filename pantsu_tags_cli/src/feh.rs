use std::io;
use std::process::{Child, Command, Stdio};

pub struct FehProcesses {
    processes: Vec<Child>,
}

impl FehProcesses {
    pub fn kill(mut self) {
        for proc in &mut self.processes {
            let _ = proc.kill(); // don't care if kill fails
        }
    }
}

//feh --info 'echo "$((%u -1))"' https://img3.gelbooru.com/images/bb/62/bb626c2a621cbc1642256c0ebefbd219.jpg https://img3.gelbooru.com/images/12/ee/12ee1ac61779f5ccfcc383485c7c3191.png

pub fn feh_compare_image(image: &str, other_images: &Vec<&str>, image_label: &str, other_images_label: &str) -> FehProcesses {
    let command_image = format!("feh -q.ZB black --info \'echo \"{}\"\' \'{}\'", image_label, image);

    let mut command_other_images = format!("feh -q.ZB black --info \'echo \"$((%u -1)) {}\"\'", other_images_label);
    for &image in other_images {
        command_other_images.push_str(" \'");
        command_other_images.push_str(image);
        command_other_images.push('\'');
    }

    let mut cmds = FehProcesses { processes: Vec::new() };

    let cmd_image = match spawn_process(&command_image) {
        Ok(cmd) => cmd,
        Err(_) => return cmds,
    };
    cmds.processes.push(cmd_image);

    let cmd_other_images = match spawn_process(&command_other_images) {
        Ok(cmd) => cmd,
        Err(_) => {
            cmds.kill();
            return FehProcesses { processes: Vec::new()};
        }
    };
    cmds.processes.push(cmd_other_images);
    cmds
}

fn spawn_process(command: &str) -> io::Result<Child>{
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
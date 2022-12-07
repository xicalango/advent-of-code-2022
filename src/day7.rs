use std::collections::HashMap;
use std::str::FromStr;
use crate::Error;

#[derive(Debug)]
pub enum Tree {
    File(usize),
    Dir(Vec<DirEnt>),
}

impl Tree {

    pub fn du(&self) -> usize {
        match self {
            Tree::File(size) => *size,
            Tree::Dir(content) => content.iter().map(|e| e.du()).sum(),
        }
    }

    pub fn find_dir_ent_name(&self, name: &str) -> Option<&DirEnt> {
        match self {
            Tree::File(_) => None,
            Tree::Dir(content) => content.iter().find(|e| e.name == name)
        }
    }

    pub fn find_dir_ent_name_mut(&mut self, name: &str) -> Option<&mut DirEnt> {
        match self {
            Tree::File(_) => None,
            Tree::Dir(content) => content.iter_mut().find(|e| e.name == name)
        }
    }
}

#[derive(Debug)]
pub struct DirEnt {
    name: String,
    content: Tree,
}

impl DirEnt {

    pub fn dir(name: &str, content: Vec<DirEnt>) -> DirEnt {
        DirEnt {
            name: name.to_string(),
            content: Tree::Dir(content),
        }
    }

    pub fn empty_dir(name: &str) -> DirEnt {
        Self::dir(name, Vec::new())
    }

    pub fn file(name: &str, size: usize) -> DirEnt {
        DirEnt {
            name: name.to_string(),
            content: Tree::File(size),
        }
    }

    pub fn mkdir(&mut self, name: &str) -> &mut DirEnt {
        match &mut self.content {
            Tree::File(_) => panic!("not a dir"),
            Tree::Dir(content) => {
                let dir_ent = DirEnt::dir(name, Vec::new());
                content.push(dir_ent);
                content.last_mut().unwrap()
            }
        }
    }

    pub fn push_file(&mut self, name: &str, size: usize) {
        match &mut self.content {
            Tree::File(_) => panic!("not a dir"),
            Tree::Dir(content) => {
                let dir_ent = DirEnt::file(name, size);
                content.push(dir_ent);
            }
        }
    }

    pub fn du(&self) -> usize {
        self.content.du()
    }

    pub fn du_by_dir(&self) -> HashMap<String, usize> {
        self.du_by_dir_with_prefix("".to_string())
    }

    fn du_by_dir_with_prefix(&self, prefix: String) -> HashMap<String, usize> {
        let mut result = HashMap::new();

        match &self.content {
            Tree::File(_) => {
                //result.insert(self.name.clone(), *size);
            }
            Tree::Dir(dir) => {
                let dir_name = format!("{}{}", prefix, self.name);
                result.insert(dir_name.clone(), self.du());
                for ent in dir.iter() {
                    let by_ent = ent.du_by_dir_with_prefix(format!("{}/", dir_name));
                    result.extend(by_ent.into_iter())
                }
            }
        }

        result
    }

    pub fn resolve<T: AsRef<str>>(&self, bits: &[T]) -> Option<&DirEnt> {
        let mut cur = self;

        for bit in bits {
            let resolve = cur.content.find_dir_ent_name(bit.as_ref());
            if let None = resolve {
                return None;
            }
            cur = resolve.unwrap();
        }

        Some(cur)
    }

    pub fn resolve_mut<T: AsRef<str>>(&mut self, bits: &[T]) -> Option<&mut DirEnt> {
        let mut cur = self;

        for bit in bits {
            let resolve = cur.content.find_dir_ent_name_mut(bit.as_ref());
            if let None = resolve {
                return None;
            }
            cur = resolve.unwrap();
        }

        Some(cur)
    }
}

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    Dir(String),
    File(String, usize),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(" ").ok_or(Error(format!("invalid command: {}", s)))?;

        let command = match (first, second) {
            ("$", cmd) => {
                match cmd.split_once(" ") {
                    Some(("cd", dir)) => Command::Cd(dir.to_string()),
                    _ => Command::Ls,
                }
            }
            ("dir", dir_name) => Command::Dir(dir_name.to_string()),
            (size, file_name) => Command::File(file_name.to_string(), size.parse().unwrap()),
        };

        Ok(command)
    }
}

pub struct Environment<'a> {
    cur_path: Vec<String>,
    root_dir_ent: &'a mut DirEnt,
}

impl<'a> Environment<'a> {

    pub fn new(root_dir_ent: &'a mut DirEnt) -> Environment {
        Environment {
            cur_path: Vec::new(),
            root_dir_ent,
        }
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), Error>{
        if dir == ".." {
            if self.cur_path.len() > 0 {
                self.cur_path.pop();
            }
            return Ok(())
        } else if dir == "/" {
            self.cur_path.clear();
            return Ok(())
        }
        let cur_dir_ent = self.get_cur_dir_ent();
        if cur_dir_ent.resolve(&[dir]).is_none() {
            return Err(Error(format!("invalid directory: {}", dir)));
        }
        self.cur_path.push(dir.to_string());
        Ok(())
    }

    pub fn get_cur_dir_ent(&self) -> &DirEnt {
        self.root_dir_ent.resolve(&self.cur_path[..]).unwrap()
    }

    pub fn get_cur_dir_ent_mut(&mut self) -> &mut DirEnt {
        self.root_dir_ent.resolve_mut(&self.cur_path[..]).unwrap()
    }

    pub fn eval(&mut self, cmd: &Command) {
        match cmd {
            Command::Cd(dir) => {self.cd(dir).unwrap();},
            Command::Dir(name) => {self.get_cur_dir_ent_mut().mkdir(name);},
            Command::File(name, size) => {self.get_cur_dir_ent_mut().push_file(name, *size);},
            _ => {}
        };
    }

}

#[cfg(test)]
mod test {

    /*
    - / (dir)
      - a (dir)
        - e (dir)
          - i (file, size=584)
        - f (file, size=29116)
        - g (file, size=2557)
        - h.lst (file, size=62596)
      - b.txt (file, size=14848514)
      - c.dat (file, size=8504156)
      - d (dir)
        - j (file, size=4060174)
        - d.log (file, size=8033020)
        - d.ext (file, size=5626152)
        - k (file, size=7214296)
     */

    use super::*;

    static SCRIPT: &'static str = include_str!("../res/day7-bash_example.txt");

    #[test]
    fn test_env() {
        let mut root = DirEnt::empty_dir("/");
        let mut env = Environment::new(&mut root);

        env.cd("/").unwrap();
        env.get_cur_dir_ent_mut().push_file("b.txt", 14848514);
        env.get_cur_dir_ent_mut().push_file("c.dat", 8504156);

        env.get_cur_dir_ent_mut().mkdir("a");
        env.get_cur_dir_ent_mut().mkdir("d");
        env.cd("a").unwrap();
        env.get_cur_dir_ent_mut().mkdir("e");
        env.cd("e").unwrap();
        env.get_cur_dir_ent_mut().push_file("i", 584);
        env.cd("..").unwrap();
        env.cd("..").unwrap();
        env.cd("d").unwrap();
        env.get_cur_dir_ent_mut().push_file("j", 4060174);

        println!("{:#?}", root);
    }

    #[test]
    fn test_tree() {

        let mut r = DirEnt::empty_dir("/");

        let r_a = r.mkdir("a");
        let r_a_e = r_a.mkdir("e");
        r_a_e.push_file("i", 584);

        r_a.push_file("f", 29116);
        r_a.push_file("g", 2557);
        r_a.push_file("h.lst", 62596);

        r.push_file("b.txt", 14848514);
        r.push_file("c.dat", 8504156);

        let r_d = r.mkdir("d");
        r_d.push_file("j", 4060174);
        r_d.push_file("d.log", 8033020);
        r_d.push_file("d.ext", 5626152);
        r_d.push_file("k", 7214296);

        println!("{:#?}", r);

        println!("total size: {}", r.du());

    }

    #[test]
    fn test_parse_cmds() {
        let commands: Result<Vec<Command>, Error> = SCRIPT.lines().map(|l| l.trim_end().parse()).collect();

        println!("{:#?}", commands);
    }

    #[test]
    fn test_eval_cmds() {
        let commands: Result<Vec<Command>, Error> = SCRIPT.lines().map(|l| l.trim_end().parse()).collect();
        let commands = commands.unwrap();

        let mut root = DirEnt::empty_dir("/");
        let mut env = Environment::new(&mut root);

        for cmd in commands {
            env.eval(&cmd);
        }

        let du_by_dir = root.du_by_dir();
        println!("{:#?}", du_by_dir);
        let sum_to_delete: usize = du_by_dir.values().filter(|v| **v <= 100000).sum();
        assert_eq!(sum_to_delete, 95437);
    }
}

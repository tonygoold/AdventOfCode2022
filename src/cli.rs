use regex::Regex;

pub enum Node {
    File(File),
    Directory(Directory),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Self::File(file) => file.name(),
            Self::Directory(dir) => dir.name(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::File(file) => file.size(),
            Self::Directory(dir) => dir.size(),
        }
    }

    pub fn size_mut(&mut self) -> usize {
        match self {
            Self::File(file) => file.size(),
            Self::Directory(dir) => dir.size_mut(),
        }
    }
}

pub struct NodeIter <'a> {
    queue: Vec<&'a Node>,
}

impl <'a> NodeIter <'a> {
    pub fn new(root: &'a Node) -> Self {
        Self { queue: vec![root] }
    }
}

impl <'a> Iterator for NodeIter <'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop()?;
        if let Node::Directory(dir) = node {
            self.queue.extend(dir.items.iter());
        }
        Some(node)
    }
}

/*
pub struct DirectoryIterMut <'a> {
    queue: Vec<&'a mut Node>,
}

impl <'a> DirectoryIterMut <'a> {
    pub fn new(root: &'a mut Node) -> Self {
        Self { queue: vec![root] }
    }
}

impl <'a> Iterator for DirectoryIterMut <'a> {
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop()?;
        if let Node::Directory(dir) = node {
            self.queue.extend(dir.items.iter_mut());
        }
        Some(node)
    }
}
*/

pub struct Directory {
    name: String,
    items: Vec<Node>,
    cached_size: Option<usize>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            items: Vec::new(),
            cached_size: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.items.iter().map(|item| item.size()).sum()
    }

    pub fn size_mut(&mut self) -> usize {
        if let Some(size) = self.cached_size {
            size
        } else {
            let size = self.items.iter_mut().map(|item| item.size_mut()).sum();
            self.cached_size = Some(size);
            size
        }        
    }

    pub fn get(&self, name: &str) -> Option<&Node> {
        self.items.iter()
            .find(|item| item.name() == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.items.iter_mut()
            .find(|item| item.name() == name)
    }

    pub fn mkdir(&mut self, name: &str) {
        for item in self.items.iter() {
            if item.name() == name {
                return;
            }
        }
        self.items.push(Node::Directory(Directory::new(name)));
    }

    pub fn mkfile(&mut self, name: &str, size: usize) {
        for item in self.items.iter() {
            if item.name() == name {
                return;
            }
        }
        self.items.push(Node::File(File::new(name, size)));
    }
}

pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct Cli {
    root: Node,
    pwd: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            root: Node::Directory(Directory::new("/")),
            pwd: Vec::new(),
        }
    }

    pub fn cd(&mut self, name: &str) {
        match name {
            "/" => self.pwd.clear(),
            ".." => _ = self.pwd.pop(),
            _ => self.pwd.push(name.to_owned()),
        };
    }

    pub fn root_dir(&mut self) -> &mut Node {
        &mut self.root
    }

    fn cur_dir_mut(&mut self) -> Option<&mut Directory> {
        let mut dir = if let Node::Directory(d) = &mut self.root {
            d
        } else {
            return None;
        };
        for name in self.pwd.iter() {
            let child = dir.get_mut(name)?;
            if let Node::Directory(d) = child {
                dir = d;
            } else {
                return None;
            }
        }
        Some(dir)
    }

    pub fn add_item(&mut self, item: &str) {
        let parent = self.cur_dir_mut().expect("no directory at the current path");
        let dir_re = Regex::new(r"dir (\w+)").expect("failed to build regex");
        let file_re = Regex::new(r"(\d+) (.*)").expect("failed to build regex");
        if let Some(caps) = dir_re.captures(item) {
            let name = &caps[1];
            parent.mkdir(name);
        } else if let Some(caps) = file_re.captures(item) {
            let size = &caps[1].parse().expect("file size is not a number");
            let name = &caps[2];
            parent.mkfile(name, *size);
        } else {
            panic!("item is not a file or dirextory");
        }
    }

    pub fn handle_line(&mut self, line: &str) {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            self.cd(dir);
        } else if line != "$ ls" {
            self.add_item(line);
        }
    }
}

impl <'a> Cli {
    pub fn iter(&'a self) -> NodeIter<'a> {
        NodeIter::new(&self.root)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

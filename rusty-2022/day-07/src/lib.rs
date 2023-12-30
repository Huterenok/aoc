use std::iter::Peekable;

pub fn count_100k_dirs(input: &str) -> usize {
    let mut logs = parse_logs(input).into_iter().peekable();
    let mut res = 0;

    check_dir(&mut logs, &mut res);
    res
}

pub fn find_fat_dir(input: &str) -> usize {
    let mut logs = parse_logs(input).into_iter().peekable();
    let res = delete_dir(&mut logs);
    search(&res, res.1 - 40_000_000).unwrap()
}

pub fn delete_dir(logs: &mut Peekable<impl Iterator<Item = Log>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(log) = logs.next() {
        match log {
            Log::Command(Command::ChangeDir(ChangeDirType::Upper)) => break,
            Log::Command(Command::List) => {
                size = std::iter::from_fn(|| logs.next_if(|log| !log.is_command()))
                    .filter_map(|log| match log {
                        Log::FsElement(FsElement::File(_, size)) => Some(size),
                        _ => None,
                    })
                    .sum();
            }
            _ => dirs.push(delete_dir(logs)),
        }
    }

    size += dirs.iter().map(|d| d.1).sum::<usize>();
    Dir(dirs, size)
}

fn search(d: &Dir, min: usize) -> Option<usize> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
}

pub fn check_dir(logs: &mut Peekable<impl Iterator<Item = Log>>, res: &mut usize) -> usize {
    let mut size = 0;
    while let Some(log) = logs.next() {
        match log {
            Log::Command(Command::ChangeDir(ChangeDirType::Upper)) => break,
            Log::Command(Command::List) => {
                size = std::iter::from_fn(|| logs.next_if(|log| !log.is_command()))
                    .filter_map(|log| match log {
                        Log::FsElement(FsElement::File(_, size)) => Some(size),
                        _ => None,
                    })
                    .sum();
            }
            _ => size += check_dir(logs, res),
        }
    }

    if size <= 100000 {
        *res += size;
    }
    size
}

pub fn parse_logs(input: &str) -> Vec<Log> {
    input
        .lines()
        .map(|line| {
            if line.starts_with('$') {
                let mut splitted_parts = line.split(" ");
                if splitted_parts.nth(1).unwrap().eq("cd") {
                    let dir_name = splitted_parts.nth(0).unwrap();
                    match dir_name {
                        "/" => Log::Command(Command::ChangeDir(ChangeDirType::Outermost)),
                        ".." => Log::Command(Command::ChangeDir(ChangeDirType::Upper)),
                        to => Log::Command(Command::ChangeDir(ChangeDirType::In(to.into()))),
                    }
                } else {
                    Log::Command(Command::List)
                }
            } else {
                let (dir_or_size, name) = line.split_once(" ").unwrap();
                if dir_or_size.eq("dir") {
                    Log::FsElement(FsElement::Dir(name.into()))
                } else {
                    Log::FsElement(FsElement::File(name.into(), dir_or_size.parse().unwrap()))
                }
            }
        })
        .collect()
}

#[derive(Debug)]
pub enum Log {
    Command(Command),
    FsElement(FsElement),
}

impl Log {
    pub fn is_command(&self) -> bool {
        match self {
            Self::Command(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Command {
    List,
    ChangeDir(ChangeDirType),
}

#[derive(Debug)]
pub enum ChangeDirType {
    Outermost,
    Upper,
    In(String),
}

pub struct Dir(Vec<Dir>, usize);

#[derive(Debug)]
pub enum FsElement {
    Dir(String),
    File(String, usize),
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = count_100k_dirs(&example_input);
        let res1 = count_100k_dirs(&input);
        assert_eq!(res1_example, 95437);
        assert_eq!(res1, 1367870);

        let res2_example = find_fat_dir(&example_input);
        let res2 = find_fat_dir(&input);
        assert_eq!(res2_example, 24933642);
        assert_eq!(res2, 549173);
    }
}

use std::io::Write;
use std::{env, io};
use sysinfo::{Pid, ProcessesToUpdate, System};

// показать текущую директорию
fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("ошибка команды pwd: {}", err),
    }
}

// вывести аргумент в std
fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

// изменить рабочую директорию
fn cd(argument: &str) {
    match env::set_current_dir(argument) {
        Ok(_) => (),
        Err(err) => eprintln!("ошибка команды cd: {:?}", err),
    }
    println!("Текущая директория: ");
    pwd()
}

// показать процессы в работе
fn ps(system: &mut System) {
    system.refresh_all();

    for (pid, process) in system.processes() {
        println!(
            "PID: {:?}; имя: {:?}; время жизни в секундах: {:?}",
            pid,
            process.name(),
            process.run_time()
        );
    }
}

// убить процесс
fn kill(system: &mut System, pid: usize) {
    // преобразование pid в нужную форму
    let pid_parsed = Pid::from(pid);
    system.refresh_processes(ProcessesToUpdate::Some(&[pid_parsed]));

    // попытаться убить процессы
    match system.process(pid_parsed) {
        Some(process) => {
            if process.kill() {
                println!("процесс {:?} убит успешно X_X", pid)
            } else {
                eprintln!("не удалось убить процесс с id {:?} ", pid);
            }
        }
        None => eprintln!("процесс с PID {:?} не найден", pid),
    }
}

fn main() {
    // баффер для комманд
    let mut input_buffer = String::new();
    // обработка пока не \q или не \quit
    loop {
        // чтение из stdin
        input_buffer.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input = input_buffer.trim();

        // преобразование строчи с пайпом в вектор из команд и аргументов
        let commands: Vec<Vec<&str>> = input
            .split('|')
            .map(|cmd| cmd.split_whitespace().collect())
            .collect();
        // струтура системы
        let mut system = System::new_all();
        // выполнение каждой команды и проверка входных аргументов
        for cmd in commands {
            match cmd[0] {
                "pwd" => pwd(),
                "echo" => {
                    if cmd.len() < 2 {
                        eprintln!("неверное количество элементов для команды echo")
                    }
                    echo(&cmd[1..])
                }
                "ps" => ps(&mut system),
                "cd" => {
                    if cmd.len() != 2 {
                        eprintln!("неверное количество элементов для команды cd")
                    }
                    cd(cmd[1])
                }
                "kill" => {
                    if cmd.len() != 2 {
                        eprintln!("неверное количество элементов для команды kill")
                    }
                    let pid: usize = cmd[1]
                        .parse::<usize>()
                        .expect("неверный формат ID для команды kill");
                    kill(&mut system, pid)
                }
                "\\quit" => return,
                "\\q" => return,
                _ => eprintln!("неверное название команды: {}", cmd[0]),
            }
        }
    }
}

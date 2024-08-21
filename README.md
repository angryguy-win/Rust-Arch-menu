# This is a work in Progress!!!!...
This is a learning project.

## Todo
```
[ ] Add Distro install
[ ] Add microcode check
[ ] Add GPU card check, Quetions
[ ] Add BTRFS Quetions and setup
[ ] Add logging / errors
[ ] Add refactor the program into app, ui, main ect..
[x] Add themes
[ ] Add more
[ ] Add 

[ ] Add The install scripts
```

# Arch-menu

This app was created to give the user a menu writen in rust, 
To ask the User quetion's that they would use to install arch.
All of these quetions are compiled into a arch_config.toml 
file..
## Modules Used:
```
[dependencies]
ratatui = "0.20.1"
crossterm = "0.25"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.5"
```


This config file can be used in bash script to install Arch.
### Splash Screen
![image](https://github.com/user-attachments/assets/a1adb145-13a4-46b2-9091-810e1395066d)
### Text input, Quetions.
![image](https://github.com/user-attachments/assets/894980c7-6548-472b-aec5-395729b055aa)
### Muliple Choice to select, Question.
![image](https://github.com/user-attachments/assets/b1256586-5086-4df7-a3bf-d8a5cfccb931)
### Boolean True/false, Question.
![image](https://github.com/user-attachments/assets/c4944778-9866-433a-b7d7-4767cfebdbbc)

## The configuration file created ->  acrh_config.toml

![image](https://github.com/user-attachments/assets/c20ca85d-1cdd-405c-a396-6bae3ccb8cf5)


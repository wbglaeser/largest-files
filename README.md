## Rust File Scanner

This repo contains code for a small rust application that helps identify large files
in a given directory and cascading sub-directories. Seeing that there are plenty
of optimised tools available to perform this task this project was meant for me to better
understand file systems and the rust language.

#### Installation
1. Using ssh, clone repo to local enviroment

`git clone git@github.com:wbglaeser/largest-files.git`

2. Run using cargo, where -e allows you to exlude certain file types

`cargo run <Path> -e <exclude>`

3. Build using cargo. Binary is stored in `/target`

`cargo build`

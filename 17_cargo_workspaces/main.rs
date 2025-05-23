/*

1. create a wokspace folder and open in vs code
    $mkdir myworkspace
    $code .

2. add cargo.toml file 

3. create the packages in workspace folder by
    $cargo new package1
    $cargo new package2

4. then build by
    $cargo build // this going to add cargo.lock file and target folder in workspace folder

5. set the cargo.toml file as and build

    [workspace]

    members = [
                "package1",
                "package2",
              ]
6. in the packages you will see local cargo.toml files for each package. 
   we can use these files to define dependencies 

        [package]
        name = "package1"
        version = "0.1.0"
        authors = ["email"]
        edition = "2025"

        [dependencies]
        package2 = { path = "../package2"} 
    
    now in package1's main.rs we can take the package2's main or lib file to scope by
        use package2; (if package name has '-' we must use '_' with 'use' keyword)

7. if we want to run a specific package
    $cargo run -p package1

8.External Dependencies 
    Even if the external crates used by the packages are the same, 
    they must be added to the dependencies separately in the cargo.toml of each package.

9.When cargo test is called in the workspace folder, tests in all packages are performed.

10. if we want to run tests for only a specific package:
    $cargo test -p package1




    













*/
# sqlant

Generate PlantUML/Mermaid ER diagram textual description from SQL connection string  
**Inspired by [planter](https://github.com/achiku/planter)**  

## Why created
I like the [idea of planter](https://github.com/achiku/planter#why-created) and I use it in 
internal confluence documentation with PlantUML plugin for my projects.  
But I want to make it better

## Installation 
### Compiled Binary (Linux only)
Download the binary file from [releases](https://github.com/kurotych/sqlant/releases) page
### Docker (Compressed size around 5MB)
The Docker image contains the `sqlant` binary and serves as a wrapper for executing it.
[link](https://hub.docker.com/r/kurotych/sqlant)
```bash
docker pull kurotych/sqlant:latest
```
### Cargo
```bash
cargo install sqlant
```


## Usage
### PlantUML
#### Binary
```bash
sqlant postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db --legend -e
```
#### Docker
```bash
docker run --network host kurotych/sqlant postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db 
```

![example link ](https://www.plantuml.com/plantuml/png/hLNTSjii5BpdAVWotuKp4yUDyrEdCvt9cp08ZYrgu50un77gl7kbE6f3HgD6KUumx0AxQortYMIax2ohZGQkJ5GMMDeQ7sIKZblZWVO1E4QgaR7_Z5SsDyYQAAHlYqMKk_EDeJfNEK5Kw0aydIjqYssEI7jLBz9FApqjgYLSw-eMzEhc-dQzN5qilwpapUghlBowgYwlrklBPkfMwKN8piwjgHQw4krcxM_6I5OMPYedGWVnbFzYd2kqsVcPqMVyf38Ru-daZFyVjjyfPcX6tZ-FJXleV3x_Iv1QHqYfOH4yq4c1x31UEXW4X1ez29zT1N4G665Z4a44BIIrIECWaNI1xmpLlFt97z53F_lH1A5GzzxbwQqj0gEUQiwVlTumrptCZgF1cdk8U-60QjI3Tc3K7_KYoBq3J-yv9TKc1ECtuZrP4vAq8aIZMfjzTj0CXw0a7uHqc3qL-9vadjKA3IIDBN8f8nFzCVNQFY7RjCrZOkqaTl1FpxDrNCWrGMmj7VQ-WrUmnWWjVeptGeOGyLv-UWZbAUDsAB8vNl1ZHcA0A0bB1RsUX8WwAvfM4VVWDOug22K9DXZt3U7b1gARwbUaC7kA-x4LNOfDspn9QEVMAALeCGu_73cgMYOGsHjwN-iaQI6DmPl7uSTh1sPJ_xEyZZbFi73gEzfpcwxGYriTbwPyVShGy9_D6WyNWuX4aZgfez_oyg1bXtYMpygQuayudnyEl9jbx7K5bQiTfn-JjfcntfRaXoFYzI9ZBvz3Hp-wpbYpJJVrWc2i38iVIWO3dztiRHCqei62eeZUgER5_W4xiErqZGuQAvZKMbZOWDUpKtO7NcTdVmC0)
```
postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db -o mermaid
```
### Mermaid
![image](https://github.com/kurotych/sqlant/assets/20345096/a7d64db6-2d78-4631-bbfc-58cad5a77adb)

# sqlant

Generate PlantUML/Mermaid ER diagram textual description from SQL connection string  
**Inspired by [planter](https://github.com/achiku/planter)**  

## Installation 
### Compiled Binary (Linux only)
Download the binary file from [releases](https://github.com/kurotych/sqlant/releases) page
### Docker (Compressed size around 6MB)
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

![example link ](https://www.plantuml.com/plantuml/png/hLRjKYCh4FtFKynDtHzQPOpHwqTkrLRv9XG3dGmLFbXe4iUDxxxa0rTqYZN6Np9mJjDz3dRdCI3p6BKYdHJGSEcv0XAMqZZccMwKD82zWyPwx2mX_qZ3LKp83j65_oSJpzQN2ubTR6C0pwr1C7Z9hPuiexVOysuIVYfcS39hfpDnTpURjhUt_E7ceRrwk9f1shc_keUxwUtD3VkFp-RN4vVI6IlPJaHBjy6stuGWQnMSyHZGQl3dpI_IDDoggCsP51VDg9KBQN1qqVphbZ_GHqWhOtQhymGTZyT_24m83o4a5i8JZWfanXYceGfmdJL0JTGj-2hGmq8610-2CjmYfOQ0JBjcBR5hjf_DipKmp7wMZd8h1dDvUIyBjLwSAivhh7VC-G0pSmGekGBVKmtOML6LmthnLIqSwpKO_CmjePFEIREWd_4QBJ95dPSS4iv43MbPWo9xeapRQB303pYpgvOAG2PLuKjf6HssgIxxyTw6PJp6rbnYXd-tdpl5APiZ-AsaTUqpl8MvzL313sfFcNFUhjYtcr3UKezGzMQVuDU4j0uyGrjMCAY9yrP4ZgUrY1KOOOzg49mXBApl4-6G0SrRes62ZGPzVsoBBeiDotXQeJdNOogrolhwu8YUTom0ZKRYvxfEO0h2CNZvN1zUQv2Bxc_Dw-3pQHAUD4S7iiaDlSZgS7J2Vn-NM7ziIXgOLX416PQbrcTvvQBhZDYLJrOIrcVQ6knKJ_Tl8KIjcUzJ1CqGl6GgMIvsiGfbTgmKJIJKwCCSs_Mky2hQHq1mKCoubCyX7RIEvlQPpQZyWFYcgqdPiPiNEnvlDatU9UHjDywd9MSO4vQHaC61qMpsYBU7X1mwGwRl7o0XAKpUEY08ATzjioaOZ-bFrEet)
```
postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db -o mermaid
```
### Mermaid
![image](https://github.com/kurotych/sqlant/assets/20345096/a7d64db6-2d78-4631-bbfc-58cad5a77adb)
## Links
- [Optimizing the Process of ER Diagram Creation with PlantUML](https://kurotych.com/posts/er-diagram-creation/)

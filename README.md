# sqlant

Generate PlantUML/Mermaid ER diagram textual description from SQL connection string  
**Inspired by [planter](https://github.com/achiku/planter)**  
##### **Currently, supports only PostgreSQL NoTls connection**

## Why created
I like the [idea of planter](https://github.com/achiku/planter#why-created) and I use it in 
internal confluence documentation with PlantUML plugin for my projects.  
But I want to make it better

## Installation 
```bash
cargo install sqlant
```

## Usage
### PlantUML
```bash
sqlant postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db
```

![example link ](https://www.plantuml.com/plantuml/png/pPP1Jzim5CVl_XHMaD1MhQhWW81r5QVji0tjjgFbne_4gcD7poz0MFZkImEpGcUvMONGUgYI-EVdlFV_vzbNdYHIKnd6Igs0vnfp0ynljAqboeeRRO7Q6hX3AXrZO4bJoxFPB6yykGfmDiluBxPShvQvCmulZzJ5XJh9LylZ3RIhvM9ukkh7iqohB5ikrgjBlbXasLeNsbAMzJTiFf-psS18fH2y7uC4zy_O4s9b1QdnkdGMX6sgDM2AGoYq9q1GGj8BK5VWILOrlzDKykqjVt0MnfCVT2rofo2-m6EiKiAXUjLwFGUGmoyM8AmSZJJAHe7Hju2jg81Afr9LSwWEB5ajsmeiJLZ3bxgkjIt413UG8sb4ZJg7CTRB3_splqcIzq9Mhh4KmIFg3VaA1IIrcQZTT5qSgfGwRsGJlsdsEYIKBC4SRer1FY5l270hK4h8GA_XnY4ay8IGGjOrkblv7ogwTwaNUKnggKEk1-dYQa19BfvGfCOrEjNaz9OzxXQiSlixN2u92eVjc7f-Mn76nlpMxW5FxqFfzkytpTlL8oIrjo33bEGssc1UIW-YiPecDLWl3VX_mFL3SX-_0rsK4xlqamjUgywlSz28l-EVFUtmpfFsVoAVN5FIq4PRE8a3OVmqkM-J94W0-qzDB7fUUdJpUTVjd3ePssdgSJ94IV7XCsKijLDqWTw9mzYu-OMmOsCo5XO9m-CvZ4KjqiKYCMQWw6yvRhyGnfT7NqhdwMUf2JwChbSTvxpxn8NuDl12y8xnRcIwzVdvsSbwrrUzDXt_WRgIsFAlq0wpabRi-6U7e0iRS7vyT7vwUlQnl_ekhPBa-bWt6epYxwuTyUl6cFSz2LOz9SEk-h-cChy1)
```
postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db -o mermaid
```
### Mermaid
![image](https://github.com/kurotych/sqlant/assets/20345096/a7d64db6-2d78-4631-bbfc-58cad5a77adb)

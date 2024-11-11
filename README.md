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
sqlant postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db --legend -e
```

![example link ](https://www.plantuml.com/plantuml/png/hLPDK-Cu4BthL_GOTaYyK2auJ36p8JMNsJtixjJUzkXIx7QiGPPSKXi86_xxIZ82nnLC1iB5ZVJwez-JMlbgYHjgQyagKI3amkGIcRqMgk66ro25Gbet2DfGfHbZ7yfMvIIqWbpBjbQJOyLhF8LZX_AVvlggvD9witmsbUhCAGFu87NEfKCQBCKTN46ICByiIfWuTxYw0Z9jWBhL7cpkCchpvQZu_BaupnVpZzUuNindONMH-13eKeZIbQHUZf9sI5M8yZbj-NBQQFtwN00FVuC21c4DKcjKp_jalbmaoOcp0cD-9Kdci-NYn9jV87ncimuIsEmWC9kTC1yu4j_NWFXIOYV1j9Vr11K9Mhjcqws4QHgY9y0j975Z6BmnEiUYDGYeslf9tnjk15TaGvvZAImvSdwnZHHaoKoUzayXEKscdWEz_84v-I92Z3I5HmARrZGI5saw-JPDqqXWgAehzRzEX58Rv7a5nuAm1g4Wf0rxq2q6EzIlZRCVGYzF3MmrmuiYaxXo-nFVGcU24QigH9mV_L5fI_zMWl2qPoysRdDp1LdsL_38CjWml-JaZETXUNdvwNmvE0zUsWtyOqJDpHh-n7Nq6pj46_XT6nGhrNE0ZqUN5nUVVdFMtrmLdBIBLmgK1Ko-QuBlhPJJY1rbsEr_Sv86h6kbes186UlELDvQqZMQgHSZtdLHE7MNOYKK9TrT9zDsLprQIWNBKtnsTlRiwkWtQFtf8BoZP0iPCLxxKK1JQOMPQs69PXHeS2KieS4Yu-IHeaPBl6xWLb05_b-uTx5sUz-B9ijriREMXo51cJR5mMXwZjgBiRl80gMuGRFEQgHA5wDqk1BVoaN3rpKgsYUBhKJJ2BNAzj6QD76PDKRadcOdAJfp34hyBkErRXKDyKDbGhTPWSI5xEioa2pxEPdUhwszRb-NDROOGGVNqpEUyPBGU4h7kO_m9VfH7F7xSf_nfZ7wXXzUXAqixr2aA_1jHEe63ISzx6079ov7qUPqg15nWwhGUmo8d1Ekj5axs6C5MLnwmS6VFBD-quYEpVf3VgX6OzMlT-fVj_67epLvnIrchHAqtzInNABz6Rou7t-yRb6igbU7LwVk-oayq8iwg2kEzdvzSUizhvwWyMCdXSBpKOGbJh7Lrr_rqKGN4H7QPzi32nxqsTbc0t6LiVum6kvsFhinxgOmjdDUmgP3z3SPUxmEqRrZdJ4N9Yuz8XZR5Wissnui_AuVpWJND4hCoMWbSdiAF_0MfL-kN2EE2hRr0_gh-xIr_1y0)
```
postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db -o mermaid
```
### Mermaid
![image](https://github.com/kurotych/sqlant/assets/20345096/a7d64db6-2d78-4631-bbfc-58cad5a77adb)

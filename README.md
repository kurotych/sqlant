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

![example link ](https://www.plantuml.com/plantuml/png/nPR1Jzi-5CVl_YkUWeIW-bKL713mwogqmtROXdRQZfPZFnfJnuxi5o0i_EzhGx525qnN8TPJKd_yxFU-tppx8f3mL9U65LeXIEsbGHOMsbR2YnACjaXDXU0y5OunjAIfWMmqadKWLw8VZJBunQRvR2gTSVvyJvsTgICvc-uli9bD9zs_i-LubccLPTD9FZk7oIJBSZtNbh9i-SB6up4RTOnGocC8VP3mxX5R8rQKc1gl2cUHsxhCqQSuZtCT23qgBkWPg4iC9CeARZKLi7g5UsUnF_bULrmwzS967eOgnPrNRrglTG3H_OO2n9LVY1IpKMZq3Vg6bqY5KwcgEQzsB5abcX8jRR56A7HLQJldMxY170d3Awybzh6b_EYr_9CaSST5wMfBm_YcUbs-N249RRRKRdjTd5SfTTx99c5Hl-gGMB84ixirElcuk2BqAu5IGaOwnoldCO4dXEIYghox4VzSqHsMUfR7LYsrotKdNLo5Nf1B9FIynfLtgfRqdZtk1grohtSuAGZdpZTzQhrR7iVw_Chk4K-VG-ds7nRr-zKZ9lLgGRefoMEqzh8G0NbjDMtLO8Cm6Fu2r_V9S_mMzRpOiai_kk2jwlms5utXFJwTQJjsdhJ_q_b8JKb36stnA0K44eGf87qhYeiSlrWjDdXVUd3ZSTjkt3ePssZgGsICakRZPyfOwgHe8xqHXx5-yujX-yPaB2uIX_jp61kqI3iKYRdy6YL1jhR7HrFZ3-qCqYo6Z5rjD7ho2noWhSl334I0JMhfHK23ahopMWO6rtYBnZSC2bTY9ut1ELe5VlLLCfZkVIvSe9-59lolzpnh0Q3z09n_xNZEeMNGzeoUdvwU7EMRNbxweONlNfV2D_05csvMqlujV78UzTn67FRtJe-FJ_vVFdqJLWbQ9NQbqIWuk7G4bxKnXqiIhNfCXbsWLNLfVWC0)
```
postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db -o mermaid
```
### Mermaid
![image](https://github.com/kurotych/sqlant/assets/20345096/a7d64db6-2d78-4631-bbfc-58cad5a77adb)

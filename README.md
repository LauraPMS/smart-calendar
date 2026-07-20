# Organisation d'astreinte

## Installation

- Cloner le repos:  
```git clone https://github.com/LauraPMS/smart-calendar.git```

- Installer Cargo et rustup :  
    1. Se rendre sur le site offciel rustup.rs et télécharger le rustup-init.exe  
    2. Suivre les instructions  
    3. Recharger le terminal  

- Se déplacer dans le bon repos :  
````cd astreinte```

- Lors du premier lancement, il faut initialisé la base de donnés
    1. ```DATABASE_URL="sqlite://*********.db"```  
    2. ```cargo sqlx database create```  
    3. ```cargo sqlx migrate run```  

- Lancer l'application avec Cargo :  
    ```cargo run```

- Cela va rendre l'app accessible à l'adresse :  
    ```http://127.0.0.1:3000/```

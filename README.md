# Embedding Project: Une Base de Données avec Recherche par Similarité Cosinus

## Description
Ce projet est une implémentation en Rust d'une base de données simple permettant de stocker des collections de vecteurs, d'ajouter, de modifier et de supprimer des documents, et de rechercher les documents les plus similaires à un vecteur donné à l'aide de la similarité cosinus.

---

## Fonctionnalités
- **Gestion des collections** : Créez et gérez plusieurs collections indépendantes.
- **Insertion et mise à jour des documents** : Ajoutez ou modifiez des documents dans une collection.
- **Recherche par similarité** : Trouvez les `k` documents les plus similaires à un vecteur donné dans une collection.
- **Threading** : Calcul de la similarité cosinus optimisé grâce au multi-threading.
- **Génération de documentation** : Utilisez Rustdoc pour générer une documentation claire et complète directement depuis le code source.

---

## Prérequis

- Rust (version stable ou plus récente)
- Une bibliothèque pour gérer les UUID : [uuid](https://crates.io/crates/uuid)

---

## Installation

1. Clonez le dépôt :
   ```bash
   git clone https://github.com/ClementMounic/EmbeddingProject
   cd EmbeddingProject
   ```
2. Assurez-vous que Rust est installé :
   ```bash
   rustc --version
   ```
3. Lancez le projet :
   ```bash
   cargo run
   ```

---

## Utilisation

### Initialisation
1. Créez une instance de base de données :
   ```rust
   let mut bdd = BaseDeDonnees::new();
   ```
2. Ajoutez des collections :
   ```rust
   bdd.add("NomDeLaCollection".to_string());
   ```

### Ajout de Documents
Ajoutez des vecteurs à une collection :
```rust
if let Some(collection) = bdd.get_mut("NomDeLaCollection") {
    collection.upsert(Uuid::new_v4(), vec![1.0, 2.0, 3.0]);
}
```

### Recherche
Effectuez une recherche par similarité :
```rust
let requete = vec![1.0, 0.0, 3.0];
if let Some(resultats) = bdd.search("NomDeLaCollection", &requete, 3) {
    for (uuid, similarite) in resultats {
        println!("UUID: {}, Similarité: {}", uuid, similarite);
    }
}
```

### Suppression de Documents
Supprimez un document par son UUID :
```rust
if let Some(collection) = bdd.get_mut("NomDeLaCollection") {
    collection.delete(&uuid);
}
```

---

## Calcul de la Similarité Cosinus
La fonction de similarité cosinus est définie comme suit :
```rust
cos(vector1, vector2) = produit_scalaire / (magnitude1 * magnitude2)
```
- Le calcul du produit scalaire et des magnitudes est parallélisé à l'aide de threads pour optimiser les performances.

---

## Documentation
Ce projet utilise Rustdoc pour générer une documentation automatique à partir des commentaires du code source. Pour générer la documentation locale :
```bash
cargo doc --open
```
Vous pouvez consulter la documentation dans votre navigateur une fois le processus terminé.

---






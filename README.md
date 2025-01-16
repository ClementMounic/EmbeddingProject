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

## Documentation
Ce projet utilise Rustdoc pour générer une documentation automatique à partir des commentaires du code source. Pour générer la documentation locale :
```bash
cargo doc --open
```
Vous pouvez consulter la documentation dans votre navigateur une fois le processus terminé.

---






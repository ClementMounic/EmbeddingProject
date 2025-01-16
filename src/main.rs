use std::collections::HashMap;
use std::thread;
use uuid::Uuid;

/// Type alias pour représenter un document sous forme d'une liste de tuples contenant un `Uuid` et une similarité (f32).
type Document = Vec<(Uuid, f32)>;

/// Une structure représentant une collection de documents, chaque document est identifié par un `Uuid` et contient un vecteur de f32.
struct Collection {
    documents: HashMap<Uuid, Vec<f32>>,
}

impl Collection {
    /// Crée une nouvelle instance de `Collection`.
    fn new() -> Self {
        Collection {
            documents: HashMap::new(),
        }
    }

    /// Insère ou met à jour un document identifié par `key` avec le vecteur `vector`.
    ///
    /// # Arguments
    /// * `key` - Identifiant unique du document (Uuid).
    /// * `vector` - Vecteur représentant le document.
    fn upsert(&mut self, key: Uuid, vector: Vec<f32>) {
        self.documents.insert(key, vector);
    }

    /// Lit un document à partir de son `key`.
    ///
    /// # Arguments
    /// * `key` - Référence à l'identifiant unique du document.
    ///
    /// # Retourne
    /// * Option<&Vec<f32>> - Une référence optionnelle au vecteur du document.
    #[allow(unused)]
    fn read(&self, key: &Uuid) -> Option<&Vec<f32>> {
        return self.documents.get(key);
    }

    /// Supprime un document à partir de son `key`.
    ///
    /// # Arguments
    /// * `key` - Référence à l'identifiant unique du document.
    #[allow(unused)]
    fn delete(&mut self, key: &Uuid) {
        self.documents.remove(key);
    }

    /// Recherche les `k` documents les plus similaires à la requête donnée en utilisant la similarité cosinus.
    ///
    /// # Arguments
    /// * `request` - Vecteur de requête.
    /// * `k` - Nombre de résultats à retourner.
    ///
    /// # Retourne
    /// * Document - Liste des `k` documents les plus similaires avec leur similarité.
    fn search(&self, request: &[f32], k: usize) -> Document {
        let mut results: Document = self
            .documents
            .iter()
            .filter_map(|(key, vector)| {
                if vector.len() != request.len() {
                    return None;
                }
                let similarity = cos(request, vector);
                Some((*key, similarity))
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        return results.into_iter().take(k).collect();
    }
}

/// Une structure représentant une base de données composée de plusieurs collections.
struct BaseDeDonnees {
    collections: HashMap<String, Collection>,
}

impl BaseDeDonnees {
    /// Crée une nouvelle instance de `BaseDeDonnees`.
    fn new() -> Self {
        BaseDeDonnees {
            collections: HashMap::new(),
        }
    }

    /// Ajoute une nouvelle collection à la base de données.
    ///
    /// # Arguments
    /// * `nom` - Nom de la collection.
    fn add(&mut self, nom: String) {
        self.collections.insert(nom, Collection::new());
    }

    /// Récupère une référence immuable à une collection par son nom.
    ///
    /// # Arguments
    /// * `nom` - Nom de la collection.
    ///
    /// # Retourne
    /// * Option<&Collection> - Référence optionnelle à la collection.
    #[allow(unused)]
    fn get(&self, nom: &str) -> Option<&Collection> {
        return self.collections.get(nom);
    }

    /// Récupère une référence mutable à une collection par son nom.
    ///
    /// # Arguments
    /// * `nom` - Nom de la collection.
    ///
    /// # Retourne
    /// * Option<&mut Collection> - Référence mutable optionnelle à la collection.
    fn get_mut(&mut self, nom: &str) -> Option<&mut Collection> {
        return self.collections.get_mut(nom);
    }

    /// Effectue une recherche dans une collection spécifique.
    ///
    /// # Arguments
    /// * `cname` - Nom de la collection.
    /// * `request` - Vecteur de requête.
    /// * `k` - Nombre de résultats à retourner.
    ///
    /// # Retourne
    /// * Option<Document> - Résultats de la recherche dans la collection spécifiée.
    fn search(&self, cname: &str, request: &[f32], k: usize) -> Option<Document> {
        return self
            .collections
            .get(cname)
            .map(|collection| collection.search(request, k));
    }
}

/// Calcule la similarité cosinus entre deux vecteurs parallèlement.
///
/// # Arguments
/// * `vector1` - Premier vecteur.
/// * `vector2` - Deuxième vecteur.
///
/// # Retourne
/// * f32 - Similarité cosinus entre les deux vecteurs.
fn cos(vector1: &[f32], vector2: &[f32]) -> f32 {
    let produit_scalaire_handle = thread::spawn({
        let vector1 = vector1.to_vec();
        let vector2 = vector2.to_vec();
        move || {
            vector1
                .iter()
                .zip(vector2.iter())
                .map(|(x, y)| x * y)
                .sum::<f32>()
        }
    });

    let magnitude1_handle = thread::spawn({
        let vector1 = vector1.to_vec();
        move || {
            let somme_carre1: f32 = vector1.iter().map(|x| x * x).sum();
            somme_carre1.sqrt()
        }
    });

    let magnitude2_handle = thread::spawn({
        let vector2 = vector2.to_vec();
        move || {
            let somme_carre2: f32 = vector2.iter().map(|y| y * y).sum();
            somme_carre2.sqrt()
        }
    });

    let produit_scalaire = produit_scalaire_handle.join().unwrap();
    let magnitude1 = magnitude1_handle.join().unwrap();
    let magnitude2 = magnitude2_handle.join().unwrap();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        produit_scalaire / (magnitude1 * magnitude2)
    }
}

fn main() {
    let mut bdd = BaseDeDonnees::new();

    bdd.add("ICC".to_string());
    bdd.add("IA".to_string());

    if let Some(collection) = bdd.get_mut("ICC") {
        collection.upsert(Uuid::new_v4(), vec![12.0, 72.0, 63.0]);
        collection.upsert(Uuid::new_v4(), vec![24.0, 45.0, 36.0]);
    }

    if let Some(collection) = bdd.get_mut("IA") {
        collection.upsert(Uuid::new_v4(), vec![14.0, 30.0, 60.0]);
        collection.upsert(Uuid::new_v4(), vec![10.0, 12.0, 100.0]);
    }

    let requeste = vec![41.0, 51.0, 31.0];
    if let Some(results) = bdd.search("ICC", &requeste, 3) {
        display(results, "ICC");
    }

    if let Some(results) = bdd.search("IA", &requeste, 3) {
        display(results, "IA");
    }
}

/// Affiche les résultats de la recherche.
///
/// # Arguments
/// * `results` - Liste des documents avec leur similarité.
/// * `cname` - Nom de la collection où la recherche a été effectuée.
fn display(results: Document, cname: &str) {
    println!("Résultats de la recherche dans la collection '{}' :", cname);
    for (key, similarity) in results {
        println!("UUID : {}, Similarité : {}", key, similarity);
    }
}

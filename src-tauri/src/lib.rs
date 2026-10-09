//! Lecture du fichier et écriture du résultat.
//!
//! Ce fichier construit le tableau `Fichier` (nom, extension, octets) et le
//! passe à `protocole.rs`. Le protocole ne lit pas le disque lui-même.

mod protocole;

use std::path::{Path, PathBuf};

fn lire_fichier(chemin: &Path) -> Result<protocole::Fichier, String> {
    let nom = chemin
        .file_name()
        .and_then(|nom| nom.to_str())
        .ok_or_else(|| "Le nom du fichier n'est pas lisible.".to_string())?
        .to_string();
    let extension = chemin
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_string();
    let contenu = std::fs::read(chemin).map_err(|erreur| {
        format!(
            "Impossible de lire le fichier {} : {erreur}",
            chemin.display()
        )
    })?;

    Ok(protocole::Fichier {
        nom,
        extension,
        contenu,
    })
}

fn chemin_avec_suffixe(source: &Path, suffixe: &str) -> Result<PathBuf, String> {
    let nom = source
        .file_name()
        .ok_or_else(|| "Ce chemin ne désigne pas un fichier.".to_string())?;
    let mut nouveau_nom = nom.to_os_string();
    nouveau_nom.push(suffixe);
    Ok(source.with_file_name(nouveau_nom))
}

fn nom_de_sortie(nom: &str) -> Result<&str, String> {
    let nom = nom.trim();
    if nom.is_empty() || nom == "." || nom == ".." || nom.contains('/') || nom.contains('\\') {
        return Err(
            "Le protocole doit renvoyer un simple nom de fichier, avec son extension, sans dossier."
                .to_string(),
        );
    }
    Ok(nom)
}

fn ecrire_sans_ecraser(chemin: &Path, contenu: &[u8]) -> Result<(), String> {
    if chemin.exists() {
        return Err(format!(
            "Le fichier de sortie existe déjà : {}. Déplace-le ou renomme-le avant de recommencer.",
            chemin.display()
        ));
    }

    std::fs::write(chemin, contenu).map_err(|erreur| {
        format!(
            "Impossible d'écrire le fichier {} : {erreur}",
            chemin.display()
        )
    })
}

#[tauri::command]
fn chiffrer_fichier(
    chemin: String,
    mot_de_passe: String,
    reponse_1: String,
    reponse_2: String,
    reponse_3: String,
) -> Result<String, String> {
    let chemin = Path::new(&chemin);
    let fichier = lire_fichier(chemin)?;
    let octets = protocole::chiffrer(&fichier, &mot_de_passe, &reponse_1, &reponse_2, &reponse_3)?;
    let sortie = chemin_avec_suffixe(chemin, ".enc")?;
    ecrire_sans_ecraser(&sortie, &octets)?;
    Ok(sortie.display().to_string())
}

#[tauri::command]
fn dechiffrer_fichier(
    chemin: String,
    mot_de_passe: String,
    reponse_1: String,
    reponse_2: String,
    reponse_3: String,
) -> Result<String, String> {
    let chemin = Path::new(&chemin);
    let fichier = lire_fichier(chemin)?;
    let restaure =
        protocole::dechiffrer(&fichier, &mot_de_passe, &reponse_1, &reponse_2, &reponse_3)?;
    let nom = nom_de_sortie(&restaure.nom)?;
    let dossier = chemin.parent().unwrap_or_else(|| Path::new("."));
    let sortie = dossier.join(nom);
    ecrire_sans_ecraser(&sortie, &restaure.contenu)?;
    Ok(sortie.display().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            chiffrer_fichier,
            dechiffrer_fichier
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

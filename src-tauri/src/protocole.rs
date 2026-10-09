//! Protocole de chiffrement et de déchiffrement.
//!
//! C'est le seul fichier à modifier pour écrire le protocole. Le reste de
//! l'application lit le fichier choisi, le range dans un `Fichier`, appelle
//! `chiffrer` ou `dechiffrer`, puis écrit les octets renvoyés.
//!
//! # Le tableau `Fichier`
//!
//! Trois cases, les mêmes pour un texte, une image, une vidéo ou un audio :
//!
//! - `nom` : le nom complet, par exemple `"test.txt"` ou `"photo.png"`.
//!   C'est du texte. Le dossier et la date n'y sont pas.
//! - `extension` : la partie après le dernier point, par exemple `"txt"` ou `"png"`.
//!   Elle est vide s'il n'y a pas de point. Elle sert à reconnaître le type.
//!   Elle ne remplace pas les octets du contenu.
//! - `contenu` : tous les octets du fichier, dans l'ordre, sans rien enlever.
//!   L'en-tête et la structure interne sont déjà là-dedans. Les séparer casserait
//!   le fichier au moment de le réécrire.
//!
//! `contenu` est un `Vec<u8>` : une liste de nombres. Chaque nombre, un `u8`,
//! va de 0 à 255. Ce sont les 256 symboles. Pour `test.txt`, la liste fait 26
//! nombres : les lettres de `coucou c'est un test ! :)`, puis 10, le saut de ligne.
//!
//! Quand une fonction reçoit `&Fichier`, elle emprunte le tableau : elle peut le
//! lire, pas le garder après avoir retourné. `fichier.contenu.clone()` en fait
//! une copie. `&[u8]` est une vue empruntée sur ces octets. `Vec<u8>` est une
//! liste que la fonction possède et peut donc renvoyer.
//!
//! # Les deux fonctions
//!
//! `chiffrer` reçoit le fichier d'origine et renvoie `Ok(octets_chiffres)`.
//! L'application écrit ces octets dans `nom.enc`, à côté de l'original.
//!
//! `dechiffrer` reçoit le fichier sélectionné, souvent un `.enc`, sous la même
//! forme. Son `nom` est alors celui du fichier chiffré (`test.txt.enc`), et son
//! `contenu` est la suite d'octets chiffrés. La fonction doit renvoyer le
//! `Fichier` d'origine : `nom` vaut `"test.txt"`, `extension` vaut `"txt"`,
//! `contenu` est la liste d'octets initiale. L'application réécrit ce contenu
//! sous ce nom, à côté du fichier chiffré.
//!
//! `Err("message".to_string())` affiche le message dans l'interface.
//! Le mot de passe et les trois réponses sont du texte. Le libellé des questions
//! n'est pas transmis. L'emplacement 1 reste l'emplacement 1.
//!
//! La ligne `let _ = ...` évite un avertissement tant que les valeurs ne servent pas.

/// Nom, extension et octets bruts d'un fichier.
pub struct Fichier {
    pub nom: String,
    pub extension: String,
    pub contenu: Vec<u8>,
}

/// Chiffre le tableau et renvoie les octets à écrire dans le fichier `.enc`.
pub fn chiffrer(
    fichier: &Fichier,
    mot_de_passe: &str,
    reponse_1: &str,
    reponse_2: &str,
    reponse_3: &str,
) -> Result<Vec<u8>, String> {
    let _ = (
        &fichier.nom,
        &fichier.extension,
        &fichier.contenu,
        mot_de_passe,
        reponse_1,
        reponse_2,
        reponse_3,
    );
    Err(
        "Le protocole de chiffrement n'est pas encore écrit. Ouvre src-tauri/src/protocole.rs et remplis la fonction chiffrer."
            .to_string(),
    )
}

/// Déchiffre le fichier sélectionné et renvoie le tableau d'origine.
pub fn dechiffrer(
    fichier: &Fichier,
    mot_de_passe: &str,
    reponse_1: &str,
    reponse_2: &str,
    reponse_3: &str,
) -> Result<Fichier, String> {
    let _ = (
        &fichier.nom,
        &fichier.extension,
        &fichier.contenu,
        mot_de_passe,
        reponse_1,
        reponse_2,
        reponse_3,
    );
    Err(
        "Le protocole de déchiffrement n'est pas encore écrit. Ouvre src-tauri/src/protocole.rs et remplis la fonction dechiffrer."
            .to_string(),
    )
}

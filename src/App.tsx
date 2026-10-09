import { useEffect, useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { QUESTIONS } from "./questions";
import "./App.css";

type Mode = "chiffrer" | "dechiffrer";
type Resultat = { type: "ok" | "err"; message: string };

function nomDuFichier(chemin: string): string {
  const parties = chemin.split(/[/\\]/);
  return parties[parties.length - 1] || chemin;
}

async function ouvrirSelecteur(): Promise<string | null> {
  const choisi = await open({
    multiple: false,
    directory: false,
    title: "Choisir un fichier",
  });
  return typeof choisi === "string" ? choisi : null;
}

function modePour(chemin: string): Mode {
  return nomDuFichier(chemin).toLowerCase().endsWith(".enc")
    ? "dechiffrer"
    : "chiffrer";
}

export default function App() {
  const [chemin, setChemin] = useState<string | null>(null);
  const [mode, setMode] = useState<Mode>("chiffrer");
  const [motDePasse, setMotDePasse] = useState("");
  const [confirmation, setConfirmation] = useState("");
  const [afficherMdp, setAfficherMdp] = useState(false);
  const [questions, setQuestions] = useState(["", "", ""]);
  const [reponses, setReponses] = useState(["", "", ""]);
  const [enCours, setEnCours] = useState(false);
  const [resultat, setResultat] = useState<Resultat | null>(null);

  async function choisirFichier() {
    const choisi = await ouvrirSelecteur();
    if (!choisi) {
      return;
    }
    setChemin(choisi);
    setMode(modePour(choisi));
    setResultat(null);
  }

  useEffect(() => {
    void choisirFichier();
  }, []);

  function changerQuestion(index: number, valeur: string) {
    setQuestions((actuel) =>
      actuel.map((question, i) => (i === index ? valeur : question)),
    );
  }

  function changerReponse(index: number, valeur: string) {
    setReponses((actuel) =>
      actuel.map((reponse, i) => (i === index ? valeur : reponse)),
    );
  }

  async function soumettre(evenement: FormEvent) {
    evenement.preventDefault();
    if (!chemin) {
      return;
    }
    if (motDePasse.trim() === "") {
      setResultat({ type: "err", message: "Écrivez un mot de passe." });
      return;
    }
    if (mode === "chiffrer" && motDePasse !== confirmation) {
      setResultat({
        type: "err",
        message: "La confirmation du mot de passe ne correspond pas.",
      });
      return;
    }
    if (new Set(questions).size !== 3 || questions.some((question) => question === "")) {
      setResultat({
        type: "err",
        message: "Choisissez trois questions différentes.",
      });
      return;
    }
    if (reponses.some((reponse) => reponse.trim() === "")) {
      setResultat({
        type: "err",
        message: "Écrivez une réponse pour chacune des trois questions.",
      });
      return;
    }

    setEnCours(true);
    setResultat(null);
    try {
      const commande = mode === "chiffrer" ? "chiffrer_fichier" : "dechiffrer_fichier";
      const sortie = await invoke<string>(commande, {
        chemin,
        motDePasse,
        reponse1: reponses[0],
        reponse2: reponses[1],
        reponse3: reponses[2],
      });
      setResultat({ type: "ok", message: sortie });
    } catch (erreur) {
      const message =
        typeof erreur === "string" ? erreur : "L'opération a échoué.";
      setResultat({ type: "err", message });
    } finally {
      setEnCours(false);
    }
  }

  return (
    <main className="app">
      <header className="entete">
        <p className="surtitre">En local</p>
        <h1>Chiffrement de fichier</h1>
        <p>
          N'importe quel fichier. Le protocole reçoit son nom, son extension et tous ses octets, de 0 à 255.
        </p>
      </header>

      <section className="carte">
        <div className="fichier">
          {chemin ? (
            <>
              <p className="nom-fichier">{nomDuFichier(chemin)}</p>
              <p className="chemin-fichier">{chemin}</p>
            </>
          ) : (
            <p className="nom-fichier">Aucun fichier sélectionné</p>
          )}
          <button type="button" className="secondaire" onClick={() => void choisirFichier()}>
            {chemin ? "Changer de fichier" : "Choisir un fichier"}
          </button>
        </div>

        {chemin && (
          <form onSubmit={(evenement) => void soumettre(evenement)}>
            <div className="modes" role="group" aria-label="Opération">
              <button
                type="button"
                aria-pressed={mode === "chiffrer"}
                onClick={() => {
                  setMode("chiffrer");
                  setResultat(null);
                }}
              >
                Chiffrer
              </button>
              <button
                type="button"
                aria-pressed={mode === "dechiffrer"}
                onClick={() => {
                  setMode("dechiffrer");
                  setResultat(null);
                }}
              >
                Déchiffrer
              </button>
            </div>

            <label htmlFor="mot-de-passe">Mot de passe</label>
            <input
              id="mot-de-passe"
              type={afficherMdp ? "text" : "password"}
              value={motDePasse}
              autoComplete="new-password"
              required
              onChange={(evenement) => setMotDePasse(evenement.currentTarget.value)}
            />

            {mode === "chiffrer" && (
              <>
                <label htmlFor="confirmation">Confirmer le mot de passe</label>
                <input
                  id="confirmation"
                  type={afficherMdp ? "text" : "password"}
                  value={confirmation}
                  autoComplete="new-password"
                  required
                  onChange={(evenement) => setConfirmation(evenement.currentTarget.value)}
                />
              </>
            )}

            <label className="case">
              <input
                type="checkbox"
                checked={afficherMdp}
                onChange={(evenement) => setAfficherMdp(evenement.currentTarget.checked)}
              />
              Afficher le mot de passe
            </label>

            {questions.map((questionChoisie, index) => (
              <fieldset key={index}>
                <legend>Question {index + 1}</legend>
                <label htmlFor={`question-${index}`}>Question de sécurité</label>
                <select
                  id={`question-${index}`}
                  value={questionChoisie}
                  required
                  onChange={(evenement) => changerQuestion(index, evenement.currentTarget.value)}
                >
                  <option value="">Choisir une question</option>
                  {QUESTIONS.map((question) => (
                    <option
                      key={question}
                      value={question}
                      disabled={questions.some(
                        (prise, autre) => autre !== index && prise === question,
                      )}
                    >
                      {question}
                    </option>
                  ))}
                </select>
                <label htmlFor={`reponse-${index}`}>Réponse</label>
                <input
                  id={`reponse-${index}`}
                  type="text"
                  value={reponses[index]}
                  autoComplete="off"
                  required
                  onChange={(evenement) => changerReponse(index, evenement.currentTarget.value)}
                />
              </fieldset>
            ))}

            <button type="submit" className="principal" disabled={enCours}>
              {enCours
                ? "Opération en cours…"
                : mode === "chiffrer"
                  ? "Chiffrer le fichier"
                  : "Déchiffrer le fichier"}
            </button>
          </form>
        )}

        {resultat && (
          <p className={resultat.type === "ok" ? "resultat ok" : "resultat err"} role="status">
            {resultat.type === "ok"
              ? `Fichier écrit : ${resultat.message}`
              : resultat.message}
          </p>
        )}
      </section>
    </main>
  );
}

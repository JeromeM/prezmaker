import { useState } from "react";
import type { Game, TechInfo } from "../types/api";

interface Props {
  game: Game;
  claudeDescription: string | null;
  onGenerate: (
    game: Game,
    description: string | null,
    installation: string | null,
    techInfo: TechInfo
  ) => void;
  onCancel: () => void;
}

export default function GameExtrasForm({
  game,
  claudeDescription,
  onGenerate,
  onCancel,
}: Props) {
  const [description, setDescription] = useState(
    claudeDescription || game.synopsis || ""
  );
  const [installation, setInstallation] = useState("");
  const [platform, setPlatform] = useState("");
  const [languages, setLanguages] = useState("");
  const [size, setSize] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onGenerate(
      game,
      description || null,
      installation || null,
      { platform, languages, size }
    );
  };

  return (
    <div className="max-w-2xl mx-auto p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold">{game.title}</h2>
        <button
          onClick={onCancel}
          className="text-gray-400 hover:text-white text-sm"
        >
          Annuler
        </button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Description{" "}
            {claudeDescription && (
              <span className="text-green-400">(pré-remplie par Claude)</span>
            )}
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={6}
            className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500 resize-y"
            placeholder="Description du jeu en français..."
          />
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Installation (étapes numérotées, une par ligne)
          </label>
          <textarea
            value={installation}
            onChange={(e) => setInstallation(e.target.value)}
            rows={4}
            className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500 resize-y"
            placeholder={"1. Extraire l'archive\n2. Lancer le setup\n3. Jouer"}
          />
        </div>

        <div className="grid grid-cols-3 gap-3">
          <div>
            <label className="block text-sm text-gray-400 mb-1">
              Plateforme
            </label>
            <input
              type="text"
              value={platform}
              onChange={(e) => setPlatform(e.target.value)}
              className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="PC"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-1">
              Langue(s)
            </label>
            <input
              type="text"
              value={languages}
              onChange={(e) => setLanguages(e.target.value)}
              className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="FR, EN"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-1">Taille</label>
            <input
              type="text"
              value={size}
              onChange={(e) => setSize(e.target.value)}
              className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="10 Go"
            />
          </div>
        </div>

        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
        >
          Générer le BBCode
        </button>
      </form>
    </div>
  );
}

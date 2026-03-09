import { useState, useRef, useEffect } from "react";
import type { Game, TechInfo, TorrentInfo } from "../types/api";

const PLATFORMS = [
  "Windows",
  "macOS",
  "Linux",
  "PlayStation 5",
  "PlayStation 4",
  "Xbox Series X|S",
  "Xbox One",
  "Nintendo Switch",
  "Android",
  "iOS",
];

const LANGUAGES = [
  "Multi",
  "Français",
  "Anglais",
  "Allemand",
  "Espagnol",
  "Italien",
  "Portugais",
  "Russe",
  "Japonais",
  "Chinois",
  "Coréen",
  "Arabe",
  "Polonais",
  "Néerlandais",
  "Suédois",
  "Turc",
];

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
  torrentInfo?: TorrentInfo;
}

function LanguageDropdown({
  selected,
  onChange,
}: {
  selected: string[];
  onChange: (langs: string[]) => void;
}) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const toggle = (lang: string) => {
    if (selected.includes(lang)) {
      onChange(selected.filter((l) => l !== lang));
    } else {
      onChange([...selected, lang]);
    }
  };

  const display = selected.length === 0 ? "Aucune" : selected.join(", ");

  return (
    <div className="relative" ref={ref}>
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500 text-left truncate"
      >
        {display}
      </button>
      {open && (
        <div className="absolute z-10 mt-1 w-full bg-[#16213e] border border-[#2a2a4a] rounded shadow-lg max-h-48 overflow-y-auto">
          {LANGUAGES.map((lang) => (
            <label
              key={lang}
              className="flex items-center gap-2 px-3 py-1.5 hover:bg-[#2a2a4a] cursor-pointer text-sm"
            >
              <input
                type="checkbox"
                checked={selected.includes(lang)}
                onChange={() => toggle(lang)}
                className="accent-blue-500"
              />
              {lang}
            </label>
          ))}
        </div>
      )}
    </div>
  );
}

export default function GameExtrasForm({
  game,
  claudeDescription,
  onGenerate,
  onCancel,
  torrentInfo,
}: Props) {
  const [description, setDescription] = useState(
    claudeDescription || game.synopsis || ""
  );
  const [installation, setInstallation] = useState("");
  const [platform, setPlatform] = useState(
    torrentInfo ? "Windows" : "Windows"
  );
  const [selectedLanguages, setSelectedLanguages] = useState<string[]>(
    torrentInfo?.parsed.language
      ? torrentInfo.parsed.language.split(/,\s*/)
      : ["Multi"]
  );
  const [size, setSize] = useState(torrentInfo?.size_formatted || "");
  const [installSize, setInstallSize] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const languages = selectedLanguages.join(", ");
    onGenerate(
      game,
      description || null,
      installation || null,
      { platform, languages, size, install_size: installSize }
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

        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Plateforme
          </label>
          <div className="flex flex-wrap gap-2">
            {PLATFORMS.map((p) => (
              <label
                key={p}
                className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-sm cursor-pointer border transition-colors ${
                  platform === p
                    ? "bg-blue-600/30 border-blue-500 text-blue-300"
                    : "bg-[#16213e] border-[#2a2a4a] text-gray-400 hover:border-gray-500"
                }`}
              >
                <input
                  type="radio"
                  name="platform"
                  value={p}
                  checked={platform === p}
                  onChange={() => setPlatform(p)}
                  className="hidden"
                />
                {p}
              </label>
            ))}
          </div>
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">
            Langue(s)
          </label>
          <LanguageDropdown
            selected={selectedLanguages}
            onChange={setSelectedLanguages}
          />
        </div>

        <div className="grid grid-cols-2 gap-3">
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
          <div>
            <label className="block text-sm text-gray-400 mb-1">Taille d'installation</label>
            <input
              type="text"
              value={installSize}
              onChange={(e) => setInstallSize(e.target.value)}
              className="w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="20 Go"
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

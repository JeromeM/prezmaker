import { useState, useRef, useEffect } from "react";
import type { Game, TechInfo, SystemReqs, TorrentInfo } from "../types/api";

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

const EMPTY_REQS: SystemReqs = { os: "", cpu: "", ram: "", gpu: "", storage: "" };

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

const inputClass =
  "w-full bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500";

function ReqsFields({
  label,
  reqs,
  onChange,
}: {
  label: string;
  reqs: SystemReqs;
  onChange: (reqs: SystemReqs) => void;
}) {
  const set = (field: keyof SystemReqs, value: string) =>
    onChange({ ...reqs, [field]: value });

  return (
    <div className="space-y-2">
      <p className="text-sm text-gray-300 font-medium">{label}</p>
      <div className="grid grid-cols-2 gap-2">
        <input
          type="text"
          value={reqs.os}
          onChange={(e) => set("os", e.target.value)}
          className={inputClass}
          placeholder="OS (ex: Windows 10 64-bit)"
        />
        <input
          type="text"
          value={reqs.cpu}
          onChange={(e) => set("cpu", e.target.value)}
          className={inputClass}
          placeholder="CPU (ex: Intel i5-3570K)"
        />
        <input
          type="text"
          value={reqs.ram}
          onChange={(e) => set("ram", e.target.value)}
          className={inputClass}
          placeholder="RAM (ex: 8 Go)"
        />
        <input
          type="text"
          value={reqs.gpu}
          onChange={(e) => set("gpu", e.target.value)}
          className={inputClass}
          placeholder="GPU (ex: GTX 970)"
        />
        <input
          type="text"
          value={reqs.storage}
          onChange={(e) => set("storage", e.target.value)}
          className={inputClass}
          placeholder="Stockage (ex: 70 Go SSD)"
        />
      </div>
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
  const hasPrefilledReqs = !!(game.min_reqs || game.rec_reqs);
  const [showReqs, setShowReqs] = useState(hasPrefilledReqs);
  const [minReqs, setMinReqs] = useState<SystemReqs>(game.min_reqs ?? { ...EMPTY_REQS });
  const [recReqs, setRecReqs] = useState<SystemReqs>(game.rec_reqs ?? { ...EMPTY_REQS });

  const isReqsEmpty = (r: SystemReqs) =>
    !r.os && !r.cpu && !r.ram && !r.gpu && !r.storage;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const languages = selectedLanguages.join(", ");
    const updatedGame = {
      ...game,
      min_reqs: isReqsEmpty(minReqs) ? null : minReqs,
      rec_reqs: isReqsEmpty(recReqs) ? null : recReqs,
    };
    onGenerate(
      updatedGame,
      description || null,
      installation || null,
      { platform, languages, size, install_size: installSize }
    );
  };

  return (
    <div className="max-w-2xl mx-auto p-6 overflow-y-auto">
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
            Description
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
            Installation (etapes numerotees, une par ligne)
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
              className={inputClass}
              placeholder="10 Go"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-1">Taille d'installation</label>
            <input
              type="text"
              value={installSize}
              onChange={(e) => setInstallSize(e.target.value)}
              className={inputClass}
              placeholder="20 Go"
            />
          </div>
        </div>

        <div>
          <button
            type="button"
            onClick={() => setShowReqs(!showReqs)}
            className="flex items-center gap-2 text-sm text-gray-400 hover:text-white transition-colors"
          >
            <svg
              className={`w-4 h-4 transition-transform ${showReqs ? "rotate-90" : ""}`}
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              strokeWidth={2}
            >
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
            </svg>
            Configuration requise (min / recommandee)
          </button>
          {showReqs && (
            <div className="mt-3 space-y-4 pl-2 border-l-2 border-[#2a2a4a]">
              <ReqsFields label="Minimum" reqs={minReqs} onChange={setMinReqs} />
              <ReqsFields label="Recommandee" reqs={recReqs} onChange={setRecReqs} />
            </div>
          )}
        </div>

        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
        >
          Generer le BBCode
        </button>
      </form>
    </div>
  );
}

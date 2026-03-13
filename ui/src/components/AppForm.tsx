import { useState } from "react";
import type { AppPayload } from "../types/api";

interface Props {
  onGenerate: (payload: AppPayload) => void;
  onCancel: () => void;
}

export default function AppForm({ onGenerate, onCancel }: Props) {
  const [name, setName] = useState("");
  const [version, setVersion] = useState("");
  const [developer, setDeveloper] = useState("");
  const [description, setDescription] = useState("");
  const [website, setWebsite] = useState("");
  const [license, setLicense] = useState("");
  const [platforms, setPlatforms] = useState("");
  const [logoUrl, setLogoUrl] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    onGenerate({
      name: name.trim(),
      version: version || null,
      developer: developer || null,
      description: description || null,
      website: website || null,
      license: license || null,
      platforms: platforms
        ? platforms.split(",").map((s) => s.trim()).filter(Boolean)
        : [],
      logo_url: logoUrl || null,
    });
  };

  return (
    <div className="max-w-2xl mx-auto p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-xl font-semibold">Nouvelle application</h2>
        <button
          onClick={onCancel}
          className="text-fg-muted hover:text-fg-bright text-sm"
        >
          Annuler
        </button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-3">
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="block text-sm text-fg-muted mb-1">Nom *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="VLC"
              required
            />
          </div>
          <div>
            <label className="block text-sm text-fg-muted mb-1">Version</label>
            <input
              type="text"
              value={version}
              onChange={(e) => setVersion(e.target.value)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="3.0.20"
            />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="block text-sm text-fg-muted mb-1">
              Développeur
            </label>
            <input
              type="text"
              value={developer}
              onChange={(e) => setDeveloper(e.target.value)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="VideoLAN"
            />
          </div>
          <div>
            <label className="block text-sm text-fg-muted mb-1">Licence</label>
            <input
              type="text"
              value={license}
              onChange={(e) => setLicense(e.target.value)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              placeholder="GPLv2"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm text-fg-muted mb-1">Site web</label>
          <input
            type="text"
            value={website}
            onChange={(e) => setWebsite(e.target.value)}
            className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
            placeholder="https://www.videolan.org"
          />
        </div>

        <div>
          <label className="block text-sm text-fg-muted mb-1">
            Plateformes (séparées par des virgules)
          </label>
          <input
            type="text"
            value={platforms}
            onChange={(e) => setPlatforms(e.target.value)}
            className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
            placeholder="Windows, macOS, Linux"
          />
        </div>

        <div>
          <label className="block text-sm text-fg-muted mb-1">URL logo</label>
          <input
            type="text"
            value={logoUrl}
            onChange={(e) => setLogoUrl(e.target.value)}
            className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
            placeholder="https://..."
          />
        </div>

        <div>
          <label className="block text-sm text-fg-muted mb-1">
            Description
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={4}
            className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500 resize-y"
            placeholder="Description de l'application..."
          />
        </div>

        <button
          type="submit"
          disabled={!name.trim()}
          className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
        >
          Générer le BBCode
        </button>
      </form>
    </div>
  );
}

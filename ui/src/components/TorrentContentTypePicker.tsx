import type { ContentType, TorrentInfo } from "../types/api";

interface Props {
  torrentInfo: TorrentInfo;
  onConfirm: (contentType: ContentType, torrentInfo: TorrentInfo) => void;
  onCancel: () => void;
}

export default function TorrentContentTypePicker({
  torrentInfo,
  onConfirm,
  onCancel,
}: Props) {
  const options: { value: ContentType; label: string }[] = [
    { value: "film", label: "Film" },
    { value: "serie", label: "Série" },
    { value: "jeu", label: "Jeu" },
  ];

  return (
    <div className="max-w-lg mx-auto p-6">
      <h2 className="text-xl font-semibold mb-2">Type de contenu non détecté</h2>
      <p className="text-fg-muted text-sm mb-1">
        Torrent : <span className="text-fg-bright">{torrentInfo.meta.name}</span>
      </p>
      <p className="text-fg-muted text-sm mb-4">
        Taille : {torrentInfo.size_formatted}
        {torrentInfo.parsed.title && (
          <> &middot; Titre détecté : <span className="text-fg-bright">{torrentInfo.parsed.title}</span></>
        )}
      </p>

      <p className="text-sm text-fg mb-3">
        Quel type de contenu est-ce ?
      </p>

      <div className="flex gap-3">
        {options.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onConfirm(opt.value, torrentInfo)}
            className="flex-1 bg-input hover:bg-blue-600 border border-edge hover:border-blue-500 text-white px-4 py-3 rounded text-sm font-medium transition-colors"
          >
            {opt.label}
          </button>
        ))}
      </div>

      <button
        onClick={onCancel}
        className="mt-4 text-fg-muted hover:text-fg-bright text-sm transition-colors"
      >
        Annuler
      </button>
    </div>
  );
}
